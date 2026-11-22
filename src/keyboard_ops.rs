//! SigmaOS keyboard operations module.
//!
//! This module provides actual keyboard and input device operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::keyboard_ops::*;
//!
//! let keyboard = KeyboardOps::new();
//! if let Ok(layouts) = keyboard.list_keymaps() {
//!     println!("Found {} keymaps", layouts.len());
//! }
//! ```

use std::fs;
use std::path::Path;

/// Errors that can occur in keyboard operations.
#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardError {
    /// Keymap not found.
    KeymapNotFound(String),
    /// Failed to set keyboard.
    SetError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for KeyboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardError::KeymapNotFound(name) => write!(f, "Keymap '{}' not found", name),
            KeyboardError::SetError(msg) => write!(f, "Failed to set keyboard: {}", msg),
            KeyboardError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for KeyboardError {}

/// Keyboard operations manager.
#[derive(Debug)]
pub struct KeyboardOps;

impl KeyboardOps {
    pub fn new() -> Self { Self }

    pub fn list_keymaps(&self) -> Result<Vec<String>, KeyboardError> {
        let path = Path::new("/usr/share/kbd/keymaps");
        if !path.exists() {
            return Ok(vec!["us".to_string()]);
        }

        let mut keymaps = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    if name.ends_with(".map.gz") || name.ends_with(".map") {
                        keymaps.push(name);
                    }
                }
            }
        }

        Ok(keymaps)
    }

    pub fn get_current_keymap(&self) -> Result<String, KeyboardError> {
        let path = Path::new("/etc/vconsole.conf");
        if !path.exists() {
            return Ok("us".to_string());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| KeyboardError::IoError(format!("Failed to read vconsole.conf: {}", e)))?;

        for line in content.lines() {
            if line.starts_with("KEYMAP=") {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() > 1 {
                    return Ok(parts[1].trim().to_string());
                }
            }
        }

        Ok("us".to_string())
    }
}

impl Default for KeyboardOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let keyboard = KeyboardOps::new();
        let _ = keyboard;
    }

    #[test]
    fn test_get_current_keymap() {
        let keyboard = KeyboardOps::new();
        if let Ok(_) = keyboard.get_current_keymap() {
            // Should not panic
        }
    }
}
