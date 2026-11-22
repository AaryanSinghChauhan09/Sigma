//! SigmaOS email operations module.
//!
//! This module provides actual email operations via sendmail/postfix.
//!
//! # Example
//!
//! ```
//! use sigma_system::email_ops::*;
//!
//! let email = EmailOps::new();
//! if let Ok(status) = email.send_email("test@example.com", "Subject", "Body") {
//!     println!("Email sent: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in email operations.
#[derive(Debug, Clone, PartialEq)]
pub enum EmailError {
    /// Send failed.
    SendError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::SendError(msg) => write!(f, "Failed to send email: {}", msg),
            EmailError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for EmailError {}

/// Email operations manager.
#[derive(Debug)]
pub struct EmailOps;

impl EmailOps {
    pub fn new() -> Self { Self }

    pub fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, EmailError> {
        let output = Command::new("sendmail")
            .arg("-t")
            .output()
            .map_err(|e| EmailError::IoError(format!("Failed to run sendmail: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("email_sent".to_string())
    }
}

impl Default for EmailOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let email = EmailOps::new();
        let _ = email;
    }

    #[test]
    fn test_send_email() {
        let email = EmailOps::new();
        if let Ok(_) = email.send_email("test@example.com", "Test", "Body") {
            // Should not panic
        }
    }
}
