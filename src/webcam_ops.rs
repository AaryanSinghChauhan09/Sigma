//! SigmaOS webcam operations module.
//!
//! This module provides actual webcam operations via v4l2-ctl.
//!
//! # Example
//!
//! ```
//! use sigma_system::webcam_ops::*;
//!
//! let webcam = WebcamOps::new();
//! if let Ok(status) = webcam.list_cameras() {
//!     println!("Webcams: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in webcam operations.
#[derive(Debug, Clone, PartialEq)]
pub enum WebcamError {
    /// Webcam failed.
    WebcamError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for WebcamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebcamError::WebcamError(msg) => write!(f, "Failed to manage webcam: {}", msg),
            WebcamError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for WebcamError {}

/// Webcam operations manager.
#[derive(Debug)]
pub struct WebcamOps;

impl WebcamOps {
    pub fn new() -> Self { Self }

    pub fn list_cameras(&self) -> Result<String, WebcamError> {
        let output = Command::new("v4l2-ctl")
            .args(&["--list-devices"])
            .output()
            .map_err(|e| WebcamError::IoError(format!("Failed to run v4l2-ctl: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for WebcamOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let webcam = WebcamOps::new();
        let _ = webcam;
    }

    #[test]
    fn test_list_cameras() {
        let webcam = WebcamOps::new();
        if let Ok(_) = webcam.list_cameras() {
            // Should not panic
        }
    }
}
