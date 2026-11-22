# SigmaOS Network Management Module

## Overview
The network management module provides network configuration, firewall rules, and connection tracking. It is inspired by Linux kernel networking and Arch Linux security tools.

## Features
- **Firewall Management**: Configure iptables and nftables rules
- **Network Configuration**: Generate systemd-networkd configurations
- **Connection Tracking**: Support for ESTABLISHED, RELATED, NEW, INVALID states
- **IPv4/IPv6**: Dual-stack support with separate rule generation

## Usage

```rust
use sigma_system::{NetworkManager, FirewallRule, NetworkProtocol, ConnectionState};

// Create a network manager
let mut mgr = NetworkManager::new("eth0")
    .with_default_policy(sigma_system::FirewallAction::Drop);

// Add firewall rules
let ssh_rule = FirewallRule::new("ssh", sigma_system::FirewallAction::Accept)
    .with_protocol(NetworkProtocol::Tcp)
    .with_destination_port(22)
    .unwrap()
    .with_state(ConnectionState::New);
mgr.add_rule(ssh_rule).unwrap();

// Generate iptables script
let iptables = mgr.generate_iptables_script();
println!("{}", iptables);

// Generate nftables config
let nftables = mgr.generate_nftables_config();
println!("{}", nftables);
```

## Configuration

### FirewallRule
- `protocol`: TCP, UDP, ICMP, or All
- `source`: Source IP address (optional)
- `destination`: Destination IP address (optional)
- `source_port`: Source port (optional)
- `destination_port`: Destination port (optional)
- `action`: Accept, Drop, or Reject
- `state`: Connection state (optional)

### NetworkManager
- `interface`: Network interface name
- `rules`: Map of firewall rules
- `default_policy`: Default action for unmatched packets
- `ipv4_enabled`: Enable IPv4 rules
- `ipv6_enabled`: Enable IPv6 rules

## API Reference

### NetworkProtocol
- `Tcp`: TCP protocol
- `Udp`: UDP protocol
- `Icmp`: ICMP protocol
- `All`: All protocols

### ConnectionState
- `Established`: Established connections
- `Related`: Related connections
- `New`: New connections
- `Invalid`: Invalid packets

### FirewallAction
- `Accept`: Allow packet
- `Drop`: Silently drop packet
- `Reject`: Reject with ICMP response

## Examples

### Creating Firewall Rules
```rust
// Allow SSH
let ssh = FirewallRule::new("ssh", FirewallAction::Accept)
    .with_protocol(NetworkProtocol::Tcp)
    .with_destination_port(22)
    .unwrap();

// Allow HTTP/HTTPS
let http = FirewallRule::new("http", FirewallAction::Accept)
    .with_protocol(NetworkProtocol::Tcp)
    .with_destination_port(80)
    .unwrap();

let https = FirewallRule::new("https", FirewallAction::Accept)
    .with_protocol(NetworkProtocol::Tcp)
    .with_destination_port(443)
    .unwrap();
```

### Generating Configuration Files
```rust
// Save iptables rules
std::fs::write("/etc/iptables/rules.v4", mgr.generate_iptables_script())?;

// Save nftables rules
std::fs::write("/etc/nftables.conf", mgr.generate_nftables_config())?;

// Save networkd config
std::fs::write("/etc/systemd/network/10-eth0.network", mgr.generate_networkd_config())?;
```

## Integration with Other Modules
The network module integrates with:
- **Security module**: Firewall rules for secure boot
- **Monitoring module**: Network metrics (RX/TX)
- **Init module**: Network-dependent service ordering

## Principles Applied
- Single Responsibility: Only manages network subsystem
- Fail-Fast: Validates ports before adding rules
- Clear Abstraction: Well-defined public API
- AAA Testing: All tests follow Arrange-Act-Assert pattern
