//! SigmaOS log manager operations module.
//!
//! This module provides log management operations including:
//! - Log rotation
//! - Log cleanup
//! - Log analysis
//!
//! Follows Single Responsibility Principle: only manages log operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::logmgr_ops::*;
//!
//! let logmgr = LogMgrOps::new();
//! if let Ok(_) = logmgr.rotate_logs() {
//!     println!("Logs rotated");
//! }
//! ```

use std::process::Command;

/// Log manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum LogMgrError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for LogMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            LogMgrError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for LogMgrError {}

/// Log manager operations manager.
#[derive(Debug)]
pub struct LogMgrOps;

impl LogMgrOps {
    pub fn new() -> Self { Self }

    /// Rotate logs.
    pub fn rotate_logs(&self) -> Result<(), LogMgrError> {
        let output = Command::new("logrotate")
            .arg("-f")
            .arg("/etc/logrotate.conf")
            .output()
            .map_err(|e| LogMgrError::CommandError(format!("Failed to run logrotate: {}", e)))?;

        if !output.status.success() {
            return Err(LogMgrError::CommandError("logrotate failed".to_string()));
        }

        Ok(())
    }

    /// Clean old logs.
    pub fn clean_logs(&self, days: u32) -> Result<(), LogMgrError> {
        let output = Command::new("journalctl")
            .args(&["--vacuum-time", &format!("{}d", days)])
            .output()
            .map_err(|e| LogMgrError::CommandError(format!("Failed to run journalctl: {}", e)))?;

        if !output.status.success() {
            return Err(LogMgrError::CommandError("journalctl failed".to_string()));
        }

        Ok(())
    }
}

impl Default for LogMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let logmgr = LogMgrOps::new();
        let _ = logmgr;
    }
}
