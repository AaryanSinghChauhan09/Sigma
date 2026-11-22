//! SigmaOS hostname operations module.
//!
//! This module provides actual hostname operations using hostnamectl and /etc/hostname.
//!
//! # Example
//!
//! ```
//! use sigma_system::hostname_ops::*;
//!
//! let mut host_ops = HostnameOps::new();
//! if let Ok(hostname) = host_ops.get_hostname() {
//!     println!("Current hostname: {}", hostname);
//! }
//! ```

use std::fs;
use std::process::Command;

/// Errors that can occur in hostname operations.
#[derive(Debug, Clone, PartialEq)]
pub enum HostnameError {
    /// Failed to get hostname.
    GetError(String),
    /// Failed to set hostname.
    SetError(String),
    /// Invalid hostname.
    InvalidHostname(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for HostnameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostnameError::GetError(msg) => write!(f, "Failed to get hostname: {}", msg),
            HostnameError::SetError(msg) => write!(f, "Failed to set hostname: {}", msg),
            HostnameError::InvalidHostname(name) => write!(f, "Invalid hostname: {}", name),
            HostnameError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            HostnameError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for HostnameError {}

/// Hostname operations manager.
#[derive(Debug)]
pub struct HostnameOps {
    /// Hostname file path.
    hostname_path: String,
    /// Whether running as root.
    is_root: bool,
}

impl HostnameOps {
    /// Create a new hostname operations manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            hostname_path: "/etc/hostname".to_string(),
            is_root,
        }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Get hostname.
    pub fn get_hostname(&self) -> Result<String, HostnameError> {
        // Try to read from /etc/hostname
        if let Ok(content) = fs::read_to_string(&self.hostname_path) {
            let hostname = content.trim().to_string();
            if !hostname.is_empty() {
                return Ok(hostname);
            }
        }

        // Try hostname command
        if let Ok(output) = Command::new("hostname").output() {
            let hostname = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !hostname.is_empty() {
                return Ok(hostname);
            }
        }

        // Try hostnamectl
        if let Ok(output) = Command::new("hostnamectl").arg("status").output() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("Static hostname") {
                    if let Some(hostname) = line.split(':').nth(1) {
                        let hostname = hostname.trim().to_string();
                        if !hostname.is_empty() && hostname != "n/a" {
                            return Ok(hostname);
                        }
                    }
                }
            }
        }

        Err(HostnameError::GetError("Could not determine hostname".to_string()))
    }

    /// Set hostname.
    pub fn set_hostname(&self, hostname: &str) -> Result<(), HostnameError> {
        if !self.is_root {
            return Err(HostnameError::PermissionDenied("Must be root to set hostname".to_string()));
        }

        // Validate hostname
        self.validate_hostname(hostname)?;

        // Write to /etc/hostname
        fs::write(&self.hostname_path, hostname)
            .map_err(|e| HostnameError::IoError(format!("Failed to write hostname: {}", e)))?;

        // Try to set using hostnamectl if available
        let _ = Command::new("hostnamectl").arg("set-hostname").arg(hostname).status();

        Ok(())
    }

    /// Validate hostname.
    fn validate_hostname(&self, hostname: &str) -> Result<(), HostnameError> {
        if hostname.is_empty() {
            return Err(HostnameError::InvalidHostname("Hostname cannot be empty".to_string()));
        }

        if hostname.len() > 253 {
            return Err(HostnameError::InvalidHostname("Hostname too long (max 253 characters)".to_string()));
        }

        // Hostname can contain alphanumeric, hyphens, and dots
        for c in hostname.chars() {
            if !c.is_alphanumeric() && c != '-' && c != '.' {
                return Err(HostnameError::InvalidHostname(format!("Invalid character '{}' in hostname", c)));
            }
        }

        // Cannot start or end with hyphen or dot
        if hostname.starts_with('-') || hostname.starts_with('.') || hostname.ends_with('-') || hostname.ends_with('.') {
            return Err(HostnameError::InvalidHostname("Hostname cannot start or end with hyphen or dot".to_string()));
        }

        Ok(())
    }

    /// Get pretty hostname (from hostnamectl).
    pub fn get_pretty_hostname(&self) -> Result<String, HostnameError> {
        let output = Command::new("hostnamectl").arg("status").output()
            .map_err(|e| HostnameError::IoError(format!("Failed to get hostnamectl status: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines() {
            if line.contains("Pretty hostname") {
                if let Some(hostname) = line.split(':').nth(1) {
                    let hostname = hostname.trim().to_string();
                    if !hostname.is_empty() && hostname != "n/a" {
                        return Ok(hostname);
                    }
                }
            }
        }

        Ok(String::new())
    }

    /// Set pretty hostname.
    pub fn set_pretty_hostname(&self, hostname: &str) -> Result<(), HostnameError> {
        if !self.is_root {
            return Err(HostnameError::PermissionDenied("Must be root to set pretty hostname".to_string()));
        }

        let result = Command::new("hostnamectl")
            .arg("set-hostname")
            .arg("--pretty")
            .arg(hostname)
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(HostnameError::SetError("hostnamectl command failed".to_string())),
            Err(e) => Err(HostnameError::IoError(format!("Failed to run hostnamectl: {}", e))),
        }
    }

    /// Get chassis type.
    pub fn get_chassis(&self) -> Result<String, HostnameError> {
        let output = Command::new("hostnamectl").arg("status").output()
            .map_err(|e| HostnameError::IoError(format!("Failed to get hostnamectl status: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines() {
            if line.contains("Chassis") {
                if let Some(chassis) = line.split(':').nth(1) {
                    let chassis = chassis.trim().to_string();
                    if !chassis.is_empty() && chassis != "n/a" {
                        return Ok(chassis);
                    }
                }
            }
        }

        Ok("unknown".to_string())
    }

    /// Get hardware vendor.
    pub fn get_hardware_vendor(&self) -> Result<String, HostnameError> {
        let output = Command::new("hostnamectl").arg("status").output()
            .map_err(|e| HostnameError::IoError(format!("Failed to get hostnamectl status: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines() {
            if line.contains("Hardware Vendor") {
                if let Some(vendor) = line.split(':').nth(1) {
                    let vendor = vendor.trim().to_string();
                    if !vendor.is_empty() && vendor != "n/a" {
                        return Ok(vendor);
                    }
                }
            }
        }

        Ok("unknown".to_string())
    }

    /// Get operating system name.
    pub fn get_os_name(&self) -> Result<String, HostnameError> {
        let output = Command::new("hostnamectl").arg("status").output()
            .map_err(|e| HostnameError::IoError(format!("Failed to get hostnamectl status: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines() {
            if line.contains("Operating System") {
                if let Some(os) = line.split(':').nth(1) {
                    let os = os.trim().to_string();
                    if !os.is_empty() && os != "n/a" {
                        return Ok(os);
                    }
                }
            }
        }

        Ok("unknown".to_string())
    }

    /// Get kernel version.
    pub fn get_kernel_version(&self) -> Result<String, HostnameError> {
        let output = Command::new("hostnamectl").arg("status").output()
            .map_err(|e| HostnameError::IoError(format!("Failed to get hostnamectl status: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines() {
            if line.contains("Kernel") {
                if let Some(kernel) = line.split(':').nth(1) {
                    let kernel = kernel.trim().to_string();
                    if !kernel.is_empty() && kernel != "n/a" {
                        return Ok(kernel);
                    }
                }
            }
        }

        Ok("unknown".to_string())
    }

    /// Get system summary.
    pub fn get_system_summary(&self) -> Result<String, HostnameError> {
        let hostname = self.get_hostname().unwrap_or_else(|_| "unknown".to_string());
        let pretty_hostname = self.get_pretty_hostname().unwrap_or_else(|_| "".to_string());
        let chassis = self.get_chassis().unwrap_or_else(|_| "unknown".to_string());
        let os = self.get_os_name().unwrap_or_else(|_| "unknown".to_string());
        let kernel = self.get_kernel_version().unwrap_or_else(|_| "unknown".to_string());

        let mut summary = format!("Hostname: {}", hostname);
        if !pretty_hostname.is_empty() {
            summary.push_str(&format!(" ({})", pretty_hostname));
        }
        summary.push_str(&format!("\nChassis: {}", chassis));
        summary.push_str(&format!("\nOS: {}", os));
        summary.push_str(&format!("\nKernel: {}", kernel));

        Ok(summary)
    }
}

impl Default for HostnameOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let host_ops = HostnameOps::new();
        let _ = host_ops.is_root();
    }

    #[test]
    fn test_get_hostname() {
        let host_ops = HostnameOps::new();
        if let Ok(hostname) = host_ops.get_hostname() {
            assert!(!hostname.is_empty());
        }
    }

    #[test]
    fn test_validate_hostname() {
        let host_ops = HostnameOps::new();
        assert!(host_ops.validate_hostname("valid-hostname").is_ok());
        assert!(host_ops.validate_hostname("invalid_hostname").is_err());
        assert!(host_ops.validate_hostname("").is_err());
    }

    #[test]
    fn test_get_os_name() {
        let host_ops = HostnameOps::new();
        if let Ok(os) = host_ops.get_os_name() {
            assert!(!os.is_empty());
        }
    }
}
