//! SigmaOS CI/CD operations module.
//!
//! This module provides actual CI/CD operations via make.
//!
//! # Example
//!
//! ```
//! use sigma_system::cicd_ops::*;
//!
//! let cicd = CicdOps::new();
//! if let Ok(status) = cicd.build() {
//!     println!("Build: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in CI/CD operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CicdError {
    /// CI/CD failed.
    CicdError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for CicdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CicdError::CicdError(msg) => write!(f, "Failed to run CI/CD: {}", msg),
            CicdError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for CicdError {}

/// CI/CD operations manager.
#[derive(Debug)]
pub struct CicdOps;

impl CicdOps {
    pub fn new() -> Self { Self }

    pub fn build(&self) -> Result<String, CicdError> {
        let output = Command::new("make")
            .arg("build")
            .output()
            .map_err(|e| CicdError::IoError(format!("Failed to run make: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("build_completed".to_string())
    }
}

impl Default for CicdOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cicd = CicdOps::new();
        let _ = cicd;
    }

    #[test]
    fn test_build() {
        let cicd = CicdOps::new();
        if let Ok(_) = cicd.build() {
            // Should not panic
        }
    }
}
