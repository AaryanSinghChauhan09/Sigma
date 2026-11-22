//! SigmaOS printer operations module.
//!
//! This module provides actual printer operations via lp.
//!
//! # Example
//!
//! ```
//! use sigma_system::printer_ops::*;
//!
//! let printer = PrinterOps::new();
//! if let Ok(status) = printer.list_printers() {
//!     println!("Printers: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in printer operations.
#[derive(Debug, Clone, PartialEq)]
pub enum PrinterError {
    /// Printer failed.
    PrinterError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for PrinterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrinterError::PrinterError(msg) => write!(f, "Failed to manage printer: {}", msg),
            PrinterError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for PrinterError {}

/// Printer operations manager.
#[derive(Debug)]
pub struct PrinterOps;

impl PrinterOps {
    pub fn new() -> Self { Self }

    pub fn list_printers(&self) -> Result<String, PrinterError> {
        let output = Command::new("lpstat")
            .arg("-p")
            .output()
            .map_err(|e| PrinterError::IoError(format!("Failed to run lpstat: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for PrinterOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let printer = PrinterOps::new();
        let _ = printer;
    }

    #[test]
    fn test_list_printers() {
        let printer = PrinterOps::new();
        if let Ok(_) = printer.list_printers() {
            // Should not panic
        }
    }
}
