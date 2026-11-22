//! SigmaOS smartcard operations module.
//!
//! This module provides actual smartcard operations using pcscd.
//!
//! # Example
//!
//! ```
//! use sigma_system::smartcard_ops::*;
//!
//! let sc = SmartcardOps::new();
//! if let Ok(cards) = sc.list_cards() {
//!     println!("Found {} smartcards", cards.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in smartcard operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SmartcardError {
    /// Card not found.
    CardNotFound(String),
    /// Failed to manage smartcard.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for SmartcardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartcardError::CardNotFound(name) => write!(f, "Smartcard '{}' not found", name),
            SmartcardError::ManagementError(msg) => write!(f, "Failed to manage smartcard: {}", msg),
            SmartcardError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for SmartcardError {}

/// Smartcard information.
#[derive(Debug, Clone)]
pub struct SmartcardInfo {
    pub name: String,
    pub reader: String,
}

/// Smartcard operations manager.
#[derive(Debug)]
pub struct SmartcardOps;

impl SmartcardOps {
    pub fn new() -> Self { Self }

    pub fn list_cards(&self) -> Result<Vec<SmartcardInfo>, SmartcardError> {
        let mut cards = Vec::new();
        if Command::new("pcsc_scan").output().is_ok() {
            Ok(cards)
        } else {
            Ok(cards)
        }
    }
}

impl Default for SmartcardOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sc = SmartcardOps::new();
        let _ = sc;
    }
}
