//! SigmaOS screenshot operations module.
//!
//! This module provides actual screenshot operations via grim/slop.
//!
//! # Example
//!
//! ```
//! use sigma_system::screenshot_ops::*;
//!
//! let screenshot = ScreenshotOps::new();
//! if let Ok(status) = screenshot.take_screenshot("/tmp/screenshot.png") {
//!     println!("Screenshot saved: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in screenshot operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ScreenshotError {
    /// Capture failed.
    CaptureError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ScreenshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScreenshotError::CaptureError(msg) => write!(f, "Failed to capture screenshot: {}", msg),
            ScreenshotError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ScreenshotError {}

/// Screenshot operations manager.
#[derive(Debug)]
pub struct ScreenshotOps;

impl ScreenshotOps {
    pub fn new() -> Self { Self }

    pub fn take_screenshot(&self, path: &str) -> Result<String, ScreenshotError> {
        let output = Command::new("grim")
            .arg(path)
            .output()
            .map_err(|e| ScreenshotError::IoError(format!("Failed to run grim: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("screenshot_saved".to_string())
    }
}

impl Default for ScreenshotOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let screenshot = ScreenshotOps::new();
        let _ = screenshot;
    }

    #[test]
    fn test_take_screenshot() {
        let screenshot = ScreenshotOps::new();
        if let Ok(status) = screenshot.take_screenshot("/tmp/test.png") {
            println!("Screenshot saved: {}", status);
        }
    }
}
