//! SigmaOS systemd unit operations module.
//!
//! This module provides actual systemd unit operations using systemctl.
//!
//! # Example
//!
//! ```
//! use sigma_system::systemd_ops::*;
//!
//! let mut systemd_ops = SystemdOps::new();
//! if let Ok(status) = systemd_ops.get_system_status() {
//!     println!("System status: {}", status.state);
//! }
//! ```

use std::process::Command;

/// Errors that can occur in systemd operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SystemdError {
    /// Unit not found.
    UnitNotFound(String),
    /// Failed to get status.
    StatusError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for SystemdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemdError::UnitNotFound(unit) => write!(f, "Unit '{}' not found", unit),
            SystemdError::StatusError(msg) => write!(f, "Failed to get status: {}", msg),
            SystemdError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            SystemdError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for SystemdError {}

/// System status.
#[derive(Debug, Clone)]
pub struct SystemStatus {
    /// System state.
    pub state: String,
    /// Default target.
    pub default_target: String,
    /// Init state.
    pub init_state: String,
}

/// Unit status.
#[derive(Debug, Clone)]
pub struct UnitStatus {
    /// Unit name.
    pub name: String,
    /// Load state.
    pub load_state: String,
    /// Active state.
    pub active_state: String,
    /// Sub state.
    pub sub_state: String,
    /// Description.
    pub description: String,
}

/// Systemd operations manager.
#[derive(Debug)]
pub struct SystemdOps {
    /// Whether running as root.
    is_root: bool,
}

impl SystemdOps {
    /// Create a new systemd operations manager.
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

    /// Get system status.
    pub fn get_system_status(&self) -> Result<SystemStatus, SystemdError> {
        let output = Command::new("systemctl").arg("is-system-running").output()
            .map_err(|e| SystemdError::IoError(format!("Failed to get system status: {}", e)))?;
        
        let state = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        let default_target = self.get_default_target().unwrap_or_else(|_| "unknown".to_string());
        let init_state = if state == "running" { "active" } else { state.as_str() }.to_string();
        
        Ok(SystemStatus {
            state,
            default_target,
            init_state,
        })
    }

    /// Get default target.
    pub fn get_default_target(&self) -> Result<String, SystemdError> {
        let output = Command::new("systemctl").arg("get-default").output()
            .map_err(|e| SystemdError::IoError(format!("Failed to get default target: {}", e)))?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Set default target.
    pub fn set_default_target(&self, target: &str) -> Result<(), SystemdError> {
        if !self.is_root {
            return Err(SystemdError::PermissionDenied("Must be root to set default target".to_string()));
        }

        let result = Command::new("systemctl").arg("set-default").arg(target).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(SystemdError::StatusError("systemctl set-default failed".to_string())),
            Err(e) => Err(SystemdError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Get unit status.
    pub fn get_unit_status(&self, unit: &str) -> Result<UnitStatus, SystemdError> {
        let output = Command::new("systemctl").arg("status").arg(unit).arg("--no-pager").output()
            .map_err(|e| SystemdError::IoError(format!("Failed to get unit status: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        
        let mut name = unit.to_string();
        let mut load_state = "unknown".to_string();
        let mut active_state = "unknown".to_string();
        let mut sub_state = "unknown".to_string();
        let mut description = String::new();
        
        for line in content.lines() {
            if line.contains("Loaded:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    load_state = parts[1].to_string();
                }
            } else if line.contains("Active:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    active_state = parts[1].to_string();
                }
                if parts.len() > 2 {
                    sub_state = parts[2].trim_end_matches(')').to_string();
                }
            } else if line.contains("Description:") {
                if let Some(desc) = line.split(':').nth(1) {
                    description = desc.trim().to_string();
                }
            }
        }
        
        Ok(UnitStatus {
            name,
            load_state,
            active_state,
            sub_state,
            description,
        })
    }

    /// List all units.
    pub fn list_units(&self) -> Result<Vec<UnitStatus>, SystemdError> {
        let output = Command::new("systemctl").arg("list-units").arg("--no-pager").arg("--all").output()
            .map_err(|e| SystemdError::IoError(format!("Failed to list units: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let mut units = Vec::new();
        
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                units.push(UnitStatus {
                    name: parts[0].to_string(),
                    load_state: parts[1].to_string(),
                    active_state: parts[2].to_string(),
                    sub_state: parts[3].to_string(),
                    description: if parts.len() > 4 { parts[4..].join(" ") } else { String::new() },
                });
            }
        }
        
        Ok(units)
    }

    /// List failed units.
    pub fn list_failed_units(&self) -> Result<Vec<UnitStatus>, SystemdError> {
        let output = Command::new("systemctl").arg("list-units").arg("--failed").arg("--no-pager").output()
            .map_err(|e| SystemdError::IoError(format!("Failed to list failed units: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let mut units = Vec::new();
        
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                units.push(UnitStatus {
                    name: parts[0].to_string(),
                    load_state: parts[1].to_string(),
                    active_state: parts[2].to_string(),
                    sub_state: parts[3].to_string(),
                    description: if parts.len() > 4 { parts[4..].join(" ") } else { String::new() },
                });
            }
        }
        
        Ok(units)
    }

    /// Get active services count.
    pub fn get_active_services_count(&self) -> Result<usize, SystemdError> {
        let units = self.list_units()?;
        Ok(units.iter().filter(|u| u.active_state == "active" && u.name.ends_with(".service")).count())
    }

    /// Get failed services count.
    pub fn get_failed_services_count(&self) -> Result<usize, SystemdError> {
        let failed = self.list_failed_units()?;
        Ok(failed.iter().filter(|u| u.name.ends_with(".service")).count())
    }

    /// Get journal logs for a unit.
    pub fn get_unit_logs(&self, unit: &str, lines: u32) -> Result<String, SystemdError> {
        let output = Command::new("journalctl")
            .args(&["-u", unit, "-n", &lines.to_string(), "--no-pager"])
            .output()
            .map_err(|e| SystemdError::IoError(format!("Failed to get unit logs: {}", e)))?;
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Restart a unit.
    pub fn restart_unit(&self, unit: &str) -> Result<(), SystemdError> {
        if !self.is_root {
            return Err(SystemdError::PermissionDenied("Must be root to restart unit".to_string()));
        }

        let result = Command::new("systemctl").arg("restart").arg(unit).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(SystemdError::StatusError("systemctl restart failed".to_string())),
            Err(e) => Err(SystemdError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Reload a unit.
    pub fn reload_unit(&self, unit: &str) -> Result<(), SystemdError> {
        if !self.is_root {
            return Err(SystemdError::PermissionDenied("Must be root to reload unit".to_string()));
        }

        let result = Command::new("systemctl").arg("reload").arg(unit).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(SystemdError::StatusError("systemctl reload failed".to_string())),
            Err(e) => Err(SystemdError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Get system summary.
    pub fn get_system_summary(&self) -> Result<String, SystemdError> {
        let status = self.get_system_status()?;
        let active_count = self.get_active_services_count().unwrap_or(0);
        let failed_count = self.get_failed_services_count().unwrap_or(0);
        
        Ok(format!(
            "System state: {}\nDefault target: {}\nActive services: {}\nFailed services: {}",
            status.state,
            status.default_target,
            active_count,
            failed_count
        ))
    }
}

impl Default for SystemdOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let systemd_ops = SystemdOps::new();
        let _ = systemd_ops.is_root();
    }

    #[test]
    fn test_get_system_status() {
        let systemd_ops = SystemdOps::new();
        if let Ok(status) = systemd_ops.get_system_status() {
            assert!(!status.state.is_empty());
        }
    }

    #[test]
    fn test_get_default_target() {
        let systemd_ops = SystemdOps::new();
        if let Ok(target) = systemd_ops.get_default_target() {
            assert!(!target.is_empty());
        }
    }

    #[test]
    fn test_list_units() {
        let systemd_ops = SystemdOps::new();
        if let Ok(units) = systemd_ops.list_units() {
            let _ = units;
        }
    }
}
