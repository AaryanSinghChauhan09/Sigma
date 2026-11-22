//! SigmaOS I/O statistics module.
//!
//! This module provides actual I/O statistics using /proc/diskstats.
//!
//! # Example
//!
//! ```
//! use sigma_system::io_stats::*;
//!
//! let mut io_stats = IoStats::new();
//! if let Ok(stats) = io_stats.get_disk_io_stats("sda") {
//!     println!("Read: {} sectors, Write: {} sectors", stats.read_sectors, stats.write_sectors);
//! }
//! ```

use std::collections::BTreeMap;
use std::fs;

/// Errors that can occur in I/O stats operations.
#[derive(Debug, Clone, PartialEq)]
pub enum IoStatsError {
    /// Device not found.
    DeviceNotFound(String),
    /// Failed to read stats.
    ReadError(String),
    /// Invalid stats data.
    InvalidData(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for IoStatsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoStatsError::DeviceNotFound(dev) => write!(f, "Device '{}' not found", dev),
            IoStatsError::ReadError(msg) => write!(f, "Failed to read stats: {}", msg),
            IoStatsError::InvalidData(msg) => write!(f, "Invalid stats data: {}", msg),
            IoStatsError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for IoStatsError {}

/// Disk I/O statistics.
#[derive(Debug, Clone)]
pub struct DiskIoStats {
    /// Device name.
    pub device: String,
    /// Reads completed.
    pub reads_completed: u64,
    /// Reads merged.
    pub reads_merged: u64,
    /// Sectors read.
    pub read_sectors: u64,
    /// Time reading (ms).
    pub read_time_ms: u64,
    /// Writes completed.
    pub writes_completed: u64,
    /// Writes merged.
    pub writes_merged: u64,
    /// Sectors written.
    pub write_sectors: u64,
    /// Time writing (ms).
    pub write_time_ms: u64,
    /// I/Os currently in progress.
    pub ios_in_progress: u64,
    /// Time doing I/Os (ms).
    pub io_time_ms: u64,
    /// Time weighted doing I/Os (ms).
    pub io_time_weighted_ms: u64,
    /// Discards completed.
    pub discards_completed: u64,
    /// Discards merged.
    pub discards_merged: u64,
    /// Sectors discarded.
    pub discard_sectors: u64,
    /// Time discarding (ms).
    pub discard_time_ms: u64,
}

impl DiskIoStats {
    /// Calculate total sectors read+written.
    pub fn total_sectors(&self) -> u64 {
        self.read_sectors + self.write_sectors
    }

    /// Calculate total bytes read (assuming 512-byte sectors).
    pub fn total_read_bytes(&self) -> u64 {
        self.read_sectors * 512
    }

    /// Calculate total bytes written (assuming 512-byte sectors).
    pub fn total_write_bytes(&self) -> u64 {
        self.write_sectors * 512
    }

    /// Calculate total bytes (read+write).
    pub fn total_bytes(&self) -> u64 {
        self.total_read_bytes() + self.total_write_bytes()
    }

    /// Calculate I/O operations per second (approximate).
    pub fn iops(&self) -> f64 {
        let total_ios = self.reads_completed + self.writes_completed;
        if self.io_time_ms == 0 {
            return 0.0;
        }
        (total_ios as f64 / self.io_time_ms as f64) * 1000.0
    }

    /// Calculate throughput in MB/s (approximate).
    pub fn throughput_mbps(&self) -> f64 {
        let total_bytes = self.total_bytes();
        if self.io_time_ms == 0 {
            return 0.0;
        }
        (total_bytes as f64 / self.io_time_ms as f64) * 1000.0 / (1024.0 * 1024.0)
    }

    /// Format bytes in human readable format.
    pub fn format_bytes(&self, bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// I/O statistics manager.
#[derive(Debug)]
pub struct IoStats {
    /// Cached disk stats.
    disk_stats: BTreeMap<String, DiskIoStats>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl IoStats {
    /// Create a new I/O stats manager.
    pub fn new() -> Self {
        Self {
            disk_stats: BTreeMap::new(),
            last_update: std::time::Instant::now(),
        }
    }

    /// Get disk I/O statistics.
    pub fn get_disk_io_stats(&mut self, device: &str) -> Result<&DiskIoStats, IoStatsError> {
        self.refresh_stats()?;
        self.disk_stats.get(device)
            .ok_or_else(|| IoStatsError::DeviceNotFound(device.to_string()))
    }

    /// Get all disk I/O statistics.
    pub fn get_all_stats(&mut self) -> Result<Vec<&DiskIoStats>, IoStatsError> {
        self.refresh_stats()?;
        Ok(self.disk_stats.values().collect())
    }

    /// Get total I/O statistics.
    pub fn get_total_stats(&mut self) -> Result<DiskIoStats, IoStatsError> {
        self.refresh_stats()?;
        
        let mut total = DiskIoStats {
            device: "total".to_string(),
            reads_completed: 0,
            reads_merged: 0,
            read_sectors: 0,
            read_time_ms: 0,
            writes_completed: 0,
            writes_merged: 0,
            write_sectors: 0,
            write_time_ms: 0,
            ios_in_progress: 0,
            io_time_ms: 0,
            io_time_weighted_ms: 0,
            discards_completed: 0,
            discards_merged: 0,
            discard_sectors: 0,
            discard_time_ms: 0,
        };
        
        for stats in self.disk_stats.values() {
            total.reads_completed += stats.reads_completed;
            total.reads_merged += stats.reads_merged;
            total.read_sectors += stats.read_sectors;
            total.read_time_ms += stats.read_time_ms;
            total.writes_completed += stats.writes_completed;
            total.writes_merged += stats.writes_merged;
            total.write_sectors += stats.write_sectors;
            total.write_time_ms += stats.write_time_ms;
            total.ios_in_progress += stats.ios_in_progress;
            total.io_time_ms += stats.io_time_ms;
            total.io_time_weighted_ms += stats.io_time_weighted_ms;
            total.discards_completed += stats.discards_completed;
            total.discards_merged += stats.discards_merged;
            total.discard_sectors += stats.discard_sectors;
            total.discard_time_ms += stats.discard_time_ms;
        }
        
        Ok(total)
    }

    /// Get device count.
    pub fn get_device_count(&mut self) -> Result<usize, IoStatsError> {
        self.refresh_stats()?;
        Ok(self.disk_stats.len())
    }

    /// Refresh stats from /proc/diskstats.
    fn refresh_stats(&mut self) -> Result<(), IoStatsError> {
        // Only refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() < 1 {
            return Ok(());
        }

        self.disk_stats.clear();

        let content = fs::read_to_string("/proc/diskstats")
            .map_err(|e| IoStatsError::IoError(format!("Failed to read /proc/diskstats: {}", e)))?;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 14 {
                let device = parts[2].to_string();
                
                // Skip partitions (they have numbers at the end)
                if device.chars().last().map_or(false, |c| c.is_ascii_digit()) {
                    continue;
                }
                
                // Skip loop devices
                if device.starts_with("loop") {
                    continue;
                }
                
                let stats = DiskIoStats {
                    device: device.clone(),
                    reads_completed: parts[3].parse().unwrap_or(0),
                    reads_merged: parts[4].parse().unwrap_or(0),
                    read_sectors: parts[5].parse().unwrap_or(0),
                    read_time_ms: parts[6].parse().unwrap_or(0),
                    writes_completed: parts[7].parse().unwrap_or(0),
                    writes_merged: parts[8].parse().unwrap_or(0),
                    write_sectors: parts[9].parse().unwrap_or(0),
                    write_time_ms: parts[10].parse().unwrap_or(0),
                    ios_in_progress: parts[11].parse().unwrap_or(0),
                    io_time_ms: parts[12].parse().unwrap_or(0),
                    io_time_weighted_ms: parts[13].parse().unwrap_or(0),
                    discards_completed: if parts.len() > 14 { parts[14].parse().unwrap_or(0) } else { 0 },
                    discards_merged: if parts.len() > 15 { parts[15].parse().unwrap_or(0) } else { 0 },
                    discard_sectors: if parts.len() > 16 { parts[16].parse().unwrap_or(0) } else { 0 },
                    discard_time_ms: if parts.len() > 17 { parts[17].parse().unwrap_or(0) } else { 0 },
                };
                
                self.disk_stats.insert(device, stats);
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Get I/O summary.
    pub fn get_io_summary(&mut self) -> Result<String, IoStatsError> {
        let total = self.get_total_stats()?;
        
        Ok(format!(
            "Devices: {}\nTotal read: {}\nTotal write: {}\nTotal I/O: {}\nIOPS: {:.0}\nThroughput: {:.2} MB/s",
            self.get_device_count()?,
            total.format_bytes(total.total_read_bytes()),
            total.format_bytes(total.total_write_bytes()),
            total.format_bytes(total.total_bytes()),
            total.iops(),
            total.throughput_mbps()
        ))
    }
}

impl Default for IoStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mgr = IoStats::new();
        let _ = mgr.last_update;
    }

    #[test]
    fn test_get_disk_io_stats() {
        let mut mgr = IoStats::new();
        // Try to get stats for common devices
        if let Ok(stats) = mgr.get_disk_io_stats("sda") {
            assert_eq!(stats.device, "sda");
        }
    }

    #[test]
    fn test_get_total_stats() {
        let mut mgr = IoStats::new();
        if let Ok(total) = mgr.get_total_stats() {
            assert_eq!(total.device, "total");
        }
    }

    #[test]
    fn test_disk_io_stats_calculations() {
        let stats = DiskIoStats {
            device: "test".to_string(),
            reads_completed: 100,
            reads_merged: 0,
            read_sectors: 1000,
            read_time_ms: 100,
            writes_completed: 50,
            writes_merged: 0,
            write_sectors: 500,
            write_time_ms: 50,
            ios_in_progress: 0,
            io_time_ms: 150,
            io_time_weighted_ms: 15000,
            discards_completed: 0,
            discards_merged: 0,
            discard_sectors: 0,
            discard_time_ms: 0,
        };
        
        assert_eq!(stats.total_sectors(), 1500);
        assert_eq!(stats.total_read_bytes(), 512000);
        assert_eq!(stats.total_write_bytes(), 256000);
        assert!(stats.iops() > 0.0);
        assert_eq!(stats.format_bytes(1024), "1.00 KB");
    }
}
