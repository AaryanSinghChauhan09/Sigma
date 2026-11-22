//! SigmaOS cross-platform operations module.
//!
//! This module provides cross-platform operations for Linux, macOS, and Windows.
//!
//! Follows principles: Cross Platform Compatibility, Portability, Cross Compilation
//!
//! # Example
//!
//! ```
//! use sigma_system::crossplatform_ops::*;
//!
//! let cp = CrossPlatformOps::new();
//! let platform = cp.get_platform();
//! ```

use std::process::Command;

/// Cross-platform error types.
#[derive(Debug, Clone, PartialEq)]
pub enum CrossPlatformError {
    /// Platform not supported.
    PlatformNotSupported(String),
    /// Command error.
    CommandError(String),
}

impl std::fmt::Display for CrossPlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrossPlatformError::PlatformNotSupported(msg) => write!(f, "Platform not supported: {}", msg),
            CrossPlatformError::CommandError(msg) => write!(f, "Command error: {}", msg),
        }
    }
}

impl std::error::Error for CrossPlatformError {}

/// Cross-platform operations manager.
#[derive(Debug)]
pub struct CrossPlatformOps;

impl CrossPlatformOps {
    pub fn new() -> Self { Self }

    /// Get current platform.
    pub fn get_platform(&self) -> String {
        std::env::consts::OS.to_string()
    }

    /// Get architecture.
    pub fn get_architecture(&self) -> String {
        std::env::consts::ARCH.to_string()
    }

    /// Check if platform is supported.
    pub fn is_supported(&self) -> bool {
        matches!(std::env::consts::OS, std::env::consts::ARCH)
    }
}

impl Default for CrossPlatformOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cp = CrossPlatformOps::new();
        let _ = cp;
    }

    #[test]
    fn test_get_platform() {
        let cp = CrossPlatformOps::new();
        let platform = cp.get_platform();
        assert!(!platform.is_empty());
    }
}
