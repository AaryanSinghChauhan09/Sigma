//! SigmaOS instrumentation module.
//!
//! This module provides instrumentation for monitoring and logging system events.
//!
//! Follows principles: Instrumentation, Log Everything, Defensive Programming
//!
//! # Example
//!
//! ```
//! use sigma_system::instrumentation_ops::*;
//!
//! let instr = InstrumentationOps::new();
//! instr.log_info("System started");
//! ```

use std::fs::OpenOptions;
use std::io::Write;

/// Instrumentation operations manager.
#[derive(Debug)]
pub struct InstrumentationOps {
    log_file: String,
}

impl InstrumentationOps {
    pub fn new() -> Self {
        Self {
            log_file: "/var/log/sigmaos/instrumentation.log".to_string(),
        }
    }

    /// Log info.
    pub fn log_info(&self, info: &str) {
        let timestamp = std::env::var("SIGMAOS_LOG_TIMESTAMP").unwrap_or_else(|_| {
            std::process::Command::new("date")
                .arg("+%Y-%m-%d %H:%M:%S")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "unknown".to_string())
        });
        let log_entry = format!("[{}] [INFO] {}\n", timestamp, info);
        
        let _ = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
            .and_then(|mut file| file.write_all(log_entry.as_bytes()));
    }
}

impl Default for InstrumentationOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let instr = InstrumentationOps::new();
        let _ = instr;
    }
}
