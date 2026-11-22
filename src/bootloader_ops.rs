//! SigmaOS bootloader operations module.
//!
//! This module provides bootloader management operations including:
//! - GRUB configuration management
//! - Boot entry management
//! - Boot parameter configuration
//! - UEFI boot entry management
//!
//! Follows Single Responsibility Principle: only manages bootloader operations.
//! Follows Interface Segregation Principle: minimal, focused interfaces.
//!
//! # Example
//!
//! ```
//! use sigma_system::bootloader_ops::*;
//!
//! let bootloader = BootloaderOps::new();
//! if let Ok(entries) = bootloader.list_boot_entries() {
//!     println!("Boot entries: {}", entries);
//! }
//! ```

use std::process::Command;

/// Bootloader operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum BootloaderError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
    /// Invalid entry.
    InvalidEntry(String),
}

impl std::fmt::Display for BootloaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootloaderError::CommandError(msg) => write!(f, "Command error: {}", msg),
            BootloaderError::IoError(msg) => write!(f, "IO error: {}", msg),
            BootloaderError::InvalidEntry(msg) => write!(f, "Invalid entry: {}", msg),
        }
    }
}

impl std::error::Error for BootloaderError {}

/// Bootloader operations manager.
#[derive(Debug)]
pub struct BootloaderOps;

impl BootloaderOps {
    pub fn new() -> Self { Self }

    /// List boot entries.
    pub fn list_boot_entries(&self) -> Result<String, BootloaderError> {
        let output = Command::new("efibootmgr")
            .output()
            .map_err(|e| BootloaderError::CommandError(format!("Failed to run efibootmgr: {}", e)))?;

        let entries = String::from_utf8_lossy(&output.stdout);
        Ok(entries.trim().to_string())
    }

    /// Generate GRUB configuration.
    pub fn generate_grub_config(&self) -> Result<(), BootloaderError> {
        let output = Command::new("grub-mkconfig")
            .arg("-o")
            .arg("/boot/grub/grub.cfg")
            .output()
            .map_err(|e| BootloaderError::CommandError(format!("Failed to run grub-mkconfig: {}", e)))?;

        if !output.status.success() {
            return Err(BootloaderError::CommandError("grub-mkconfig failed".to_string()));
        }

        Ok(())
    }

    /// Install GRUB.
    pub fn install_grub(&self, device: &str) -> Result<(), BootloaderError> {
        let output = Command::new("grub-install")
            .args(&["--target=x86_64-efi", "--efi-directory=/boot", "--bootloader-id=SigmaOS", "--recheck", device])
            .output()
            .map_err(|e| BootloaderError::CommandError(format!("Failed to run grub-install: {}", e)))?;

        if !output.status.success() {
            return Err(BootloaderError::CommandError("grub-install failed".to_string()));
        }

        Ok(())
    }
}

impl Default for BootloaderOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bootloader = BootloaderOps::new();
        let _ = bootloader;
    }

    #[test]
    fn test_list_boot_entries() {
        let bootloader = BootloaderOps::new();
        if let Ok(entries) = bootloader.list_boot_entries() {
            // Should not panic
            let _ = entries;
        }
    }
}
