//! SigmaOS touchscreen operations module.
//!
//! This module provides actual touchscreen operations via xinput.
//!
//! # Example
//!
//! ```
//! use sigma_system::touchscreen_ops::*;
//!
//! let touchscreen = TouchscreenOps::new();
//! if let Ok(status) = touchscreen.list_touchscreens() {
//!     println!("Touchscreens: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in touchscreen operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TouchscreenError {
    /// Touchscreen failed.
    TouchscreenError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for TouchscreenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TouchscreenError::TouchscreenError(msg) => write!(f, "Failed to manage touchscreen: {}", msg),
            TouchscreenError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for TouchscreenError {}

/// Touchscreen operations manager.
#[derive(Debug)]
pub struct TouchscreenOps;

impl TouchscreenOps {
    pub fn new() -> Self { Self }

    pub fn list_touchscreens(&self) -> Result<String, TouchscreenError> {
        let output = Command::new("xinput")
            .arg("list")
            .output()
            .map_err(|e| TouchscreenError::IoError(format!("Failed to run xinput: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for TouchscreenOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let touchscreen = TouchscreenOps::new();
        let _ = touchscreen;
    }

    #[test]
    fn test_list_touchscreens() {
        let touchscreen = TouchscreenOps::new();
        if let Ok(_) = touchscreen.list_touchscreens() {
            // Should not panic
        }
    }
}
