//! SigmaOS DNS operations module.
//!
//! This module provides actual DNS configuration operations using /etc/resolv.conf and resolvectl.
//!
//! # Example
//!
//! ```
//! use sigma_system::dns_ops::*;
//!
//! let mut dns_ops = DnsOps::new();
//! if let Ok(servers) = dns_ops.get_dns_servers() {
//!     println!("DNS servers: {:?}", servers);
//! }
//! ```

use std::fs;
use std::process::Command;

/// Errors that can occur in DNS operations.
#[derive(Debug, Clone, PartialEq)]
pub enum DnsError {
    /// Failed to read DNS config.
    ReadError(String),
    /// Failed to set DNS.
    SetError(String),
    /// Invalid DNS server.
    InvalidDnsServer(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsError::ReadError(msg) => write!(f, "Failed to read DNS config: {}", msg),
            DnsError::SetError(msg) => write!(f, "Failed to set DNS: {}", msg),
            DnsError::InvalidDnsServer(server) => write!(f, "Invalid DNS server: {}", server),
            DnsError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            DnsError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for DnsError {}

/// DNS operations manager.
#[derive(Debug)]
pub struct DnsOps {
    /// Resolv.conf path.
    resolv_conf_path: String,
    /// Whether running as root.
    is_root: bool,
}

impl DnsOps {
    /// Create a new DNS operations manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            resolv_conf_path: "/etc/resolv.conf".to_string(),
            is_root,
        }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Get DNS servers from /etc/resolv.conf.
    pub fn get_dns_servers(&self) -> Result<Vec<String>, DnsError> {
        let content = fs::read_to_string(&self.resolv_conf_path)
            .map_err(|e| DnsError::IoError(format!("Failed to read resolv.conf: {}", e)))?;
        
        let mut servers = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("nameserver") {
                if let Some(server) = line.split_whitespace().nth(1) {
                    servers.push(server.to_string());
                }
            }
        }
        
        Ok(servers)
    }

    /// Get search domains.
    pub fn get_search_domains(&self) -> Result<Vec<String>, DnsError> {
        let content = fs::read_to_string(&self.resolv_conf_path)
            .map_err(|e| DnsError::IoError(format!("Failed to read resolv.conf: {}", e)))?;
        
        let mut domains = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("search") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for part in parts.iter().skip(1) {
                    domains.push(part.to_string());
                }
            }
        }
        
        Ok(domains)
    }

    /// Set DNS servers using resolvectl.
    pub fn set_dns_servers(&self, servers: &[&str]) -> Result<(), DnsError> {
        if !self.is_root {
            return Err(DnsError::PermissionDenied("Must be root to set DNS servers".to_string()));
        }

        for server in servers {
            self.validate_dns_server(server)?;
        }

        // Try resolvectl first (systemd-resolved)
        if Command::new("resolvectl").arg("--version").output().is_ok() {
            for server in servers {
                let result = Command::new("resolvectl")
                    .args(&["dns", server])
                    .status();

                if let Ok(status) = result {
                    if !status.success() {
                        return Err(DnsError::SetError(format!("Failed to set DNS server: {}", server)));
                    }
                }
            }
            return Ok(());
        }

        // Fallback to writing resolv.conf directly
        let mut content = String::new();
        for server in servers {
            content.push_str(&format!("nameserver {}\n", server));
        }
        
        fs::write(&self.resolv_conf_path, content)
            .map_err(|e| DnsError::IoError(format!("Failed to write resolv.conf: {}", e)))?;

        Ok(())
    }

    /// Add DNS server.
    pub fn add_dns_server(&self, server: &str) -> Result<(), DnsError> {
        if !self.is_root {
            return Err(DnsError::PermissionDenied("Must be root to add DNS server".to_string()));
        }

        self.validate_dns_server(server)?;

        let mut servers = self.get_dns_servers().unwrap_or_default();
        if !servers.contains(&server.to_string()) {
            servers.push(server.to_string());
            let server_refs: Vec<&str> = servers.iter().map(|s| s.as_str()).collect();
            self.set_dns_servers(&server_refs)?;
        }

        Ok(())
    }

    /// Remove DNS server.
    pub fn remove_dns_server(&self, server: &str) -> Result<(), DnsError> {
        if !self.is_root {
            return Err(DnsError::PermissionDenied("Must be root to remove DNS server".to_string()));
        }

        let mut servers = self.get_dns_servers().unwrap_or_default();
        servers.retain(|s| s != server);
        
        let server_refs: Vec<&str> = servers.iter().map(|s| s.as_str()).collect();
        self.set_dns_servers(&server_refs)?;

        Ok(())
    }

    /// Validate DNS server address.
    fn validate_dns_server(&self, server: &str) -> Result<(), DnsError> {
        // Basic IP validation
        let parts: Vec<&str> = server.split('.').collect();
        if parts.len() != 4 {
            return Err(DnsError::InvalidDnsServer("Invalid IP address format".to_string()));
        }

        for part in parts {
            let num: u8 = part.parse()
                .map_err(|_| DnsError::InvalidDnsServer(format!("Invalid octet: {}", part)))?;
        }

        Ok(())
    }

    /// Flush DNS cache.
    pub fn flush_dns_cache(&self) -> Result<(), DnsError> {
        if !self.is_root {
            return Err(DnsError::PermissionDenied("Must be root to flush DNS cache".to_string()));
        }

        // Try systemd-resolved
        if Command::new("systemctl").arg("is-active").arg("systemd-resolved").output().is_ok() {
            let result = Command::new("resolvectl").arg("flush-caches").status();
            if let Ok(status) = result {
                if status.success() {
                    return Ok(());
                }
            }
        }

        // Try dnsmasq
        if Command::new("systemctl").arg("is-active").arg("dnsmasq").output().is_ok() {
            let result = Command::new("systemctl").arg("restart").arg("dnsmasq").status();
            if let Ok(status) = result {
                if status.success() {
                    return Ok(());
                }
            }
        }

        Err(DnsError::SetError("No supported DNS service found".to_string()))
    }

    /// Test DNS resolution.
    pub fn test_dns(&self, hostname: &str) -> Result<String, DnsError> {
        let output = Command::new("nslookup").arg(hostname).output()
            .map_err(|e| DnsError::IoError(format!("Failed to run nslookup: {}", e)))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(DnsError::ReadError("DNS resolution failed".to_string()))
        }
    }

    /// Get DNS summary.
    pub fn get_dns_summary(&self) -> Result<String, DnsError> {
        let servers = self.get_dns_servers()?;
        let domains = self.get_search_domains().unwrap_or_default();
        
        let mut summary = format!("DNS servers: {}\n", servers.join(", "));
        if !domains.is_empty() {
            summary.push_str(&format!("Search domains: {}\n", domains.join(", ")));
        }
        
        Ok(summary)
    }
}

impl Default for DnsOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dns_ops = DnsOps::new();
        let _ = dns_ops.is_root();
    }

    #[test]
    fn test_get_dns_servers() {
        let dns_ops = DnsOps::new();
        if let Ok(servers) = dns_ops.get_dns_servers() {
            let _ = servers;
        }
    }

    #[test]
    fn test_validate_dns_server() {
        let dns_ops = DnsOps::new();
        assert!(dns_ops.validate_dns_server("8.8.8.8").is_ok());
        assert!(dns_ops.validate_dns_server("1.1.1.1").is_ok());
        assert!(dns_ops.validate_dns_server("invalid").is_err());
    }

    #[test]
    fn test_get_search_domains() {
        let dns_ops = DnsOps::new();
        if let Ok(domains) = dns_ops.get_search_domains() {
            let _ = domains;
        }
    }
}
