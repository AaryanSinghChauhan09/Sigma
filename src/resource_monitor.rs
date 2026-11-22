//! SigmaOS resource monitor module.
//!
//! This module provides actual system resource monitoring using /proc and /sys.
//!
//! # Example
//!
//! ```
//! use sigma_system::resource_monitor::*;
//!
//! let mut monitor = ResourceMonitor::new();
//! if let Ok(summary) = monitor.get_resource_summary() {
//!     println!("Resource usage: {}", summary);
//! }
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in resource monitoring.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceError {
    /// Failed to read resource info.
    ReadError(String),
    /// Invalid resource data.
    InvalidData(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceError::ReadError(msg) => write!(f, "Failed to read resource info: {}", msg),
            ResourceError::InvalidData(msg) => write!(f, "Invalid resource data: {}", msg),
            ResourceError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ResourceError {}

/// Resource usage snapshot.
#[derive(Debug, Clone)]
pub struct ResourceSnapshot {
    /// CPU usage percentage.
    pub cpu_percent: f64,
    /// Memory usage percentage.
    pub memory_percent: f64,
    /// Swap usage percentage.
    pub swap_percent: f64,
    /// Disk usage percentage.
    pub disk_percent: f64,
    /// Network RX bytes per second.
    pub network_rx_bps: u64,
    /// Network TX bytes per second.
    pub network_tx_bps: u64,
    /// Load average (1min, 5min, 15min).
    pub load_average: (f64, f64, f64),
    /// Process count.
    pub process_count: u32,
    /// Thread count.
    pub thread_count: u32,
}

/// Resource monitor.
#[derive(Debug)]
pub struct ResourceMonitor {
    /// Previous network stats for rate calculation.
    prev_network_rx: u64,
    prev_network_tx: u64,
    /// Previous snapshot time.
    prev_time: std::time::Instant,
}

impl ResourceMonitor {
    /// Create a new resource monitor.
    pub fn new() -> Self {
        Self {
            prev_network_rx: 0,
            prev_network_tx: 0,
            prev_time: std::time::Instant::now(),
        }
    }

    /// Get current resource snapshot.
    pub fn get_snapshot(&mut self) -> Result<ResourceSnapshot, ResourceError> {
        let cpu_percent = self.get_cpu_percent()?;
        let memory_percent = self.get_memory_percent()?;
        let swap_percent = self.get_swap_percent()?;
        let disk_percent = self.get_disk_percent()?;
        let (network_rx, network_tx) = self.get_network_stats()?;
        let load_average = self.get_load_average()?;
        let process_count = self.get_process_count()?;
        let thread_count = self.get_thread_count()?;

        // Calculate network rates
        let elapsed = self.prev_time.elapsed().as_secs_f64();
        let network_rx_bps = if elapsed > 0.0 {
            ((network_rx - self.prev_network_rx) as f64 / elapsed) as u64
        } else {
            0
        };
        let network_tx_bps = if elapsed > 0.0 {
            ((network_tx - self.prev_network_tx) as f64 / elapsed) as u64
        } else {
            0
        };

        self.prev_network_rx = network_rx;
        self.prev_network_tx = network_tx;
        self.prev_time = std::time::Instant::now();

        Ok(ResourceSnapshot {
            cpu_percent,
            memory_percent,
            swap_percent,
            disk_percent,
            network_rx_bps,
            network_tx_bps,
            load_average,
            process_count,
            thread_count,
        })
    }

    /// Get CPU usage percentage.
    fn get_cpu_percent(&self) -> Result<f64, ResourceError> {
        let content = std::fs::read_to_string("/proc/stat")
            .map_err(|e| ResourceError::IoError(format!("Failed to read /proc/stat: {}", e)))?;
        
        for line in content.lines() {
            if line.starts_with("cpu ") {
                let parts: Vec<u64> = line.split_whitespace()
                    .skip(1)
                    .map(|s| s.parse().unwrap_or(0))
                    .collect();
                
                if parts.len() >= 4 {
                    let idle = parts[3];
                    let total: u64 = parts.iter().sum();
                    let usage = if total > 0 {
                        100.0 * (1.0 - (idle as f64 / total as f64))
                    } else {
                        0.0
                    };
                    return Ok(usage);
                }
            }
        }
        
        Ok(0.0)
    }

    /// Get memory usage percentage.
    fn get_memory_percent(&self) -> Result<f64, ResourceError> {
        let content = std::fs::read_to_string("/proc/meminfo")
            .map_err(|e| ResourceError::IoError(format!("Failed to read /proc/meminfo: {}", e)))?;
        
        let mut total = 0;
        let mut available = 0;
        
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    total = val.parse().unwrap_or(0);
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    available = val.parse().unwrap_or(0);
                }
            }
        }
        
        if total > 0 {
            Ok(100.0 * (1.0 - (available as f64 / total as f64)))
        } else {
            Ok(0.0)
        }
    }

    /// Get swap usage percentage.
    fn get_swap_percent(&self) -> Result<f64, ResourceError> {
        let content = std::fs::read_to_string("/proc/meminfo")
            .map_err(|e| ResourceError::IoError(format!("Failed to read /proc/meminfo: {}", e)))?;
        
        let mut total = 0;
        let mut free = 0;
        
        for line in content.lines() {
            if line.starts_with("SwapTotal:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    total = val.parse().unwrap_or(0);
                }
            } else if line.starts_with("SwapFree:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    free = val.parse().unwrap_or(0);
                }
            }
        }
        
        if total > 0 {
            Ok(100.0 * (1.0 - (free as f64 / total as f64)))
        } else {
            Ok(0.0)
        }
    }

    /// Get disk usage percentage.
    fn get_disk_percent(&self) -> Result<f64, ResourceError> {
        let output = std::process::Command::new("df").arg("/").output()
            .map_err(|e| ResourceError::IoError(format!("Failed to run df: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let used_percent = parts[4].trim_end_matches('%');
                if let Ok(percent) = used_percent.parse::<f64>() {
                    return Ok(percent);
                }
            }
        }
        
        Ok(0.0)
    }

    /// Get network stats.
    fn get_network_stats(&self) -> Result<(u64, u64), ResourceError> {
        let mut total_rx = 0;
        let mut total_tx = 0;
        
        let net_dir = std::path::Path::new("/sys/class/net");
        if !net_dir.exists() {
            return Ok((0, 0));
        }
        
        let entries = std::fs::read_dir(net_dir)
            .map_err(|e| ResourceError::IoError(format!("Failed to read /sys/class/net: {}", e)))?;
        
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            
            // Skip loopback
            if name_str == "lo" {
                continue;
            }
            
            let rx_path = entry.path().join("statistics").join("rx_bytes");
            let tx_path = entry.path().join("statistics").join("tx_bytes");
            
            if let Ok(rx) = std::fs::read_to_string(&rx_path) {
                total_rx += rx.trim().parse().unwrap_or(0);
            }
            if let Ok(tx) = std::fs::read_to_string(&tx_path) {
                total_tx += tx.trim().parse().unwrap_or(0);
            }
        }
        
        Ok((total_rx, total_tx))
    }

    /// Get load average.
    fn get_load_average(&self) -> Result<(f64, f64, f64), ResourceError> {
        let content = std::fs::read_to_string("/proc/loadavg")
            .map_err(|e| ResourceError::IoError(format!("Failed to read /proc/loadavg: {}", e)))?;
        
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 3 {
            let load1 = parts[0].parse().unwrap_or(0.0);
            let load5 = parts[1].parse().unwrap_or(0.0);
            let load15 = parts[2].parse().unwrap_or(0.0);
            return Ok((load1, load5, load15));
        }
        
        Ok((0.0, 0.0, 0.0))
    }

    /// Get process count.
    fn get_process_count(&self) -> Result<u32, ResourceError> {
        let proc_dir = std::path::Path::new("/proc");
        let entries = std::fs::read_dir(proc_dir)
            .map_err(|e| ResourceError::IoError(format!("Failed to read /proc: {}", e)))?;
        
        let mut count = 0;
        for entry in entries.flatten() {
            let name = entry.file_name();
            if let Ok(pid) = name.to_string_lossy().parse::<u32>() {
                if pid > 0 {
                    count += 1;
                }
            }
        }
        
        Ok(count)
    }

    /// Get thread count.
    fn get_thread_count(&self) -> Result<u32, ResourceError> {
        let content = std::fs::read_to_string("/proc/stat")
            .map_err(|e| ResourceError::IoError(format!("Failed to read /proc/stat: {}", e)))?;
        
        for line in content.lines() {
            if line.starts_with("threads") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    return Ok(val.parse().unwrap_or(0));
                }
            }
        }
        
        Ok(0)
    }

    /// Get resource summary.
    pub fn get_resource_summary(&mut self) -> Result<String, ResourceError> {
        let snapshot = self.get_snapshot()?;
        
        Ok(format!(
            "CPU: {:.1}% | Memory: {:.1}% | Swap: {:.1}% | Disk: {:.1}%\nLoad: {:.2} {:.2} {:.2} | Processes: {} | Threads: {}\nNetwork: {} RX/s, {} TX/s",
            snapshot.cpu_percent,
            snapshot.memory_percent,
            snapshot.swap_percent,
            snapshot.disk_percent,
            snapshot.load_average.0,
            snapshot.load_average.1,
            snapshot.load_average.2,
            snapshot.process_count,
            snapshot.thread_count,
            Self::format_bytes(snapshot.network_rx_bps),
            Self::format_bytes(snapshot.network_tx_bps)
        ))
    }

    /// Format bytes.
    fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

impl Default for ResourceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let monitor = ResourceMonitor::new();
        let _ = monitor.prev_time;
    }

    #[test]
    fn test_get_snapshot() {
        let mut monitor = ResourceMonitor::new();
        if let Ok(snapshot) = monitor.get_snapshot() {
            assert!(snapshot.cpu_percent >= 0.0);
            assert!(snapshot.memory_percent >= 0.0);
        }
    }

    #[test]
    fn test_get_resource_summary() {
        let mut monitor = ResourceMonitor::new();
        if let Ok(summary) = monitor.get_resource_summary() {
            assert!(!summary.is_empty());
        }
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(ResourceMonitor::format_bytes(1024), "1.0 KB");
        assert_eq!(ResourceMonitor::format_bytes(1048576), "1.0 MB");
    }
}
