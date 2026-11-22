//! SigmaOS backup manager operations module.
//!
//! This module provides backup management operations including:
//! - System backup
//! - System restore
//! - Backup scheduling
//!
//! Follows Single Responsibility Principle: only manages backup operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::backupmgr_ops::*;
//!
//! let backupmgr = BackupMgrOps::new();
//! if let Ok(_) = backupmgr.create_backup("/backup") {
//!     println!("Backup created");
//! }
//! ```

use std::process::Command;

/// Backup manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum BackupMgrError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for BackupMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            BackupMgrError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for BackupMgrError {}

/// Backup manager operations manager.
#[derive(Debug)]
pub struct BackupMgrOps;

impl BackupMgrOps {
    pub fn new() -> Self { Self }

    /// Create system backup.
    pub fn create_backup(&self, destination: &str) -> Result<(), BackupMgrError> {
        let output = Command::new("rsync")
            .args(&["-av", "/", destination])
            .output()
            .map_err(|e| BackupMgrError::CommandError(format!("Failed to run rsync: {}", e)))?;

        if !output.status.success() {
            return Err(BackupMgrError::CommandError("rsync failed".to_string()));
        }

        Ok(())
    }

    /// Restore from backup.
    pub fn restore_backup(&self, source: &str) -> Result<(), BackupMgrError> {
        let output = Command::new("rsync")
            .args(&["-av", source, "/"])
            .output()
            .map_err(|e| BackupMgrError::CommandError(format!("Failed to run rsync: {}", e)))?;

        if !output.status.success() {
            return Err(BackupMgrError::CommandError("rsync failed".to_string()));
        }

        Ok(())
    }
}

impl Default for BackupMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let backupmgr = BackupMgrOps::new();
        let _ = backupmgr;
    }
}
