//! SigmaOS kernel module management.
//!
//! This module provides actual kernel module management using modprobe/lsmod.
//!
//! # Example
//!
//! ```
//! use sigma_system::kernel::*;
//!
//! let mut kernel_mgr = KernelManager::new();
//! if kernel_mgr.is_root() {
//!     kernel_mgr.load_module("nf_tables").unwrap();
//!     kernel_mgr.list_loaded_modules().unwrap();
//! }
//! ```

use std::process::Command;
use std::collections::BTreeMap;

/// Errors that can occur in kernel operations.
#[derive(Debug, Clone, PartialEq)]
pub enum KernelError {
    /// Module not found.
    ModuleNotFound(String),
    /// Module already loaded.
    ModuleLoaded(String),
    /// Module not loaded.
    ModuleNotLoaded(String),
    /// Permission denied.
    PermissionDenied(String),
    /// Kernel not available.
    KernelNotAvailable(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KernelError::ModuleNotFound(name) => write!(f, "Module '{}' not found", name),
            KernelError::ModuleLoaded(name) => write!(f, "Module '{}' already loaded", name),
            KernelError::ModuleNotLoaded(name) => write!(f, "Module '{}' not loaded", name),
            KernelError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            KernelError::KernelNotAvailable(msg) => write!(f, "Kernel not available: {}", msg),
            KernelError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for KernelError {}

/// Kernel module information.
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    /// Module name.
    pub name: String,
    /// Module size in bytes.
    pub size: u64,
    /// Module dependencies.
    pub dependencies: Vec<String>,
    /// Module state.
    pub state: ModuleState,
    /// Module address.
    pub address: Option<String>,
}

/// Module state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState {
    /// Module is loaded.
    Loaded,
    /// Module is not loaded.
    NotLoaded,
    /// Module is blacklisted.
    Blacklisted,
}

impl ModuleState {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleState::Loaded => "loaded",
            ModuleState::NotLoaded => "not_loaded",
            ModuleState::Blacklisted => "blacklisted",
        }
    }
}

/// Kernel information.
#[derive(Debug, Clone)]
pub struct KernelInfo {
    /// Kernel version.
    pub version: String,
    /// Kernel release.
    pub release: String,
    /// Kernel architecture.
    pub architecture: String,
    /// Kernel command line.
    pub cmdline: String,
    /// Kernel modules count.
    pub modules_count: usize,
}

/// Kernel manager.
#[derive(Debug)]
pub struct KernelManager {
    /// Loaded modules cache.
    loaded_modules: BTreeMap<String, ModuleInfo>,
    /// Last update time.
    last_update: std::time::Instant,
    /// Whether running as root.
    is_root: bool,
}

impl KernelManager {
    /// Create a new kernel manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            loaded_modules: BTreeMap::new(),
            last_update: std::time::Instant::now(),
            is_root,
        }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Get kernel information.
    pub fn get_kernel_info(&mut self) -> Result<KernelInfo, KernelError> {
        // Read /proc/version
        let version_content = std::fs::read_to_string("/proc/version")
            .map_err(|e| KernelError::IoError(format!("Failed to read /proc/version: {}", e)))?;

        // Read /proc/sys/kernel/osrelease
        let release = std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .map_err(|e| KernelError::IoError(format!("Failed to read osrelease: {}", e)))?
            .trim()
            .to_string();

        // Read /proc/cmdline
        let cmdline = std::fs::read_to_string("/proc/cmdline")
            .map_err(|e| KernelError::IoError(format!("Failed to read cmdline: {}", e)))?
            .trim()
            .to_string();

        // Get architecture from uname
        let arch = Command::new("uname").arg("-m").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        let modules_count = self.list_loaded_modules().unwrap_or_default().len();

        Ok(KernelInfo {
            version: version_content.trim().to_string(),
            release,
            architecture: arch,
            cmdline,
            modules_count,
        })
    }

    /// List loaded modules.
    pub fn list_loaded_modules(&mut self) -> Result<Vec<&ModuleInfo>, KernelError> {
        self.refresh_modules()?;
        Ok(self.loaded_modules.values().collect())
    }

    /// Get a specific module.
    pub fn get_module(&mut self, name: &str) -> Result<Option<&ModuleInfo>, KernelError> {
        self.refresh_modules()?;
        Ok(self.loaded_modules.get(name))
    }

    /// Check if module is loaded.
    pub fn is_module_loaded(&mut self, name: &str) -> Result<bool, KernelError> {
        self.refresh_modules()?;
        Ok(self.loaded_modules.contains_key(name))
    }

    /// Load a module.
    pub fn load_module(&mut self, name: &str) -> Result<(), KernelError> {
        if !self.is_root {
            return Err(KernelError::PermissionDenied("Must be root to load modules".to_string()));
        }

        if self.is_module_loaded(name)? {
            return Err(KernelError::ModuleLoaded(name.to_string()));
        }

        let result = Command::new("modprobe").arg(name).output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_modules()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(KernelError::IoError(format!("Failed to load module: {}", stderr)))
                }
            }
            Err(e) => Err(KernelError::IoError(format!("Failed to run modprobe: {}", e))),
        }
    }

    /// Unload a module.
    pub fn unload_module(&mut self, name: &str) -> Result<(), KernelError> {
        if !self.is_root {
            return Err(KernelError::PermissionDenied("Must be root to unload modules".to_string()));
        }

        if !self.is_module_loaded(name)? {
            return Err(KernelError::ModuleNotLoaded(name.to_string()));
        }

        let result = Command::new("modprobe").args(&["-r", name]).output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.loaded_modules.remove(name);
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(KernelError::IoError(format!("Failed to unload module: {}", stderr)))
                }
            }
            Err(e) => Err(KernelError::IoError(format!("Failed to run modprobe: {}", e))),
        }
    }

    /// Refresh module list from lsmod.
    fn refresh_modules(&mut self) -> Result<(), KernelError> {
        // Only refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() < 1 {
            return Ok(());
        }

        self.loaded_modules.clear();

        let result = Command::new("lsmod").output();

        if let Ok(output) = result {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines().skip(1) { // Skip header
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let name = parts[0].to_string();
                        let size = parts[1].parse().unwrap_or(0);
                        let dependencies: Vec<String> = parts[3].split(',')
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect();

                        self.loaded_modules.insert(name.clone(), ModuleInfo {
                            name,
                            size,
                            dependencies,
                            state: ModuleState::Loaded,
                            address: None,
                        });
                    }
                }
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Get module parameters.
    pub fn get_module_params(&self, name: &str) -> Result<Vec<String>, KernelError> {
        let result = Command::new("modinfo").arg(name).output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let params: Vec<String> = stdout.lines()
                        .filter(|line| line.starts_with("parm:"))
                        .map(|line| line.trim_start_matches("parm:").to_string())
                        .collect();
                    Ok(params)
                } else {
                    Err(KernelError::ModuleNotFound(name.to_string()))
                }
            }
            Err(e) => Err(KernelError::IoError(format!("Failed to run modinfo: {}", e))),
        }
    }

    /// Blacklist a module.
    pub fn blacklist_module(&self, name: &str) -> Result<(), KernelError> {
        if !self.is_root {
            return Err(KernelError::PermissionDenied("Must be root to blacklist modules".to_string()));
        }

        // Write to /etc/modprobe.d/blacklist.conf
        let blacklist_path = "/etc/modprobe.d/sigmaos-blacklist.conf";
        let content = format!("blacklist {}\ninstall {} /bin/false\n", name, name);

        std::fs::write(blacklist_path, content)
            .map_err(|e| KernelError::IoError(format!("Failed to write blacklist: {}", e)))
    }

    /// Get kernel command line.
    pub fn get_cmdline(&self) -> Result<String, KernelError> {
        std::fs::read_to_string("/proc/cmdline")
            .map(|s| s.trim().to_string())
            .map_err(|e| KernelError::IoError(format!("Failed to read cmdline: {}", e)))
    }

    /// Get kernel version.
    pub fn get_version(&self) -> Result<String, KernelError> {
        std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .map(|s| s.trim().to_string())
            .map_err(|e| KernelError::IoError(format!("Failed to read version: {}", e)))
    }
}

impl Default for KernelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mgr = KernelManager::new();
        let _ = mgr.is_root();
    }

    #[test]
    fn test_get_kernel_info() {
        let mut mgr = KernelManager::new();
        if let Ok(info) = mgr.get_kernel_info() {
            assert!(!info.version.is_empty());
            assert!(!info.release.is_empty());
        }
    }

    #[test]
    fn test_list_loaded_modules() {
        let mut mgr = KernelManager::new();
        if let Ok(modules) = mgr.list_loaded_modules() {
            // May be empty in restricted environments
            let _ = modules;
        }
    }

    #[test]
    fn test_module_state() {
        assert_eq!(ModuleState::Loaded.as_str(), "loaded");
        assert_eq!(ModuleState::NotLoaded.as_str(), "not_loaded");
    }
}
