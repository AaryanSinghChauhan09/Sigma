//! SigmaOS archive operations module.
//!
//! This module provides actual archive operations using tar, zip, etc.
//!
//! # Example
//!
//! ```
//! use sigma_system::archive_ops::*;
//!
//! let mut archive_ops = ArchiveOps::new();
//! if let Ok(_) = archive_ops.create_tar("/home/user", "backup.tar") {
//!     println!("Archive created successfully");
//! }
//! ```

use std::fs;
use std::path::Path;
use std::process::Command;

/// Errors that can occur in archive operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ArchiveError {
    /// Source not found.
    SourceNotFound(String),
    /// Archive failed.
    ArchiveFailed(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::SourceNotFound(path) => write!(f, "Source '{}' not found", path),
            ArchiveError::ArchiveFailed(msg) => write!(f, "Archive failed: {}", msg),
            ArchiveError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            ArchiveError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ArchiveError {}

/// Archive operations manager.
#[derive(Debug)]
pub struct ArchiveOps {
    /// Whether running as root.
    is_root: bool,
}

impl ArchiveOps {
    /// Create a new archive operations manager.
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

    /// Create a tar archive.
    pub fn create_tar(&self, source: &str, output: &str) -> Result<(), ArchiveError> {
        let source_path = Path::new(source);
        if !source_path.exists() {
            return Err(ArchiveError::SourceNotFound(source.to_string()));
        }

        let result = Command::new("tar")
            .args(&["-cf", output, source])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(ArchiveError::ArchiveFailed("tar command failed".to_string())),
            Err(e) => Err(ArchiveError::IoError(format!("Failed to run tar: {}", e))),
        }
    }

    /// Create a compressed tar archive.
    pub fn create_tar_gz(&self, source: &str, output: &str) -> Result<(), ArchiveError> {
        let source_path = Path::new(source);
        if !source_path.exists() {
            return Err(ArchiveError::SourceNotFound(source.to_string()));
        }

        let result = Command::new("tar")
            .args(&["-czf", output, source])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(ArchiveError::ArchiveFailed("tar command failed".to_string())),
            Err(e) => Err(ArchiveError::IoError(format!("Failed to run tar: {}", e))),
        }
    }

    /// Extract a tar archive.
    pub fn extract_tar(&self, archive: &str, destination: &str) -> Result<(), ArchiveError> {
        let archive_path = Path::new(archive);
        if !archive_path.exists() {
            return Err(ArchiveError::SourceNotFound(archive.to_string()));
        }

        let dest_path = Path::new(destination);
        if !dest_path.exists() {
            fs::create_dir_all(dest_path)
                .map_err(|e| ArchiveError::IoError(format!("Failed to create destination: {}", e)))?;
        }

        let result = Command::new("tar")
            .args(&["-xf", archive, "-C", destination])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(ArchiveError::ArchiveFailed("tar extract failed".to_string())),
            Err(e) => Err(ArchiveError::IoError(format!("Failed to run tar: {}", e))),
        }
    }

    /// Create a zip archive.
    pub fn create_zip(&self, source: &str, output: &str) -> Result<(), ArchiveError> {
        let source_path = Path::new(source);
        if !source_path.exists() {
            return Err(ArchiveError::SourceNotFound(source.to_string()));
        }

        let result = Command::new("zip")
            .args(&["-r", output, source])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(ArchiveError::ArchiveFailed("zip command failed".to_string())),
            Err(e) => Err(ArchiveError::IoError(format!("Failed to run zip: {}", e))),
        }
    }

    /// Extract a zip archive.
    pub fn extract_zip(&self, archive: &str, destination: &str) -> Result<(), ArchiveError> {
        let archive_path = Path::new(archive);
        if !archive_path.exists() {
            return Err(ArchiveError::SourceNotFound(archive.to_string()));
        }

        let dest_path = Path::new(destination);
        if !dest_path.exists() {
            fs::create_dir_all(dest_path)
                .map_err(|e| ArchiveError::IoError(format!("Failed to create destination: {}", e)))?;
        }

        let result = Command::new("unzip")
            .args(&["-q", archive, "-d", destination])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(ArchiveError::ArchiveFailed("unzip command failed".to_string())),
            Err(e) => Err(ArchiveError::IoError(format!("Failed to run unzip: {}", e))),
        }
    }

    /// Get archive size.
    pub fn get_archive_size(&self, archive: &str) -> Result<u64, ArchiveError> {
        let metadata = fs::metadata(archive)
            .map_err(|e| ArchiveError::IoError(format!("Failed to get metadata: {}", e)))?;
        
        Ok(metadata.len())
    }
}

impl Default for ArchiveOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let archive_ops = ArchiveOps::new();
        let _ = archive_ops.is_root();
    }

    #[test]
    fn test_get_archive_size() {
        let archive_ops = ArchiveOps::new();
        // Test with a file that should exist
        if let Ok(_) = archive_ops.get_archive_size("/etc/hostname") {
            // Should not panic
        }
    }
}
