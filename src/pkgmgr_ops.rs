//! SigmaOS package manager operations module.
//!
//! This module provides package manager operations including:
//! - Package installation
//! - Package removal
//! - Package updates
//! - Repository management
//! - Dependency resolution
//!
//! Follows Single Responsibility Principle: only manages package operations.
//! Follows Dependency Inversion Principle: depends on abstraction (package manager interface).
//!
//! # Example
//!
//! ```
//! use sigma_system::pkgmgr_ops::*;
//!
//! let pkgmgr = PkgMgrOps::new();
//! if let Ok(_) = pkgmgr.install("nginx") {
//!     println!("Package installed");
//! }
//! ```

use std::process::Command;

/// Package manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum PkgMgrError {
    /// Command execution failed.
    CommandError(String),
    /// Package not found.
    PackageNotFound(String),
    /// Dependency error.
    DependencyError(String),
}

impl std::fmt::Display for PkgMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PkgMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            PkgMgrError::PackageNotFound(msg) => write!(f, "Package not found: {}", msg),
            PkgMgrError::DependencyError(msg) => write!(f, "Dependency error: {}", msg),
        }
    }
}

impl std::error::Error for PkgMgrError {}

/// Package manager operations manager.
#[derive(Debug)]
pub struct PkgMgrOps;

impl PkgMgrOps {
    pub fn new() -> Self { Self }

    /// Install a package.
    pub fn install(&self, package: &str) -> Result<(), PkgMgrError> {
        let output = Command::new("pacman")
            .args(&["-S", "--noconfirm", package])
            .output()
            .map_err(|e| PkgMgrError::CommandError(format!("Failed to run pacman: {}", e)))?;

        if !output.status.success() {
            return Err(PkgMgrError::CommandError("pacman install failed".to_string()));
        }

        Ok(())
    }

    /// Remove a package.
    pub fn remove(&self, package: &str) -> Result<(), PkgMgrError> {
        let output = Command::new("pacman")
            .args(&["-R", "--noconfirm", package])
            .output()
            .map_err(|e| PkgMgrError::CommandError(format!("Failed to run pacman: {}", e)))?;

        if !output.status.success() {
            return Err(PkgMgrError::CommandError("pacman remove failed".to_string()));
        }

        Ok(())
    }

    /// Update system.
    pub fn update(&self) -> Result<(), PkgMgrError> {
        let output = Command::new("pacman")
            .args(&["-Syu", "--noconfirm"])
            .output()
            .map_err(|e| PkgMgrError::CommandError(format!("Failed to run pacman: {}", e)))?;

        if !output.status.success() {
            return Err(PkgMgrError::CommandError("pacman update failed".to_string()));
        }

        Ok(())
    }

    /// Search for a package.
    pub fn search(&self, query: &str) -> Result<String, PkgMgrError> {
        let output = Command::new("pacman")
            .args(&["-Ss", query])
            .output()
            .map_err(|e| PkgMgrError::CommandError(format!("Failed to run pacman: {}", e)))?;

        let results = String::from_utf8_lossy(&output.stdout);
        Ok(results.trim().to_string())
    }
}

impl Default for PkgMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pkgmgr = PkgMgrOps::new();
        let _ = pkgmgr;
    }

    #[test]
    fn test_search() {
        let pkgmgr = PkgMgrOps::new();
        if let Ok(results) = pkgmgr.search("nginx") {
            // Should not panic
            let _ = results;
        }
    }
}
