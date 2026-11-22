//! SigmaOS log rotation module.
//!
//! This module provides actual log rotation operations using logrotate.
//!
//! # Example
//!
//! ```
//! use sigma_system::logrotate_ops::*;
//!
//! let mut logrotate = LogrotateOps::new();
//! if let Ok(_) = logrotate.rotate_logs() {
//!     println!("Logs rotated successfully");
//! }
//! ```

use std::fs;
use std::path::Path;
use std::process::Command;

/// Errors that can occur in log rotation operations.
#[derive(Debug, Clone, PartialEq)]
pub enum LogrotateError {
    /// Logrotate not found.
    LogrotateNotFound,
    /// Failed to rotate logs.
    RotateError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for LogrotateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogrotateError::LogrotateNotFound => write!(f, "logrotate not found"),
            LogrotateError::RotateError(msg) => write!(f, "Failed to rotate logs: {}", msg),
            LogrotateError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            LogrotateError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for LogrotateError {}

/// Log rotation manager.
#[derive(Debug)]
pub struct LogrotateOps {
    /// Whether running as root.
    is_root: bool,
}

impl LogrotateOps {
    /// Create a new log rotation manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self { is_root }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Check if logrotate is available.
    pub fn is_logrotate_available(&self) -> bool {
        Command::new("logrotate").arg("--version").output().is_ok()
    }

    /// Rotate all logs.
    pub fn rotate_logs(&self) -> Result<(), LogrotateError> {
        if !self.is_root {
            return Err(LogrotateError::PermissionDenied("Must be root to rotate logs".to_string()));
        }

        if !self.is_logrotate_available() {
            return Err(LogrotateError::LogrotateNotFound);
        }

        let result = Command::new("logrotate")
            .arg("-f")
            .arg("/etc/logrotate.conf")
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(LogrotateError::RotateError("logrotate command failed".to_string())),
            Err(e) => Err(LogrotateError::IoError(format!("Failed to run logrotate: {}", e))),
        }
    }

    /// Rotate specific log file.
    pub fn rotate_log_file(&self, log_file: &str) -> Result<(), LogrotateError> {
        if !self.is_root {
            return Err(LogrotateError::PermissionDenied("Must be root to rotate logs".to_string()));
        }

        if !self.is_logrotate_available() {
            return Err(LogrotateError::LogrotateNotFound);
        }

        let result = Command::new("logrotate")
            .args(&["-f", log_file])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(LogrotateError::RotateError("logrotate command failed".to_string())),
            Err(e) => Err(LogrotateError::IoError(format!("Failed to run logrotate: {}", e))),
        }
    }

    /// Get log file size.
    pub fn get_log_size(&self, log_file: &str) -> Result<u64, LogrotateError> {
        let metadata = fs::metadata(log_file)
            .map_err(|e| LogrotateError::IoError(format!("Failed to get metadata: {}", e)))?;
        
        Ok(metadata.len())
    }

    /// List large log files.
    pub fn list_large_logs(&self, directory: &str, max_size_mb: u64) -> Result<Vec<String>, LogrotateError> {
        let dir_path = Path::new(directory);
        if !dir_path.exists() {
            return Err(LogrotateError::IoError("Directory not found".to_string()));
        }

        let entries = fs::read_dir(dir_path)
            .map_err(|e| LogrotateError::IoError(format!("Failed to read directory: {}", e)))?;

        let mut large_logs = Vec::new();
        let max_size_bytes = max_size_mb * 1024 * 1024;

        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.len() > max_size_bytes {
                    if let Ok(name) = entry.file_name().into_string() {
                        large_logs.push(name);
                    }
                }
            }
        }

        Ok(large_logs)
    }
}

impl Default for LogrotateOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let logrotate = LogrotateOps::new();
        let _ = logrotate.is_root();
    }

    #[test]
    fn test_is_logrotate_available() {
        let logrotate = LogrotateOps::new();
        let _ = logrotate.is_logrotate_available();
    }

    #[test]
    fn test_get_log_size() {
        let logrotate = LogrotateOps::new();
        if let Ok(_) = logrotate.get_log_size("/var/log/syslog") {
            // Should not panic
        }
    }
}
