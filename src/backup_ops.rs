//! SigmaOS backup operations module.
//!
//! This module provides actual backup operations using rsync and tar.
//!
//! # Example
//!
//! ```
//! use sigma_system::backup_ops::*;
//!
//! let mut backup_ops = BackupOps::new();
//! if let Ok(_) = backup_ops.create_backup("/home/user", "/backup") {
//!     println!("Backup created successfully");
//! }
//! ```

use std::fs;
use std::path::Path;
use std::process::Command;

/// Errors that can occur in backup operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BackupError {
    /// Source not found.
    SourceNotFound(String),
    /// Destination not writable.
    DestinationNotWritable(String),
    /// Backup failed.
    BackupFailed(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::SourceNotFound(path) => write!(f, "Source '{}' not found", path),
            BackupError::DestinationNotWritable(path) => write!(f, "Destination '{}' not writable", path),
            BackupError::BackupFailed(msg) => write!(f, "Backup failed: {}", msg),
            BackupError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            BackupError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for BackupError {}

/// Backup operations manager.
#[derive(Debug)]
pub struct BackupOps {
    /// Whether running as root.
    is_root: bool,
}

impl BackupOps {
    /// Create a new backup operations manager.
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

    /// Create a backup using rsync.
    pub fn create_backup(&self, source: &str, destination: &str) -> Result<(), BackupError> {
        let source_path = Path::new(source);
        if !source_path.exists() {
            return Err(BackupError::SourceNotFound(source.to_string()));
        }

        let dest_path = Path::new(destination);
        if !dest_path.exists() {
            fs::create_dir_all(dest_path)
                .map_err(|e| BackupError::IoError(format!("Failed to create destination: {}", e)))?;
        }

        let result = Command::new("rsync")
            .args(&["-av", "--progress", source, destination])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(BackupError::BackupFailed("rsync command failed".to_string())),
            Err(e) => Err(BackupError::IoError(format!("Failed to run rsync: {}", e))),
        }
    }

    /// Create a compressed backup using tar.
    pub fn create_compressed_backup(&self, source: &str, destination: &str) -> Result<(), BackupError> {
        let source_path = Path::new(source);
        if !source_path.exists() {
            return Err(BackupError::SourceNotFound(source.to_string()));
        }

        let dest_dir = Path::new(destination).parent().unwrap_or(Path::new("."));
        if !dest_dir.exists() {
            fs::create_dir_all(dest_dir)
                .map_err(|e| BackupError::IoError(format!("Failed to create destination: {}", e)))?;
        }

        let result = Command::new("tar")
            .args(&["-czf", destination, source])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(BackupError::BackupFailed("tar command failed".to_string())),
            Err(e) => Err(BackupError::IoError(format!("Failed to run tar: {}", e))),
        }
    }

    /// Restore from a tar backup.
    pub fn restore_backup(&self, backup_file: &str, destination: &str) -> Result<(), BackupError> {
        let backup_path = Path::new(backup_file);
        if !backup_path.exists() {
            return Err(BackupError::SourceNotFound(backup_file.to_string()));
        }

        let dest_path = Path::new(destination);
        if !dest_path.exists() {
            fs::create_dir_all(dest_path)
                .map_err(|e| BackupError::IoError(format!("Failed to create destination: {}", e)))?;
        }

        let result = Command::new("tar")
            .args(&["-xzf", backup_file, "-C", destination])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(BackupError::BackupFailed("tar extract failed".to_string())),
            Err(e) => Err(BackupError::IoError(format!("Failed to run tar: {}", e))),
        }
    }

    /// Get backup size.
    pub fn get_backup_size(&self, backup_file: &str) -> Result<u64, BackupError> {
        let metadata = fs::metadata(backup_file)
            .map_err(|e| BackupError::IoError(format!("Failed to get metadata: {}", e)))?;
        
        Ok(metadata.len())
    }

    /// List backups in a directory.
    pub fn list_backups(&self, directory: &str) -> Result<Vec<String>, BackupError> {
        let dir_path = Path::new(directory);
        if !dir_path.exists() {
            return Err(BackupError::SourceNotFound(directory.to_string()));
        }

        let entries = fs::read_dir(dir_path)
            .map_err(|e| BackupError::IoError(format!("Failed to read directory: {}", e)))?;

        let mut backups = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "tar" || ext == "gz" || ext == "bz2" || ext == "xz" {
                    if let Some(name) = path.file_name() {
                        backups.push(name.to_string_lossy().to_string());
                    }
                }
            }
        }

        Ok(backups)
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
        let backup_ops = BackupOps::new();
        let _ = backup_ops.is_root();
    }

    #[test]
    fn test_list_backups() {
        let backup_ops = BackupOps::new();
        if let Ok(backups) = backup_ops.list_backups("/tmp") {
            let _ = backups;
        }
    }

    #[test]
    fn test_get_backup_size() {
        let backup_ops = BackupOps::new();
        // Test with a file that should exist
        if let Ok(_) = backup_ops.get_backup_size("/etc/hostname") {
            // Should not panic
        }
    }
}
