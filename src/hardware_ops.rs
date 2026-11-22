//! SigmaOS hardware operations module.
//!
//! This module provides actual hardware operations via lspci.
//!
//! # Example
//!
//! ```
//! use sigma_system::hardware_ops::*;
//!
//! let hw = HardwareOps::new();
//! if let Ok(status) = hw.list_hardware() {
//!     println!("Hardware: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in hardware operations.
#[derive(Debug, Clone, PartialEq)]
pub enum HardwareError {
    /// Hardware query failed.
    HardwareError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for HardwareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HardwareError::HardwareError(msg) => write!(f, "Failed to query hardware: {}", msg),
            HardwareError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for HardwareError {}

/// Hardware operations manager.
#[derive(Debug)]
pub struct HardwareOps;

impl HardwareOps {
    pub fn new() -> Self { Self }

    pub fn list_hardware(&self) -> Result<String, HardwareError> {
        let output = Command::new("lspci")
            .output()
            .map_err(|e| HardwareError::IoError(format!("Failed to run lspci: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for HardwareOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hw = HardwareOps::new();
        let _ = hw;
    }

    #[test]
    fn test_list_hardware() {
        let hw = HardwareOps::new();
        if let Ok(_) = hw.list_hardware() {
            // Should not panic
        }
    }
}
