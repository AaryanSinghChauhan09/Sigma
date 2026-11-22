//! SigmaOS disk space operations module.
//!
//! This module provides actual disk space operations using df and du.
//!
//! # Example
//!
//! ```
//! use sigma_system::disk_space_ops::*;
//!
//! let mut disk_ops = DiskSpaceOps::new();
//! if let Ok(space) = disk_ops.get_disk_space("/") {
//!     println!("Used: {}%", space.used_percent);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in disk space operations.
#[derive(Debug, Clone, PartialEq)]
pub enum DiskSpaceError {
    /// Path not found.
    PathNotFound(String),
    /// Failed to get disk space.
    SpaceError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for DiskSpaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskSpaceError::PathNotFound(path) => write!(f, "Path '{}' not found", path),
            DiskSpaceError::SpaceError(msg) => write!(f, "Failed to get disk space: {}", msg),
            DiskSpaceError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for DiskSpaceError {}

/// Disk space information.
#[derive(Debug, Clone)]
pub struct DiskSpace {
    /// Total space in bytes.
    pub total_bytes: u64,
    /// Used space in bytes.
    pub used_bytes: u64,
    /// Available space in bytes.
    pub available_bytes: u64,
    /// Used percentage.
    pub used_percent: f64,
}

/// Disk space operations manager.
#[derive(Debug)]
pub struct DiskSpaceOps {
    /// Whether running as root.
    is_root: bool,
}

impl DiskSpaceOps {
    /// Create a new disk space operations manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self { is_root }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Get disk space for a path.
    pub fn get_disk_space(&self, path: &str) -> Result<DiskSpace, DiskSpaceError> {
        let output = Command::new("df").args(&["-B", path]).output()
            .map_err(|e| DiskSpaceError::IoError(format!("Failed to run df: {}", e)))?;

        if !output.status.success() {
            return Err(DiskSpaceError::SpaceError("df command failed".to_string()));
        }

        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let total = parts[1].parse::<u64>().unwrap_or(0);
                let used = parts[2].parse::<u64>().unwrap_or(0);
                let available = parts[3].parse::<u64>().unwrap_or(0);
                let used_percent = if total > 0 {
                    (used as f64 / total as f64) * 100.0
                } else {
                    0.0
                };

                return Ok(DiskSpace {
                    total_bytes: total,
                    used_bytes: used,
                    available_bytes: available,
                    used_percent,
                });
            }
        }

        Err(DiskSpaceError::SpaceError("Failed to parse df output".to_string()))
    }

    /// Get directory size using du.
    pub fn get_directory_size(&self, path: &str) -> Result<u64, DiskSpaceError> {
        let output = Command::new("du").args(&["-sb", path]).output()
            .map_err(|e| DiskSpaceError::IoError(format!("Failed to run du: {}", e)))?;

        if !output.status.success() {
            return Err(DiskSpaceError::SpaceError("du command failed".to_string()));
        }

        let content = String::from_utf8_lossy(&output.stdout);
        if let Some(size_str) = content.lines().next() {
            let size = size_str.split_whitespace().next()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            Ok(size)
        } else {
            Ok(0)
        }
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

impl Default for DiskSpaceOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let disk_ops = DiskSpaceOps::new();
        let _ = disk_ops.is_root();
    }

    #[test]
    fn test_get_disk_space() {
        let disk_ops = DiskSpaceOps::new();
        if let Ok(space) = disk_ops.get_disk_space("/") {
            assert!(space.total_bytes > 0);
        }
    }

    #[test]
    fn test_format_bytes() {
        let disk_ops = DiskSpaceOps::new();
        assert_eq!(disk_ops.format_bytes(1024), "1.00 KB");
        assert_eq!(disk_ops.format_bytes(1048576), "1.00 MB");
    }
}
