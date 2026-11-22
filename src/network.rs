//! Network management inspired by Linux kernel networking and Arch Linux security.
//! Provides network configuration, firewall rules, and connection tracking.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkProtocol {
    Tcp,
    Udp,
    Icmp,
    All,
}

impl NetworkProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Icmp => "icmp",
            Self::All => "all",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Established,
    Related,
    New,
    Invalid,
}

impl ConnectionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Established => "ESTABLISHED",
            Self::Related => "RELATED",
            Self::New => "NEW",
            Self::Invalid => "INVALID",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    InvalidPort(String),
    InvalidIpAddress(String),
    InvalidRule(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPort(msg) => write!(f, "invalid port: {}", msg),
            Self::InvalidIpAddress(msg) => write!(f, "invalid IP address: {}", msg),
            Self::InvalidRule(msg) => write!(f, "invalid rule: {}", msg),
        }
    }
}

impl std::error::Error for NetworkError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirewallRule {
    pub id: String,
    pub protocol: NetworkProtocol,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub source_port: Option<u16>,
    pub destination_port: Option<u16>,
    pub action: FirewallAction,
    pub state: Option<ConnectionState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Accept,
    Drop,
    Reject,
}

impl FirewallAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accept => "ACCEPT",
            Self::Drop => "DROP",
            Self::Reject => "REJECT",
        }
    }
}

impl FirewallRule {
    pub fn new(id: impl Into<String>, action: FirewallAction) -> Self {
        Self {
            id: id.into(),
            protocol: NetworkProtocol::All,
            source: None,
            destination: None,
            source_port: None,
            destination_port: None,
            action,
            state: None,
        }
    }

    pub fn with_protocol(mut self, protocol: NetworkProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = Some(destination.into());
        self
    }

    pub fn with_source_port(mut self, port: u16) -> Result<Self, NetworkError> {
        if port == 0 {
            return Err(NetworkError::InvalidPort("port cannot be 0".to_string()));
        }
        self.source_port = Some(port);
        Ok(self)
    }

    pub fn with_destination_port(mut self, port: u16) -> Result<Self, NetworkError> {
        if port == 0 {
            return Err(NetworkError::InvalidPort("port cannot be 0".to_string()));
        }
        self.destination_port = Some(port);
        Ok(self)
    }

    pub fn with_state(mut self, state: ConnectionState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn to_iptables_rule(&self) -> String {
        let mut parts = vec!["-A INPUT".to_string()];
        
        if self.protocol != NetworkProtocol::All {
            parts.push(format!("-p {}", self.protocol.as_str()));
        }
        
        if let Some(ref src) = self.source {
            parts.push(format!("-s {}", src));
        }
        
        if let Some(ref dst) = self.destination {
            parts.push(format!("-d {}", dst));
        }
        
        if let Some(sport) = self.source_port {
            parts.push(format!("--sport {}", sport));
        }
        
        if let Some(dport) = self.destination_port {
            parts.push(format!("--dport {}", dport));
        }
        
        if let Some(ref state) = self.state {
            parts.push(format!("-m conntrack --ctstate {}", state.as_str()));
        }
        
        parts.push(format!("-j {}", self.action.as_str()));
        
        parts.join(" ")
    }
}

#[derive(Debug, Clone)]
pub struct NetworkManager {
    pub interface: String,
    pub rules: BTreeMap<String, FirewallRule>,
    pub default_policy: FirewallAction,
    pub ipv4_enabled: bool,
    pub ipv6_enabled: bool,
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self {
            interface: "eth0".to_string(),
            rules: BTreeMap::new(),
            default_policy: FirewallAction::Drop,
            ipv4_enabled: true,
            ipv6_enabled: true,
        }
    }
}

impl NetworkManager {
    pub fn new(interface: impl Into<String>) -> Self {
        Self {
            interface: interface.into(),
            rules: BTreeMap::new(),
            default_policy: FirewallAction::Drop,
            ipv4_enabled: true,
            ipv6_enabled: true,
        }
    }

    pub fn with_default_policy(mut self, policy: FirewallAction) -> Self {
        self.default_policy = policy;
        self
    }

    pub fn disable_ipv6(mut self) -> Self {
        self.ipv6_enabled = false;
        self
    }

    pub fn add_rule(&mut self, rule: FirewallRule) -> Result<(), NetworkError> {
        let id = rule.id.clone();
        if id.trim().is_empty() {
            return Err(NetworkError::InvalidRule("rule ID cannot be empty".to_string()));
        }
        self.rules.insert(id, rule);
        Ok(())
    }

    pub fn remove_rule(&mut self, id: &str) -> Option<FirewallRule> {
        self.rules.remove(id)
    }

    pub fn get_rule(&self, id: &str) -> Option<&FirewallRule> {
        self.rules.get(id)
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn generate_iptables_script(&self) -> String {
        let mut script = String::new();
        
        script.push_str("#!/bin/bash\n");
        script.push_str("# SigmaOS Firewall Configuration\n");
        script.push_str("# Generated by SigmaOS NetworkManager\n\n");
        
        script.push_str("# Flush existing rules\n");
        script.push_str("iptables -F\n");
        script.push_str("iptables -X\n");
        if self.ipv6_enabled {
            script.push_str("ip6tables -F\n");
            script.push_str("ip6tables -X\n");
        }
        script.push_str("\n");
        
        script.push_str("# Set default policy\n");
        script.push_str(&format!("iptables -P INPUT {}\n", self.default_policy.as_str()));
        script.push_str("iptables -P FORWARD DROP\n");
        script.push_str("iptables -P OUTPUT ACCEPT\n");
        if self.ipv6_enabled {
            script.push_str(&format!("ip6tables -P INPUT {}\n", self.default_policy.as_str()));
            script.push_str("ip6tables -P FORWARD DROP\n");
            script.push_str("ip6tables -P OUTPUT ACCEPT\n");
        }
        script.push_str("\n");
        
        script.push_str("# Allow loopback\n");
        script.push_str("iptables -A INPUT -i lo -j ACCEPT\n");
        if self.ipv6_enabled {
            script.push_str("ip6tables -A INPUT -i lo -j ACCEPT\n");
        }
        script.push_str("\n");
        
        script.push_str("# Allow established connections\n");
        script.push_str("iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT\n");
        if self.ipv6_enabled {
            script.push_str("ip6tables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT\n");
        }
        script.push_str("\n");
        
        script.push_str("# Custom rules\n");
        for rule in self.rules.values() {
            script.push_str(&format!("iptables {}\n", rule.to_iptables_rule()));
            if self.ipv6_enabled {
                script.push_str(&format!("ip6tables {}\n", rule.to_iptables_rule()));
            }
        }
        script.push_str("\n");
        
        script.push_str("# Save rules\n");
        script.push_str("iptables-save > /etc/iptables/rules.v4\n");
        if self.ipv6_enabled {
            script.push_str("ip6tables-save > /etc/iptables/rules.v6\n");
        }
        
        script
    }

    pub fn generate_nftables_config(&self) -> String {
        let mut config = String::new();
        
        config.push_str("#!/usr/sbin/nft -f\n");
        config.push_str("# SigmaOS nftables Configuration\n");
        config.push_str("# Generated by SigmaOS NetworkManager\n\n");
        
        config.push_str("flush ruleset\n\n");
        
        config.push_str("table inet sigmaos {\n");
        config.push_str("    chain input {\n");
        config.push_str(&format!("        type filter hook input priority 0; policy {};\n", 
            match self.default_policy {
                FirewallAction::Accept => "accept",
                FirewallAction::Drop => "drop",
                FirewallAction::Reject => "reject",
            }
        ));
        config.push_str("\n");
        config.push_str("        iif lo accept\n");
        config.push_str("        ct state established,related accept\n");
        config.push_str("\n");
        
        for rule in self.rules.values() {
            config.push_str("        ");
            if rule.protocol != NetworkProtocol::All {
                config.push_str(&format!("ip protocol {} ", rule.protocol.as_str()));
            }
            if let Some(ref dst) = rule.destination {
                config.push_str(&format!("ip daddr {} ", dst));
            }
            if let Some(dport) = rule.destination_port {
                config.push_str(&format!("tcp dport {} ", dport));
            }
            config.push_str(&format!("{}\n", match rule.action {
                FirewallAction::Accept => "accept",
                FirewallAction::Drop => "drop",
                FirewallAction::Reject => "reject",
            }));
        }
        
        config.push_str("    }\n");
        config.push_str("}\n");
        
        config
    }

    pub fn generate_networkd_config(&self) -> String {
        format!(
            "# SigmaOS systemd-networkd Configuration\n\
             # Interface: {}\n\
             [Match]\n\
             Name={}\n\
             \n\
             [Network]\n\
             DHCP=yes\n\
             IPv6LinkLocal=yes\n",
            self.interface, self.interface
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_rule_new() {
        let rule = FirewallRule::new("test-rule", FirewallAction::Accept);
        assert_eq!(rule.id, "test-rule");
        assert_eq!(rule.action, FirewallAction::Accept);
    }

    #[test]
    fn test_firewall_rule_with_protocol() {
        let rule = FirewallRule::new("test", FirewallAction::Accept)
            .with_protocol(NetworkProtocol::Tcp);
        assert_eq!(rule.protocol, NetworkProtocol::Tcp);
    }

    #[test]
    fn test_firewall_rule_with_port_valid() {
        let rule = FirewallRule::new("test", FirewallAction::Accept)
            .with_destination_port(80);
        assert!(rule.is_ok());
        assert_eq!(rule.unwrap().destination_port, Some(80));
    }

    #[test]
    fn test_firewall_rule_with_port_invalid() {
        let rule = FirewallRule::new("test", FirewallAction::Accept)
            .with_destination_port(0);
        assert!(rule.is_err());
    }

    #[test]
    fn test_firewall_rule_to_iptables() {
        let rule = FirewallRule::new("test", FirewallAction::Accept)
            .with_protocol(NetworkProtocol::Tcp)
            .with_destination_port(80)
            .unwrap();
        let iptables = rule.to_iptables_rule();
        assert!(iptables.contains("-p tcp"));
        assert!(iptables.contains("--dport 80"));
        assert!(iptables.contains("ACCEPT"));
    }

    #[test]
    fn test_network_manager_new() {
        let mgr = NetworkManager::new("eth1");
        assert_eq!(mgr.interface, "eth1");
        assert_eq!(mgr.default_policy, FirewallAction::Drop);
    }

    #[test]
    fn test_network_manager_add_rule() {
        let mut mgr = NetworkManager::new("eth0");
        let rule = FirewallRule::new("ssh", FirewallAction::Accept)
            .with_protocol(NetworkProtocol::Tcp)
            .with_destination_port(22)
            .unwrap();
        mgr.add_rule(rule).unwrap();
        assert_eq!(mgr.rule_count(), 1);
    }

    #[test]
    fn test_network_manager_add_rule_invalid_id() {
        let mut mgr = NetworkManager::new("eth0");
        let rule = FirewallRule::new("", FirewallAction::Accept);
        assert!(mgr.add_rule(rule).is_err());
    }

    #[test]
    fn test_network_manager_remove_rule() {
        let mut mgr = NetworkManager::new("eth0");
        let rule = FirewallRule::new("ssh", FirewallAction::Accept);
        mgr.add_rule(rule).unwrap();
        mgr.remove_rule("ssh");
        assert_eq!(mgr.rule_count(), 0);
    }

    #[test]
    fn test_generate_iptables_script() {
        let mut mgr = NetworkManager::new("eth0");
        let rule = FirewallRule::new("ssh", FirewallAction::Accept)
            .with_protocol(NetworkProtocol::Tcp)
            .with_destination_port(22)
            .unwrap();
        mgr.add_rule(rule).unwrap();
        let script = mgr.generate_iptables_script();
        assert!(script.contains("# SigmaOS Firewall Configuration"));
        assert!(script.contains("iptables -A INPUT"));
    }

    #[test]
    fn test_generate_nftables_config() {
        let mut mgr = NetworkManager::new("eth0");
        let rule = FirewallRule::new("ssh", FirewallAction::Accept)
            .with_protocol(NetworkProtocol::Tcp)
            .with_destination_port(22)
            .unwrap();
        mgr.add_rule(rule).unwrap();
        let config = mgr.generate_nftables_config();
        assert!(config.contains("table inet sigmaos"));
        assert!(config.contains("chain input"));
    }

    #[test]
    fn test_generate_networkd_config() {
        let mgr = NetworkManager::new("eth0");
        let config = mgr.generate_networkd_config();
        assert!(config.contains("Name=eth0"));
        assert!(config.contains("DHCP=yes"));
    }
}
