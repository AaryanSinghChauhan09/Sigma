//! SigmaOS service management module.
//!
//! This module provides actual service management using systemctl.
//!
//! # Example
//!
//! ```
//! use sigma_system::service::*;
//!
//! let mut svc_mgr = ServiceManager::new();
//! if svc_mgr.is_root() {
//!     svc_mgr.start_service("sshd").unwrap();
//!     svc_mgr.enable_service("sshd").unwrap();
//! }
//! ```

use std::process::Command;
use std::collections::BTreeMap;

/// Errors that can occur in service management.
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceError {
    /// Service not found.
    ServiceNotFound(String),
    /// Service already running.
    ServiceRunning(String),
    /// Service not running.
    ServiceNotRunning(String),
    /// Permission denied.
    PermissionDenied(String),
    /// Systemd not available.
    SystemdNotAvailable(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::ServiceNotFound(name) => write!(f, "Service '{}' not found", name),
            ServiceError::ServiceRunning(name) => write!(f, "Service '{}' already running", name),
            ServiceError::ServiceNotRunning(name) => write!(f, "Service '{}' not running", name),
            ServiceError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            ServiceError::SystemdNotAvailable(msg) => write!(f, "systemd not available: {}", msg),
            ServiceError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}

/// Service status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    /// Service is active.
    Active,
    /// Service is inactive.
    Inactive,
    /// Service is failed.
    Failed,
    /// Service is activating.
    Activating,
    /// Service is deactivating.
    Deactivating,
    /// Service is unknown.
    Unknown,
}

impl ServiceStatus {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            ServiceStatus::Active => "active",
            ServiceStatus::Inactive => "inactive",
            ServiceStatus::Failed => "failed",
            ServiceStatus::Activating => "activating",
            ServiceStatus::Deactivating => "deactivating",
            ServiceStatus::Unknown => "unknown",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" | "running" => ServiceStatus::Active,
            "inactive" | "stopped" => ServiceStatus::Inactive,
            "failed" | "error" => ServiceStatus::Failed,
            "activating" | "starting" => ServiceStatus::Activating,
            "deactivating" | "stopping" => ServiceStatus::Deactivating,
            _ => ServiceStatus::Unknown,
        }
    }
}

/// Service information.
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    /// Service name.
    pub name: String,
    /// Service status.
    pub status: ServiceStatus,
    /// Whether service is enabled.
    pub enabled: bool,
    /// Service description.
    pub description: String,
    /// PID of main process.
    pub pid: Option<u32>,
    /// Memory usage in KB.
    pub memory_kb: u64,
    /// CPU usage percentage.
    pub cpu_percent: f64,
}

/// Service manager.
#[derive(Debug)]
pub struct ServiceManager {
    /// Whether systemd is available.
    systemd_available: bool,
    /// Whether running as root.
    is_root: bool,
    /// Cached services.
    services: BTreeMap<String, ServiceInfo>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl ServiceManager {
    /// Create a new service manager.
    pub fn new() -> Self {
        let systemd_available = Command::new("systemctl").arg("--version").output().is_ok();
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            systemd_available,
            is_root,
            services: BTreeMap::new(),
            last_update: std::time::Instant::now(),
        }
    }

    /// Check if systemd is available.
    pub fn is_systemd_available(&self) -> bool {
        self.systemd_available
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Start a service.
    pub fn start_service(&mut self, name: &str) -> Result<(), ServiceError> {
        if !self.systemd_available {
            return Err(ServiceError::SystemdNotAvailable("systemctl not found".to_string()));
        }
        if !self.is_root {
            return Err(ServiceError::PermissionDenied("Must be root to start services".to_string()));
        }

        let result = Command::new("systemctl")
            .args(&["start", name])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_services()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(ServiceError::IoError(format!("Failed to start service: {}", stderr)))
                }
            }
            Err(e) => Err(ServiceError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Stop a service.
    pub fn stop_service(&mut self, name: &str) -> Result<(), ServiceError> {
        if !self.systemd_available {
            return Err(ServiceError::SystemdNotAvailable("systemctl not found".to_string()));
        }
        if !self.is_root {
            return Err(ServiceError::PermissionDenied("Must be root to stop services".to_string()));
        }

        let result = Command::new("systemctl")
            .args(&["stop", name])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_services()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(ServiceError::IoError(format!("Failed to stop service: {}", stderr)))
                }
            }
            Err(e) => Err(ServiceError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Restart a service.
    pub fn restart_service(&mut self, name: &str) -> Result<(), ServiceError> {
        if !self.systemd_available {
            return Err(ServiceError::SystemdNotAvailable("systemctl not found".to_string()));
        }
        if !self.is_root {
            return Err(ServiceError::PermissionDenied("Must be root to restart services".to_string()));
        }

        let result = Command::new("systemctl")
            .args(&["restart", name])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_services()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(ServiceError::IoError(format!("Failed to restart service: {}", stderr)))
                }
            }
            Err(e) => Err(ServiceError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Enable a service.
    pub fn enable_service(&mut self, name: &str) -> Result<(), ServiceError> {
        if !self.systemd_available {
            return Err(ServiceError::SystemdNotAvailable("systemctl not found".to_string()));
        }
        if !self.is_root {
            return Err(ServiceError::PermissionDenied("Must be root to enable services".to_string()));
        }

        let result = Command::new("systemctl")
            .args(&["enable", name])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_services()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(ServiceError::IoError(format!("Failed to enable service: {}", stderr)))
                }
            }
            Err(e) => Err(ServiceError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Disable a service.
    pub fn disable_service(&mut self, name: &str) -> Result<(), ServiceError> {
        if !self.systemd_available {
            return Err(ServiceError::SystemdNotAvailable("systemctl not found".to_string()));
        }
        if !self.is_root {
            return Err(ServiceError::PermissionDenied("Must be root to disable services".to_string()));
        }

        let result = Command::new("systemctl")
            .args(&["disable", name])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_services()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(ServiceError::IoError(format!("Failed to disable service: {}", stderr)))
                }
            }
            Err(e) => Err(ServiceError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }

    /// Get service status.
    pub fn get_service_status(&mut self, name: &str) -> Result<ServiceStatus, ServiceError> {
        self.refresh_services()?;
        if let Some(service) = self.services.get(name) {
            Ok(service.status)
        } else {
            Err(ServiceError::ServiceNotFound(name.to_string()))
        }
    }

    /// Get service info.
    pub fn get_service_info(&mut self, name: &str) -> Result<Option<&ServiceInfo>, ServiceError> {
        self.refresh_services()?;
        Ok(self.services.get(name))
    }

    /// List all services.
    pub fn list_services(&mut self) -> Result<Vec<&ServiceInfo>, ServiceError> {
        self.refresh_services()?;
        Ok(self.services.values().collect())
    }

    /// List running services.
    pub fn list_running_services(&mut self) -> Result<Vec<&ServiceInfo>, ServiceError> {
        self.refresh_services()?;
        Ok(self.services.values()
            .filter(|s| s.status == ServiceStatus::Active)
            .collect())
    }

    /// List failed services.
    pub fn list_failed_services(&mut self) -> Result<Vec<&ServiceInfo>, ServiceError> {
        self.refresh_services()?;
        Ok(self.services.values()
            .filter(|s| s.status == ServiceStatus::Failed)
            .collect())
    }

    /// Refresh services list from systemctl.
    fn refresh_services(&mut self) -> Result<(), ServiceError> {
        // Only refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() < 1 {
            return Ok(());
        }

        self.services.clear();

        if !self.systemd_available {
            return Ok(());
        }

        // Get list of services
        let result = Command::new("systemctl")
            .args(&["list-units", "--type=service", "--all", "--no-legend", "--no-pager"])
            .output();

        if let Ok(output) = result {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let name = parts[0].trim_end_matches(".service").to_string();
                        let status = ServiceStatus::parse(parts[2]);
                        let enabled = parts[3].contains("enabled");
                        
                        self.services.insert(name.clone(), ServiceInfo {
                            name,
                            status,
                            enabled,
                            description: String::new(),
                            pid: None,
                            memory_kb: 0,
                            cpu_percent: 0.0,
                        });
                    }
                }
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Get service logs.
    pub fn get_service_logs(&self, name: &str, lines: u32) -> Result<String, ServiceError> {
        if !self.systemd_available {
            return Err(ServiceError::SystemdNotAvailable("systemctl not found".to_string()));
        }

        let result = Command::new("journalctl")
            .args(&["-u", name, "-n", &lines.to_string(), "--no-pager"])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(ServiceError::IoError(format!("Failed to get logs: {}", stderr)))
                }
            }
            Err(e) => Err(ServiceError::IoError(format!("Failed to run journalctl: {}", e))),
        }
    }

    /// Reload systemd configuration.
    pub fn reload_config(&mut self) -> Result<(), ServiceError> {
        if !self.systemd_available {
            return Err(ServiceError::SystemdNotAvailable("systemctl not found".to_string()));
        }
        if !self.is_root {
            return Err(ServiceError::PermissionDenied("Must be root to reload config".to_string()));
        }

        let result = Command::new("systemctl")
            .args(&["daemon-reload"])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(ServiceError::IoError(format!("Failed to reload config: {}", stderr)))
                }
            }
            Err(e) => Err(ServiceError::IoError(format!("Failed to run systemctl: {}", e))),
        }
    }
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mgr = ServiceManager::new();
        // Just verify it was created
        let _ = mgr.is_systemd_available();
        let _ = mgr.is_root();
    }

    #[test]
    fn test_service_status_parse() {
        assert_eq!(ServiceStatus::parse("active"), ServiceStatus::Active);
        assert_eq!(ServiceStatus::parse("inactive"), ServiceStatus::Inactive);
        assert_eq!(ServiceStatus::parse("failed"), ServiceStatus::Failed);
        assert_eq!(ServiceStatus::parse("unknown"), ServiceStatus::Unknown);
    }

    #[test]
    fn test_list_services() {
        let mut mgr = ServiceManager::new();
        // May fail if systemd not available
        if let Ok(services) = mgr.list_services() {
            let _ = services;
        }
    }

    #[test]
    fn test_get_service_status() {
        let mut mgr = ServiceManager::new();
        // Try to get a common service
        if let Ok(status) = mgr.get_service_status("dbus") {
            // May or may not exist depending on system
            let _ = status.as_str();
        }
    }
}
