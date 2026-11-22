//! SigmaOS firewall operations module.
//!
//! This module provides actual firewall operations using iptables and nftables.
//!
//! # Example
//!
//! ```
//! use sigma_system::firewall_ops::*;
//!
//! let mut firewall = FirewallOps::new();
//! if let Ok(rules) = firewall.list_rules() {
//!     println!("Found {} firewall rules", rules.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in firewall operations.
#[derive(Debug, Clone, PartialEq)]
pub enum FirewallError {
    /// Failed to get rules.
    RulesError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// Invalid rule.
    InvalidRule(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for FirewallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirewallError::RulesError(msg) => write!(f, "Failed to get rules: {}", msg),
            FirewallError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            FirewallError::InvalidRule(rule) => write!(f, "Invalid rule: {}", rule),
            FirewallError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for FirewallError {}

/// Firewall backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallBackend {
    /// iptables backend.
    Iptables,
    /// nftables backend.
    Nftables,
}

/// Firewall rule.
#[derive(Debug, Clone)]
pub struct FirewallRule {
    /// Rule number.
    pub number: u32,
    /// Target (ACCEPT, DROP, REJECT, etc.).
    pub target: String,
    /// Protocol (tcp, udp, etc.).
    pub protocol: String,
    /// Source address.
    pub source: String,
    /// Destination address.
    pub destination: String,
    /// Source port.
    pub source_port: String,
    /// Destination port.
    pub destination_port: String,
    /// Options.
    pub options: String,
}

/// Firewall operations manager.
#[derive(Debug)]
pub struct FirewallOps {
    /// Preferred backend.
    backend: FirewallBackend,
    /// Whether running as root.
    is_root: bool,
}

impl FirewallOps {
    /// Create a new firewall operations manager.
    pub fn new() -> Self {
        let backend = if Command::new("nft").arg("--version").output().is_ok() {
            FirewallBackend::Nftables
        } else {
            FirewallBackend::Iptables
        };

        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self { backend, is_root }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Get backend.
    pub fn get_backend(&self) -> FirewallBackend {
        self.backend
    }

    /// List firewall rules.
    pub fn list_rules(&self) -> Result<Vec<FirewallRule>, FirewallError> {
        match self.backend {
            FirewallBackend::Iptables => self.list_iptables_rules(),
            FirewallBackend::Nftables => self.list_nftables_rules(),
        }
    }

    /// List iptables rules.
    fn list_iptables_rules(&self) -> Result<Vec<FirewallRule>, FirewallError> {
        let output = Command::new("iptables").args(&["-L", "-n", "--line-numbers"]).output()
            .map_err(|e| FirewallError::IoError(format!("Failed to run iptables: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let mut rules = Vec::new();
        
        for (i, line) in content.lines().enumerate() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && i > 0 {
                rules.push(FirewallRule {
                    number: i as u32,
                    target: parts[1].to_string(),
                    protocol: if parts.len() > 2 { parts[2].to_string() } else { "".to_string() },
                    source: if parts.len() > 3 { parts[3].to_string() } else { "".to_string() },
                    destination: if parts.len() > 4 { parts[4].to_string() } else { "".to_string() },
                    source_port: String::new(),
                    destination_port: String::new(),
                    options: if parts.len() > 5 { parts[5..].join(" ") } else { "".to_string() },
                });
            }
        }
        
        Ok(rules)
    }

    /// List nftables rules.
    fn list_nftables_rules(&self) -> Result<Vec<FirewallRule>, FirewallError> {
        let output = Command::new("nft").args(&["list", "ruleset"]).output()
            .map_err(|e| FirewallError::IoError(format!("Failed to run nft: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let mut rules = Vec::new();
        
        for (i, line) in content.lines().enumerate() {
            if line.contains("accept") || line.contains("drop") || line.contains("reject") {
                rules.push(FirewallRule {
                    number: i as u32,
                    target: if line.contains("accept") { "ACCEPT".to_string() }
                            else if line.contains("drop") { "DROP".to_string() }
                            else if line.contains("reject") { "REJECT".to_string() }
                            else { "UNKNOWN".to_string() },
                    protocol: String::new(),
                    source: String::new(),
                    destination: String::new(),
                    source_port: String::new(),
                    destination_port: String::new(),
                    options: line.to_string(),
                });
            }
        }
        
        Ok(rules)
    }

    /// Add a rule.
    pub fn add_rule(&self, rule: &str) -> Result<(), FirewallError> {
        if !self.is_root {
            return Err(FirewallError::PermissionDenied("Must be root to add firewall rule".to_string()));
        }

        let result = match self.backend {
            FirewallBackend::Iptables => {
                Command::new("iptables").args(&["-A", rule]).status()
            }
            FirewallBackend::Nftables => {
                Command::new("nft").args(&["add", "rule", rule]).status()
            }
        };

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(FirewallError::InvalidRule("Failed to add rule".to_string())),
            Err(e) => Err(FirewallError::IoError(format!("Failed to run firewall command: {}", e))),
        }
    }

    /// Delete a rule.
    pub fn delete_rule(&self, rule: &str) -> Result<(), FirewallError> {
        if !self.is_root {
            return Err(FirewallError::PermissionDenied("Must be root to delete firewall rule".to_string()));
        }

        let result = match self.backend {
            FirewallBackend::Iptables => {
                Command::new("iptables").args(&["-D", rule]).status()
            }
            FirewallBackend::Nftables => {
                Command::new("nft").args(&["delete", "rule", rule]).status()
            }
        };

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(FirewallError::InvalidRule("Failed to delete rule".to_string())),
            Err(e) => Err(FirewallError::IoError(format!("Failed to run firewall command: {}", e))),
        }
    }

    /// Flush all rules.
    pub fn flush_rules(&self) -> Result<(), FirewallError> {
        if !self.is_root {
            return Err(FirewallError::PermissionDenied("Must be root to flush firewall rules".to_string()));
        }

        let result = match self.backend {
            FirewallBackend::Iptables => {
                Command::new("iptables").arg("-F").status()
            }
            FirewallBackend::Nftables => {
                Command::new("nft").args(&["flush", "ruleset"]).status()
            }
        };

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(FirewallError::RulesError("Failed to flush rules".to_string())),
            Err(e) => Err(FirewallError::IoError(format!("Failed to run firewall command: {}", e))),
        }
    }

    /// Get firewall summary.
    pub fn get_firewall_summary(&self) -> Result<String, FirewallError> {
        let rules = self.list_rules()?;
        
        Ok(format!(
            "Backend: {:?}\nTotal rules: {}",
            self.backend,
            rules.len()
        ))
    }
}

impl Default for FirewallOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let firewall = FirewallOps::new();
        let _ = firewall.is_root();
        let _ = firewall.get_backend();
    }

    #[test]
    fn test_list_rules() {
        let firewall = FirewallOps::new();
        if let Ok(rules) = firewall.list_rules() {
            let _ = rules;
        }
    }

    #[test]
    fn test_get_firewall_summary() {
        let firewall = FirewallOps::new();
        if let Ok(summary) = firewall.get_firewall_summary() {
            assert!(!summary.is_empty());
        }
    }
}
