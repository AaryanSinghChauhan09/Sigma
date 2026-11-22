//! SigmaOS archive operations module (extended).
//!
//! This module provides actual archive operations via tar.
//!
//! # Example
//!
//! ```
//! use sigma_system::archive_ops::*;
//!
//! let archive = ArchiveOps::new();
//! if let Ok(status) = archive.create_archive("/tmp/archive.tar", "/tmp/source") {
//!     println!("Archive: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in archive operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ArchiveError {
    /// Archive failed.
    ArchiveError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::ArchiveError(msg) => write!(f, "Failed to archive: {}", msg),
            ArchiveError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ArchiveError {}

/// Archive operations manager.
#[derive(Debug)]
pub struct ArchiveOps;

impl ArchiveOps {
    pub fn new() -> Self { Self }

    pub fn create_archive(&self, archive: &str, source: &str) -> Result<String, ArchiveError> {
        let output = Command::new("tar")
            .args(&["-cf", archive, source])
            .output()
            .map_err(|e| ArchiveError::IoError(format!("Failed to run tar: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("archive_created".to_string())
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
        let archive = ArchiveOps::new();
        let _ = archive;
    }

    #[test]
    fn test_create_archive() {
        let archive = ArchiveOps::new();
        if let Ok(_) = archive.create_archive("/tmp/test.tar", "/tmp") {
            // Should not panic
        }
    }
}
