//! SigmaOS kernel operations module.
//!
//! This module provides kernel management operations including:
//! - Kernel parameter management via sysctl
//! - Kernel module management via modprobe
//! - Kernel version information
//! - Kernel boot parameter configuration
//!
//! Follows Single Responsibility Principle: only manages kernel operations.
//! Follows Interface Segregation Principle: minimal, focused interfaces.
//! Follows Dependency Inversion Principle: depends on abstraction (kernel interface).
//!
//! # Example
//!
//! ```
//! use sigma_system::kernel_ops::*;
//!
//! let kernel = KernelOps::new();
//! if let Ok(version) = kernel.get_version() {
//!     println!("Kernel version: {}", version);
//! }
//! ```

use std::process::Command;

/// Kernel operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum KernelError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
    /// Invalid parameter.
    InvalidParameter(String),
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KernelError::CommandError(msg) => write!(f, "Command error: {}", msg),
            KernelError::IoError(msg) => write!(f, "IO error: {}", msg),
            KernelError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

impl std::error::Error for KernelError {}

/// Kernel operations manager.
#[derive(Debug)]
pub struct KernelOps;

impl KernelOps {
    pub fn new() -> Self { Self }

    /// Get kernel version.
    pub fn get_version(&self) -> Result<String, KernelError> {
        let output = Command::new("uname")
            .arg("-r")
            .output()
            .map_err(|e| KernelError::CommandError(format!("Failed to run uname: {}", e)))?;

        let version = String::from_utf8_lossy(&output.stdout);
        Ok(version.trim().to_string())
    }

    /// Get kernel release information.
    pub fn get_release(&self) -> Result<String, KernelError> {
        let output = Command::new("uname")
            .arg("-a")
            .output()
            .map_err(|e| KernelError::CommandError(format!("Failed to run uname: {}", e)))?;

        let release = String::from_utf8_lossy(&output.stdout);
        Ok(release.trim().to_string())
    }

    /// Get kernel parameter value.
    pub fn get_parameter(&self, param: &str) -> Result<String, KernelError> {
        let output = Command::new("sysctl")
            .arg("-n")
            .arg(param)
            .output()
            .map_err(|e| KernelError::CommandError(format!("Failed to run sysctl: {}", e)))?;

        let value = String::from_utf8_lossy(&output.stdout);
        Ok(value.trim().to_string())
    }

    /// Set kernel parameter value.
    pub fn set_parameter(&self, param: &str, value: &str) -> Result<(), KernelError> {
        let output = Command::new("sysctl")
            .args(&["-w", &format!("{}={}", param, value)])
            .output()
            .map_err(|e| KernelError::CommandError(format!("Failed to run sysctl: {}", e)))?;

        if !output.status.success() {
            return Err(KernelError::CommandError("sysctl failed".to_string()));
        }

        Ok(())
    }

    /// Load kernel module.
    pub fn load_module(&self, module: &str) -> Result<(), KernelError> {
        let output = Command::new("modprobe")
            .arg(module)
            .output()
            .map_err(|e| KernelError::CommandError(format!("Failed to run modprobe: {}", e)))?;

        if !output.status.success() {
            return Err(KernelError::CommandError("modprobe failed".to_string()));
        }

        Ok(())
    }

    /// Unload kernel module.
    pub fn unload_module(&self, module: &str) -> Result<(), KernelError> {
        let output = Command::new("modprobe")
            .args(&["-r", module])
            .output()
            .map_err(|e| KernelError::CommandError(format!("Failed to run modprobe: {}", e)))?;

        if !output.status.success() {
            return Err(KernelError::CommandError("modprobe failed".to_string()));
        }

        Ok(())
    }

    /// List loaded kernel modules.
    pub fn list_modules(&self) -> Result<String, KernelError> {
        let output = Command::new("lsmod")
            .output()
            .map_err(|e| KernelError::CommandError(format!("Failed to run lsmod: {}", e)))?;

        let modules = String::from_utf8_lossy(&output.stdout);
        Ok(modules.trim().to_string())
    }
}

impl Default for KernelOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let kernel = KernelOps::new();
        let _ = kernel;
    }

    #[test]
    fn test_get_version() {
        let kernel = KernelOps::new();
        if let Ok(version) = kernel.get_version() {
            // Should not panic
            let _ = version;
        }
    }

    #[test]
    fn test_get_release() {
        let kernel = KernelOps::new();
        if let Ok(release) = kernel.get_release() {
            // Should not panic
            let _ = release;
        }
    }

    #[test]
    fn test_list_modules() {
        let kernel = KernelOps::new();
        if let Ok(modules) = kernel.list_modules() {
            // Should not panic
            let _ = modules;
        }
    }
}
