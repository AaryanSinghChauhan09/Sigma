//! SigmaOS printing operations module.
//!
//! This module provides actual printing operations using CUPS.
//!
//! # Example
//!
//! ```
//! use sigma_system::print_ops::*;
//!
//! let print = PrintOps::new();
//! if let Ok(printers) = print.list_printers() {
//!     println!("Found {} printers", printers.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in printing operations.
#[derive(Debug, Clone, PartialEq)]
pub enum PrintError {
    /// Printer not found.
    PrinterNotFound(String),
    /// Failed to manage printer.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for PrintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintError::PrinterNotFound(name) => write!(f, "Printer '{}' not found", name),
            PrintError::ManagementError(msg) => write!(f, "Failed to manage printer: {}", msg),
            PrintError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for PrintError {}

/// Printer information.
#[derive(Debug, Clone)]
pub struct PrinterInfo {
    /// Printer name.
    pub name: String,
    /// Printer location.
    pub location: String,
    /// Printer status.
    pub status: String,
}

/// Printing operations manager.
#[derive(Debug)]
pub struct PrintOps;

impl PrintOps {
    pub fn new() -> Self { Self }

    pub fn list_printers(&self) -> Result<Vec<PrinterInfo>, PrintError> {
        let mut printers = Vec::new();

        if Command::new("lpstat").arg("-p").output().is_ok() {
            let output = Command::new("lpstat")
                .args(&["-p"])
                .output()
                .map_err(|e| PrintError::IoError(format!("Failed to run lpstat: {}", e)))?;

            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("printer") {
                    let name = line.split_whitespace().nth(1).unwrap_or("unknown");
                    printers.push(PrinterInfo {
                        name: name.to_string(),
                        location: "local".to_string(),
                        status: "ready".to_string(),
                    });
                }
            }
        }

        Ok(printers)
    }

    pub fn is_cups_available(&self) -> bool {
        Command::new("lpstat").arg("-p").output().is_ok()
    }
}

impl Default for PrintOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let print = PrintOps::new();
        let _ = print;
    }

    #[test]
    fn test_is_cups_available() {
        let print = PrintOps::new();
        let _ = print.is_cups_available();
    }
}
