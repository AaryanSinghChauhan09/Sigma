//! SigmaOS initramfs operations module.
//!
//! This module provides initramfs management operations including:
//! - Initramfs generation via mkinitcpio
//! - Initramfs configuration management
//! - Hook management
//!
//! Follows Single Responsibility Principle: only manages initramfs operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::initramfs_ops::*;
//!
//! let initramfs = InitramfsOps::new();
//! if let Ok(_) = initramfs.generate() {
//!     println!("Initramfs generated");
//! }
//! ```

use std::process::Command;

/// Initramfs operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum InitramfsError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
    /// Configuration error.
    ConfigError(String),
}

impl std::fmt::Display for InitramfsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitramfsError::CommandError(msg) => write!(f, "Command error: {}", msg),
            InitramfsError::IoError(msg) => write!(f, "IO error: {}", msg),
            InitramfsError::ConfigError(msg) => write!(f, "Config error: {}", msg),
        }
    }
}

impl std::error::Error for InitramfsError {}

/// Initramfs operations manager.
#[derive(Debug)]
pub struct InitramfsOps;

impl InitramfsOps {
    pub fn new() -> Self { Self }

    /// Generate initramfs.
    pub fn generate(&self) -> Result<(), InitramfsError> {
        let output = Command::new("mkinitcpio")
            .arg("-P")
            .output()
            .map_err(|e| InitramfsError::CommandError(format!("Failed to run mkinitcpio: {}", e)))?;

        if !output.status.success() {
            return Err(InitramfsError::CommandError("mkinitcpio failed".to_string()));
        }

        Ok(())
    }
}

impl Default for InitramfsOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let initramfs = InitramfsOps::new();
        let _ = initramfs;
    }
}
