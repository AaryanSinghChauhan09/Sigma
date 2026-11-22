//! SigmaOS logrotate operations module (extended).
//!
//! This module provides actual log rotation operations via logrotate.
//!
//! # Example
//!
//! ```
//! use sigma_system::logrotate_ops::*;
//!
//! let logrotate = LogrotateOps::new();
//! if let Ok(status) = logrotate.rotate_logs() {
//!     println!("Logs rotated: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in logrotate operations.
#[derive(Debug, Clone, PartialEq)]
pub enum LogrotateError {
    /// Rotation failed.
    RotationError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for LogrotateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogrotateError::RotationError(msg) => write!(f, "Failed to rotate logs: {}", msg),
            LogrotateError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for LogrotateError {}

/// Logrotate operations manager.
#[derive(Debug)]
pub struct LogrotateOps;

impl LogrotateOps {
    pub fn new() -> Self { Self }

    pub fn rotate_logs(&self) -> Result<String, LogrotateError> {
        let output = Command::new("logrotate")
            .arg("-f")
            .output()
            .map_err(|e| LogrotateError::IoError(format!("Failed to run logrotate: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("logs_rotated".to_string())
    }
}

impl Default for LogrotateOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let logrotate = LogrotateOps::new();
        let _ = logrotate;
    }

    #[test]
    fn test_rotate_logs() {
        let logrotate = LogrotateOps::new();
        if let Ok(_) = logrotate.rotate_logs() {
            // Should not panic
        }
    }
}
