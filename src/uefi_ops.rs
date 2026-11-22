//! SigmaOS UEFI operations module.
//!
//! This module provides actual UEFI operations using efibootmgr.
//!
//! # Example
//!
//! ```
//! use sigma_system::uefi_ops::*;
//!
//! let uefi = UefiOps::new();
//! if let Ok(entries) = uefi.list_boot_entries() {
//!     println!("Found {} boot entries", entries.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in UEFI operations.
#[derive(Debug, Clone, PartialEq)]
pub enum UefiError {
    /// UEFI not available.
    UefiNotAvailable,
    /// Failed to get boot entries.
    UefiBootError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for UefiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UefiError::UefiNotAvailable => write!(f, "UEFI not available"),
            UefiError::UefiBootError(msg) => write!(f, "Failed to get boot entries: {}", msg),
            UefiError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            UefiError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for UefiError {}

/// Boot entry.
#[derive(Debug, Clone)]
pub struct BootEntry {
    /// Entry number.
    pub number: u32,
    /// Boot name.
    pub name: String,
    /// Boot loader.
    pub loader: String,
}

/// UEFI operations manager.
#[derive(Debug)]
pub struct UefiOps {
    /// Whether running as root.
    is_root: bool,
}

impl UefiOps {
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);
        Self { is_root }
    }

    pub fn is_root(&self) -> bool { self.is_root }

    pub fn is_uefi_available(&self) -> bool {
        Command::new("efibootmgr").arg("--version").output().is_ok()
    }

    pub fn list_boot_entries(&self) -> Result<Vec<BootEntry>, UefiError> {
        if !self.is_uefi_available() {
            return Err(UefiError::UefiNotAvailable);
        }

        let output = Command::new("efibootmgr").output()
            .map_err(|e| UefiError::IoError(format!("Failed to run efibootmgr: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        let mut entries = Vec::new();

        for line in content.lines() {
            if line.contains("Boot") {
                if let Some(num) = line.split_whitespace().next() {
                    let number = num.parse().unwrap_or(0);
                    entries.push(BootEntry {
                        number,
                        name: line.to_string(),
                        loader: "unknown".to_string(),
                    });
                }
            }
        }

        Ok(entries)
    }
}

impl Default for UefiOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uefi = UefiOps::new();
        let _ = uefi.is_root();
    }

    #[test]
    fn test_is_uefi_available() {
        let uefi = UefiOps::new();
        let _ = uefi.is_uefi_available();
    }
}
