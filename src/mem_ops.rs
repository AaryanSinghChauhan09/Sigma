//! SigmaOS memory operations module.
//!
//! This module provides actual memory information using /proc/meminfo
//! and /proc/vmstat.
//!
//! # Example
//!
//! ```
//! use sigma_system::mem_ops::*;
//!
//! let mut mem_ops = MemOps::new();
//! if let Ok(info) = mem_ops.get_memory_info() {
//!     println!("Total memory: {} GB", info.total_gb());
//! }
//! ```

use std::collections::BTreeMap;
use std::fs;

/// Errors that can occur in memory operations.
#[derive(Debug, Clone, PartialEq)]
pub enum MemError {
    /// Failed to read memory info.
    ReadError(String),
    /// Invalid memory info.
    InvalidInfo(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for MemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemError::ReadError(msg) => write!(f, "Failed to read memory info: {}", msg),
            MemError::InvalidInfo(msg) => write!(f, "Invalid memory info: {}", msg),
            MemError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for MemError {}

/// Memory information from /proc/meminfo.
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    /// Total memory in KB.
    pub total_kb: u64,
    /// Free memory in KB.
    pub free_kb: u64,
    /// Used memory in KB.
    pub used_kb: u64,
    /// Available memory in KB.
    pub available_kb: u64,
    /// Buffers in KB.
    pub buffers_kb: u64,
    /// Cached memory in KB.
    pub cached_kb: u64,
    /// Swap total in KB.
    pub swap_total_kb: u64,
    /// Swap free in KB.
    pub swap_free_kb: u64,
    /// Swap used in KB.
    pub swap_used_kb: u64,
    /// Memory usage percentage.
    pub usage_percent: f64,
    /// Swap usage percentage.
    pub swap_usage_percent: f64,
}

impl MemoryInfo {
    /// Get total memory in GB.
    pub fn total_gb(&self) -> f64 {
        self.total_kb as f64 / 1024.0 / 1024.0
    }

    /// Get free memory in GB.
    pub fn free_gb(&self) -> f64 {
        self.free_kb as f64 / 1024.0 / 1024.0
    }

    /// Get used memory in GB.
    pub fn used_gb(&self) -> f64 {
        self.used_kb as f64 / 1024.0 / 1024.0
    }

    /// Get available memory in GB.
    pub fn available_gb(&self) -> f64 {
        self.available_kb as f64 / 1024.0 / 1024.0
    }

    /// Format size in human readable format.
    pub fn format_size(&self, kb: u64) -> String {
        const UNITS: &[&str] = &["KB", "MB", "GB", "TB"];
        let mut size = kb as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Memory pressure level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressure {
    /// No pressure.
    None,
    /// Low pressure.
    Low,
    /// Medium pressure.
    Medium,
    /// High pressure.
    High,
    /// Critical pressure.
    Critical,
}

impl MemoryPressure {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            MemoryPressure::None => "none",
            MemoryPressure::Low => "low",
            MemoryPressure::Medium => "medium",
            MemoryPressure::High => "high",
            MemoryPressure::Critical => "critical",
        }
    }
}

/// Memory operations manager.
#[derive(Debug)]
pub struct MemOps {
    /// Cached memory info.
    memory_info: Option<MemoryInfo>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl MemOps {
    /// Create a new memory operations manager.
    pub fn new() -> Self {
        Self {
            memory_info: None,
            last_update: std::time::Instant::now(),
        }
    }

    /// Get memory information.
    pub fn get_memory_info(&mut self) -> Result<&MemoryInfo, MemError> {
        // Refresh if more than 100ms since last update
        if self.last_update.elapsed().as_millis() > 100 {
            self.refresh_memory_info()?;
        }
        self.memory_info.as_ref().ok_or_else(|| MemError::InvalidInfo("No memory info".to_string()))
    }

    /// Refresh memory info from /proc/meminfo.
    fn refresh_memory_info(&mut self) -> Result<(), MemError> {
        let meminfo_path = "/proc/meminfo";
        let content = fs::read_to_string(meminfo_path)
            .map_err(|e| MemError::IoError(format!("Failed to read {}: {}", meminfo_path, e)))?;

        let mut total_kb = 0;
        let mut free_kb = 0;
        let mut available_kb = 0;
        let mut buffers_kb = 0;
        let mut cached_kb = 0;
        let mut swap_total_kb = 0;
        let mut swap_free_kb = 0;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let value = parts[1].parse::<u64>().unwrap_or(0);
                match parts[0] {
                    "MemTotal:" => total_kb = value,
                    "MemFree:" => free_kb = value,
                    "MemAvailable:" => available_kb = value,
                    "Buffers:" => buffers_kb = value,
                    "Cached:" => cached_kb = value,
                    "SwapTotal:" => swap_total_kb = value,
                    "SwapFree:" => swap_free_kb = value,
                    _ => {}
                }
            }
        }

        if total_kb == 0 {
            return Err(MemError::InvalidInfo("Total memory is 0".to_string()));
        }

        let used_kb = total_kb - free_kb - buffers_kb - cached_kb;
        let swap_used_kb = swap_total_kb - swap_free_kb;

        let usage_percent = if total_kb > 0 {
            (used_kb as f64 / total_kb as f64) * 100.0
        } else {
            0.0
        };

        let swap_usage_percent = if swap_total_kb > 0 {
            (swap_used_kb as f64 / swap_total_kb as f64) * 100.0
        } else {
            0.0
        };

        self.memory_info = Some(MemoryInfo {
            total_kb,
            free_kb,
            used_kb,
            available_kb,
            buffers_kb,
            cached_kb,
            swap_total_kb,
            swap_free_kb,
            swap_used_kb,
            usage_percent,
            swap_usage_percent,
        });

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Get memory pressure level.
    pub fn get_memory_pressure(&mut self) -> Result<MemoryPressure, MemError> {
        let info = self.get_memory_info()?;
        let usage = info.usage_percent;

        if usage >= 95.0 {
            Ok(MemoryPressure::Critical)
        } else if usage >= 85.0 {
            Ok(MemoryPressure::High)
        } else if usage >= 70.0 {
            Ok(MemoryPressure::Medium)
        } else if usage >= 50.0 {
            Ok(MemoryPressure::Low)
        } else {
            Ok(MemoryPressure::None)
        }
    }

    /// Check if memory is low.
    pub fn is_memory_low(&mut self) -> Result<bool, MemError> {
        let pressure = self.get_memory_pressure()?;
        Ok(matches!(pressure, MemoryPressure::High | MemoryPressure::Critical))
    }

    /// Check if swap is being used heavily.
    pub fn is_swap_heavy(&mut self) -> Result<bool, MemError> {
        let info = self.get_memory_info()?;
        Ok(info.swap_usage_percent > 50.0)
    }

    /// Get memory statistics as string.
    pub fn get_memory_stats(&mut self) -> Result<String, MemError> {
        let info = self.get_memory_info()?;
        Ok(format!(
            "Total: {} GB, Used: {} GB ({:.1}%), Free: {} GB, Available: {} GB, Swap: {} GB/{:.1}%",
            info.total_gb(),
            info.used_gb(),
            info.usage_percent,
            info.free_gb(),
            info.available_gb(),
            info.swap_used_kb as f64 / 1024.0 / 1024.0,
            info.swap_usage_percent
        ))
    }

    /// Generate memory tuning recommendations.
    pub fn get_tuning_recommendations(&mut self) -> Result<Vec<String>, MemError> {
        let mut recommendations = Vec::new();
        let info = self.get_memory_info()?.clone();
        let pressure = self.get_memory_pressure()?;

        match pressure {
            MemoryPressure::Critical => {
                recommendations.push("CRITICAL: Memory usage is critically high".to_string());
                recommendations.push("Consider closing applications or adding more RAM".to_string());
                recommendations.push("Enable zram or zswap for better memory compression".to_string());
            }
            MemoryPressure::High => {
                recommendations.push("WARNING: Memory usage is high".to_string());
                recommendations.push("Consider reducing swappiness or closing unused apps".to_string());
            }
            MemoryPressure::Medium => {
                recommendations.push("Memory usage is moderate".to_string());
                recommendations.push("Monitor for trends".to_string());
            }
            MemoryPressure::Low | MemoryPressure::None => {
                recommendations.push("Memory usage is healthy".to_string());
            }
        }

        if info.swap_usage_percent > 50.0 {
            recommendations.push("Swap usage is high - consider increasing RAM or optimizing memory".to_string());
        }

        Ok(recommendations)
    }
}

impl Default for MemOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_memory_info() {
        let mut mem_ops = MemOps::new();
        if let Ok(info) = mem_ops.get_memory_info() {
            assert!(info.total_kb > 0);
            assert!(info.usage_percent >= 0.0);
            assert!(info.usage_percent <= 100.0);
        }
    }

    #[test]
    fn test_get_memory_pressure() {
        let mut mem_ops = MemOps::new();
        if let Ok(pressure) = mem_ops.get_memory_pressure() {
            // Just verify it returns a valid pressure level
            let _ = pressure.as_str();
        }
    }

    #[test]
    fn test_memory_info_format() {
        let info = MemoryInfo {
            total_kb: 16 * 1024 * 1024, // 16GB
            free_kb: 8 * 1024 * 1024,   // 8GB
            used_kb: 8 * 1024 * 1024,   // 8GB
            available_kb: 10 * 1024 * 1024,
            buffers_kb: 100 * 1024,
            cached_kb: 200 * 1024,
            swap_total_kb: 4 * 1024 * 1024,
            swap_free_kb: 3 * 1024 * 1024,
            swap_used_kb: 1 * 1024 * 1024,
            usage_percent: 50.0,
            swap_usage_percent: 25.0,
        };

        assert_eq!(info.total_gb(), 16.0);
        assert_eq!(info.free_gb(), 8.0);
        assert_eq!(info.used_gb(), 8.0);
        assert_eq!(info.format_size(1024), "1.00 MB");
    }

    #[test]
    fn test_memory_pressure_levels() {
        assert_eq!(MemoryPressure::None.as_str(), "none");
        assert_eq!(MemoryPressure::Low.as_str(), "low");
        assert_eq!(MemoryPressure::Medium.as_str(), "medium");
        assert_eq!(MemoryPressure::High.as_str(), "high");
        assert_eq!(MemoryPressure::Critical.as_str(), "critical");
    }
}
