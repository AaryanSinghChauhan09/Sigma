//! SigmaOS timezone operations module.
//!
//! This module provides actual timezone operations using timedatectl and /etc/localtime.
//!
//! # Example
//!
//! ```
//! use sigma_system::timezone_ops::*;
//!
//! let mut tz_ops = TimezoneOps::new();
//! if let Ok(timezone) = tz_ops.get_timezone() {
//!     println!("Current timezone: {}", timezone);
//! }
//! ```

use std::fs;
use std::path::Path;
use std::process::Command;

/// Errors that can occur in timezone operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TimezoneError {
    /// Timezone not found.
    TimezoneNotFound(String),
    /// Failed to set timezone.
    SetError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for TimezoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimezoneError::TimezoneNotFound(tz) => write!(f, "Timezone '{}' not found", tz),
            TimezoneError::SetError(msg) => write!(f, "Failed to set timezone: {}", msg),
            TimezoneError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            TimezoneError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for TimezoneError {}

/// Timezone operations manager.
#[derive(Debug)]
pub struct TimezoneOps {
    /// Zoneinfo path.
    zoneinfo_path: String,
    /// Localtime path.
    localtime_path: String,
    /// Timezone data path.
    timezone_path: String,
    /// Whether running as root.
    is_root: bool,
}

impl TimezoneOps {
    /// Create a new timezone operations manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            zoneinfo_path: "/usr/share/zoneinfo".to_string(),
            localtime_path: "/etc/localtime".to_string(),
            timezone_path: "/etc/timezone".to_string(),
            is_root,
        }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Get current timezone.
    pub fn get_timezone(&self) -> Result<String, TimezoneError> {
        // Try to read from /etc/timezone
        if let Ok(content) = fs::read_to_string(&self.timezone_path) {
            return Ok(content.trim().to_string());
        }

        // Try to read symlink from /etc/localtime
        if let Ok(path) = fs::read_link(&self.localtime_path) {
            if let Some(tz) = path.to_string_lossy().strip_prefix("/usr/share/zoneinfo/") {
                return Ok(tz.to_string());
            }
        }

        // Try timedatectl
        if let Ok(output) = Command::new("timedatectl").arg("status").output() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("Time zone") {
                    if let Some(tz) = line.split(':').nth(1) {
                        return Ok(tz.trim().to_string());
                    }
                }
            }
        }

        Err(TimezoneError::IoError("Could not determine timezone".to_string()))
    }

    /// Set timezone.
    pub fn set_timezone(&self, timezone: &str) -> Result<(), TimezoneError> {
        if !self.is_root {
            return Err(TimezoneError::PermissionDenied("Must be root to set timezone".to_string()));
        }

        let zoneinfo_file = Path::new(&self.zoneinfo_path).join(timezone);
        if !zoneinfo_file.exists() {
            return Err(TimezoneError::TimezoneNotFound(timezone.to_string()));
        }

        // Remove existing localtime
        if Path::new(&self.localtime_path).exists() {
            fs::remove_file(&self.localtime_path)
                .map_err(|e| TimezoneError::IoError(format!("Failed to remove localtime: {}", e)))?;
        }

        // Create symlink
        fs::create_dir_all("/etc")
            .map_err(|e| TimezoneError::IoError(format!("Failed to create /etc: {}", e)))?;
        
        std::os::unix::fs::symlink(&zoneinfo_file, &self.localtime_path)
            .map_err(|e| TimezoneError::IoError(format!("Failed to create symlink: {}", e)))?;

        // Write to /etc/timezone if it exists
        fs::write(&self.timezone_path, timezone)
            .ok();

        Ok(())
    }

    /// List available timezones.
    pub fn list_timezones(&self) -> Result<Vec<String>, TimezoneError> {
        let zoneinfo_dir = Path::new(&self.zoneinfo_path);
        if !zoneinfo_dir.exists() {
            return Err(TimezoneError::IoError("Zoneinfo directory not found".to_string()));
        }

        let mut timezones = Vec::new();
        self.collect_timezones(zoneinfo_dir, "", &mut timezones)?;
        timezones.sort();
        Ok(timezones)
    }

    /// Collect timezones recursively.
    fn collect_timezones(&self, dir: &Path, prefix: &str, timezones: &mut Vec<String>) -> Result<(), TimezoneError> {
        let entries = fs::read_dir(dir)
            .map_err(|e| TimezoneError::IoError(format!("Failed to read directory: {}", e)))?;

        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                let path = entry.path();

                if path.is_dir() {
                    if !name_str.starts_with('.') {
                        let new_prefix = if prefix.is_empty() {
                            name_str.to_string()
                        } else {
                            format!("{}/{}", prefix, name_str)
                        };
                        self.collect_timezones(&path, &new_prefix, timezones)?;
                    }
                } else if path.is_file() {
                    let tz = if prefix.is_empty() {
                        name_str.to_string()
                    } else {
                        format!("{}/{}", prefix, name_str)
                    };
                    timezones.push(tz);
                }
            }
        }

        Ok(())
    }

    /// Get UTC time.
    pub fn get_utc_time(&self) -> Result<String, TimezoneError> {
        let output = Command::new("date").arg("-u").arg("+%Y-%m-%d %H:%M:%S UTC").output()
            .map_err(|e| TimezoneError::IoError(format!("Failed to get UTC time: {}", e)))?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Get local time.
    pub fn get_local_time(&self) -> Result<String, TimezoneError> {
        let output = Command::new("date").arg("+%Y-%m-%d %H:%M:%S").output()
            .map_err(|e| TimezoneError::IoError(format!("Failed to get local time: {}", e)))?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Sync system time with NTP.
    pub fn sync_ntp(&self) -> Result<(), TimezoneError> {
        if !self.is_root {
            return Err(TimezoneError::PermissionDenied("Must be root to sync NTP".to_string()));
        }

        let result = Command::new("timedatectl").arg("set-ntp").arg("true").output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(TimezoneError::SetError(stderr.to_string()))
                }
            }
            Err(e) => Err(TimezoneError::IoError(format!("Failed to run timedatectl: {}", e))),
        }
    }

    /// Get time synchronization status.
    pub fn get_ntp_status(&self) -> Result<bool, TimezoneError> {
        let output = Command::new("timedatectl").arg("status").output()
            .map_err(|e| TimezoneError::IoError(format!("Failed to get NTP status: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.contains("System clock synchronized: yes"))
    }
}

impl Default for TimezoneOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tz_ops = TimezoneOps::new();
        let _ = tz_ops.is_root();
    }

    #[test]
    fn test_get_timezone() {
        let tz_ops = TimezoneOps::new();
        if let Ok(tz) = tz_ops.get_timezone() {
            assert!(!tz.is_empty());
        }
    }

    #[test]
    fn test_get_utc_time() {
        let tz_ops = TimezoneOps::new();
        if let Ok(time) = tz_ops.get_utc_time() {
            assert!(time.contains("UTC"));
        }
    }

    #[test]
    fn test_get_local_time() {
        let tz_ops = TimezoneOps::new();
        if let Ok(time) = tz_ops.get_local_time() {
            assert!(!time.is_empty());
        }
    }
}
