//! SigmaOS system configuration operations module.
//!
//! This module provides system configuration operations including:
//! - System-wide configuration management
//! - User configuration management
//! - Configuration validation
//!
//! Follows Single Responsibility Principle: only manages configuration operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::sysconfig_ops::*;
//!
//! let sysconfig = SysConfigOps::new();
//! if let Ok(config) = sysconfig.get_config("sigmaos") {
//!     println!("Config: {}", config);
//! }
//! ```

use std::process::Command;

/// System configuration operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum SysConfigError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
    /// Invalid configuration.
    InvalidConfig(String),
}

impl std::fmt::Display for SysConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SysConfigError::CommandError(msg) => write!(f, "Command error: {}", msg),
            SysConfigError::IoError(msg) => write!(f, "IO error: {}", msg),
            SysConfigError::InvalidConfig(msg) => write!(f, "Invalid config: {}", msg),
        }
    }
}

impl std::error::Error for SysConfigError {}

/// System configuration operations manager.
#[derive(Debug)]
pub struct SysConfigOps;

impl SysConfigOps {
    pub fn new() -> Self { Self }

    /// Get system configuration.
    pub fn get_config(&self, key: &str) -> Result<String, SysConfigError> {
        let output = Command::new("sigmaos-config")
            .arg("get")
            .arg(key)
            .output()
            .map_err(|e| SysConfigError::CommandError(format!("Failed to run sigmaos-config: {}", e)))?;

        let config = String::from_utf8_lossy(&output.stdout);
        Ok(config.trim().to_string())
    }

    /// Set system configuration.
    pub fn set_config(&self, key: &str, value: &str) -> Result<(), SysConfigError> {
        let output = Command::new("sigmaos-config")
            .args(&["set", key, value])
            .output()
            .map_err(|e| SysConfigError::CommandError(format!("Failed to run sigmaos-config: {}", e)))?;

        if !output.status.success() {
            return Err(SysConfigError::CommandError("sigmaos-config failed".to_string()));
        }

        Ok(())
    }
}

impl Default for SysConfigOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sysconfig = SysConfigOps::new();
        let _ = sysconfig;
    }
}
