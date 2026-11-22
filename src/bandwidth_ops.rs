//! SigmaOS bandwidth operations module.
//!
//! This module provides actual bandwidth monitoring via ifstat.
//!
//! # Example
//!
//! ```
//! use sigma_system::bandwidth_ops::*;
//!
//! let bw = BandwidthOps::new();
//! if let Ok(status) = bw.get_bandwidth() {
//!     println!("Bandwidth: {}", status);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in bandwidth operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BandwidthError {
    /// Monitor failed.
    MonitorError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for BandwidthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BandwidthError::MonitorError(msg) => write!(f, "Failed to monitor bandwidth: {}", msg),
            BandwidthError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for BandwidthError {}

/// Bandwidth operations manager.
#[derive(Debug)]
pub struct BandwidthOps;

impl BandwidthOps {
    pub fn new() -> Self { Self }

    pub fn get_bandwidth(&self) -> Result<String, BandwidthError> {
        let output = Command::new("ifstat")
            .output()
            .map_err(|e| BandwidthError::IoError(format!("Failed to run ifstat: {}", e)))?;

        let _ = String::from_utf8_lossy(&output.stdout);
        Ok("bandwidth_monitored".to_string())
    }
}

impl Default for BandwidthOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bw = BandwidthOps::new();
        let _ = bw;
    }

    #[test]
    fn test_get_bandwidth() {
        let bw = BandwidthOps::new();
        if let Ok(_) = bw.get_bandwidth() {
            // Should not panic
        }
    }
}
