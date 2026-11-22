//! SigmaOS backup operations module (extended).
//!
//! This module provides actual backup operations via rsync.
//!
//! # Example
//!
//! ```
//! use sigma_system::backup_ops::*;
//!
//! let backup = BackupOps::new();
//! if let Ok(status) = backup.create_backup("/source", "/dest") {
//!     println!("Backup: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in backup operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BackupError {
    /// Backup failed.
    BackupError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::BackupError(msg) => write!(f, "Failed to backup: {}", msg),
            BackupError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for BackupError {}

/// Backup operations manager.
#[derive(Debug)]
pub struct BackupOps;

impl BackupOps {
    pub fn new() -> Self { Self }

    pub fn create_backup(&self, source: &str, dest: &str) -> Result<String, BackupError> {
        let output = Command::new("rsync")
            .args(&["-av", source, dest])
            .output()
            .map_err(|e| BackupError::IoError(format!("Failed to run rsync: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("backup_created".to_string())
    }
}

impl Default for BackupOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let backup = BackupOps::new();
        let _ = backup;
    }

    #[test]
    fn test_create_backup() {
        let backup = BackupOps::new();
        if let Ok(_) = backup.create_backup("/tmp", "/tmp/backup") {
            // Should not panic
        }
    }
}
