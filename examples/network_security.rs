//! Example: Network security configuration using SigmaOS library
//!
//! This example demonstrates how to configure network security,
//! firewall rules, and intrusion detection using SigmaOS.

use sigma_system::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SigmaOS Network Security Configuration ===\n");

    // 1. Firewall Configuration
    println!("1. Firewall Configuration");
    let mut network_mgr = NetworkManager::new();

    // Allow SSH
    network_mgr.add_firewall_rule(FirewallRule::new(
        NetworkProtocol::Tcp,
        "22",
        FirewallAction::Allow,
        "SSH access",
    )?)?;

    // Allow HTTP/HTTPS
    network_mgr.add_firewall_rule(FirewallRule::new(
        NetworkProtocol::Tcp,
        "80,443",
        FirewallAction::Allow,
        "Web traffic",
    )?)?;

    // Allow DNS
    network_mgr.add_firewall_rule(FirewallRule::new(
        NetworkProtocol::Udp,
        "53",
        FirewallAction::Allow,
        "DNS resolution",
    )?)?;

    // Allow NTP
    network_mgr.add_firewall_rule(FirewallRule::new(
        NetworkProtocol::Udp,
        "123",
        FirewallAction::Allow,
        "Time synchronization",
    )?)?;

    // Block all other incoming
    network_mgr.add_firewall_rule(FirewallRule::new(
        NetworkProtocol::Tcp,
        "*",
        FirewallAction::Drop,
        "Block all other TCP",
    )?)?;
    network_mgr.add_firewall_rule(FirewallRule::new(
        NetworkProtocol::Udp,
        "*",
        FirewallAction::Drop,
        "Block all other UDP",
    )?)?;

    println!("   Configured {} firewall rules", network_mgr.firewall_rules.len());
    println!("   nftables config:\n{}", network_mgr.generate_nftables_config());

    // 2. Network Interface Configuration
    println!("\n2. Network Interface Configuration");
    let network_config = r#"
[Match]
Name=en*

[Network]
DHCP=yes
IPv6AcceptRA=yes

[DHCPv4]
UseDNS=yes
UseNTP=yes

[DHCPv6]
UseDNS=yes
UseNTP=yes
"#;
    println!("{}", network_config);

    // 3. Security Hardening
    println!("\n3. Security Hardening");
    let mut security_mgr = SecurityManager::new();
    security_mgr.enable_secure_boot()?;
    security_mgr.enable_tpm(TpmVersion::V2)?;
    security_mgr.enable_integrity_check()?;
    security_mgr.generate_secure_boot_config();
    println!("   Secure Boot: {}", security_mgr.secure_boot_state.as_str());
    println!("   TPM: {}", security_mgr.tpm_version.as_ref().unwrap().as_str());
    println!("   Security score: {}/100", security_mgr.calculate_security_score());

    // 4. Intrusion Detection
    println!("\n4. Intrusion Detection");
    let mut audit_mgr = AuditManager::new();
    audit_mgr.add_rule(AuditRule::new(
        "file_access",
        "/etc/passwd",
        "read",
        VulnerabilitySeverity::Medium,
    )?)?;
    audit_mgr.add_rule(AuditRule::new(
        "file_access",
        "/etc/shadow",
        "read",
        VulnerabilitySeverity::High,
    )?)?;
    audit_mgr.add_rule(AuditRule::new(
        "process_exec",
        "/usr/bin/sudo",
        "execute",
        VulnerabilitySeverity::Medium,
    )?)?;

    println!("   Audit rules: {}", audit_mgr.rules.len());
    println!("   Audit rules config:\n{}", audit_mgr.generate_audit_rules());

    // 5. VPN Configuration
    println!("\n5. VPN Configuration");
    let vpn_config = r#"
[Interface]
PrivateKey = <your-private-key>
Address = 10.0.0.2/32
DNS = 1.1.1.1
DNS = 8.8.8.8

[Peer]
PublicKey = <server-public-key>
Endpoint = vpn.example.com:51820
AllowedIPs = 0.0.0.0/0
PersistentKeepalive = 25
"#;
    println!("{}", vpn_config);

    // 6. Generate Complete Security Report
    println!("\n6. Security Report");
    let security_report = security_mgr.generate_security_report();
    println!("{}", security_report);

    println!("\n=== Network Security Configuration Complete ===");
    println!("\nTo apply these settings:");
    println!("1. Copy nftables rules to /etc/nftables.conf");
    println!("2. Copy network config to /etc/systemd/network/");
    println!("3. Enable SigmaOS security services");
    println!("4. Configure VPN if needed");
    println!("5. Reboot or reload services");

    Ok(())
}
