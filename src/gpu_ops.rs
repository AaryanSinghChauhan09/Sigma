//! SigmaOS GPU operations module.
//!
//! This module provides actual GPU operations using nvidia-smi/AMD tools.
//!
//! # Example
//!
//! ```
//! use sigma_system::gpu_ops::*;
//!
//! let gpu = GpuOps::new();
//! if let Ok(gpus) = gpu.list_gpus() {
//!     println!("Found {} GPUs", gpus.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in GPU operations.
#[derive(Debug, Clone, PartialEq)]
pub enum GpuError {
    /// GPU not found.
    GpuNotFound(String),
    /// Failed to manage GPU.
    ManagementError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for GpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GpuError::GpuNotFound(name) => write!(f, "GPU '{}' not found", name),
            GpuError::ManagementError(msg) => write!(f, "Failed to manage GPU: {}", msg),
            GpuError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for GpuError {}

/// GPU information.
#[derive(Debug, Clone)]
pub struct GpuInfo {
    /// GPU ID.
    pub id: u32,
    /// GPU name.
    pub name: String,
    /// GPU memory in MB.
    pub memory_mb: u64,
    /// GPU type.
    pub gpu_type: String,
}

/// GPU operations manager.
#[derive(Debug)]
pub struct GpuOps;

impl GpuOps {
    pub fn new() -> Self { Self }

    pub fn get_gpu_backend(&self) -> Result<String, GpuError> {
        if Command::new("nvidia-smi").arg("--version").output().is_ok() {
            Ok("nvidia".to_string())
        } else if Command::new("rocm-smi").arg("--version").output().is_ok() {
            Ok("amd".to_string())
        } else {
            Ok("none".to_string())
        }
    }

    pub fn list_gpus(&self) -> Result<Vec<GpuInfo>, GpuError> {
        let mut gpus = Vec::new();
        let backend = self.get_gpu_backend()?;

        if backend == "nvidia" {
            let output = Command::new("nvidia-smi")
                .args(&["--query-gpu", "--format=csv,noheader,name,memory.total"])
                .output()
                .map_err(|e| GpuError::IoError(format!("Failed to run nvidia-smi: {}", e)))?;

            let content = String::from_utf8_lossy(&output.stdout);
            for (i, line) in content.lines().enumerate() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    let memory_mb = parts[1].trim().replace(" MiB", "").parse::<u64>().unwrap_or(0);
                    gpus.push(GpuInfo {
                        id: i as u32,
                        name: parts[0].to_string(),
                        memory_mb,
                        gpu_type: "nvidia".to_string(),
                    });
                }
            }
        }

        Ok(gpus)
    }
}

impl Default for GpuOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gpu = GpuOps::new();
        let _ = gpu;
    }

    #[test]
    fn test_get_gpu_backend() {
        let gpu = GpuOps::new();
        if let Ok(_) = gpu.get_gpu_backend() {
            // Should not panic
        }
    }
}
