//! SigmaOS audio operations module.
//!
//! This module provides actual audio operations using PipeWire/PulseAudio.
//!
//! # Example
//!
//! ```
//! use sigma_system::audio_ops::*;
//!
//! let audio = AudioOps::new();
//! if let Ok(sinks) = audio.list_audio_sinks() {
//!     println!("Found {} audio sinks", sinks.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in audio operations.
#[derive(Debug, Clone, PartialEq)]
pub enum AudioError {
    /// Audio device not found.
    DeviceNotFound(String),
    /// Failed to manage audio.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioError::DeviceNotFound(name) => write!(f, "Audio device '{}' not found", name),
            AudioError::ManagementError(msg) => write!(f, "Failed to manage audio: {}", msg),
            AudioError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for AudioError {}

/// Audio sink information.
#[derive(Debug, Clone)]
pub struct AudioSink {
    /// Sink name.
    pub name: String,
    /// Sink description.
    pub description: String,
    /// Is default.
    pub is_default: bool,
}

/// Audio operations manager.
#[derive(Debug)]
pub struct AudioOps;

impl AudioOps {
    pub fn new() -> Self { Self }

    pub fn list_audio_sinks(&self) -> Result<Vec<AudioSink>, AudioError> {
        let mut sinks = Vec::new();

        if Command::new("pactl").arg("--version").output().is_ok() {
            let output = Command::new("pactl")
                .args(&["list", "sinks"])
                .output()
                .map_err(|e| AudioError::IoError(format!("Failed to run pactl: {}", e)))?;

            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("Name:") {
                    let name = line.split(':').nth(1).unwrap_or("").trim();
                    sinks.push(AudioSink {
                        name: name.to_string(),
                        description: "Audio Sink".to_string(),
                        is_default: false,
                    });
                }
            }
        }

        Ok(sinks)
    }

    pub fn get_audio_backend(&self) -> Result<String, AudioError> {
        if Command::new("pipewire").arg("--version").output().is_ok() {
            Ok("pipewire".to_string())
        } else if Command::new("pulseaudio").arg("--version").output().is_ok() {
            Ok("pulseaudio".to_string())
        } else {
            Ok("none".to_string())
        }
    }
}

impl Default for AudioOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let audio = AudioOps::new();
        let _ = audio;
    }

    #[test]
    fn test_get_audio_backend() {
        let audio = AudioOps::new();
        if let Ok(_) = audio.get_audio_backend() {
            // Should not panic
        }
    }
}
