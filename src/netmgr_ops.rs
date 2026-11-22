//! SigmaOS network manager operations module.
//!
//! This module provides network management operations including:
//! - Network connection management
//! - WiFi configuration
//! - Network interface management
//!
//! Follows Single Responsibility Principle: only manages network operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::netmgr_ops::*;
//!
//! let netmgr = NetMgrOps::new();
//! if let Ok(status) = netmgr.list_connections() {
//!     println!("Connections: {}", status);
//! }
//! ```

use std::process::Command;

/// Network manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum NetMgrError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for NetMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            NetMgrError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for NetMgrError {}

/// Network manager operations manager.
#[derive(Debug)]
pub struct NetMgrOps;

impl NetMgrOps {
    pub fn new() -> Self { Self }

    /// List network connections.
    pub fn list_connections(&self) -> Result<String, NetMgrError> {
        let output = Command::new("nmcli")
            .args(&["connection", "show"])
            .output()
            .map_err(|e| NetMgrError::CommandError(format!("Failed to run nmcli: {}", e)))?;

        let connections = String::from_utf8_lossy(&output.stdout);
        Ok(connections.trim().to_string())
    }

    /// Connect to a network.
    pub fn connect(&self, connection: &str) -> Result<(), NetMgrError> {
        let output = Command::new("nmcli")
            .args(&["connection", "up", connection])
            .output()
            .map_err(|e| NetMgrError::CommandError(format!("Failed to run nmcli: {}", e)))?;

        if !output.status.success() {
            return Err(NetMgrError::CommandError("nmcli connect failed".to_string()));
        }

        Ok(())
    }

    /// Disconnect from a network.
    pub fn disconnect(&self, connection: &str) -> Result<(), NetMgrError> {
        let output = Command::new("nmcli")
            .args(&["connection", "down", connection])
            .output()
            .map_err(|e| NetMgrError::CommandError(format!("Failed to run nmcli: {}", e)))?;

        if !output.status.success() {
            return Err(NetMgrError::CommandError("nmcli disconnect failed".to_string()));
        }

        Ok(())
    }
}

impl Default for NetMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let netmgr = NetMgrOps::new();
        let _ = netmgr;
    }

    #[test]
    fn test_list_connections() {
        let netmgr = NetMgrOps::new();
        if let Ok(connections) = netmgr.list_connections() {
            // Should not panic
            let _ = connections;
        }
    }
}
