//! SigmaOS system update operations module.
//!
//! This module provides system update operations including:
//! - System updates
//! - Security updates
//! - Kernel updates
//!
//! Follows Single Responsibility Principle: only manages update operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::sysupdate_ops::*;
//!
//! let sysupdate = SysUpdateOps::new();
//! if let Ok(_) = sysupdate.update_system() {
//!     println!("System updated");
//! }
//! ```

use std::process::Command;

/// System update operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum SysUpdateError {
    /// Command execution failed.
    CommandError(String),
    /// Update failed.
    UpdateError(String),
}

impl std::fmt::Display for SysUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SysUpdateError::CommandError(msg) => write!(f, "Command error: {}", msg),
            SysUpdateError::UpdateError(msg) => write!(f, "Update error: {}", msg),
        }
    }
}

impl std::error::Error for SysUpdateError {}

/// System update operations manager.
#[derive(Debug)]
pub struct SysUpdateOps;

impl SysUpdateOps {
    pub fn new() -> Self { Self }

    /// Update the system.
    pub fn update_system(&self) -> Result<(), SysUpdateError> {
        let output = Command::new("sigmaos-update")
            .arg("system")
            .output()
            .map_err(|e| SysUpdateError::CommandError(format!("Failed to run sigmaos-update: {}", e)))?;

        if !output.status.success() {
            return Err(SysUpdateError::UpdateError("System update failed".to_string()));
        }

        Ok(())
    }

    /// Update security packages.
    pub fn update_security(&self) -> Result<(), SysUpdateError> {
        let output = Command::new("sigmaos-update")
            .arg("security")
            .output()
            .map_err(|e| SysUpdateError::CommandError(format!("Failed to run sigmaos-update: {}", e)))?;

        if !output.status.success() {
            return Err(SysUpdateError::UpdateError("Security update failed".to_string()));
        }

        Ok(())
    }
}

impl Default for SysUpdateOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sysupdate = SysUpdateOps::new();
        let _ = sysupdate;
    }
}
