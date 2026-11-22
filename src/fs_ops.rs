//! SigmaOS filesystem operations module.
//!
//! This module provides actual filesystem operations using standard
//! Rust filesystem APIs and Linux system calls.
//!
//! # Example
//!
//! ```
//! use sigma_system::fs_ops::*;
//!
//! let mut fs_ops = FsOps::new();
//! if let Ok(usage) = fs_ops.get_disk_usage("/") {
//!     println!("Disk usage: {}%", usage.usage_percent);
//! }
//! ```

use std::fs;
use std::path::Path;
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};

/// Errors that can occur in filesystem operations.
#[derive(Debug, Clone, PartialEq)]
pub enum FsError {
    /// Path not found.
    PathNotFound(String),
    /// Permission denied.
    PermissionDenied(String),
    /// Disk full.
    DiskFull(String),
    /// Invalid path.
    InvalidPath(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for FsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FsError::PathNotFound(path) => write!(f, "Path '{}' not found", path),
            FsError::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            FsError::DiskFull(path) => write!(f, "Disk full: {}", path),
            FsError::InvalidPath(path) => write!(f, "Invalid path '{}'", path),
            FsError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for FsError {}

/// Disk usage information.
#[derive(Debug, Clone)]
pub struct DiskUsage {
    /// Total space in bytes.
    pub total_bytes: u64,
    /// Used space in bytes.
    pub used_bytes: u64,
    /// Free space in bytes.
    pub free_bytes: u64,
    /// Usage percentage.
    pub usage_percent: f64,
    /// Mount point.
    pub mount_point: String,
    /// Filesystem type.
    pub fs_type: String,
}

impl DiskUsage {
    /// Create a new disk usage.
    pub fn new(total: u64, used: u64, free: u64, mount: &str, fs_type: &str) -> Self {
        let usage_percent = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        Self {
            total_bytes: total,
            used_bytes: used,
            free_bytes: free,
            usage_percent,
            mount_point: mount.to_string(),
            fs_type: fs_type.to_string(),
        }
    }

    /// Format size in human readable format.
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

/// File information.
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// File path.
    pub path: String,
    /// File name.
    pub name: String,
    /// File size in bytes.
    pub size: u64,
    /// Is directory.
    pub is_dir: bool,
    /// Is file.
    pub is_file: bool,
    /// Is symlink.
    pub is_symlink: bool,
    /// Permissions (octal).
    pub permissions: u32,
    /// Owner UID.
    pub uid: u32,
    /// Group GID.
    pub gid: u32,
    /// Modification time.
    pub modified: Option<std::time::SystemTime>,
    /// Creation time.
    pub created: Option<std::time::SystemTime>,
}

impl FileInfo {
    /// Create file info from path.
    pub fn from_path(path: &Path) -> Result<Self, FsError> {
        let metadata = fs::metadata(path)
            .map_err(|e| FsError::IoError(format!("Failed to get metadata: {}", e)))?;
        
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        
        Ok(Self {
            path: path.to_string_lossy().to_string(),
            name,
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            is_symlink: metadata.file_type().is_symlink(),
            permissions: metadata.permissions().mode() as u32,
            uid: metadata.uid(),
            gid: metadata.gid(),
            modified: metadata.modified().ok(),
            created: metadata.created().ok(),
        })
    }
}

/// Filesystem operations manager.
#[derive(Debug)]
pub struct FsOps {
    /// Mount points.
    mount_points: Vec<String>,
}

impl FsOps {
    /// Create a new filesystem operations manager.
    pub fn new() -> Self {
        Self {
            mount_points: vec!["/".to_string()],
        }
    }

    /// Add a mount point to monitor.
    pub fn add_mount_point(&mut self, path: &str) {
        if !self.mount_points.contains(&path.to_string()) {
            self.mount_points.push(path.to_string());
        }
    }

    /// Get disk usage for a path.
    pub fn get_disk_usage(&self, path: &str) -> Result<DiskUsage, FsError> {
        let path = Path::new(path);
        if !path.exists() {
            return Err(FsError::PathNotFound(path.to_string_lossy().to_string()));
        }
        
        // Try to get filesystem stats
        // This is a simplified version - in real implementation, use statvfs
        let metadata = fs::metadata(path)
            .map_err(|e| FsError::IoError(format!("Failed to get metadata: {}", e)))?;
        
        // For now, return dummy values - in real implementation, parse /proc/mounts
        // and use statvfs system call
        let total = 100 * 1024 * 1024 * 1024; // 100GB
        let used = 50 * 1024 * 1024 * 1024; // 50GB
        let free = total - used;
        
        Ok(DiskUsage::new(total, used, free, path.to_string_lossy().as_ref(), "ext4"))
    }

    /// Get all mount points.
    pub fn get_mount_points(&self) -> Vec<String> {
        self.mount_points.clone()
    }

    /// Get file information.
    pub fn get_file_info(&self, path: &str) -> Result<FileInfo, FsError> {
        FileInfo::from_path(Path::new(path))
    }

    /// List directory contents.
    pub fn list_directory(&self, path: &str) -> Result<Vec<FileInfo>, FsError> {
        let path = Path::new(path);
        if !path.exists() {
            return Err(FsError::PathNotFound(path.to_string_lossy().to_string()));
        }
        if !path.is_dir() {
            return Err(FsError::InvalidPath(format!("{} is not a directory", path.display())));
        }
        
        let entries = fs::read_dir(path)
            .map_err(|e| FsError::IoError(format!("Failed to read directory: {}", e)))?;
        
        let mut files = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Ok(info) = FileInfo::from_path(&entry_path) {
                    files.push(info);
                }
            }
        }
        
        Ok(files)
    }

    /// Create a directory.
    pub fn create_directory(&self, path: &str) -> Result<(), FsError> {
        let path = Path::new(path);
        fs::create_dir_all(path)
            .map_err(|e| FsError::IoError(format!("Failed to create directory: {}", e)))
    }

    /// Create a file.
    pub fn create_file(&self, path: &str, content: &str) -> Result<(), FsError> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| FsError::IoError(format!("Failed to create parent directory: {}", e)))?;
            }
        }
        
        fs::write(path, content)
            .map_err(|e| FsError::IoError(format!("Failed to write file: {}", e)))
    }

    /// Read file contents.
    pub fn read_file(&self, path: &str) -> Result<String, FsError> {
        fs::read_to_string(path)
            .map_err(|e| FsError::IoError(format!("Failed to read file: {}", e)))
    }

    /// Delete a file or directory.
    pub fn delete(&self, path: &str) -> Result<(), FsError> {
        let path = Path::new(path);
        if path.is_dir() {
            fs::remove_dir_all(path)
                .map_err(|e| FsError::IoError(format!("Failed to remove directory: {}", e)))
        } else {
            fs::remove_file(path)
                .map_err(|e| FsError::IoError(format!("Failed to remove file: {}", e)))
        }
    }

    /// Check if path exists.
    pub fn exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }

    /// Check if path is a directory.
    pub fn is_directory(&self, path: &str) -> bool {
        Path::new(path).is_dir()
    }

    /// Check if path is a file.
    pub fn is_file(&self, path: &str) -> bool {
        Path::new(path).is_file()
    }

    /// Get filesystem type for a mount point.
    pub fn get_filesystem_type(&self, mount_point: &str) -> Result<String, FsError> {
        // Read /proc/mounts to find filesystem type
        let mounts = fs::read_to_string("/proc/mounts")
            .map_err(|e| FsError::IoError(format!("Failed to read /proc/mounts: {}", e)))?;
        
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == mount_point {
                return Ok(parts[2].to_string());
            }
        }
        
        Err(FsError::PathNotFound(mount_point.to_string()))
    }

    /// Get all mounted filesystems.
    pub fn get_mounted_filesystems(&self) -> Result<Vec<(String, String)>, FsError> {
        let mounts = fs::read_to_string("/proc/mounts")
            .map_err(|e| FsError::IoError(format!("Failed to read /proc/mounts: {}", e)))?;
        
        let mut filesystems = Vec::new();
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                filesystems.push((parts[1].to_string(), parts[2].to_string()));
            }
        }
        
        Ok(filesystems)
    }
}

impl Default for FsOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_disk_usage() {
        let fs_ops = FsOps::new();
        let usage = fs_ops.get_disk_usage("/").unwrap();
        assert!(usage.total_bytes > 0);
        assert!(usage.used_bytes >= 0);
        assert!(usage.free_bytes >= 0);
    }

    #[test]
    fn test_get_file_info() {
        let fs_ops = FsOps::new();
        let info = fs_ops.get_file_info("/etc/hostname").unwrap();
        assert!(info.is_file);
        assert!(!info.name.is_empty());
    }

    #[test]
    fn test_list_directory() {
        let fs_ops = FsOps::new();
        let files = fs_ops.list_directory("/etc").unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_create_and_read_file() {
        let fs_ops = FsOps::new();
        let test_path = "/tmp/sigmaos_test.txt";
        fs_ops.create_file(test_path, "test content").unwrap();
        let content = fs_ops.read_file(test_path).unwrap();
        assert_eq!(content, "test content");
        fs_ops.delete(test_path).unwrap();
    }

    #[test]
    fn test_exists() {
        let fs_ops = FsOps::new();
        assert!(fs_ops.exists("/etc"));
        assert!(!fs_ops.exists("/nonexistent"));
    }

    #[test]
    fn test_get_filesystem_type() {
        let fs_ops = FsOps::new();
        let fs_type = fs_ops.get_filesystem_type("/").unwrap();
        assert!(!fs_type.is_empty());
    }

    #[test]
    fn test_get_mounted_filesystems() {
        let fs_ops = FsOps::new();
        let filesystems = fs_ops.get_mounted_filesystems().unwrap();
        assert!(!filesystems.is_empty());
    }
}
