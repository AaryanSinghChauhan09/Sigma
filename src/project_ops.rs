//! SigmaOS project management operations module.
//!
//! This module provides actual project management operations via git.
//!
//! # Example
//!
//! ```
//! use sigma_system::project_ops::*;
//!
//! let project = ProjectOps::new();
//! if let Ok(status) = project.get_status() {
//!     println!("Project status: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in project operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectError {
    /// Project failed.
    ProjectError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::ProjectError(msg) => write!(f, "Failed to manage project: {}", msg),
            ProjectError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ProjectError {}

/// Project operations manager.
#[derive(Debug)]
pub struct ProjectOps;

impl ProjectOps {
    pub fn new() -> Self { Self }

    pub fn get_status(&self) -> Result<String, ProjectError> {
        let output = Command::new("git")
            .args(&["status"])
            .output()
            .map_err(|e| ProjectError::IoError(format!("Failed to run git: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for ProjectOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let project = ProjectOps::new();
        let _ = project;
    }

    #[test]
    fn test_get_status() {
        let project = ProjectOps::new();
        if let Ok(_) = project.get_status() {
            // Should not panic
        }
    }
}
