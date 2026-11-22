//! SigmaOS update management module.
//!
//! This module provides abstractions for system updates, package updates,
//! and rolling release management.
//!
//! # Example
//!
//! ```
//! use sigma_system::update::*;
//!
//! let mut update_mgr = UpdateManager::new();
//! update_mgr.check_updates().unwrap();
//! update_mgr.apply_updates(&["linux"]).unwrap();
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in update operations.
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateError {
    /// Update check failed.
    CheckFailed(String),
    /// Update failed.
    UpdateFailed(String),
    /// Package not found.
    PackageNotFound(String),
    /// Dependency conflict.
    DependencyConflict(String),
    /// Insufficient space.
    InsufficientSpace(String),
    /// Network error.
    NetworkError(String),
}

impl std::fmt::Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateError::CheckFailed(msg) => write!(f, "Update check failed: {}", msg),
            UpdateError::UpdateFailed(msg) => write!(f, "Update failed: {}", msg),
            UpdateError::PackageNotFound(pkg) => write!(f, "Package '{}' not found", pkg),
            UpdateError::DependencyConflict(msg) => write!(f, "Dependency conflict: {}", msg),
            UpdateError::InsufficientSpace(msg) => write!(f, "Insufficient space: {}", msg),
            UpdateError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for UpdateError {}

/// Update type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdateType {
    /// System update.
    System,
    /// Package update.
    Package,
    /// Kernel update.
    Kernel,
    /// Firmware update.
    Firmware,
    /// Security update.
    Security,
}

impl UpdateType {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            UpdateType::System => "system",
            UpdateType::Package => "package",
            UpdateType::Kernel => "kernel",
            UpdateType::Firmware => "firmware",
            UpdateType::Security => "security",
        }
    }
}

/// Update status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStatus {
    /// Update is available.
    Available,
    /// Update is in progress.
    InProgress,
    /// Update completed.
    Completed,
    /// Update failed.
    Failed,
    /// Update is not needed.
    NotNeeded,
}

/// Update information.
#[derive(Debug, Clone)]
pub struct Update {
    /// Update name.
    pub name: String,
    /// Update type.
    pub update_type: UpdateType,
    /// Current version.
    pub current_version: String,
    /// New version.
    pub new_version: String,
    /// Update status.
    pub status: UpdateStatus,
    /// Update size in bytes.
    pub size: u64,
    /// Whether update requires reboot.
    pub requires_reboot: bool,
    /// Update description.
    pub description: String,
}

impl Update {
    /// Create a new update.
    pub fn new(name: &str, update_type: UpdateType, current: &str, new: &str) -> Self {
        Self {
            name: name.to_string(),
            update_type,
            current_version: current.to_string(),
            new_version: new.to_string(),
            status: UpdateStatus::Available,
            size: 0,
            requires_reboot: false,
            description: String::new(),
        }
    }

    /// Mark update as in progress.
    pub fn start(&mut self) {
        self.status = UpdateStatus::InProgress;
    }

    /// Mark update as completed.
    pub fn complete(&mut self) {
        self.status = UpdateStatus::Completed;
        self.current_version = self.new_version.clone();
    }

    /// Mark update as failed.
    pub fn fail(&mut self) {
        self.status = UpdateStatus::Failed;
    }
}

/// Update manager.
#[derive(Debug)]
pub struct UpdateManager {
    /// Updates by name.
    updates: BTreeMap<String, Update>,
    /// Last update check timestamp.
    last_check: Option<i64>,
    /// Whether updates are enabled.
    updates_enabled: bool,
    /// Automatic update check interval (seconds).
    auto_check_interval: u64,
}

impl UpdateManager {
    /// Create a new update manager.
    pub fn new() -> Self {
        Self {
            updates: BTreeMap::new(),
            last_check: None,
            updates_enabled: true,
            auto_check_interval: 86400, // 24 hours
        }
    }

    /// Check for updates.
    pub fn check_updates(&mut self) -> Result<Vec<&Update>, UpdateError> {
        // Simulate update check
        self.last_check = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64);
        
        // Add some example updates
        if self.updates.is_empty() {
            self.add_example_updates();
        }
        
        Ok(self.updates.values().collect())
    }

    /// Add example updates (simulated).
    fn add_example_updates(&mut self) {
        let examples = [
            ("linux", UpdateType::Kernel, "6.1.0", "6.1.1", true),
            ("systemd", UpdateType::System, "252", "253", false),
            ("networkmanager", UpdateType::Package, "1.40.0", "1.40.1", false),
            ("mesa", UpdateType::Package, "23.1.0", "23.1.1", false),
            ("firmware", UpdateType::Firmware, "20230610", "20230615", false),
        ];
        
        for (name, update_type, current, new, requires_reboot) in examples {
            let mut update = Update::new(name, update_type, current, new);
            update.requires_reboot = requires_reboot;
            update.size = 1024 * 1024; // 1MB
            self.updates.insert(name.to_string(), update);
        }
    }

    /// Apply updates.
    pub fn apply_updates(&mut self, update_names: &[&str]) -> Result<(), UpdateError> {
        for name in update_names {
            if let Some(update) = self.updates.get_mut(*name) {
                update.start();
                // Simulate update process
                update.complete();
            } else {
                return Err(UpdateError::PackageNotFound(name.to_string()));
            }
        }
        Ok(())
    }

    /// Apply all available updates.
    pub fn apply_all_updates(&mut self) -> Result<(), UpdateError> {
        let names: Vec<String> = self.updates
            .values()
            .filter(|u| u.status == UpdateStatus::Available)
            .map(|u| u.name.clone())
            .collect();
        
        let name_refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
        self.apply_updates(&name_refs)
    }

    /// Get an update.
    pub fn get_update(&self, name: &str) -> Option<&Update> {
        self.updates.get(name)
    }

    /// Get all updates.
    pub fn get_updates(&self) -> Vec<&Update> {
        self.updates.values().collect()
    }

    /// Get available updates.
    pub fn get_available_updates(&self) -> Vec<&Update> {
        self.updates.values()
            .filter(|u| u.status == UpdateStatus::Available)
            .collect()
    }

    /// Get updates requiring reboot.
    pub fn get_updates_requiring_reboot(&self) -> Vec<&Update> {
        self.updates.values()
            .filter(|u| u.requires_reboot && u.status == UpdateStatus::Completed)
            .collect()
    }

    /// Enable/disable updates.
    pub fn set_updates_enabled(&mut self, enabled: bool) {
        self.updates_enabled = enabled;
    }

    /// Check if updates are enabled.
    pub fn is_updates_enabled(&self) -> bool {
        self.updates_enabled
    }

    /// Set auto-check interval.
    pub fn set_auto_check_interval(&mut self, seconds: u64) {
        self.auto_check_interval = seconds;
    }

    /// Get auto-check interval.
    pub fn get_auto_check_interval(&self) -> u64 {
        self.auto_check_interval
    }

    /// Generate update script.
    pub fn generate_update_script(&self) -> String {
        let mut script = String::new();
        script.push_str("#!/bin/bash\n");
        script.push_str("# SigmaOS Update Script\n\n");
        script.push_str("set -e\n\n");
        
        script.push_str("# Check for updates\n");
        script.push_str("sigmaos-update check\n\n");
        
        script.push_str("# Apply updates\n");
        for update in self.updates.values() {
            if update.status == UpdateStatus::Available {
                script.push_str(&format!("sigmaos-update apply {}\n", update.name));
            }
        }
        
        script.push_str("\n# Cleanup\n");
        script.push_str("sigmaos-update cleanup\n");
        
        script
    }

    /// Generate systemd update service.
    pub fn generate_systemd_service(&self) -> String {
        let mut service = String::new();
        service.push_str("[Unit]\n");
        service.push_str("Description=SigmaOS Update Service\n");
        service.push_str("After=network.target\n\n");
        
        service.push_str("[Service]\n");
        service.push_str("Type=oneshot\n");
        service.push_str("ExecStart=/usr/bin/sigmaos-update check\n");
        service.push_str("ExecStart=/usr/bin/sigmaos-update apply\n");
        service.push_str("ExecStart=/usr/bin/sigmaos-update cleanup\n\n");
        
        service.push_str("[Install]\n");
        service.push_str("WantedBy=multi-user.target\n");
        
        service
    }

    /// Add default update checks.
    pub fn add_default_checks(&mut self) {
        // This would add default repositories and checks
        // For now, it's a placeholder
    }
}

impl Default for UpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_updates() {
        let mut mgr = UpdateManager::new();
        let updates = mgr.check_updates().unwrap();
        assert!(!updates.is_empty());
    }

    #[test]
    fn test_apply_updates() {
        let mut mgr = UpdateManager::new();
        mgr.check_updates().unwrap();
        mgr.apply_updates(&["linux"]).unwrap();
        let update = mgr.get_update("linux").unwrap();
        assert_eq!(update.status, UpdateStatus::Completed);
    }

    #[test]
    fn test_apply_all_updates() {
        let mut mgr = UpdateManager::new();
        mgr.check_updates().unwrap();
        mgr.apply_all_updates().unwrap();
        let available = mgr.get_available_updates();
        assert!(available.is_empty());
    }

    #[test]
    fn test_get_available_updates() {
        let mut mgr = UpdateManager::new();
        mgr.check_updates().unwrap();
        let available = mgr.get_available_updates();
        assert!(!available.is_empty());
    }

    #[test]
    fn test_updates_requiring_reboot() {
        let mut mgr = UpdateManager::new();
        mgr.check_updates().unwrap();
        mgr.apply_updates(&["linux"]).unwrap();
        let reboot_updates = mgr.get_updates_requiring_reboot();
        assert!(!reboot_updates.is_empty());
    }

    #[test]
    fn test_generate_update_script() {
        let mut mgr = UpdateManager::new();
        mgr.check_updates().unwrap();
        let script = mgr.generate_update_script();
        assert!(script.contains("sigmaos-update"));
        assert!(script.contains("check"));
    }

    #[test]
    fn test_update_type_as_str() {
        assert_eq!(UpdateType::System.as_str(), "system");
        assert_eq!(UpdateType::Kernel.as_str(), "kernel");
    }

    #[test]
    fn test_update_status() {
        let mut update = Update::new("test", UpdateType::System, "1.0", "2.0");
        assert_eq!(update.status, UpdateStatus::Available);
        update.start();
        assert_eq!(update.status, UpdateStatus::InProgress);
        update.complete();
        assert_eq!(update.status, UpdateStatus::Completed);
        assert_eq!(update.current_version, "2.0");
    }
}
