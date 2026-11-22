//! SigmaOS package installation module.
//!
//! This module provides actual package installation capabilities
//! using pacman (Arch Linux package manager).
//!
//! # Example
//!
//! ```
//! use sigma_system::pkg_install::*;
//!
//! let mut installer = PackageInstaller::new();
//! if installer.is_root() {
//!     installer.install("vim").unwrap();
//!     installer.install("htop").unwrap();
//! }
//! ```

use std::process::Command;
use std::collections::BTreeMap;

/// Errors that can occur in package installation.
#[derive(Debug, Clone, PartialEq)]
pub enum InstallError {
    /// Package not found.
    PackageNotFound(String),
    /// Installation failed.
    InstallFailed(String),
    /// Dependency resolution failed.
    DependencyFailed(String),
    /// Permission denied.
    PermissionDenied(String),
    /// Network error.
    NetworkError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for InstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallError::PackageNotFound(pkg) => write!(f, "Package '{}' not found", pkg),
            InstallError::InstallFailed(msg) => write!(f, "Installation failed: {}", msg),
            InstallError::DependencyFailed(msg) => write!(f, "Dependency resolution failed: {}", msg),
            InstallError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            InstallError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            InstallError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for InstallError {}

/// Package information.
#[derive(Debug, Clone)]
pub struct PackageInfo {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: String,
    /// Package description.
    pub description: String,
    /// Package size in bytes.
    pub size: u64,
    /// Whether package is installed.
    pub installed: bool,
    /// Installation date.
    pub install_date: Option<String>,
}

/// Package installer.
#[derive(Debug)]
pub struct PackageInstaller {
    /// Package manager to use.
    package_manager: String,
    /// Whether running as root.
    is_root: bool,
    /// Installed packages cache.
    installed_packages: BTreeMap<String, PackageInfo>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl PackageInstaller {
    /// Create a new package installer.
    pub fn new() -> Self {
        let package_manager = if Command::new("pacman").arg("--version").output().is_ok() {
            "pacman".to_string()
        } else if Command::new("apt").arg("--version").output().is_ok() {
            "apt".to_string()
        } else if Command::new("dnf").arg("--version").output().is_ok() {
            "dnf".to_string()
        } else {
            "unknown".to_string()
        };

        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            package_manager,
            is_root,
            installed_packages: BTreeMap::new(),
            last_update: std::time::Instant::now(),
        }
    }

    /// Get package manager name.
    pub fn get_package_manager(&self) -> &str {
        &self.package_manager
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Install a package.
    pub fn install(&mut self, package: &str) -> Result<(), InstallError> {
        if !self.is_root {
            return Err(InstallError::PermissionDenied("Must be root to install packages".to_string()));
        }

        let result = match self.package_manager.as_str() {
            "pacman" => Command::new("pacman")
                .args(&["-S", "--noconfirm", package])
                .output(),
            "apt" => Command::new("apt")
                .args(&["install", "-y", package])
                .output(),
            "dnf" => Command::new("dnf")
                .args(&["install", "-y", package])
                .output(),
            _ => return Err(InstallError::InstallFailed("Unknown package manager".to_string())),
        };

        match result {
            Ok(output) => {
                if output.status.success() {
                    // Refresh package list
                    self.refresh_installed_packages()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(InstallError::InstallFailed(stderr.to_string()))
                }
            }
            Err(e) => Err(InstallError::IoError(format!("Failed to run package manager: {}", e))),
        }
    }

    /// Install multiple packages.
    pub fn install_multiple(&mut self, packages: &[&str]) -> Result<(), InstallError> {
        for package in packages {
            self.install(package)?;
        }
        Ok(())
    }

    /// Remove a package.
    pub fn remove(&mut self, package: &str) -> Result<(), InstallError> {
        if !self.is_root {
            return Err(InstallError::PermissionDenied("Must be root to remove packages".to_string()));
        }

        let result = match self.package_manager.as_str() {
            "pacman" => Command::new("pacman")
                .args(&["-R", "--noconfirm", package])
                .output(),
            "apt" => Command::new("apt")
                .args(&["remove", "-y", package])
                .output(),
            "dnf" => Command::new("dnf")
                .args(&["remove", "-y", package])
                .output(),
            _ => return Err(InstallError::InstallFailed("Unknown package manager".to_string())),
        };

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.installed_packages.remove(package);
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(InstallError::InstallFailed(stderr.to_string()))
                }
            }
            Err(e) => Err(InstallError::IoError(format!("Failed to run package manager: {}", e))),
        }
    }

    /// Update package database.
    pub fn update_database(&mut self) -> Result<(), InstallError> {
        if !self.is_root {
            return Err(InstallError::PermissionDenied("Must be root to update database".to_string()));
        }

        let result = match self.package_manager.as_str() {
            "pacman" => Command::new("pacman")
                .args(&["-Sy"])
                .output(),
            "apt" => Command::new("apt")
                .args(&["update"])
                .output(),
            "dnf" => Command::new("dnf")
                .args(&["makecache"])
                .output(),
            _ => return Err(InstallError::InstallFailed("Unknown package manager".to_string())),
        };

        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(InstallError::InstallFailed(stderr.to_string()))
                }
            }
            Err(e) => Err(InstallError::IoError(format!("Failed to run package manager: {}", e))),
        }
    }

    /// Upgrade all packages.
    pub fn upgrade_all(&mut self) -> Result<(), InstallError> {
        if !self.is_root {
            return Err(InstallError::PermissionDenied("Must be root to upgrade packages".to_string()));
        }

        let result = match self.package_manager.as_str() {
            "pacman" => Command::new("pacman")
                .args(&["-Syu", "--noconfirm"])
                .output(),
            "apt" => Command::new("apt")
                .args(&["upgrade", "-y"])
                .output(),
            "dnf" => Command::new("dnf")
                .args(&["upgrade", "-y"])
                .output(),
            _ => return Err(InstallError::InstallFailed("Unknown package manager".to_string())),
        };

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.refresh_installed_packages()?;
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(InstallError::InstallFailed(stderr.to_string()))
                }
            }
            Err(e) => Err(InstallError::IoError(format!("Failed to run package manager: {}", e))),
        }
    }

    /// Search for packages.
    pub fn search(&self, query: &str) -> Result<Vec<String>, InstallError> {
        let result = match self.package_manager.as_str() {
            "pacman" => Command::new("pacman")
                .args(&["-Ss", query])
                .output(),
            "apt" => Command::new("apt")
                .args(&["search", query])
                .output(),
            "dnf" => Command::new("dnf")
                .args(&["search", query])
                .output(),
            _ => return Err(InstallError::InstallFailed("Unknown package manager".to_string())),
        };

        match result {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let packages: Vec<String> = stdout.lines()
                        .filter(|line| !line.trim().is_empty())
                        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    Ok(packages)
                } else {
                    Err(InstallError::InstallFailed("Search failed".to_string()))
                }
            }
            Err(e) => Err(InstallError::IoError(format!("Failed to run package manager: {}", e))),
        }
    }

    /// Check if package is installed.
    pub fn is_installed(&mut self, package: &str) -> Result<bool, InstallError> {
        self.refresh_installed_packages()?;
        Ok(self.installed_packages.contains_key(package))
    }

    /// Get installed package info.
    pub fn get_package_info(&mut self, package: &str) -> Result<Option<PackageInfo>, InstallError> {
        self.refresh_installed_packages()?;
        Ok(self.installed_packages.get(package).cloned())
    }

    /// Get all installed packages.
    pub fn get_installed_packages(&mut self) -> Result<Vec<&PackageInfo>, InstallError> {
        self.refresh_installed_packages()?;
        Ok(self.installed_packages.values().collect())
    }

    /// Refresh installed packages list.
    fn refresh_installed_packages(&mut self) -> Result<(), InstallError> {
        // Only refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() < 1 {
            return Ok(());
        }

        self.installed_packages.clear();

        let result = match self.package_manager.as_str() {
            "pacman" => Command::new("pacman")
                .args(&["-Q"])
                .output(),
            "apt" => Command::new("dpkg")
                .args(&["-l"])
                .output(),
            "dnf" => Command::new("rpm")
                .args(&["-qa"])
                .output(),
            _ => return Err(InstallError::InstallFailed("Unknown package manager".to_string())),
        };

        match result {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            let name = parts[0].to_string();
                            let version = parts[1].to_string();
                            self.installed_packages.insert(name.clone(), PackageInfo {
                                name,
                                version,
                                description: String::new(),
                                size: 0,
                                installed: true,
                                install_date: None,
                            });
                        }
                    }
                }
            }
            Err(_) => {
                // Ignore errors - package list may not be available
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Install essential packages.
    pub fn install_essential(&mut self) -> Result<(), InstallError> {
        let essential = [
            "base",
            "linux",
            "linux-firmware",
            "systemd",
            "bash",
            "coreutils",
            "findutils",
            "sed",
            "grep",
            "gawk",
            "tar",
            "gzip",
            "xz",
            "bzip2",
            "zstd",
            "networkmanager",
            "dhclient",
            "wpa_supplicant",
            "openssh",
            "sudo",
            "vim",
            "nano",
            "htop",
            "tmux",
            "git",
            "curl",
            "wget",
            "rsync",
            "ca-certificates",
            "man-db",
            "man-pages",
            "tzdata",
            "locale",
            "kbd",
            "console-setup",
            "fontconfig",
            "ttf-dejavu",
        ];

        self.install_multiple(&essential)
    }
}

impl Default for PackageInstaller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let installer = PackageInstaller::new();
        assert!(!installer.package_manager.is_empty());
    }

    #[test]
    fn test_get_package_manager() {
        let installer = PackageInstaller::new();
        let pm = installer.get_package_manager();
        assert!(pm == "pacman" || pm == "apt" || pm == "dnf" || pm == "unknown");
    }

    #[test]
    fn test_is_root() {
        let installer = PackageInstaller::new();
        // Just verify it returns a boolean
        let _ = installer.is_root();
    }

    #[test]
    fn test_search() {
        let installer = PackageInstaller::new();
        // Search may fail if not root or no package manager
        if let Ok(results) = installer.search("vim") {
            // Results may be empty in restricted environments
            let _ = results;
        }
    }

    #[test]
    fn test_install_permission() {
        let mut installer = PackageInstaller::new();
        // If not root, install should fail with permission error
        if !installer.is_root() {
            let result = installer.install("test-package");
            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), InstallError::PermissionDenied(_)));
        }
    }
}
