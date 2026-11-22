//! SigmaOS logging operations module.
//!
//! This module provides actual logging operations using journalctl.
//!
//! # Example
//!
//! ```
//! use sigma_system::log_ops::*;
//!
//! let mut log_ops = LogOps::new();
//! if let Ok(logs) = log_ops.get_recent_logs(10) {
//!     println!("Retrieved {} log entries", logs.len());
//! }
//! ```

use std::process::Command;
use std::collections::BTreeMap;

/// Errors that can occur in logging operations.
#[derive(Debug, Clone, PartialEq)]
pub enum LogError {
    /// Log not found.
    LogNotFound(String),
    /// Permission denied.
    PermissionDenied(String),
    /// Invalid log level.
    InvalidLogLevel(String),
    /// Journal not available.
    JournalNotAvailable(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for LogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogError::LogNotFound(name) => write!(f, "Log '{}' not found", name),
            LogError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            LogError::InvalidLogLevel(level) => write!(f, "Invalid log level '{}'", level),
            LogError::JournalNotAvailable(msg) => write!(f, "Journal not available: {}", msg),
            LogError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for LogError {}

/// Log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Emergency.
    Emergency = 0,
    /// Alert.
    Alert = 1,
    /// Critical.
    Critical = 2,
    /// Error.
    Error = 3,
    /// Warning.
    Warning = 4,
    /// Notice.
    Notice = 5,
    /// Info.
    Info = 6,
    /// Debug.
    Debug = 7,
}

impl LogLevel {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Emergency => "emerg",
            LogLevel::Alert => "alert",
            LogLevel::Critical => "crit",
            LogLevel::Error => "err",
            LogLevel::Warning => "warning",
            LogLevel::Notice => "notice",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Result<Self, LogError> {
        match s.to_lowercase().as_str() {
            "emerg" | "emergency" => Ok(LogLevel::Emergency),
            "alert" => Ok(LogLevel::Alert),
            "crit" | "critical" => Ok(LogLevel::Critical),
            "err" | "error" => Ok(LogLevel::Error),
            "warning" | "warn" => Ok(LogLevel::Warning),
            "notice" => Ok(LogLevel::Notice),
            "info" | "informational" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            _ => Err(LogError::InvalidLogLevel(s.to_string())),
        }
    }
}

/// Log entry.
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Timestamp.
    pub timestamp: String,
    /// Hostname.
    pub hostname: String,
    /// Service name.
    pub service: String,
    /// Process ID.
    pub pid: u32,
    /// Log level.
    pub level: LogLevel,
    /// Log message.
    pub message: String,
    /// Priority.
    pub priority: u8,
}

/// Log operations manager.
#[derive(Debug)]
pub struct LogOps {
    /// Whether journalctl is available.
    journalctl_available: bool,
    /// Whether running as root.
    is_root: bool,
    /// Cached logs.
    logs: BTreeMap<String, Vec<LogEntry>>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl LogOps {
    /// Create a new log operations manager.
    pub fn new() -> Self {
        let journalctl_available = Command::new("journalctl").arg("--version").output().is_ok();
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            journalctl_available,
            is_root,
            logs: BTreeMap::new(),
            last_update: std::time::Instant::now(),
        }
    }

    /// Check if journalctl is available.
    pub fn is_journalctl_available(&self) -> bool {
        self.journalctl_available
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Get recent logs.
    pub fn get_recent_logs(&mut self, count: u32) -> Result<Vec<LogEntry>, LogError> {
        self.refresh_logs()?;
        let logs = self.logs.get("all").cloned().unwrap_or_default();
        Ok(logs.into_iter().take(count as usize).collect())
    }

    /// Get logs by service.
    pub fn get_service_logs(&mut self, service: &str, count: u32) -> Result<Vec<LogEntry>, LogError> {
        self.refresh_logs()?;
        let logs = self.logs.get(service).cloned().unwrap_or_default();
        Ok(logs.into_iter().take(count as usize).collect())
    }

    /// Get logs by level.
    pub fn get_logs_by_level(&mut self, level: LogLevel, count: u32) -> Result<Vec<LogEntry>, LogError> {
        self.refresh_logs()?;
        let logs = self.logs.get("all").cloned().unwrap_or_default();
        Ok(logs.into_iter()
            .filter(|log| log.level == level)
            .take(count as usize)
            .collect())
    }

    /// Search logs.
    pub fn search_logs(&mut self, query: &str, count: u32) -> Result<Vec<LogEntry>, LogError> {
        self.refresh_logs()?;
        let logs = self.logs.get("all").cloned().unwrap_or_default();
        Ok(logs.into_iter()
            .filter(|log| log.message.contains(query))
            .take(count as usize)
            .collect())
    }

    /// Get log statistics.
    pub fn get_log_stats(&mut self) -> Result<BTreeMap<String, usize>, LogError> {
        self.refresh_logs()?;
        let logs = self.logs.get("all").cloned().unwrap_or_default();
        
        let mut stats = BTreeMap::new();
        for log in logs {
            *stats.entry(log.service.clone()).or_insert(0) += 1;
        }
        
        Ok(stats)
    }

    /// Refresh logs from journalctl.
    fn refresh_logs(&mut self) -> Result<(), LogError> {
        // Only refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() < 1 {
            return Ok(());
        }

        self.logs.clear();

        if !self.journalctl_available {
            return Ok(());
        }

        // Get recent logs (last 100 entries) in short format
        let result = Command::new("journalctl")
            .args(&["-n", "100", "--no-pager", "-o", "short-iso"])
            .output();

        if let Ok(output) = result {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                
                for line in stdout.lines() {
                    if let Some(entry) = self.parse_log_line(line) {
                        let service = entry.service.clone();
                        self.logs.entry("all".to_string()).or_default().push(entry.clone());
                        self.logs.entry(service).or_default().push(entry);
                    }
                }
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Parse a journalctl short-iso log line.
    /// Format: "2024-01-01T12:00:00+0000 hostname service[pid]: message"
    fn parse_log_line(&self, line: &str) -> Option<LogEntry> {
        let parts: Vec<&str> = line.splitn(4, ' ').collect();
        if parts.len() < 4 {
            return None;
        }

        let timestamp = parts[0].to_string();
        let hostname = parts[1].to_string();
        
        // Extract service and pid from "service[pid]:"
        let service_part = parts[2];
        let (service, pid) = if let Some(bracket_pos) = service_part.find('[') {
            let svc = &service_part[..bracket_pos];
            let pid_str = service_part[bracket_pos+1..].trim_end_matches(']').trim_end_matches(':');
            (svc.to_string(), pid_str.parse().unwrap_or(0))
        } else {
            (service_part.trim_end_matches(':').to_string(), 0)
        };

        let message = parts[3].to_string();

        Some(LogEntry {
            timestamp,
            hostname,
            service,
            pid,
            level: LogLevel::Info,
            message,
            priority: 6,
        })
    }

    /// Clear logs.
    pub fn clear_logs(&mut self) -> Result<(), LogError> {
        if !self.is_root {
            return Err(LogError::PermissionDenied("Must be root to clear logs".to_string()));
        }

        let result = Command::new("journalctl")
            .args(&["--rotate"])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.logs.clear();
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(LogError::IoError(format!("Failed to clear logs: {}", stderr)))
                }
            }
            Err(e) => Err(LogError::IoError(format!("Failed to run journalctl: {}", e))),
        }
    }

    /// Vacuum logs (delete old logs).
    pub fn vacuum_logs(&mut self, days: u32) -> Result<(), LogError> {
        if !self.is_root {
            return Err(LogError::PermissionDenied("Must be root to vacuum logs".to_string()));
        }

        let result = Command::new("journalctl")
            .args(&["--vacuum-time", &format!("{}d", days)])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(LogError::IoError(format!("Failed to vacuum logs: {}", stderr)))
                }
            }
            Err(e) => Err(LogError::IoError(format!("Failed to run journalctl: {}", e))),
        }
    }
}

impl LogLevel {
    /// Create from priority number.
    pub fn from(priority: u8) -> Self {
        match priority {
            0 => LogLevel::Emergency,
            1 => LogLevel::Alert,
            2 => LogLevel::Critical,
            3 => LogLevel::Error,
            4 => LogLevel::Warning,
            5 => LogLevel::Notice,
            6 => LogLevel::Info,
            _ => LogLevel::Debug,
        }
    }
}

impl Default for LogOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mgr = LogOps::new();
        let _ = mgr.is_journalctl_available();
        let _ = mgr.is_root();
    }

    #[test]
    fn test_log_level_parse() {
        assert_eq!(LogLevel::parse("info").unwrap(), LogLevel::Info);
        assert_eq!(LogLevel::parse("error").unwrap(), LogLevel::Error);
        assert_eq!(LogLevel::parse("debug").unwrap(), LogLevel::Debug);
        assert!(LogLevel::parse("invalid").is_err());
    }

    #[test]
    fn test_get_recent_logs() {
        let mut mgr = LogOps::new();
        if let Ok(logs) = mgr.get_recent_logs(10) {
            let _ = logs;
        }
    }

    #[test]
    fn test_log_level_from_priority() {
        assert_eq!(LogLevel::from(0), LogLevel::Emergency);
        assert_eq!(LogLevel::from(3), LogLevel::Error);
        assert_eq!(LogLevel::from(6), LogLevel::Info);
        assert_eq!(LogLevel::from(7), LogLevel::Debug);
    }
}
