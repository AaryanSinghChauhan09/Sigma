//! SigmaOS CPU operations module.
//!
//! This module provides actual CPU information using /proc/cpuinfo
//! and /proc/stat.
//!
//! # Example
//!
//! ```
//! use sigma_system::cpu_ops::*;
//!
//! let mut cpu_ops = CpuOps::new();
//! if let Ok(info) = cpu_ops.get_cpu_info() {
//!     println!("CPU: {} cores", info.core_count);
//! }
//! ```

use std::collections::BTreeMap;
use std::fs;

/// Errors that can occur in CPU operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CpuError {
    /// Failed to read CPU info.
    ReadError(String),
    /// Invalid CPU info.
    InvalidInfo(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::ReadError(msg) => write!(f, "Failed to read CPU info: {}", msg),
            CpuError::InvalidInfo(msg) => write!(f, "Invalid CPU info: {}", msg),
            CpuError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for CpuError {}

/// CPU information from /proc/cpuinfo.
#[derive(Debug, Clone)]
pub struct CpuInfo {
    /// CPU model name.
    pub model_name: String,
    /// CPU vendor.
    pub vendor: String,
    /// CPU family.
    pub family: u32,
    /// CPU model.
    pub model: u32,
    /// CPU stepping.
    pub stepping: u32,
    /// CPU MHz.
    pub mhz: f64,
    /// CPU core count.
    pub core_count: u32,
    /// CPU thread count.
    pub thread_count: u32,
    /// CPU cache size.
    pub cache_size: u32,
    /// CPU flags.
    pub flags: Vec<String>,
    /// CPU architecture.
    pub architecture: String,
    /// CPU bogomips.
    pub bogomips: f64,
}

impl CpuInfo {
    /// Get CPU info as string.
    pub fn to_string(&self) -> String {
        format!(
            "{} ({} cores, {} threads, {} MHz)",
            self.model_name, self.core_count, self.thread_count, self.mhz
        )
    }
}

/// CPU usage statistics.
#[derive(Debug, Clone)]
pub struct CpuUsage {
    /// User time in jiffies.
    pub user: u64,
    /// Nice time in jiffies.
    pub nice: u64,
    /// System time in jiffies.
    pub system: u64,
    /// Idle time in jiffies.
    pub idle: u64,
    /// I/O wait time in jiffies.
    pub iowait: u64,
    /// IRQ time in jiffies.
    pub irq: u64,
    /// Soft IRQ time in jiffies.
    pub softirq: u64,
    /// Steal time in jiffies.
    pub steal: u64,
    /// Guest time in jiffies.
    pub guest: u64,
    /// Total time in jiffies.
    pub total: u64,
    /// CPU usage percentage.
    pub usage_percent: f64,
}

impl CpuUsage {
    /// Calculate CPU usage percentage.
    pub fn calculate_usage(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        let busy = self.total - self.idle - self.iowait;
        (busy as f64 / self.total as f64) * 100.0
    }
}

/// CPU operations manager.
#[derive(Debug)]
pub struct CpuOps {
    /// Cached CPU info.
    cpu_info: Option<CpuInfo>,
    /// Last CPU usage.
    last_usage: Option<CpuUsage>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl CpuOps {
    /// Create a new CPU operations manager.
    pub fn new() -> Self {
        Self {
            cpu_info: None,
            last_usage: None,
            last_update: std::time::Instant::now(),
        }
    }

    /// Get CPU information.
    pub fn get_cpu_info(&mut self) -> Result<&CpuInfo, CpuError> {
        if self.cpu_info.is_none() {
            self.refresh_cpu_info()?;
        }
        self.cpu_info.as_ref().ok_or_else(|| CpuError::InvalidInfo("No CPU info".to_string()))
    }

    /// Refresh CPU info from /proc/cpuinfo.
    fn refresh_cpu_info(&mut self) -> Result<(), CpuError> {
        let cpuinfo_path = "/proc/cpuinfo";
        let content = fs::read_to_string(cpuinfo_path)
            .map_err(|e| CpuError::IoError(format!("Failed to read {}: {}", cpuinfo_path, e)))?;

        let mut model_name = String::new();
        let mut vendor = String::new();
        let mut family = 0;
        let mut model = 0;
        let mut stepping = 0;
        let mut mhz = 0.0;
        let mut cache_size = 0;
        let mut flags = Vec::new();
        let mut architecture = String::new();
        let mut bogomips = 0.0;
        let mut core_count = 0;
        let mut thread_count = 0;

        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                
                match key {
                    "model name" => model_name = value.to_string(),
                    "vendor_id" => vendor = value.to_string(),
                    "cpu family" => family = value.parse().unwrap_or(0),
                    "model" => model = value.parse().unwrap_or(0),
                    "stepping" => stepping = value.parse().unwrap_or(0),
                    "cpu MHz" => mhz = value.parse().unwrap_or(0.0),
                    "cache size" => cache_size = value.trim_end_matches(" KB").parse().unwrap_or(0),
                    "flags" | "Features" => {
                        flags = value.split_whitespace().map(|s| s.to_string()).collect();
                    }
                    "architecture" => architecture = value.to_string(),
                    "bogomips" => bogomips = value.parse().unwrap_or(0.0),
                    "cpu cores" => core_count = value.parse().unwrap_or(0),
                    "siblings" => thread_count = value.parse().unwrap_or(0),
                    _ => {}
                }
            }
        }

        if model_name.is_empty() {
            return Err(CpuError::InvalidInfo("CPU model name is empty".to_string()));
        }

        // Count processors
        if core_count == 0 {
            core_count = content.matches("processor").count() as u32;
        }
        if thread_count == 0 {
            thread_count = core_count;
        }

        self.cpu_info = Some(CpuInfo {
            model_name,
            vendor,
            family,
            model,
            stepping,
            mhz,
            core_count,
            thread_count,
            cache_size,
            flags,
            architecture,
            bogomips,
        });

        Ok(())
    }

    /// Get CPU usage statistics.
    pub fn get_cpu_usage(&mut self) -> Result<CpuUsage, CpuError> {
        let stat_path = "/proc/stat";
        let content = fs::read_to_string(stat_path)
            .map_err(|e| CpuError::IoError(format!("Failed to read {}: {}", stat_path, e)))?;

        // Parse first line (aggregate CPU stats)
        let first_line = content.lines().next()
            .ok_or_else(|| CpuError::InvalidInfo("No CPU stats found".to_string()))?;

        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 5 {
            return Err(CpuError::InvalidInfo("Invalid CPU stats format".to_string()));
        }

        let mut values = [0u64; 10];
        for i in 0..10 {
            if i + 1 < parts.len() {
                values[i] = parts[i + 1].parse().unwrap_or(0);
            }
        }

        let usage = CpuUsage {
            user: values[0],
            nice: values[1],
            system: values[2],
            idle: values[3],
            iowait: values[4],
            irq: values[5],
            softirq: values[6],
            steal: values[7],
            guest: values[8],
            total: values.iter().sum(),
            usage_percent: 0.0,
        };

        let mut usage = usage;
        usage.usage_percent = usage.calculate_usage();

        // Calculate delta if we have previous usage
        if let Some(last) = &self.last_usage {
            let delta_total = usage.total - last.total;
            if delta_total > 0 {
                let delta_busy = (usage.total - usage.idle - usage.iowait) - (last.total - last.idle - last.iowait);
                usage.usage_percent = (delta_busy as f64 / delta_total as f64) * 100.0;
            }
        }

        self.last_usage = Some(usage.clone());
        Ok(usage)
    }

    /// Get CPU usage percentage.
    pub fn get_cpu_usage_percent(&mut self) -> Result<f64, CpuError> {
        let usage = self.get_cpu_usage()?;
        Ok(usage.usage_percent)
    }

    /// Get CPU temperature (if available).
    pub fn get_cpu_temperature(&self) -> Option<f64> {
        // Try to read from /sys/class/thermal
        let thermal_path = "/sys/class/thermal/thermal_zone0/temp";
        if let Ok(content) = fs::read_to_string(thermal_path) {
            if let Ok(temp) = content.trim().parse::<f64>() {
                return Some(temp / 1000.0); // Convert from millidegrees to degrees
            }
        }
        None
    }

    /// Get CPU frequency.
    pub fn get_cpu_frequency(&self) -> Option<u64> {
        // Try to read from /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq
        let freq_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq";
        if let Ok(content) = fs::read_to_string(freq_path) {
            if let Ok(freq) = content.trim().parse::<u64>() {
                return Some(freq);
            }
        }
        None
    }

    /// Get CPU governor.
    pub fn get_cpu_governor(&self) -> Option<String> {
        let governor_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
        fs::read_to_string(governor_path)
            .ok()
            .map(|s| s.trim().to_string())
    }

    /// Set CPU governor.
    pub fn set_cpu_governor(&self, governor: &str) -> Result<(), CpuError> {
        let governor_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
        fs::write(governor_path, governor)
            .map_err(|e| CpuError::IoError(format!("Failed to set governor: {}", e)))
    }

    /// Get CPU load average.
    pub fn get_load_average(&self) -> Result<(f64, f64, f64), CpuError> {
        let loadavg_path = "/proc/loadavg";
        let content = fs::read_to_string(loadavg_path)
            .map_err(|e| CpuError::IoError(format!("Failed to read {}: {}", loadavg_path, e)))?;

        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(CpuError::InvalidInfo("Invalid load average format".to_string()));
        }

        let load1 = parts[0].parse().unwrap_or(0.0);
        let load5 = parts[1].parse().unwrap_or(0.0);
        let load15 = parts[2].parse().unwrap_or(0.0);

        Ok((load1, load5, load15))
    }

    /// Get CPU info summary.
    pub fn get_cpu_summary(&mut self) -> Result<String, CpuError> {
        let info = self.get_cpu_info()?.clone();
        let usage = self.get_cpu_usage()?;
        let load = self.get_load_average()?;
        let freq = self.get_cpu_frequency().unwrap_or(0);
        let temp = self.get_cpu_temperature().unwrap_or(0.0);

        Ok(format!(
            "CPU: {}\nCores: {} / Threads: {}\nUsage: {:.1}%\nLoad: {:.2} {:.2} {:.2}\nFrequency: {} MHz\nTemperature: {}°C",
            info.model_name,
            info.core_count,
            info.thread_count,
            usage.usage_percent,
            load.0,
            load.1,
            load.2,
            freq,
            temp
        ))
    }
}

impl Default for CpuOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu_info() {
        let mut cpu_ops = CpuOps::new();
        if let Ok(info) = cpu_ops.get_cpu_info() {
            assert!(!info.model_name.is_empty());
            assert!(info.core_count > 0);
            assert!(info.thread_count > 0);
        }
    }

    #[test]
    fn test_get_cpu_usage() {
        let mut cpu_ops = CpuOps::new();
        if let Ok(usage) = cpu_ops.get_cpu_usage() {
            assert!(usage.usage_percent >= 0.0);
            assert!(usage.usage_percent <= 100.0);
        }
    }

    #[test]
    fn test_get_load_average() {
        let cpu_ops = CpuOps::new();
        if let Ok((load1, load5, load15)) = cpu_ops.get_load_average() {
            assert!(load1 >= 0.0);
            assert!(load5 >= 0.0);
            assert!(load15 >= 0.0);
        }
    }

    #[test]
    fn test_cpu_usage_calculation() {
        let usage = CpuUsage {
            user: 100,
            nice: 0,
            system: 50,
            idle: 800,
            iowait: 50,
            irq: 0,
            softirq: 0,
            steal: 0,
            guest: 0,
            total: 1000,
            usage_percent: 0.0,
        };
        
        let calculated = usage.calculate_usage();
        assert!(calculated > 0.0);
        assert!(calculated < 100.0);
    }

    #[test]
    fn test_cpu_info_to_string() {
        let info = CpuInfo {
            model_name: "Test CPU".to_string(),
            vendor: "Test".to_string(),
            family: 6,
            model: 158,
            stepping: 10,
            mhz: 3000.0,
            core_count: 8,
            thread_count: 16,
            cache_size: 8192,
            flags: vec!["test".to_string()],
            architecture: "x86_64".to_string(),
            bogomips: 6000.0,
        };
        
        let s = info.to_string();
        assert!(s.contains("Test CPU"));
        assert!(s.contains("8 cores"));
        assert!(s.contains("16 threads"));
    }
}
