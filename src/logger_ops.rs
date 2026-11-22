//! SigmaOS logging operations module (extended).
//!
//! This module provides actual logging operations via logger.
//!
//! # Example
//!
//! ```
//! use sigma_system::logger_ops::*;
//!
//! let logger = LoggerOps::new();
//! if let Ok(status) = logger.log("Hello, world!") {
//!     println!("Logged: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in logging operations.
#[derive(Debug, Clone, PartialEq)]
pub enum LoggerError {
    /// Logging failed.
    LoggerError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for LoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoggerError::LoggerError(msg) => write!(f, "Failed to log: {}", msg),
            LoggerError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for LoggerError {}

/// Logger operations manager.
#[derive(Debug)]
pub struct LoggerOps;

impl LoggerOps {
    pub fn new() -> Self { Self }

    pub fn log(&self, message: &str) -> Result<String, LoggerError> {
        let output = Command::new("logger")
            .arg(message)
            .output()
            .map_err(|e| LoggerError::IoError(format!("Failed to run logger: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("logged".to_string())
    }
}

impl Default for LoggerOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let logger = LoggerOps::new();
        let _ = logger;
    }

    #[test]
    fn test_log() {
        let logger = LoggerOps::new();
        if let Ok(_) = logger.log("SigmaOS test") {
            // Should not panic
        }
    }
}
