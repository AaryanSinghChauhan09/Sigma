//! SigmaOS RAID operations module.
//!
//! This module provides actual RAID operations using mdadm.
//!
//! # Example
//!
//! ```
//! use sigma_system::raid_ops::*;
//!
//! let raid = RaidOps::new();
//! if let Ok(arrays) = raid.list_arrays() {
//!     println!("Found {} RAID arrays", arrays.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in RAID operations.
#[derive(Debug, Clone, PartialEq)]
pub enum RaidError {
    /// RAID array not found.
    ArrayNotFound(String),
    /// Failed to manage RAID.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for RaidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RaidError::ArrayNotFound(name) => write!(f, "RAID array '{}' not found", name),
            RaidError::ManagementError(msg) => write!(f, "Failed to manage RAID: {}", msg),
            RaidError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for RaidError {}

/// RAID array information.
#[derive(Debug, Clone)]
pub struct RaidInfo {
    /// Array name.
    pub name: String,
    /// RAID level.
    pub level: String,
    /// Array status.
    pub status: String,
}

/// RAID operations manager.
#[derive(Debug)]
pub struct RaidOps {
    is_root: bool,
}

impl RaidOps {
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);
        Self { is_root }
    }

    pub fn is_root(&self) -> bool { self.is_root }

    pub fn list_arrays(&self) -> Result<Vec<RaidInfo>, RaidError> {
        let mut arrays = Vec::new();

        if Command::new("mdadm").arg("--detail").output().is_ok() {
            let output = Command::new("mdadm")
                .args(&["--detail", "--scan"])
                .output()
                .map_err(|e| RaidError::IoError(format!("Failed to run mdadm: {}", e)))?;

            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("ARRAY") {
                    let name = line.split_whitespace().nth(1).unwrap_or("unknown");
                    arrays.push(RaidInfo {
                        name: name.to_string(),
                        level: "unknown".to_string(),
                        status: "active".to_string(),
                    });
                }
            }
        }

        Ok(arrays)
    }
}

impl Default for RaidOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let raid = RaidOps::new();
        let _ = raid.is_root();
    }

    #[test]
    fn test_list_arrays() {
        let raid = RaidOps::new();
        if let Ok(_) = raid.list_arrays() {
            // Should not panic
        }
    }
}
