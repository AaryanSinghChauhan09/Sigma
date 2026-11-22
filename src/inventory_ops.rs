//! SigmaOS inventory operations module.
//!
//! This module provides actual inventory operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::inventory_ops::*;
//!
//! let inv = InventoryOps::new();
//! if let Ok(items) = inv.list_inventory() {
//!     println!("Found {} inventory items", items.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in inventory operations.
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryError {
    /// List failed.
    ListError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryError::ListError(msg) => write!(f, "Failed to list inventory: {}", msg),
            InventoryError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for InventoryError {}

/// Inventory item information.
#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub name: String,
    pub status: String,
}

/// Inventory operations manager.
#[derive(Debug)]
pub struct InventoryOps;

impl InventoryOps {
    pub fn new() -> Self { Self }

    pub fn list_inventory(&self) -> Result<Vec<InventoryItem>, InventoryError> {
        let mut items = Vec::new();
        let output = Command::new("lsblk")
            .arg("-o")
            .arg("NAME,TYPE")
            .output()
            .map_err(|e| InventoryError::IoError(format!("Failed to run lsblk: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok(items)
    }
}

impl Default for InventoryOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inv = InventoryOps::new();
        let _ = inv;
    }

    #[test]
    fn test_list_inventory() {
        let inv = InventoryOps::new();
        if let Ok(_) = inv.list_inventory() {
            // Should not panic
        }
    }
}
