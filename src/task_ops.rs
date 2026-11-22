//! SigmaOS task management operations module.
//!
//! This module provides actual task management operations via task.
//!
//! # Example
//!
//! ```
//! use sigma_system::task_ops::*;
//!
//! let task = TaskOps::new();
//! if let Ok(status) = task.list_tasks() {
//!     println!("Tasks: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in task operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskError {
    /// Task failed.
    TaskError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::TaskError(msg) => write!(f, "Failed to manage task: {}", msg),
            TaskError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for TaskError {}

/// Task operations manager.
#[derive(Debug)]
pub struct TaskOps;

impl TaskOps {
    pub fn new() -> Self { Self }

    pub fn list_tasks(&self) -> Result<String, TaskError> {
        let output = Command::new("task")
            .arg("list")
            .output()
            .map_err(|e| TaskError::IoError(format!("Failed to run task: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for TaskOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let task = TaskOps::new();
        let _ = task;
    }

    #[test]
    fn test_list_tasks() {
        let task = TaskOps::new();
        if let Ok(_) = task.list_tasks() {
            // Should not panic
        }
    }
}
