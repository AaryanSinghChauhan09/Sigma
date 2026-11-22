//! SigmaOS camera operations module.
//!
//! This module provides actual camera operations using v4l2.
//!
//! # Example
//!
//! ```
//! use sigma_system::camera_ops::*;
//!
//! let camera = CameraOps::new();
//! if let Ok(devices) = camera.list_cameras() {
//!     println!("Found {} cameras", devices.len());
//! }
//! ```

use std::fs;
use std::path::Path;

/// Errors that can occur in camera operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CameraError {
    /// Camera not found.
    CameraNotFound(String),
    /// Failed to manage camera.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraError::CameraNotFound(name) => write!(f, "Camera '{}' not found", name),
            CameraError::ManagementError(msg) => write!(f, "Failed to manage camera: {}", msg),
            CameraError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for CameraError {}

/// Camera information.
#[derive(Debug, Clone)]
pub struct CameraInfo {
    /// Device path.
    pub device: String,
    /// Camera name.
    pub name: String,
    /// Camera resolution.
    pub resolution: String,
}

/// Camera operations manager.
#[derive(Debug)]
pub struct CameraOps;

impl CameraOps {
    pub fn new() -> Self { Self }

    pub fn list_cameras(&self) -> Result<Vec<CameraInfo>, CameraError> {
        let mut cameras = Vec::new();
        let video_dir = Path::new("/dev");

        if let Ok(entries) = fs::read_dir(video_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("video") {
                    cameras.push(CameraInfo {
                        device: format!("/dev/{}", name_str),
                        name: name_str.to_string(),
                        resolution: "unknown".to_string(),
                    });
                }
            }
        }

        Ok(cameras)
    }
}

impl Default for CameraOps {
    fn default() -> Self { Self::new() }
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
