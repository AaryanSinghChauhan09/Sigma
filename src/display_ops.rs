//! SigmaOS display manager operations module.
//!
//! This module provides actual display manager operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::display_ops::*;
//!
//! let display = DisplayOps::new();
//! if let Ok(current) = display.get_display_manager() {
//!     println!("Display manager: {}", current);
//! }
//! ```

use std::fs;
use std::path::Path;
use std::process::Command;

/// Errors that can occur in display operations.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayError {
    /// Display manager not found.
    DisplayManagerNotFound(String),
    /// Failed to set display.
    SetError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::DisplayManagerNotFound(name) => write!(f, "Display manager '{}' not found", name),
            DisplayError::SetError(msg) => write!(f, "Failed to set display: {}", msg),
            DisplayError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for DisplayError {}

/// Display operations manager.
#[derive(Debug)]
pub struct DisplayOps {
    is_root: bool,
}

impl DisplayOps {
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);
        Self { is_root }
    }

    pub fn is_root(&self) -> bool { self.is_root }

    pub fn get_display_manager(&self) -> Result<String, DisplayError> {
        let managers = vec!["gdm", "sddm", "lightdm", "lxdm"];
        
        for manager in managers {
            let path = format!("/usr/lib/systemd/system/{}.service", manager);
            if Path::new(&path).exists() {
                return Ok(manager.to_string());
            }
        }

        Ok("none".to_string())
    }

    pub fn list_available_managers(&self) -> Result<Vec<String>, DisplayError> {
        let mut available = Vec::new();
        let managers = vec!["gdm", "sddm", "lightdm", "lxdm"];
        
        for manager in managers {
            if Command::new("which").arg(manager).output().is_ok() {
                available.push(manager.to_string());
            }
        }

        Ok(available)
    }

    pub fn get_display_server(&self) -> Result<String, DisplayError> {
        let xorg = Path::new("/usr/bin/X");
        let wayland = Path::new("/usr/bin/weston");

        if wayland.exists() {
            Ok("wayland".to_string())
        } else if xorg.exists() {
            Ok("x11".to_string())
        } else {
            Ok("none".to_string())
        }
    }
}

impl Default for DisplayOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let display = DisplayOps::new();
        let _ = display.is_root();
    }

    #[test]
    fn test_get_display_manager() {
        let display = DisplayOps::new();
        if let Ok(_) = display.get_display_manager() {
            // Should not panic
        }
    }
}
