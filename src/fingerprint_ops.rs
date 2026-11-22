//! SigmaOS fingerprint operations module.
//!
//! This module provides actual fingerprint operations using fprintd.
//!
//! # Example
//!
//! ```
//! use sigma_system::fingerprint_ops::*;
//!
//! let fp = FingerprintOps::new();
//! if let Ok(devices) = fp.list_devices() {
//!     println!("Found {} fingerprint devices", devices.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in fingerprint operations.
#[derive(Debug, Clone, PartialEq)]
pub enum FingerprintError {
    /// Device not found.
    DeviceNotFound(String),
    /// Failed to manage fingerprint.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for FingerprintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FingerprintError::DeviceNotFound(name) => write!(f, "Device '{}' not found", name),
            FingerprintError::ManagementError(msg) => write!(f, "Failed to manage fingerprint: {}", msg),
            FingerprintError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for FingerprintError {}

/// Fingerprint device information.
#[derive(Debug, Clone)]
pub struct FingerprintDevice {
    /// Device path.
    pub device: String,
    /// Device name.
    pub name: String,
}

/// Fingerprint operations manager.
#[derive(Debug)]
pub struct FingerprintOps;

impl FingerprintOps {
    pub fn new() -> Self { Self }

    pub fn list_devices(&self) -> Result<Vec<FingerprintDevice>, FingerprintError> {
        let mut devices = Vec::new();

        if Command::new("fprintd").arg("--version").output().is_ok() {
            let output = Command::new("fprintd")
                .args(&["-l"])
                .output()
                .map_err(|e| FingerprintError::IoError(format!("Failed to run fingerprintd: {}", e)))?;

            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("fingerprint") {
                    devices.push(FingerprintDevice {
                        device: "unknown".to_string(),
                        name: line.to_string(),
                    });
                }
            }
        }

        Ok(devices)
    }
}

impl Default for FingerprintOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fp = FingerprintOps::new();
        let _ = fp;
    }

    #[test]
    fn test_list_devices() {
        let fp = FingerprintOps::new();
        if let Ok(_) = fp.list_devices() {
            // Should not panic
        }
    }
}
