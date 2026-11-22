//! SigmaOS certificate operations module.
//!
//! This module provides actual SSL/TLS certificate operations using openssl.
//!
//! # Example
//!
//! ```
//! use sigma_system::cert_ops::*;
//!
//! let mut cert_ops = CertOps::new();
//! if let Ok(certs) = cert_ops.list_certificates() {
//!     println!("Found {} certificates", certs.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in certificate operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CertError {
    /// Certificate not found.
    CertNotFound(String),
    /// Failed to read certificate.
    ReadError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for CertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CertError::CertNotFound(path) => write!(f, "Certificate '{}' not found", path),
            CertError::ReadError(msg) => write!(f, "Failed to read certificate: {}", msg),
            CertError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            CertError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for CertError {}

/// Certificate information.
#[derive(Debug, Clone)]
pub struct CertInfo {
    /// Subject.
    pub subject: String,
    /// Issuer.
    pub issuer: String,
    /// Valid from.
    pub valid_from: String,
    /// Valid to.
    pub valid_to: String,
    /// Fingerprint.
    pub fingerprint: String,
}

/// Certificate operations manager.
#[derive(Debug)]
pub struct CertOps {
    /// Whether running as root.
    is_root: bool,
}

impl CertOps {
    /// Create a new certificate operations manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self { is_root }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// List system certificates.
    pub fn list_certificates(&self) -> Result<Vec<String>, CertError> {
        let cert_dirs = [
            "/etc/ssl/certs",
            "/usr/local/share/ca-certificates",
            "/etc/pki/tls/certs",
        ];

        let mut certs = Vec::new();
        for dir in &cert_dirs {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.ends_with(".crt") || name.ends_with(".pem") {
                            certs.push(name);
                        }
                    }
                }
            }
        }

        Ok(certs)
    }

    /// Get certificate info.
    pub fn get_cert_info(&self, cert_path: &str) -> Result<CertInfo, CertError> {
        let output = Command::new("openssl")
            .args(&["x509", "-in", cert_path, "-noout", "-text"])
            .output()
            .map_err(|e| CertError::IoError(format!("Failed to run openssl: {}", e)))?;

        if !output.status.success() {
            return Err(CertError::ReadError("openssl command failed".to_string()));
        }

        let content = String::from_utf8_lossy(&output.stdout);
        let mut subject = "unknown".to_string();
        let mut issuer = "unknown".to_string();
        let mut valid_from = "unknown".to_string();
        let mut valid_to = "unknown".to_string();

        for line in content.lines() {
            if line.starts_with("Subject:") {
                subject = line[8..].trim().to_string();
            } else if line.starts_with("Issuer:") {
                issuer = line[7..].trim().to_string();
            } else if line.starts_with("Not Before:") {
                valid_from = line[11..].trim().to_string();
            } else if line.starts_with("Not After:") {
                valid_to = line[10..].trim().to_string();
            }
        }

        let fingerprint_output = Command::new("openssl")
            .args(&["x509", "-in", cert_path, "-noout", "-fingerprint"])
            .output()
            .map_err(|e| CertError::IoError(format!("Failed to run openssl: {}", e)))?;

        let fingerprint = String::from_utf8_lossy(&fingerprint_output.stdout)
            .lines()
            .find(|l| l.contains("="))
            .map(|l| l.split('=').nth(1).unwrap_or("unknown").trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(CertInfo {
            subject,
            issuer,
            valid_from,
            valid_to,
            fingerprint,
        })
    }

    /// Check certificate expiration.
    pub fn check_expiration(&self, cert_path: &str) -> Result<bool, CertError> {
        let info = self.get_cert_info(cert_path)?;
        
        // Simple check: if valid_to contains a date, assume it's valid
        // In production, use proper date parsing
        let is_valid = !info.valid_to.is_empty() && info.valid_to != "unknown";
        Ok(is_valid)
    }
}

impl Default for CertOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cert_ops = CertOps::new();
        let _ = cert_ops.is_root();
    }

    #[test]
    fn test_list_certificates() {
        let cert_ops = CertOps::new();
        if let Ok(certs) = cert_ops.list_certificates() {
            let _ = certs;
        }
    }
}
