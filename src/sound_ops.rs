//! SigmaOS sound operations module.
//!
//! This module provides actual sound operations via pactl.
//!
//! # Example
//!
//! ```
//! use sigma_system::sound_ops::*;
//!
//! let sound = SoundOps::new();
//! if let Ok(status) = sound.get_volume() {
//!     println!("Volume: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in sound operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SoundError {
    /// Operation failed.
    OperationError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for SoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundError::OperationError(msg) => write!(f, "Failed sound operation: {}", msg),
            SoundError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for SoundError {}

/// Sound operations manager.
#[derive(Debug)]
pub struct SoundOps;

impl SoundOps {
    pub fn new() -> Self { Self }

    pub fn get_volume(&self) -> Result<String, SoundError> {
        let output = Command::new("pactl")
            .args(&["get-sink-volume", "@DEFAULT_SINK@"])
            .output()
            .map_err(|e| SoundError::IoError(format!("Failed to run pactl: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for SoundOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sound = SoundOps::new();
        let _ = sound;
    }

    #[test]
    fn test_get_volume() {
        let sound = SoundOps::new();
        if let Ok(_) = sound.get_volume() {
            // Should not panic
        }
    }
}
