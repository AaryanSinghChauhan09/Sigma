//! SigmaOS scheduler operations module.
//!
//! This module provides actual CPU scheduler operations.
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
use std::path::Path;

/// Errors that can occur in scheduler operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerError {
    /// Process not found.
    ProcessNotFound(u32),
    /// Failed to set scheduler.
    SetError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchedulerError::ProcessNotFound(pid) => write!(f, "Process {} not found", pid),
            SchedulerError::SetError(msg) => write!(f, "Failed to set scheduler: {}", msg),
            SchedulerError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for SchedulerError {}

/// Scheduler policy.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerPolicy {
    Normal,
    Fifo,
    RoundRobin,
    Batch,
    Idle,
    Deadline,
}

impl std::fmt::Display for SchedulerPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchedulerPolicy::Normal => write!(f, "SCHED_NORMAL"),
            SchedulerPolicy::Fifo => write!(f, "SCHED_FIFO"),
            SchedulerPolicy::RoundRobin => write!(f, "SCHED_RR"),
            SchedulerPolicy::Batch => write!(f, "SCHED_BATCH"),
            SchedulerPolicy::Idle => write!(f, "SCHED_IDLE"),
            SchedulerPolicy::Deadline => write!(f, "SCHED_DEADLINE"),
        }
    }
}

/// Scheduler operations manager.
#[derive(Debug)]
pub struct SchedulerOps;

impl SchedulerOps {
    pub fn new() -> Self { Self }

    pub fn get_scheduler_policy(&self) -> Result<SchedulerPolicy, SchedulerError> {
        let path = Path::new("/proc/self/sched");
        if !path.exists() {
            return Ok(SchedulerPolicy::Normal);
        }

        let content = fs::read_to_string(path)
            .map_err(|e| SchedulerError::IoError(format!("Failed to read sched: {}", e)))?;

        if content.contains("SCHED_FIFO") {
            Ok(SchedulerPolicy::Fifo)
        } else if content.contains("SCHED_RR") {
            Ok(SchedulerPolicy::RoundRobin)
        } else if content.contains("SCHED_BATCH") {
            Ok(SchedulerPolicy::Batch)
        } else if content.contains("SCHED_IDLE") {
            Ok(SchedulerPolicy::Idle)
        } else if content.contains("SCHED_DEADLINE") {
            Ok(SchedulerPolicy::Deadline)
        } else {
            Ok(SchedulerPolicy::Normal)
        }
    }

    pub fn get_cpu_affinity(&self, pid: u32) -> Result<Vec<usize>, SchedulerError> {
        let path = format!("/proc/{}/status", pid);
        let content = fs::read_to_string(&path)
            .map_err(|e| SchedulerError::IoError(format!("Failed to read status: {}", e)))?;

        for line in content.lines() {
            if line.starts_with("Cpus_allowed:") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() > 1 {
                    let hex = parts[1].trim();
                    let mask = u64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0);
                    let mut cpus = Vec::new();
                    for i in 0..64 {
                        if mask & (1 << i) != 0 {
                            cpus.push(i);
                        }
                    }
                    return Ok(cpus);
                }
            }
        }

        Ok(vec![0])
    }
}

impl Default for SchedulerOps {
    fn default() -> Self { Self::new() }
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
