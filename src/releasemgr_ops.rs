//! SigmaOS release manager operations module.
//!
//! This module provides release management operations including:
//! - Snapshot creation
//! - Release management
//! - Rollback operations
//!
//! Follows Single Responsibility Principle: only manages release operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::releasemgr_ops::*;
//!
//! let releasemgr = ReleaseMgrOps::new();
//! if let Ok(_) = releasemgr.create_snapshot() {
//!     println!("Snapshot created");
//! }
//! ```

use std::process::Command;

/// Release manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum ReleaseMgrError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ReleaseMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReleaseMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            ReleaseMgrError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ReleaseMgrError {}

/// Release manager operations manager.
#[derive(Debug)]
pub struct ReleaseMgrOps;

impl ReleaseMgrOps {
    pub fn new() -> Self { Self }

    /// Create system snapshot.
    pub fn create_snapshot(&self) -> Result<(), ReleaseMgrError> {
        let output = Command::new("sigmaos-release")
            .arg("snapshot")
            .output()
            .map_err(|e| ReleaseMgrError::CommandError(format!("Failed to run sigmaos-release: {}", e)))?;

        if !output.status.success() {
            return Err(ReleaseMgrError::CommandError("sigmaos-release failed".to_string()));
        }

        Ok(())
    }

    /// Rollback to snapshot.
    pub fn rollback(&self, snapshot: &str) -> Result<(), ReleaseMgrError> {
        let output = Command::new("sigmaos-release")
            .args(&["rollback", snapshot])
            .output()
            .map_err(|e| ReleaseMgrError::CommandError(format!("Failed to run sigmaos-release: {}", e)))?;

        if !output.status.success() {
            return Err(ReleaseMgrError::CommandError("sigmaos-release failed".to_string()));
        }

        Ok(())
    }
}

impl Default for ReleaseMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let releasemgr = ReleaseMgrOps::new();
        let _ = releasemgr;
    }
}
