//! SigmaOS container operations module.
//!
//! This module provides actual container operations using Podman/Docker.
//!
//! # Example
//!
//! ```
//! use sigma_system::container_ops::*;
//!
//! let container = ContainerOps::new();
//! if let Ok(containers) = container.list_containers() {
//!     println!("Found {} containers", containers.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in container operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ContainerError {
    /// Container not found.
    ContainerNotFound(String),
    /// Failed to manage container.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerError::ContainerNotFound(name) => write!(f, "Container '{}' not found", name),
            ContainerError::ManagementError(msg) => write!(f, "Failed to manage container: {}", msg),
            ContainerError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ContainerError {}

/// Container information.
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    /// Container ID.
    pub id: String,
    /// Container name.
    pub name: String,
    /// Container image.
    pub image: String,
    /// Container status.
    pub status: String,
}

/// Container operations manager.
#[derive(Debug)]
pub struct ContainerOps {
    is_root: bool,
}

impl ContainerOps {
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);
        Self { is_root }
    }

    pub fn is_root(&self) -> bool { self.is_root }

    pub fn get_container_backend(&self) -> Result<String, ContainerError> {
        if Command::new("podman").arg("--version").output().is_ok() {
            Ok("podman".to_string())
        } else if Command::new("docker").arg("--version").output().is_ok() {
            Ok("docker".to_string())
        } else {
            Ok("none".to_string())
        }
    }

    pub fn list_containers(&self) -> Result<Vec<ContainerInfo>, ContainerError> {
        let mut containers = Vec::new();
        let backend = self.get_container_backend()?;
        let cmd = if backend == "podman" { "podman" } else if backend == "docker" { "docker" } else { return Ok(containers) };

        let output = Command::new(cmd)
            .args(&["ps", "-a", "--format", "{{.ID}}\t{{.Names}}\t{{.Image}}\t{{.Status}}"])
            .output()
            .map_err(|e| ContainerError::IoError(format!("Failed to run {}: {}", cmd, e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 4 {
                containers.push(ContainerInfo {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    image: parts[2].to_string(),
                    status: parts[3].to_string(),
                });
            }
        }

        Ok(containers)
    }

    pub fn list_images(&self) -> Result<Vec<String>, ContainerError> {
        let mut images = Vec::new();
        let backend = self.get_container_backend()?;
        let cmd = if backend == "podman" { "podman" } else if backend == "docker" { "docker" } else { return Ok(images) };

        let output = Command::new(cmd)
            .args(&["images", "--format", "{{.Repository}}:{{.Tag}}"])
            .output()
            .map_err(|e| ContainerError::IoError(format!("Failed to run {}: {}", cmd, e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines() {
            if !line.is_empty() {
                images.push(line.to_string());
            }
        }

        Ok(images)
    }
}

impl Default for ContainerOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let container = ContainerOps::new();
        let _ = container.is_root();
    }

    #[test]
    fn test_get_container_backend() {
        let container = ContainerOps::new();
        if let Ok(_) = container.get_container_backend() {
            // Should not panic
        }
    }
}
