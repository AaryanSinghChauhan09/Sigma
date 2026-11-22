//! SigmaOS system information module.
//!
//! This module provides actual system information using /proc and /sys.
//!
//! # Example
//!
//! ```
//! use sigma_system::sys_info::*;
//!
//! let mut sys_info = SysInfo::new();
//! if let Ok(uptime) = sys_info.get_uptime() {
//!     println!("System uptime: {}", uptime);
//! }
//! ```

use std::fs;
use std::collections::BTreeMap;

/// Errors that can occur in system info operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SysInfoError {
    /// Failed to read system info.
    ReadError(String),
    /// Invalid system info.
    InvalidInfo(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for SysInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SysInfoError::ReadError(msg) => write!(f, "Failed to read system info: {}", msg),
            SysInfoError::InvalidInfo(msg) => write!(f, "Invalid system info: {}", msg),
            SysInfoError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for SysInfoError {}

/// System information.
#[derive(Debug, Clone)]
pub struct SystemInfo {
    /// Hostname.
    pub hostname: String,
    /// Kernel version.
    pub kernel_version: String,
    /// Kernel release.
    pub kernel_release: String,
    /// Architecture.
    pub architecture: String,
    /// Uptime in seconds.
    pub uptime_seconds: u64,
    /// Boot time.
    pub boot_time: String,
    /// Number of CPUs.
    pub cpu_count: u32,
    /// Total memory in KB.
    pub total_memory_kb: u64,
    /// Free memory in KB.
    pub free_memory_kb: u64,
    /// Total swap in KB.
    pub total_swap_kb: u64,
    /// Free swap in KB.
    pub free_swap_kb: u64,
    /// Load average.
    pub load_average: (f64, f64, f64),
}

/// System information manager.
#[derive(Debug)]
pub struct SysInfo {
    /// Cached system info.
    system_info: Option<SystemInfo>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl SysInfo {
    /// Create a new system info manager.
    pub fn new() -> Self {
        Self {
            system_info: None,
            last_update: std::time::Instant::now(),
        }
    }

    /// Get system information.
    pub fn get_system_info(&mut self) -> Result<&SystemInfo, SysInfoError> {
        // Refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() > 1 {
            self.refresh_system_info()?;
        }
        self.system_info.as_ref().ok_or_else(|| SysInfoError::InvalidInfo("No system info".to_string()))
    }

    /// Refresh system info.
    fn refresh_system_info(&mut self) -> Result<(), SysInfoError> {
        // Get hostname
        let hostname = fs::read_to_string("/etc/hostname")
            .map(|s| s.trim().to_string())
            .or_else(|_| fs::read_to_string("/proc/sys/kernel/hostname")
                .map(|s| s.trim().to_string()))
            .unwrap_or_else(|_| "unknown".to_string());

        // Get kernel version
        let kernel_version = fs::read_to_string("/proc/version")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        // Get kernel release
        let kernel_release = fs::read_to_string("/proc/sys/kernel/osrelease")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        // Get architecture
        let architecture = std::process::Command::new("uname")
            .arg("-m")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        // Get uptime
        let uptime_seconds = fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|s| s.split_whitespace().next()?.parse::<f64>().ok())
            .map(|f| f as u64)
            .unwrap_or(0);

        // Get boot time (as unix timestamp)
        let boot_time = fs::read_to_string("/proc/stat")
            .ok()
            .and_then(|content| {
                content.lines()
                    .find(|line| line.starts_with("btime"))
                    .and_then(|line| line.split_whitespace().nth(1)?.parse::<u64>().ok())
            })
            .map(|ts| ts.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Get CPU count
        let cpu_count = fs::read_to_string("/proc/cpuinfo")
            .map(|content| content.matches("processor").count() as u32)
            .unwrap_or(1);

        // Get memory info
        let (total_memory_kb, free_memory_kb, total_swap_kb, free_swap_kb) = self.get_memory_info()?;

        // Get load average
        let load_average = self.get_load_average()?;

        self.system_info = Some(SystemInfo {
            hostname,
            kernel_version,
            kernel_release,
            architecture,
            uptime_seconds,
            boot_time,
            cpu_count,
            total_memory_kb,
            free_memory_kb,
            total_swap_kb,
            free_swap_kb,
            load_average,
        });

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Get memory info from /proc/meminfo.
    fn get_memory_info(&self) -> Result<(u64, u64, u64, u64), SysInfoError> {
        let content = fs::read_to_string("/proc/meminfo")
            .map_err(|e| SysInfoError::IoError(format!("Failed to read /proc/meminfo: {}", e)))?;

        let mut total_memory_kb = 0;
        let mut free_memory_kb = 0;
        let mut total_swap_kb = 0;
        let mut free_swap_kb = 0;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let value = parts[1].parse::<u64>().unwrap_or(0);
                match parts[0] {
                    "MemTotal:" => total_memory_kb = value,
                    "MemFree:" => free_memory_kb = value,
                    "SwapTotal:" => total_swap_kb = value,
                    "SwapFree:" => free_swap_kb = value,
                    _ => {}
                }
            }
        }

        Ok((total_memory_kb, free_memory_kb, total_swap_kb, free_swap_kb))
    }

    /// Get load average from /proc/loadavg.
    fn get_load_average(&self) -> Result<(f64, f64, f64), SysInfoError> {
        let content = fs::read_to_string("/proc/loadavg")
            .map_err(|e| SysInfoError::IoError(format!("Failed to read /proc/loadavg: {}", e)))?;

        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(SysInfoError::InvalidInfo("Invalid load average format".to_string()));
        }

        let load1 = parts[0].parse().unwrap_or(0.0);
        let load5 = parts[1].parse().unwrap_or(0.0);
        let load15 = parts[2].parse().unwrap_or(0.0);

        Ok((load1, load5, load15))
    }

    /// Get uptime in human readable format.
    pub fn get_uptime(&mut self) -> Result<String, SysInfoError> {
        let info = self.get_system_info()?;
        let uptime = info.uptime_seconds;
        
        let days = uptime / 86400;
        let hours = (uptime % 86400) / 3600;
        let minutes = (uptime % 3600) / 60;
        let seconds = uptime % 60;
        
        Ok(format!("{}d {}h {}m {}s", days, hours, minutes, seconds))
    }

    /// Get memory usage in human readable format.
    pub fn get_memory_usage(&mut self) -> Result<String, SysInfoError> {
        let info = self.get_system_info()?;
        let total_gb = info.total_memory_kb as f64 / 1024.0 / 1024.0;
        let free_gb = info.free_memory_kb as f64 / 1024.0 / 1024.0;
        let used_gb = total_gb - free_gb;
        let usage_percent = if total_gb > 0.0 { (used_gb / total_gb) * 100.0 } else { 0.0 };
        
        Ok(format!("{:.1} GB / {:.1} GB ({:.1}%)", used_gb, total_gb, usage_percent))
    }

    /// Get system summary.
    pub fn get_system_summary(&mut self) -> Result<String, SysInfoError> {
        let info = self.get_system_info()?.clone();
        let uptime = self.get_uptime()?;
        let memory = self.get_memory_usage()?;
        
        Ok(format!(
            "Hostname: {}\nKernel: {} ({})\nArchitecture: {}\nUptime: {}\nCPUs: {}\nMemory: {}\nLoad: {:.2} {:.2} {:.2}",
            info.hostname,
            info.kernel_release,
            info.kernel_version,
            info.architecture,
            uptime,
            info.cpu_count,
            memory,
            info.load_average.0,
            info.load_average.1,
            info.load_average.2
        ))
    }
}

impl Default for SysInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mgr = SysInfo::new();
        let _ = mgr.last_update;
    }

    #[test]
    fn test_get_system_info() {
        let mut mgr = SysInfo::new();
        if let Ok(info) = mgr.get_system_info() {
            assert!(!info.hostname.is_empty());
            assert!(!info.kernel_release.is_empty());
            assert!(info.cpu_count > 0);
        }
    }

    #[test]
    fn test_get_uptime() {
        let mut mgr = SysInfo::new();
        if let Ok(uptime) = mgr.get_uptime() {
            assert!(!uptime.is_empty());
        }
    }

    #[test]
    fn test_get_memory_usage() {
        let mut mgr = SysInfo::new();
        if let Ok(usage) = mgr.get_memory_usage() {
            assert!(!usage.is_empty());
        }
    }
}
