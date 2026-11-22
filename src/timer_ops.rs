//! SigmaOS systemd timer operations module.
//!
//! This module provides actual systemd timer operations using systemctl and systemd-run.
//!
//! # Example
//!
//! ```
//! use sigma_system::timer_ops::*;
//!
//! let mut timer_ops = TimerOps::new();
//! if let Ok(timers) = timer_ops.list_timers() {
//!     println!("Found {} timers", timers.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in timer operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TimerError {
    /// Timer not found.
    TimerNotFound(String),
    /// Failed to get timer status.
    StatusError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for TimerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerError::TimerNotFound(timer) => write!(f, "Timer '{}' not found", timer),
            TimerError::StatusError(msg) => write!(f, "Failed to get timer status: {}", msg),
            TimerError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            TimerError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for TimerError {}

/// Timer information.
#[derive(Debug, Clone)]
pub struct TimerInfo {
    /// Timer name.
    pub name: String,
    /// Next run time.
    pub next_run: String,
    /// Last run time.
    pub last_run: String,
    /// Time passed since last run.
    pub passed: String,
    /// Unit name.
    pub unit: String,
    /// Activated status.
    pub active: bool,
}

/// Timer operations manager.
#[derive(Debug)]
pub struct TimerOps {
    /// Whether running as root.
    is_root: bool,
}

impl TimerOps {
    /// Create a new timer operations manager.
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

    /// List all timers.
    pub fn list_timers(&self) -> Result<Vec<TimerInfo>, TimerError> {
        let output = Command::new("systemctl").arg("list-timers").arg("--all").arg("--no-pager").output()
            .map_err(|e| TimerError::IoError(format!("Failed to list timers: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let mut timers = Vec::new();
        
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                timers.push(TimerInfo {
                    name: parts[0].to_string(),
                    next_run: parts[1].to_string(),
                    last_run: parts[2].to_string(),
                    passed: parts[3].to_string(),
                    unit: parts[4].to_string(),
                    active: parts.len() > 5 && parts[5] == "active",
                });
            }
        }
        
        Ok(timers)
    }

    /// Get timer status.
    pub fn get_timer_status(&self, timer: &str) -> Result<TimerInfo, TimerError> {
        let output = Command::new("systemctl").arg("status").arg(timer).arg("--no-pager").output()
            .map_err(|e| TimerError::IoError(format!("Failed to get timer status: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        
        let mut name = timer.to_string();
        let mut next_run = "unknown".to_string();
        let mut last_run = "unknown".to_string();
        let mut passed = "unknown".to_string();
        let mut unit = "unknown".to_string();
        let mut active = false;
        
        for line in content.lines() {
            if line.contains("Loaded:") {
                active = line.contains("loaded");
            } else if line.contains("Trigger:") {
                if let Some(val) = line.split(':').nth(1) {
                    next_run = val.trim().to_string();
                }
            }
        }
        
        Ok(TimerInfo {
            name,
            next_run,
            last_run,
            passed,
            unit,
            active,
        })
    }

    /// Start a timer.
    pub fn start_timer(&self, timer: &str) -> Result<(), TimerError> {
        if !self.is_root {
            return Err(TimerError::PermissionDenied("Must be root to start timer".to_string()));
        }

        let result = Command::new("systemctl").arg("start").arg(timer).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(TimerError::StatusError("systemctl start failed".to_string())),
            Err(e) => Err(TimerError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Stop a timer.
    pub fn stop_timer(&self, timer: &str) -> Result<(), TimerError> {
        if !self.is_root {
            return Err(TimerError::PermissionDenied("Must be root to stop timer".to_string()));
        }

        let result = Command::new("systemctl").arg("stop").arg(timer).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(TimerError::StatusError("systemctl stop failed".to_string())),
            Err(e) => Err(TimerError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Enable a timer.
    pub fn enable_timer(&self, timer: &str) -> Result<(), TimerError> {
        if !self.is_root {
            return Err(TimerError::PermissionDenied("Must be root to enable timer".to_string()));
        }

        let result = Command::new("systemctl").arg("enable").arg(timer).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(TimerError::StatusError("systemctl enable failed".to_string())),
            Err(e) => Err(TimerError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Disable a timer.
    pub fn disable_timer(&self, timer: &str) -> Result<(), TimerError> {
        if !self.is_root {
            return Err(TimerError::PermissionDenied("Must be root to disable timer".to_string()));
        }

        let result = Command::new("systemctl").arg("disable").arg(timer).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(TimerError::StatusError("systemctl disable failed".to_string())),
            Err(e) => Err(TimerError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Get active timers count.
    pub fn get_active_timers_count(&self) -> Result<usize, TimerError> {
        let timers = self.list_timers()?;
        Ok(timers.iter().filter(|t| t.active).count())
    }

    /// Get timer summary.
    pub fn get_timer_summary(&self) -> Result<String, TimerError> {
        let timers = self.list_timers()?;
        let active_count = self.get_active_timers_count()?;
        
        Ok(format!(
            "Total timers: {}\nActive timers: {}",
            timers.len(),
            active_count
        ))
    }
}

impl Default for TimerOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let timer_ops = TimerOps::new();
        let _ = timer_ops.is_root();
    }

    #[test]
    fn test_list_timers() {
        let timer_ops = TimerOps::new();
        if let Ok(timers) = timer_ops.list_timers() {
            let _ = timers;
        }
    }

    #[test]
    fn test_get_active_timers_count() {
        let timer_ops = TimerOps::new();
        if let Ok(count) = timer_ops.get_active_timers_count() {
            assert!(count >= 0);
        }
    }

    #[test]
    fn test_get_timer_summary() {
        let timer_ops = TimerOps::new();
        if let Ok(summary) = timer_ops.get_timer_summary() {
            assert!(!summary.is_empty());
        }
    }
}
