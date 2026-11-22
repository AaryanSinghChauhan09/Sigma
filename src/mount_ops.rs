//! SigmaOS mount operations module.
//!
//! This module provides actual mount operations using /proc/mounts and mount/umount.
//!
//! # Example
//!
//! ```
//! use sigma_system::mount_ops::*;
//!
//! let mut mount_ops = MountOps::new();
//! if let Ok(mounts) = mount_ops.list_mounts() {
//!     println!("Found {} mounts", mounts.len());
//! }
//! ```

use std::collections::BTreeMap;
use std::process::Command;
use std::fs;

/// Errors that can occur in mount operations.
#[derive(Debug, Clone, PartialEq)]
pub enum MountError {
    /// Mount point not found.
    MountPointNotFound(String),
    /// Device not found.
    DeviceNotFound(String),
    /// Permission denied.
    PermissionDenied(String),
    /// Mount failed.
    MountFailed(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for MountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MountError::MountPointNotFound(path) => write!(f, "Mount point '{}' not found", path),
            MountError::DeviceNotFound(dev) => write!(f, "Device '{}' not found", dev),
            MountError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            MountError::MountFailed(msg) => write!(f, "Mount failed: {}", msg),
            MountError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for MountError {}

/// Mount information.
#[derive(Debug, Clone)]
pub struct MountInfo {
    /// Device name.
    pub device: String,
    /// Mount point.
    pub mount_point: String,
    /// Filesystem type.
    pub fs_type: String,
    /// Mount options.
    pub options: Vec<String>,
    /// Whether mount is read-only.
    pub read_only: bool,
    /// Whether mount is noexec.
    pub noexec: bool,
    /// Whether mount is nosuid.
    pub nosuid: bool,
    /// Whether mount is nodev.
    pub nodev: bool,
}

/// Mount operations manager.
#[derive(Debug)]
pub struct MountOps {
    /// Cached mounts.
    mounts: BTreeMap<String, MountInfo>,
    /// Last update time.
    last_update: std::time::Instant,
    /// Whether running as root.
    is_root: bool,
}

impl MountOps {
    /// Create a new mount operations manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            mounts: BTreeMap::new(),
            last_update: std::time::Instant::now(),
            is_root,
        }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// List all mounts.
    pub fn list_mounts(&mut self) -> Result<Vec<&MountInfo>, MountError> {
        self.refresh_mounts()?;
        Ok(self.mounts.values().collect())
    }

    /// Get a specific mount.
    pub fn get_mount(&mut self, mount_point: &str) -> Result<Option<&MountInfo>, MountError> {
        self.refresh_mounts()?;
        Ok(self.mounts.get(mount_point))
    }

    /// Check if a path is mounted.
    pub fn is_mounted(&mut self, mount_point: &str) -> Result<bool, MountError> {
        self.refresh_mounts()?;
        Ok(self.mounts.contains_key(mount_point))
    }

    /// Get mount point for a device.
    pub fn get_mount_for_device(&mut self, device: &str) -> Result<Option<&MountInfo>, MountError> {
        self.refresh_mounts()?;
        Ok(self.mounts.values().find(|m| m.device == device))
    }

    /// Get all read-only mounts.
    pub fn get_readonly_mounts(&mut self) -> Result<Vec<&MountInfo>, MountError> {
        self.refresh_mounts()?;
        Ok(self.mounts.values().filter(|m| m.read_only).collect())
    }

    /// Get all noexec mounts.
    pub fn get_noexec_mounts(&mut self) -> Result<Vec<&MountInfo>, MountError> {
        self.refresh_mounts()?;
        Ok(self.mounts.values().filter(|m| m.noexec).collect())
    }

    /// Mount a device.
    pub fn mount(&mut self, device: &str, mount_point: &str, fs_type: &str, options: &[&str]) -> Result<(), MountError> {
        if !self.is_root {
            return Err(MountError::PermissionDenied("Must be root to mount".to_string()));
        }

        // Create mount point if it doesn't exist
        if !std::path::Path::new(mount_point).exists() {
            fs::create_dir_all(mount_point)
                .map_err(|e| MountError::IoError(format!("Failed to create mount point: {}", e)))?;
        }

        let mut args: Vec<String> = vec!["-t".to_string(), fs_type.to_string()];
        let options_str;
        if !options.is_empty() {
            options_str = options.join(",");
            args.push("-o".to_string());
            args.push(options_str);
        }
        args.push(device.to_string());
        args.push(mount_point.to_string());

        let result = Command::new("mount").args(&args).output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_mounts()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(MountError::MountFailed(stderr.to_string()))
                }
            }
            Err(e) => Err(MountError::IoError(format!("Failed to run mount: {}", e))),
        }
    }

    /// Unmount a mount point.
    pub fn unmount(&mut self, mount_point: &str) -> Result<(), MountError> {
        if !self.is_root {
            return Err(MountError::PermissionDenied("Must be root to unmount".to_string()));
        }

        let result = Command::new("umount").arg(mount_point).output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_mounts()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(MountError::MountFailed(stderr.to_string()))
                }
            }
            Err(e) => Err(MountError::IoError(format!("Failed to run umount: {}", e))),
        }
    }

    /// Remount a mount point.
    pub fn remount(&mut self, mount_point: &str, options: &[&str]) -> Result<(), MountError> {
        if !self.is_root {
            return Err(MountError::PermissionDenied("Must be root to remount".to_string()));
        }

        let mut args: Vec<String> = vec!["-o".to_string(), "remount".to_string()];
        let options_str;
        if !options.is_empty() {
            options_str = options.join(",");
            args.push(options_str);
        }
        args.push(mount_point.to_string());

        let result = Command::new("mount").args(&args).output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_mounts()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(MountError::MountFailed(stderr.to_string()))
                }
            }
            Err(e) => Err(MountError::IoError(format!("Failed to run mount: {}", e))),
        }
    }

    /// Refresh mounts from /proc/mounts.
    fn refresh_mounts(&mut self) -> Result<(), MountError> {
        // Only refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() < 1 {
            return Ok(());
        }

        self.mounts.clear();

        let mounts_content = fs::read_to_string("/proc/mounts")
            .map_err(|e| MountError::IoError(format!("Failed to read /proc/mounts: {}", e)))?;

        for line in mounts_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let device = parts[0].to_string();
                let mount_point = parts[1].to_string();
                let fs_type = parts[2].to_string();
                let options_str = parts[3];
                let options: Vec<String> = options_str.split(',').map(|s| s.to_string()).collect();
                
                let read_only = options.contains(&"ro".to_string());
                let noexec = options.contains(&"noexec".to_string());
                let nosuid = options.contains(&"nosuid".to_string());
                let nodev = options.contains(&"nodev".to_string());
                
                self.mounts.insert(mount_point.clone(), MountInfo {
                    device,
                    mount_point,
                    fs_type,
                    options,
                    read_only,
                    noexec,
                    nosuid,
                    nodev,
                });
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Get mount summary.
    pub fn get_mount_summary(&mut self) -> Result<String, MountError> {
        self.refresh_mounts()?;
        
        let mut summary = format!("Total mounts: {}\n", self.mounts.len());
        summary.push_str("Read-only mounts: ");
        summary.push_str(&self.get_readonly_mounts()?.len().to_string());
        summary.push_str("\nNoexec mounts: ");
        summary.push_str(&self.get_noexec_mounts()?.len().to_string());
        
        Ok(summary)
    }
}

impl Default for MountOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mgr = MountOps::new();
        let _ = mgr.is_root();
    }

    #[test]
    fn test_list_mounts() {
        let mut mgr = MountOps::new();
        // May fail in restricted environments without /proc/mounts access
        if let Ok(mounts) = mgr.list_mounts() {
            let _ = mounts;
        }
    }

    #[test]
    fn test_is_mounted() {
        let mut mgr = MountOps::new();
        // May fail in restricted environments
        if let Ok(mounted) = mgr.is_mounted("/") {
            let _ = mounted;
        }
    }

    #[test]
    fn test_get_mount_summary() {
        let mut mgr = MountOps::new();
        if let Ok(summary) = mgr.get_mount_summary() {
            assert!(summary.contains("Total mounts"));
        }
    }
}
