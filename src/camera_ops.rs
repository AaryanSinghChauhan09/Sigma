//! SigmaOS camera operations module.
//!
//! This module provides actual camera operations via fswebcam.
//!
//! # Example
//!
//! ```
//! use sigma_system::camera_ops::*;
//!
//! let camera = CameraOps::new();
//! if let Ok(status) = camera.list_cameras() {
//!     println!("Cameras: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in camera operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CameraError {
    /// Camera failed.
    CameraError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraError::CameraError(msg) => write!(f, "Failed to manage camera: {}", msg),
            CameraError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for CameraError {}

/// Camera operations manager.
#[derive(Debug)]
pub struct CameraOps;

impl CameraOps {
    pub fn new() -> Self { Self }

    pub fn list_cameras(&self) -> Result<String, CameraError> {
        let output = Command::new("fswebcam")
            .arg("--list")
            .output()
            .map_err(|e| CameraError::IoError(format!("Failed to run fswebcam: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for CameraOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let camera = CameraOps::new();
        let _ = camera;
    }

    #[test]
    fn test_list_cameras() {
        let camera = CameraOps::new();
        if let Ok(_) = camera.list_cameras() {
            // Should not panic
        }
    }
}
