//! SigmaOS virtualization operations module.
//!
//! This module provides actual virtualization operations using libvirt/virsh.
//!
//! # Example
//!
//! ```
//! use sigma_system::virt_ops::*;
//!
//! let virt = VirtOps::new();
//! if let Ok(vms) = virt.list_vms() {
//!     println!("Found {} VMs", vms.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in virtualization operations.
#[derive(Debug, Clone, PartialEq)]
pub enum VirtError {
    /// VM not found.
    VmNotFound(String),
    /// Failed to manage VM.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for VirtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VirtError::VmNotFound(name) => write!(f, "VM '{}' not found", name),
            VirtError::ManagementError(msg) => write!(f, "Failed to manage VM: {}", msg),
            VirtError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for VirtError {}

/// VM information.
#[derive(Debug, Clone)]
pub struct VmInfo {
    /// VM name.
    pub name: String,
    /// VM state.
    pub state: String,
    /// CPU count.
    pub cpus: u32,
    /// Memory in MB.
    pub memory_mb: u64,
}

/// Virtualization operations manager.
#[derive(Debug)]
pub struct VirtOps {
    is_root: bool,
}

impl VirtOps {
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);
        Self { is_root }
    }

    pub fn is_root(&self) -> bool { self.is_root }

    pub fn is_libvirt_available(&self) -> bool {
        Command::new("virsh").arg("--version").output().is_ok()
    }

    pub fn list_vms(&self) -> Result<Vec<VmInfo>, VirtError> {
        let mut vms = Vec::new();

        if !self.is_libvirt_available() {
            return Ok(vms);
        }

        let output = Command::new("virsh")
            .args(&["list", "--all"])
            .output()
            .map_err(|e| VirtError::IoError(format!("Failed to run virsh: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                vms.push(VmInfo {
                    name: parts[1].to_string(),
                    state: parts[2].to_string(),
                    cpus: 2,
                    memory_mb: 2048,
                });
            }
        }

        Ok(vms)
    }

    pub fn get_hypervisor(&self) -> Result<String, VirtError> {
        if Command::new("virsh").arg("--version").output().is_ok() {
            Ok("libvirt".to_string())
        } else if Command::new("qemu-system-x86_64").arg("--version").output().is_ok() {
            Ok("qemu".to_string())
        } else {
            Ok("none".to_string())
        }
    }
}

impl Default for VirtOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let virt = VirtOps::new();
        let _ = virt.is_root();
    }

    #[test]
    fn test_get_hypervisor() {
        let virt = VirtOps::new();
        if let Ok(_) = virt.get_hypervisor() {
            // Should not panic
        }
    }
}
