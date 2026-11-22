//! SigmaOS time synchronization operations module.
//!
//! This module provides actual time synchronization operations using NTP/timedatectl.
//!
//! # Example
//!
//! ```
//! use sigma_system::timesync_ops::*;
//!
//! let timesync = TimeSyncOps::new();
//! if let Ok(status) = timesync.get_ntp_status() {
//!     println!("NTP status: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in time sync operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TimeSyncError {
    /// Time sync failed.
    SyncError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for TimeSyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeSyncError::SyncError(msg) => write!(f, "Failed to sync time: {}", msg),
            TimeSyncError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for TimeSyncError {}

/// Time synchronization manager.
#[derive(Debug)]
pub struct TimeSyncOps;

impl TimeSyncOps {
    pub fn new() -> Self { Self }

    pub fn get_ntp_status(&self) -> Result<String, TimeSyncError> {
        if Command::new("timedatectl").arg("status").output().is_ok() {
            let output = Command::new("timedatectl")
                .args(&["status"])
                .output()
                .map_err(|e| TimeSyncError::IoError(format!("Failed to run timedatectl: {}", e)))?;

            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("System clock synchronized") {
                    if line.contains("yes") {
                        return Ok("synchronized".to_string());
                    } else {
                        return Ok("unsynchronized".to_string());
                    }
                }
            }
        }
        Ok("unknown".to_string())
    }

    pub fn sync_ntp(&self) -> Result<(), TimeSyncError> {
        let result = Command::new("timedatectl")
            .args(&["set-ntp", "true"])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(TimeSyncError::SyncError("timedatectl failed".to_string())),
            Err(e) => Err(TimeSyncError::IoError(format!("Failed to run timedatectl: {}", e))),
        }
    }
}

impl Default for TimeSyncOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let timesync = TimeSyncOps::new();
        let _ = timesync;
    }

    #[test]
    fn test_get_ntp_status() {
        let timesync = TimeSyncOps::new();
        if let Ok(_) = timesync.get_ntp_status() {
            // Should not panic
        }
    }
}
