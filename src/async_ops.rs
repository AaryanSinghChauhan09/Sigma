//! SigmaOS async operations module.
//!
//! This module provides async I/O operations using tokio for:
//! - Async file operations
//! - Async network operations
//! - Async process operations
//!
//! Follows principles: Asynchronous I/O & Event Loops, Race-Free Design, Safe Concurrency
//!
//! # Example
//!
//! ```
//! use sigma_system::async_ops::*;
//!
//! let async_ops = AsyncOps::new();
//! if let Ok(result) = async_ops.read_file_async("/path/to/file").await {
//!     println!("File content: {}", result);
//! }
//! ```

use std::path::Path;
use std::fs;
use tokio::fs as async_fs;

/// Async operations error types.
#[derive(Debug, Clone, PartialEq)]
pub enum AsyncError {
    /// IO error.
    IoError(String),
    /// Task join error.
    JoinError(String),
}

impl std::fmt::Display for AsyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsyncError::IoError(msg) => write!(f, "IO error: {}", msg),
            AsyncError::JoinError(msg) => write!(f, "Join error: {}", msg),
        }
    }
}

impl std::error::Error for AsyncError {}

/// Async operations manager.
#[derive(Debug)]
pub struct AsyncOps;

impl AsyncOps {
    pub fn new() -> Self { Self }

    /// Read file asynchronously.
    pub async fn read_file_async(&self, path: &str) -> Result<String, AsyncError> {
        let content = async_fs::read_to_string(path)
            .await
            .map_err(|e| AsyncError::IoError(format!("Failed to read file: {}", e)))?;
        
        Ok(content)
    }

    /// Write file asynchronously.
    pub async fn write_file_async(&self, path: &str, content: &str) -> Result<(), AsyncError> {
        async_fs::write(path, content)
            .await
            .map_err(|e| AsyncError::IoError(format!("Failed to write file: {}", e)))?;
        
        Ok(())
    }

    /// Check if file exists asynchronously.
    pub async fn file_exists_async(&self, path: &str) -> Result<bool, AsyncError> {
        let exists = async_fs::metadata(path)
            .await
            .is_ok();
        
        Ok(exists)
    }
}

impl Default for AsyncOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let async_ops = AsyncOps::new();
        let _ = async_ops;
    }
}
