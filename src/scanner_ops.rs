//! SigmaOS scanner operations module.
//!
//! This module provides actual scanner operations via scanimage.
//!
//! # Example
//!
//! ```
//! use sigma_system::scanner_ops::*;
//!
//! let scanner = ScannerOps::new();
//! if let Ok(status) = scanner.list_scanners() {
//!     println!("Scanners: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in scanner operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ScannerError {
    /// Scanner failed.
    ScannerError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScannerError::ScannerError(msg) => write!(f, "Failed to manage scanner: {}", msg),
            ScannerError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ScannerError {}

/// Scanner operations manager.
#[derive(Debug)]
pub struct ScannerOps;

impl ScannerOps {
    pub fn new() -> Self { Self }

    pub fn list_scanners(&self) -> Result<String, ScannerError> {
        let output = Command::new("scanimage")
            .arg("-L")
            .output()
            .map_err(|e| ScannerError::IoError(format!("Failed to run scanimage: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for ScannerOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let scanner = ScannerOps::new();
        let _ = scanner;
    }

    #[test]
    fn test_list_scanners() {
        let scanner = ScannerOps::new();
        if let Ok(_) = scanner.list_scanners() {
            // Should not panic
        }
    }
}
