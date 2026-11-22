//! SigmaOS gamepad operations module.
//!
//! This module provides actual gamepad operations via evtest.
//!
//! # Example
//!
//! ```
//! use sigma_system::gamepad_ops::*;
//!
//! let gamepad = GamepadOps::new();
//! if let Ok(status) = gamepad.list_gamepads() {
//!     println!("Gamepads: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in gamepad operations.
#[derive(Debug, Clone, PartialEq)]
pub enum GamepadError {
    /// Gamepad failed.
    GamepadError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for GamepadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GamepadError::GamepadError(msg) => write!(f, "Failed to manage gamepad: {}", msg),
            GamepadError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for GamepadError {}

/// Gamepad operations manager.
#[derive(Debug)]
pub struct GamepadOps;

impl GamepadOps {
    pub fn new() -> Self { Self }

    pub fn list_gamepads(&self) -> Result<String, GamepadError> {
        let output = Command::new("evtest")
            .arg("--list")
            .output()
            .map_err(|e| GamepadError::IoError(format!("Failed to run evtest: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for GamepadOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gamepad = GamepadOps::new();
        let _ = gamepad;
    }

    #[test]
    fn test_list_gamepads() {
        let gamepad = GamepadOps::new();
        if let Ok(_) = gamepad.list_gamepads() {
            // Should not panic
        }
    }
}
