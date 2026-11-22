//! SigmaOS joystick operations module.
//!
//! This module provides actual joystick operations via jstest.
//!
//! # Example
//!
//! ```
//! use sigma_system::joystick_ops::*;
//!
//! let joystick = JoystickOps::new();
//! if let Ok(status) = joystick.list_joysticks() {
//!     println!("Joysticks: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in joystick operations.
#[derive(Debug, Clone, PartialEq)]
pub enum JoystickError {
    /// Joystick failed.
    JoystickError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for JoystickError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoystickError::JoystickError(msg) => write!(f, "Failed to manage joystick: {}", msg),
            JoystickError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for JoystickError {}

/// Joystick operations manager.
#[derive(Debug)]
pub struct JoystickOps;

impl JoystickOps {
    pub fn new() -> Self { Self }

    pub fn list_joysticks(&self) -> Result<String, JoystickError> {
        let output = Command::new("jstest")
            .arg("--list")
            .output()
            .map_err(|e| JoystickError::IoError(format!("Failed to run jstest: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for JoystickOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let joystick = JoystickOps::new();
        let _ = joystick;
    }

    #[test]
    fn test_list_joysticks() {
        let joystick = JoystickOps::new();
        if let Ok(_) = joystick.list_joysticks() {
            // Should not panic
        }
    }
}
