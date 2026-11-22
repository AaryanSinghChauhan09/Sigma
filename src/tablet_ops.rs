//! SigmaOS tablet operations module.
//!
//! This module provides actual tablet operations via xsetwacom.
//!
//! # Example
//!
//! ```
//! use sigma_system::tablet_ops::*;
//!
//! let tablet = TabletOps::new();
//! if let Ok(status) = tablet.list_tablets() {
//!     println!("Tablets: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in tablet operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TabletError {
    /// Tablet failed.
    TabletError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for TabletError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TabletError::TabletError(msg) => write!(f, "Failed to manage tablet: {}", msg),
            TabletError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for TabletError {}

/// Tablet operations manager.
#[derive(Debug)]
pub struct TabletOps;

impl TabletOps {
    pub fn new() -> Self { Self }

    pub fn list_tablets(&self) -> Result<String, TabletError> {
        let output = Command::new("xsetwacom")
            .arg("list")
            .output()
            .map_err(|e| TabletError::IoError(format!("Failed to run xsetwacom: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for TabletOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tablet = TabletOps::new();
        let _ = tablet;
    }

    #[test]
    fn test_list_tablets() {
        let tablet = TabletOps::new();
        if let Ok(_) = tablet.list_tablets() {
            // Should not panic
        }
    }
}
