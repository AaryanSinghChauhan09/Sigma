//! SigmaOS audit manager operations module.
//!
//! This module provides audit operations including:
//! - System audit
//! - Security audit
//! - Compliance audit
//!
//! Follows Single Responsibility Principle: only manages audit operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::auditmgr_ops::*;
//!
//! let auditmgr = AuditMgrOps::new();
//! if let Ok(report) = auditmgr.run_audit() {
//!     println!("Audit report: {}", report);
//! }
//! ```

use std::process::Command;

/// Audit manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum AuditMgrError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for AuditMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            AuditMgrError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for AuditMgrError {}

/// Audit manager operations manager.
#[derive(Debug)]
pub struct AuditMgrOps;

impl AuditMgrOps {
    pub fn new() -> Self { Self }

    /// Run system audit.
    pub fn run_audit(&self) -> Result<String, AuditMgrError> {
        let output = Command::new("sigmaos-audit")
            .arg("system")
            .output()
            .map_err(|e| AuditMgrError::CommandError(format!("Failed to run sigmaos-audit: {}", e)))?;

        let report = String::from_utf8_lossy(&output.stdout);
        Ok(report.trim().to_string())
    }
}

impl Default for AuditMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let auditmgr = AuditMgrOps::new();
        let _ = auditmgr;
    }
}
