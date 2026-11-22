//! SigmaOS LVM operations module.
//!
//! This module provides actual LVM operations using lvm commands.
//!
//! # Example
//!
//! ```
//! use sigma_system::lvm_ops::*;
//!
//! let lvm = LvmOps::new();
//! if let Ok(vgs) = lvm.list_volume_groups() {
//!     println!("Found {} volume groups", vgs.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in LVM operations.
#[derive(Debug, Clone, PartialEq)]
pub enum LvmError {
    /// Volume group not found.
    VolumeGroupNotFound(String),
    /// Failed to manage LVM.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for LvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LvmError::VolumeGroupNotFound(name) => write!(f, "Volume group '{}' not found", name),
            LvmError::ManagementError(msg) => write!(f, "Failed to manage LVM: {}", msg),
            LvmError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for LvmError {}

/// Volume group information.
#[derive(Debug, Clone)]
pub struct VolumeGroup {
    /// VG name.
    pub name: String,
    /// VG size.
    pub size: String,
    /// VG status.
    pub status: String,
}

/// LVM operations manager.
#[derive(Debug)]
pub struct LvmOps {
    is_root: bool,
}

impl LvmOps {
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);
        Self { is_root }
    }

    pub fn is_root(&self) -> bool { self.is_root }

    pub fn list_volume_groups(&self) -> Result<Vec<VolumeGroup>, LvmError> {
        let mut vgs = Vec::new();

        if Command::new("vgs").arg("--version").output().is_ok() {
            let output = Command::new("vgs")
                .args(&["--noheadings", "-o", "vg_name,vg_size,vg_attr"])
                .output()
                .map_err(|e| LvmError::IoError(format!("Failed to run vgs: {}", e)))?;

            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    vgs.push(VolumeGroup {
                        name: parts[0].to_string(),
                        size: parts[1].to_string(),
                        status: "active".to_string(),
                    });
                }
            }
        }

        Ok(vgs)
    }
}

impl Default for LvmOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lvm = LvmOps::new();
        let _ = lvm.is_root();
    }

    #[test]
    fn test_list_volume_groups() {
        let lvm = LvmOps::new();
        if let Ok(_) = lvm.list_volume_groups() {
            // Should not panic
        }
    }
}
