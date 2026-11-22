//! SigmaOS swap operations module.
//!
//! This module provides actual swap management operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::swap_ops::*;
//!
//! let swap = SwapOps::new();
//! if let Ok(swaps) = swap.list_swaps() {
//!     println!("Found {} swap devices", swaps.len());
//! }
//! ```

use std::fs;
use std::path::Path;

/// Errors that can occur in swap operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SwapError {
    /// Swap device not found.
    SwapNotFound(String),
    /// Failed to manage swap.
    ManagementError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for SwapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwapError::SwapNotFound(device) => write!(f, "Swap device '{}' not found", device),
            SwapError::ManagementError(msg) => write!(f, "Failed to manage swap: {}", msg),
            SwapError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            SwapError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for SwapError {}

/// Swap information.
#[derive(Debug, Clone)]
pub struct SwapInfo {
    /// Swap device path.
    pub device: String,
    /// Swap type.
    pub swap_type: String,
    /// Total size in bytes.
    pub total_bytes: u64,
    /// Used bytes.
    pub used_bytes: u64,
    /// Priority.
    pub priority: i32,
}

/// Swap operations manager.
#[derive(Debug)]
pub struct SwapOps;

impl SwapOps {
    pub fn new() -> Self { Self }

    pub fn list_swaps(&self) -> Result<Vec<SwapInfo>, SwapError> {
        let path = Path::new("/proc/swaps");
        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| SwapError::IoError(format!("Failed to read swaps: {}", e)))?;

        let mut swaps = Vec::new();
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let total = parts[2].parse::<u64>().unwrap_or(0) * 1024;
                let used = parts[3].parse::<u64>().unwrap_or(0) * 1024;
                let priority = parts[4].parse::<i32>().unwrap_or(-1);

                swaps.push(SwapInfo {
                    device: parts[0].to_string(),
                    swap_type: parts[1].to_string(),
                    total_bytes: total,
                    used_bytes: used,
                    priority,
                });
            }
        }

        Ok(swaps)
    }

    pub fn get_swap_usage(&self) -> Result<(u64, u64), SwapError> {
        let swaps = self.list_swaps()?;
        let mut total = 0u64;
        let mut used = 0u64;

        for swap in swaps {
            total += swap.total_bytes;
            used += swap.used_bytes;
        }

        Ok((total, used))
    }

    pub fn get_swap_percentage(&self) -> Result<f64, SwapError> {
        let (total, used) = self.get_swap_usage()?;
        if total == 0 {
            Ok(0.0)
        } else {
            Ok((used as f64 / total as f64) * 100.0)
        }
    }
}

impl Default for SwapOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let swap = SwapOps::new();
        let _ = swap;
    }

    #[test]
    fn test_list_swaps() {
        let swap = SwapOps::new();
        if let Ok(_) = swap.list_swaps() {
            // Should not panic
        }
    }
}
