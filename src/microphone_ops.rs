//! SigmaOS microphone operations module.
//!
//! This module provides actual microphone operations via arecord.
//!
//! # Example
//!
//! ```
//! use sigma_system::microphone_ops::*;
//!
//! let mic = MicrophoneOps::new();
//! if let Ok(status) = mic.list_microphones() {
//!     println!("Microphones: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in microphone operations.
#[derive(Debug, Clone, PartialEq)]
pub enum MicrophoneError {
    /// Microphone failed.
    MicrophoneError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for MicrophoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MicrophoneError::MicrophoneError(msg) => write!(f, "Failed to manage microphone: {}", msg),
            MicrophoneError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for MicrophoneError {}

/// Microphone operations manager.
#[derive(Debug)]
pub struct MicrophoneOps;

impl MicrophoneOps {
    pub fn new() -> Self { Self }

    pub fn list_microphones(&self) -> Result<String, MicrophoneError> {
        let output = Command::new("arecord")
            .args(&["-l"])
            .output()
            .map_err(|e| MicrophoneError::IoError(format!("Failed to run arecord: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for MicrophoneOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mic = MicrophoneOps::new();
        let _ = mic;
    }

    #[test]
    fn test_list_microphones() {
        let mic = MicrophoneOps::new();
        if let Ok(_) = mic.list_microphones() {
            // Should not panic
        }
    }
}
