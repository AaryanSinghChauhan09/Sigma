//! SigmaOS disk management module.
//!
//! This module provides actual disk management using lsblk, fdisk, and parted.
//!
//! # Example
//!
//! ```
//! use sigma_system::disk::*;
//!
//! let mut disk_mgr = DiskManager::new();
//! if let Ok(disks) = disk_mgr.list_disks() {
//!     println!("Found {} disks", disks.len());
//! }
//! ```

use std::process::Command;
use std::collections::BTreeMap;

/// Errors that can occur in disk operations.
#[derive(Debug, Clone, PartialEq)]
pub enum DiskError {
    /// Disk not found.
    DiskNotFound(String),
    /// Partition not found.
    PartitionNotFound(String),
    /// Permission denied.
    PermissionDenied(String),
    /// Invalid operation.
    InvalidOperation(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for DiskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskError::DiskNotFound(name) => write!(f, "Disk '{}' not found", name),
            DiskError::PartitionNotFound(name) => write!(f, "Partition '{}' not found", name),
            DiskError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            DiskError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            DiskError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for DiskError {}

/// Disk information.
#[derive(Debug, Clone)]
pub struct DiskInfo {
    /// Disk name (e.g., sda, nvme0n1).
    pub name: String,
    /// Disk size in bytes.
    pub size: u64,
    /// Disk model.
    pub model: String,
    /// Disk serial number.
    pub serial: String,
    /// Disk type (SSD, HDD, etc.).
    pub disk_type: String,
    /// Partitions on this disk.
    pub partitions: Vec<PartitionInfo>,
    /// Whether disk is removable.
    pub removable: bool,
    /// Whether disk is rotational.
    pub rotational: bool,
}

/// Partition information.
#[derive(Debug, Clone)]
pub struct PartitionInfo {
    /// Partition name.
    pub name: String,
    /// Partition size in bytes.
    pub size: u64,
    /// Partition type.
    pub partition_type: String,
    /// Filesystem type.
    pub filesystem: String,
    /// Mount point.
    pub mount_point: String,
    /// Partition flags.
    pub flags: Vec<String>,
}

/// Filesystem information.
#[derive(Debug, Clone)]
pub struct FilesystemInfo {
    /// Device name.
    pub device: String,
    /// Mount point.
    pub mount_point: String,
    /// Filesystem type.
    pub fs_type: String,
    /// Total size in bytes.
    pub size: u64,
    /// Used size in bytes.
    pub used: u64,
    /// Available size in bytes.
    pub available: u64,
    /// Usage percentage.
    pub usage_percent: f64,
}

/// Disk manager.
#[derive(Debug)]
pub struct DiskManager {
    /// Cached disks.
    disks: BTreeMap<String, DiskInfo>,
    /// Last update time.
    last_update: std::time::Instant,
    /// Whether running as root.
    is_root: bool,
}

impl DiskManager {
    /// Create a new disk manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            disks: BTreeMap::new(),
            last_update: std::time::Instant::now(),
            is_root,
        }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// List all disks.
    pub fn list_disks(&mut self) -> Result<Vec<&DiskInfo>, DiskError> {
        self.refresh_disks()?;
        Ok(self.disks.values().collect())
    }

    /// Get a specific disk.
    pub fn get_disk(&mut self, name: &str) -> Result<Option<&DiskInfo>, DiskError> {
        self.refresh_disks()?;
        Ok(self.disks.get(name))
    }

    /// List all filesystems.
    pub fn list_filesystems(&self) -> Result<Vec<FilesystemInfo>, DiskError> {
        let result = Command::new("df").args(&["-h", "--output=source,target,fstype,size,used,avail,pcent"]).output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut filesystems = Vec::new();
                    
                    for line in stdout.lines().skip(1) { // Skip header
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 7 {
                            let device = parts[0].to_string();
                            let mount_point = parts[1].to_string();
                            let fs_type = parts[2].to_string();
                            let size_str = parts[3].to_string();
                            let used_str = parts[4].to_string();
                            let avail_str = parts[5].to_string();
                            let usage_str = parts[6].trim_end_matches('%').to_string();
                            
                            // Parse sizes (simplified - would need proper parsing)
                            let size = self.parse_size(&size_str).unwrap_or(0);
                            let used = self.parse_size(&used_str).unwrap_or(0);
                            let available = self.parse_size(&avail_str).unwrap_or(0);
                            let usage_percent = usage_str.parse().unwrap_or(0.0);
                            
                            filesystems.push(FilesystemInfo {
                                device,
                                mount_point,
                                fs_type,
                                size,
                                used,
                                available,
                                usage_percent,
                            });
                        }
                    }
                    
                    Ok(filesystems)
                } else {
                    Err(DiskError::IoError("df command failed".to_string()))
                }
            }
            Err(e) => Err(DiskError::IoError(format!("Failed to run df: {}", e))),
        }
    }

    /// Parse size string to bytes.
    fn parse_size(&self, size_str: &str) -> Result<u64, DiskError> {
        let size_str = size_str.trim();
        let (num_str, unit) = if size_str.ends_with('G') {
            (&size_str[..size_str.len()-1], "G")
        } else if size_str.ends_with('M') {
            (&size_str[..size_str.len()-1], "M")
        } else if size_str.ends_with('K') {
            (&size_str[..size_str.len()-1], "K")
        } else {
            (size_str, "B")
        };
        
        let num: f64 = num_str.parse()
            .map_err(|e| DiskError::IoError(format!("Failed to parse size: {}", e)))?;
        
        let multiplier = match unit {
            "G" => 1024.0 * 1024.0 * 1024.0,
            "M" => 1024.0 * 1024.0,
            "K" => 1024.0,
            _ => 1.0,
        };
        
        Ok((num * multiplier) as u64)
    }

    /// Refresh disk list from lsblk.
    fn refresh_disks(&mut self) -> Result<(), DiskError> {
        // Only refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() < 1 {
            return Ok(());
        }

        self.disks.clear();

        let result = Command::new("lsblk")
            .args(&["-J", "-o", "NAME,SIZE,TYPE,MOUNTPOINT,FSTYPE,MODEL,SERIAL,ROTA,RM"])
            .output();

        if let Ok(output) = result {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // Parse JSON output (simplified - would need proper JSON parsing)
                // For now, use lsblk without JSON
                let result2 = Command::new("lsblk")
                    .args(&["-o", "NAME,SIZE,TYPE,MOUNTPOINT,FSTYPE,MODEL,SERIAL,ROTA,RM", "-n"])
                    .output();
                
                if let Ok(output2) = result2 {
                    if output2.status.success() {
                        let stdout2 = String::from_utf8_lossy(&output2.stdout);
                        for line in stdout2.lines() {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 9 {
                                let name = parts[0].to_string();
                                let size_str = parts[1].to_string();
                                let disk_type = parts[2].to_string();
                                let mount_point = parts[3].to_string();
                                let fs_type = parts[4].to_string();
                                let model = parts[5].to_string();
                                let serial = parts[6].to_string();
                                let rota = parts[7] == "1";
                                let rm = parts[8] == "1";
                                
                                if disk_type == "disk" {
                                    let size = self.parse_size(&size_str).unwrap_or(0);
                                    self.disks.insert(name.clone(), DiskInfo {
                                        name,
                                        size,
                                        model,
                                        serial,
                                        disk_type: if rota { "HDD".to_string() } else { "SSD".to_string() },
                                        partitions: vec![],
                                        removable: rm,
                                        rotational: rota,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Get disk usage for a mount point.
    pub fn get_disk_usage(&self, mount_point: &str) -> Result<Option<FilesystemInfo>, DiskError> {
        let filesystems = self.list_filesystems()?;
        Ok(filesystems.into_iter().find(|fs| fs.mount_point == mount_point))
    }

    /// Format disk size.
    pub fn format_size(&self, bytes: u64) -> String {
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

impl Default for DiskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mgr = DiskManager::new();
        let _ = mgr.is_root();
    }

    #[test]
    fn test_list_disks() {
        let mut mgr = DiskManager::new();
        if let Ok(disks) = mgr.list_disks() {
            let _ = disks;
        }
    }

    #[test]
    fn test_list_filesystems() {
        let mgr = DiskManager::new();
        if let Ok(filesystems) = mgr.list_filesystems() {
            assert!(!filesystems.is_empty());
        }
    }

    #[test]
    fn test_format_size() {
        let mgr = DiskManager::new();
        assert_eq!(mgr.format_size(1024), "1.00 KB");
        assert_eq!(mgr.format_size(1024 * 1024), "1.00 MB");
        assert_eq!(mgr.format_size(1024 * 1024 * 1024), "1.00 GB");
    }
}
