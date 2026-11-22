//! SigmaOS interface statistics module.
//!
//! This module provides actual network interface statistics using /sys/class/net.
//!
//! # Example
//!
//! ```
//! use sigma_system::net_stats::*;
//!
//! let mut net_stats = NetStats::new();
//! if let Ok(stats) = net_stats.get_interface_stats("eth0") {
//!     println!("RX: {} bytes, TX: {} bytes", stats.rx_bytes, stats.tx_bytes);
//! }
//! ```

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Errors that can occur in network stats operations.
#[derive(Debug, Clone, PartialEq)]
pub enum NetStatsError {
    /// Interface not found.
    InterfaceNotFound(String),
    /// Failed to read stats.
    ReadError(String),
    /// Invalid stats data.
    InvalidData(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for NetStatsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetStatsError::InterfaceNotFound(name) => write!(f, "Interface '{}' not found", name),
            NetStatsError::ReadError(msg) => write!(f, "Failed to read stats: {}", msg),
            NetStatsError::InvalidData(msg) => write!(f, "Invalid stats data: {}", msg),
            NetStatsError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for NetStatsError {}

/// Interface statistics.
#[derive(Debug, Clone)]
pub struct InterfaceStats {
    /// Interface name.
    pub name: String,
    /// RX bytes.
    pub rx_bytes: u64,
    /// TX bytes.
    pub tx_bytes: u64,
    /// RX packets.
    pub rx_packets: u64,
    /// TX packets.
    pub tx_packets: u64,
    /// RX errors.
    pub rx_errors: u64,
    /// TX errors.
    pub tx_errors: u64,
    /// RX drops.
    pub rx_drops: u64,
    /// TX drops.
    pub tx_drops: u64,
    /// RX multicast packets.
    pub rx_multicast: u64,
    /// TX multicast packets.
    pub tx_multicast: u64,
    /// RX compressed packets.
    pub rx_compressed: u64,
    /// TX compressed packets.
    pub tx_compressed: u64,
}

impl InterfaceStats {
    /// Calculate total RX+TX bytes.
    pub fn total_bytes(&self) -> u64 {
        self.rx_bytes + self.tx_bytes
    }

    /// Calculate total RX+TX packets.
    pub fn total_packets(&self) -> u64 {
        self.rx_packets + self.tx_packets
    }

    /// Calculate error rate.
    pub fn error_rate(&self) -> f64 {
        let total_packets = self.total_packets();
        if total_packets == 0 {
            return 0.0;
        }
        let total_errors = self.rx_errors + self.tx_errors;
        (total_errors as f64 / total_packets as f64) * 100.0
    }

    /// Calculate drop rate.
    pub fn drop_rate(&self) -> f64 {
        let total_packets = self.total_packets();
        if total_packets == 0 {
            return 0.0;
        }
        let total_drops = self.rx_drops + self.tx_drops;
        (total_drops as f64 / total_packets as f64) * 100.0
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

/// Network statistics manager.
#[derive(Debug)]
pub struct NetStats {
    /// Cached interface stats.
    interface_stats: BTreeMap<String, InterfaceStats>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl NetStats {
    /// Create a new network stats manager.
    pub fn new() -> Self {
        Self {
            interface_stats: BTreeMap::new(),
            last_update: std::time::Instant::now(),
        }
    }

    /// Get interface statistics.
    pub fn get_interface_stats(&mut self, interface: &str) -> Result<&InterfaceStats, NetStatsError> {
        self.refresh_stats()?;
        self.interface_stats.get(interface)
            .ok_or_else(|| NetStatsError::InterfaceNotFound(interface.to_string()))
    }

    /// Get all interface statistics.
    pub fn get_all_stats(&mut self) -> Result<Vec<&InterfaceStats>, NetStatsError> {
        self.refresh_stats()?;
        Ok(self.interface_stats.values().collect())
    }

    /// Get total network statistics.
    pub fn get_total_stats(&mut self) -> Result<InterfaceStats, NetStatsError> {
        self.refresh_stats()?;
        
        let mut total = InterfaceStats {
            name: "total".to_string(),
            rx_bytes: 0,
            tx_bytes: 0,
            rx_packets: 0,
            tx_packets: 0,
            rx_errors: 0,
            tx_errors: 0,
            rx_drops: 0,
            tx_drops: 0,
            rx_multicast: 0,
            tx_multicast: 0,
            rx_compressed: 0,
            tx_compressed: 0,
        };
        
        for stats in self.interface_stats.values() {
            total.rx_bytes += stats.rx_bytes;
            total.tx_bytes += stats.tx_bytes;
            total.rx_packets += stats.rx_packets;
            total.tx_packets += stats.tx_packets;
            total.rx_errors += stats.rx_errors;
            total.tx_errors += stats.tx_errors;
            total.rx_drops += stats.rx_drops;
            total.tx_drops += stats.tx_drops;
            total.rx_multicast += stats.rx_multicast;
            total.tx_multicast += stats.tx_multicast;
            total.rx_compressed += stats.rx_compressed;
            total.tx_compressed += stats.tx_compressed;
        }
        
        Ok(total)
    }

    /// Get interface count.
    pub fn get_interface_count(&mut self) -> Result<usize, NetStatsError> {
        self.refresh_stats()?;
        Ok(self.interface_stats.len())
    }

    /// Refresh stats from /sys/class/net.
    fn refresh_stats(&mut self) -> Result<(), NetStatsError> {
        // Only refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() < 1 {
            return Ok(());
        }

        self.interface_stats.clear();

        let net_dir = Path::new("/sys/class/net");
        if !net_dir.exists() {
            return Err(NetStatsError::IoError("/sys/class/net not found".to_string()));
        }

        let entries = fs::read_dir(net_dir)
            .map_err(|e| NetStatsError::IoError(format!("Failed to read /sys/class/net: {}", e)))?;

        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                if let Ok(stats) = self.read_interface_stats(&name, &entry.path()) {
                    self.interface_stats.insert(name.clone(), stats);
                }
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Read interface statistics.
    fn read_interface_stats(&self, name: &str, iface_path: &Path) -> Result<InterfaceStats, NetStatsError> {
        let stats_path = iface_path.join("statistics");
        
        Ok(InterfaceStats {
            name: name.to_string(),
            rx_bytes: self.read_stat(&stats_path, "rx_bytes")?,
            tx_bytes: self.read_stat(&stats_path, "tx_bytes")?,
            rx_packets: self.read_stat(&stats_path, "rx_packets")?,
            tx_packets: self.read_stat(&stats_path, "tx_packets")?,
            rx_errors: self.read_stat(&stats_path, "rx_errors")?,
            tx_errors: self.read_stat(&stats_path, "tx_errors")?,
            rx_drops: self.read_stat(&stats_path, "rx_dropped")?,
            tx_drops: self.read_stat(&stats_path, "tx_dropped")?,
            rx_multicast: self.read_stat(&stats_path, "multicast")?,
            tx_multicast: self.read_stat(&stats_path, "tx_multicast")?,
            rx_compressed: self.read_stat(&stats_path, "rx_compressed")?,
            tx_compressed: self.read_stat(&stats_path, "tx_compressed")?,
        })
    }

    /// Read a statistic file.
    fn read_stat(&self, stats_path: &Path, name: &str) -> Result<u64, NetStatsError> {
        let path = stats_path.join(name);
        fs::read_to_string(&path)
            .map_err(|e| NetStatsError::IoError(format!("Failed to read {}: {}", name, e)))?
            .trim()
            .parse()
            .map_err(|e| NetStatsError::InvalidData(format!("Failed to parse {}: {}", name, e)))
    }

    /// Get network summary.
    pub fn get_network_summary(&mut self) -> Result<String, NetStatsError> {
        let total = self.get_total_stats()?;
        
        Ok(format!(
            "Interfaces: {}\nTotal RX: {}\nTotal TX: {}\nTotal packets: {}\nError rate: {:.2}%\nDrop rate: {:.2}%",
            self.get_interface_count()?,
            total.format_bytes(total.rx_bytes),
            total.format_bytes(total.tx_bytes),
            total.total_packets(),
            total.error_rate(),
            total.drop_rate()
        ))
    }
}

impl Default for NetStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mgr = NetStats::new();
        let _ = mgr.last_update;
    }

    #[test]
    fn test_get_interface_stats() {
        let mut mgr = NetStats::new();
        if let Ok(stats) = mgr.get_interface_stats("lo") {
            assert_eq!(stats.name, "lo");
        }
    }

    #[test]
    fn test_get_total_stats() {
        let mut mgr = NetStats::new();
        if let Ok(total) = mgr.get_total_stats() {
            assert!(total.name == "total");
        }
    }

    #[test]
    fn test_interface_stats_calculations() {
        let stats = InterfaceStats {
            name: "test".to_string(),
            rx_bytes: 1000,
            tx_bytes: 2000,
            rx_packets: 10,
            tx_packets: 20,
            rx_errors: 1,
            tx_errors: 1,
            rx_drops: 0,
            tx_drops: 0,
            rx_multicast: 0,
            tx_multicast: 0,
            rx_compressed: 0,
            tx_compressed: 0,
        };
        
        assert_eq!(stats.total_bytes(), 3000);
        assert_eq!(stats.total_packets(), 30);
        assert!(stats.error_rate() > 0.0);
        assert_eq!(stats.format_bytes(1024), "1.00 KB");
    }
}
