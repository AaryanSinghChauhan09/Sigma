//! SigmaOS service manager operations module.
//!
//! This module provides service management operations including:
//! - Service start/stop/restart
//! - Service status monitoring
//! - Service enable/disable
//!
//! Follows Single Responsibility Principle: only manages service operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::servicemgr_ops::*;
//!
//! let servicemgr = ServiceMgrOps::new();
//! if let Ok(_) = servicemgr.start_service("nginx") {
//!     println!("Service started");
//! }
//! ```

use std::process::Command;

/// Service manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceMgrError {
    /// Command execution failed.
    CommandError(String),
    /// Service not found.
    ServiceNotFound(String),
}

impl std::fmt::Display for ServiceMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            ServiceMgrError::ServiceNotFound(msg) => write!(f, "Service not found: {}", msg),
        }
    }
}

impl std::error::Error for ServiceMgrError {}

/// Service manager operations manager.
#[derive(Debug)]
pub struct ServiceMgrOps;

impl ServiceMgrOps {
    pub fn new() -> Self { Self }

    /// Start a service.
    pub fn start_service(&self, service: &str) -> Result<(), ServiceMgrError> {
        let output = Command::new("systemctl")
            .args(&["start", service])
            .output()
            .map_err(|e| ServiceMgrError::CommandError(format!("Failed to run systemctl: {}", e)))?;

        if !output.status.success() {
            return Err(ServiceMgrError::CommandError("systemctl start failed".to_string()));
        }

        Ok(())
    }

    /// Stop a service.
    pub fn stop_service(&self, service: &str) -> Result<(), ServiceMgrError> {
        let output = Command::new("systemctl")
            .args(&["stop", service])
            .output()
            .map_err(|e| ServiceMgrError::CommandError(format!("Failed to run systemctl: {}", e)))?;

        if !output.status.success() {
            return Err(ServiceMgrError::CommandError("systemctl stop failed".to_string()));
        }

        Ok(())
    }

    /// Restart a service.
    pub fn restart_service(&self, service: &str) -> Result<(), ServiceMgrError> {
        let output = Command::new("systemctl")
            .args(&["restart", service])
            .output()
            .map_err(|e| ServiceMgrError::CommandError(format!("Failed to run systemctl: {}", e)))?;

        if !output.status.success() {
            return Err(ServiceMgrError::CommandError("systemctl restart failed".to_string()));
        }

        Ok(())
    }

    /// Enable a service.
    pub fn enable_service(&self, service: &str) -> Result<(), ServiceMgrError> {
        let output = Command::new("systemctl")
            .args(&["enable", service])
            .output()
            .map_err(|e| ServiceMgrError::CommandError(format!("Failed to run systemctl: {}", e)))?;

        if !output.status.success() {
            return Err(ServiceMgrError::CommandError("systemctl enable failed".to_string()));
        }

        Ok(())
    }

    /// Disable a service.
    pub fn disable_service(&self, service: &str) -> Result<(), ServiceMgrError> {
        let output = Command::new("systemctl")
            .args(&["disable", service])
            .output()
            .map_err(|e| ServiceMgrError::CommandError(format!("Failed to run systemctl: {}", e)))?;

        if !output.status.success() {
            return Err(ServiceMgrError::CommandError("systemctl disable failed".to_string()));
        }

        Ok(())
    }
}

impl Default for ServiceMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let servicemgr = ServiceMgrOps::new();
        let _ = servicemgr;
    }
}
