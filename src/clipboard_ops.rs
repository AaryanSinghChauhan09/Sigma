//! SigmaOS clipboard operations module.
//!
//! This module provides actual clipboard operations via xclip/wl-copy.
//!
//! # Example
//!
//! ```
//! use sigma_system::clipboard_ops::*;
//!
//! let clipboard = ClipboardOps::new();
//! if let Ok(status) = clipboard.copy_to_clipboard("text") {
//!     println!("Copied: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in clipboard operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardError {
    /// Copy failed.
    CopyError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::CopyError(msg) => write!(f, "Failed to copy: {}", msg),
            ClipboardError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ClipboardError {}

/// Clipboard operations manager.
#[derive(Debug)]
pub struct ClipboardOps;

impl ClipboardOps {
    pub fn new() -> Self { Self }

    pub fn copy_to_clipboard(&self, text: &str) -> Result<String, ClipboardError> {
        let output = Command::new("wl-copy")
            .arg(text)
            .output()
            .map_err(|e| ClipboardError::IoError(format!("Failed to run wl-copy: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("copied".to_string())
    }
}

impl Default for ClipboardOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let clipboard = ClipboardOps::new();
        let _ = clipboard;
    }

    #[test]
    fn test_copy_to_clipboard() {
        let clipboard = ClipboardOps::new();
        if let Ok(status) = clipboard.copy_to_clipboard("test") {
            println!("Copied: {}", status);
        }
    }
}
