//! SigmaOS storage manager operations module.
//!
//! This module provides storage management operations including:
//! - Disk partitioning
//! - Filesystem formatting
//! - Mount management
//!
//! Follows Single Responsibility Principle: only manages storage operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::storagemgr_ops::*;
//!
//! let storagemgr = StorageMgrOps::new();
//! if let Ok(disks) = storagemgr.list_disks() {
//!     println!("Disks: {}", disks);
//! }
//! ```

use std::process::Command;

/// Storage manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum StorageMgrError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for StorageMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            StorageMgrError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for StorageMgrError {}

/// Storage manager operations manager.
#[derive(Debug)]
pub struct StorageMgrOps;

impl StorageMgrOps {
    pub fn new() -> Self { Self }

    /// List disks.
    pub fn list_disks(&self) -> Result<String, StorageMgrError> {
        let output = Command::new("lsblk")
            .args(&["-o", "NAME,SIZE,TYPE,MOUNTPOINT"])
            .output()
            .map_err(|e| StorageMgrError::CommandError(format!("Failed to run lsblk: {}", e)))?;

        let disks = String::from_utf8_lossy(&output.stdout);
        Ok(disks.trim().to_string())
    }

    /// Create partition.
    pub fn create_partition(&self, device: &str, size: &str) -> Result<(), StorageMgrError> {
        let output = Command::new("parted")
            .args(&[device, "mkpart", "primary", size])
            .output()
            .map_err(|e| StorageMgrError::CommandError(format!("Failed to run parted: {}", e)))?;

        if !output.status.success() {
            return Err(StorageMgrError::CommandError("parted failed".to_string()));
        }

        Ok(())
    }
}

impl Default for StorageMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let storagemgr = StorageMgrOps::new();
        let _ = storagemgr;
    }

    #[test]
    fn test_list_disks() {
        let storagemgr = StorageMgrOps::new();
        if let Ok(disks) = storagemgr.list_disks() {
            // Should not panic
            let _ = disks;
        }
    }
}
