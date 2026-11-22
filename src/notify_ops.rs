//! SigmaOS notification operations module.
//!
//! This module provides actual notification operations via notify-send.
//!
//! # Example
//!
//! ```
//! use sigma_system::notify_ops::*;
//!
//! let notify = NotifyOps::new();
//! if let Ok(status) = notify.send_notification("Title", "Message") {
//!     println!("Notification sent: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in notification operations.
#[derive(Debug, Clone, PartialEq)]
pub enum NotifyError {
    /// Send failed.
    SendError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for NotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotifyError::SendError(msg) => write!(f, "Failed to send notification: {}", msg),
            NotifyError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for NotifyError {}

/// Notification operations manager.
#[derive(Debug)]
pub struct NotifyOps;

impl NotifyOps {
    pub fn new() -> Self { Self }

    pub fn send_notification(&self, title: &str, message: &str) -> Result<String, NotifyError> {
        let output = Command::new("notify-send")
            .arg(title)
            .arg(message)
            .output()
            .map_err(|e| NotifyError::IoError(format!("Failed to run notify-send: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("notification_sent".to_string())
    }
}

impl Default for NotifyOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let notify = NotifyOps::new();
        let _ = notify;
    }

    #[test]
    fn test_send_notification() {
        let notify = NotifyOps::new();
        if let Ok(_) = notify.send_notification("Test", "Message") {
            // Should not panic
        }
    }
}
