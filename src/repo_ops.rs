//! SigmaOS repository operations module.
//!
//! This module provides actual package repository operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::repo_ops::*;
//!
//! let repo_ops = RepoOps::new();
//! if let Ok(repos) = repo_ops.list_repos() {
//!     println!("Found {} repositories", repos.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in repository operations.
#[derive(Debug, Clone, PartialEq)]
pub enum RepoError {
    /// Repository not found.
    RepoNotFound(String),
    /// Failed to update repository.
    UpdateError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::RepoNotFound(name) => write!(f, "Repository '{}' not found", name),
            RepoError::UpdateError(msg) => write!(f, "Failed to update repository: {}", msg),
            RepoError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            RepoError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for RepoError {}

/// Repository information.
#[derive(Debug, Clone)]
pub struct RepoInfo {
    /// Repository name.
    pub name: String,
    /// Repository URL.
    pub url: String,
    /// Whether repository is enabled.
    pub enabled: bool,
}

/// Repository operations manager.
#[derive(Debug)]
pub struct RepoOps {
    /// Whether running as root.
    is_root: bool,
}

impl RepoOps {
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);
        Self { is_root }
    }

    pub fn is_root(&self) -> bool { self.is_root }

    pub fn list_repos(&self) -> Result<Vec<RepoInfo>, RepoError> {
        let mut repos = Vec::new();

        if Command::new("pacman").arg("--version").output().is_ok() {
            repos.push(RepoInfo {
                name: "core".to_string(),
                url: "https://archive.archlinux.org/repos/core/os/x86_64".to_string(),
                enabled: true,
            });
        }

        Ok(repos)
    }

    pub fn update_repos(&self) -> Result<(), RepoError> {
        if Command::new("pacman").arg("--version").output().is_ok() {
            let result = Command::new("pacman")
                .args(&["-Sy"])
                .status();

            match result {
                Ok(status) if status.success() => Ok(()),
                Ok(_) => Err(RepoError::UpdateError("pacman -Sy failed".to_string())),
                Err(e) => Err(RepoError::IoError(format!("Failed to run pacman: {}", e))),
            }
        } else if Command::new("apt-get").arg("--version").output().is_ok() {
            let result = Command::new("apt-get")
                .args(&["update"])
                .status();

            match result {
                Ok(status) if status.success() => Ok(()),
                Ok(_) => Err(RepoError::UpdateError("apt-get update failed".to_string())),
                Err(e) => Err(RepoError::IoError(format!("Failed to run apt-get: {}", e))),
            }
        } else {
            Err(RepoError::IoError("No supported package manager found".to_string()))
        }
    }
}

impl Default for RepoOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repo_ops = RepoOps::new();
        let _ = repo_ops.is_root();
    }

    #[test]
    fn test_list_repos() {
        let repo_ops = RepoOps::new();
        if let Ok(_) = repo_ops.list_repos() {
            // Should not panic
        }
    }
}
