//! SigmaOS Bluetooth operations module.
//!
//! This module provides actual Bluetooth operations via bluetoothctl.
//!
//! # Example
//!
//! ```
//! use sigma_system::bluetooth_ops::*;
//!
//! let bt = BluetoothOps::new();
//! if let Ok(status) = bt.list_devices() {
//!     println!("Bluetooth devices: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in Bluetooth operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BluetoothError {
    /// Bluetooth failed.
    BluetoothError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for BluetoothError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BluetoothError::BluetoothError(msg) => write!(f, "Failed to manage Bluetooth: {}", msg),
            BluetoothError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for BluetoothError {}

/// Bluetooth operations manager.
#[derive(Debug)]
pub struct BluetoothOps;

impl BluetoothOps {
    pub fn new() -> Self { Self }

    pub fn list_devices(&self) -> Result<String, BluetoothError> {
        let output = Command::new("bluetoothctl")
            .args(&["devices"])
            .output()
            .map_err(|e| BluetoothError::IoError(format!("Failed to run bluetoothctl: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }
}

impl Default for BluetoothOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bt = BluetoothOps::new();
        let _ = bt;
    }

    #[test]
    fn test_list_devices() {
        let bt = BluetoothOps::new();
        if let Ok(_) = bt.list_devices() {
            // Should not panic
        }
    }
}
