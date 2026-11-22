//! SigmaOS process tree operations module (extended).
//!
//! This module provides actual process tree operations via pstree.
//!
//! # Example
//!
//! ```
//! use sigma_system::proc_tree_ops::*;
//!
//! let pt = ProcTreeOps::new();
//! if let Ok(status) = pt.get_process_tree() {
//!     println!("Process tree: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in process tree operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ProcTreeError {
    /// Tree failed.
    TreeError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ProcTreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcTreeError::TreeError(msg) => write!(f, "Failed to get process tree: {}", msg),
            ProcTreeError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ProcTreeError {}

/// Process tree operations manager.
#[derive(Debug)]
pub struct ProcTreeOps;

impl ProcTreeOps {
    pub fn new() -> Self { Self }

    pub fn get_process_tree(&self) -> Result<String, ProcTreeError> {
        let output = Command::new("pstree")
            .arg("-p")
            .output()
            .map_err(|e| ProcTreeError::IoError(format!("Failed to run pstree: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for ProcTreeOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pt = ProcTreeOps::new();
        let _ = pt;
    }

    #[test]
    fn test_get_process_tree() {
        let pt = ProcTreeOps::new();
        if let Ok(_) = pt.get_process_tree() {
            // Should not panic
        }
    }
}
