//! SigmaOS repository operations module (extended).
//!
//! This module provides repository management operations including:
//! - Repository configuration
//! - Mirror management
//! - Key management
//!
//! Follows Single Responsibility Principle: only manages repository operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::repomgr_ops::*;
//!
//! let repomgr = RepoMgrOps::new();
//! if let Ok(repos) = repomgr.list_repositories() {
//!     println!("Repositories: {}", repos);
//! }
//! ```

use std::process::Command;

/// Repository manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum RepoMgrError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for RepoMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            RepoMgrError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for RepoMgrError {}

/// Repository manager operations manager.
#[derive(Debug)]
pub struct RepoMgrOps;

impl RepoMgrOps {
    pub fn new() -> Self { Self }

    /// List repositories.
    pub fn list_repositories(&self) -> Result<String, RepoMgrError> {
        let output = Command::new("pacman")
            .args(&["-Sl"])
            .output()
            .map_err(|e| RepoMgrError::CommandError(format!("Failed to run pacman: {}", e)))?;

        let repos = String::from_utf8_lossy(&output.stdout);
        Ok(repos.trim().to_string())
    }

    /// Update repository databases.
    pub fn update_databases(&self) -> Result<(), RepoMgrError> {
        let output = Command::new("pacman")
            .args(&["-Sy"])
            .output()
            .map_err(|e| RepoMgrError::CommandError(format!("Failed to run pacman: {}", e)))?;

        if !output.status.success() {
            return Err(RepoMgrError::CommandError("pacman update failed".to_string()));
        }

        Ok(())
    }
}

impl Default for RepoMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repomgr = RepoMgrOps::new();
        let _ = repomgr;
    }

    #[test]
    fn test_list_repositories() {
        let repomgr = RepoMgrOps::new();
        if let Ok(repos) = repomgr.list_repositories() {
            // Should not panic
            let _ = repos;
        }
    }
}
