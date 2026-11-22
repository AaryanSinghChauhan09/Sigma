//! SigmaOS scheduler operations module (extended).
//!
//! This module provides actual CPU scheduler operations via /proc.
//!
//! # Example
//!
//! ```
//! use sigma_system::scheduler_ops::*;
//!
//! let scheduler = SchedulerOps::new();
//! if let Ok(policy) = scheduler.get_scheduler_policy() {
//!     println!("Scheduler policy: {}", policy);
//! }
//! ```

use std::fs;

/// Errors that can occur in scheduler operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerError {
    /// Read failed.
    ReadError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchedulerError::ReadError(msg) => write!(f, "Failed to read scheduler: {}", msg),
            SchedulerError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for SchedulerError {}

/// Scheduler operations manager.
#[derive(Debug)]
pub struct SchedulerOps;

impl SchedulerOps {
    pub fn new() -> Self { Self }

    pub fn get_scheduler_policy(&self) -> Result<String, SchedulerError> {
        let content = fs::read_to_string("/proc/sys/kernel/sched_rr_timeslice_ms")
            .map_err(|e| SchedulerError::ReadError(format!("Failed to read scheduler: {}", e)))?;
        Ok(content.trim().to_string())
    }
}

impl Default for SchedulerOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let scheduler = SchedulerOps::new();
        let _ = scheduler;
    }

    #[test]
    fn test_get_scheduler_policy() {
        let scheduler = SchedulerOps::new();
        if let Ok(_) = scheduler.get_scheduler_policy() {
            // Should not panic
        }
    }
}
