# SigmaOS Init System Module

## Overview
The init system module provides systemd service configuration, unit management, and target dependencies. It is inspired by systemd, the init system used by most modern Linux distributions including Arch Linux.

## Features
- **Service Units**: Generate systemd service unit files
- **Target Management**: Configure default targets (basic, multi-user, graphical)
- **Service Dependencies**: Manage After, Requires, WantedBy dependencies
- **Environment Variables**: Configure service environment
- **Enable/Disable**: Generate service enable/disable scripts

## Usage

```rust
use sigma_system::{InitManager, ServiceUnit, TargetType};

let mut mgr = InitManager::new(TargetType::Graphical);

let nginx = ServiceUnit::new("nginx", "Nginx Web Server")
    .with_exec_start("/usr/bin/nginx -g 'daemon off;'")
    .with_wanted_by("multi-user.target")
    .with_after("network.target");
mgr.add_service(nginx).unwrap();

mgr.enable_service("nginx").unwrap();
let units = mgr.generate_all_units();
```

## API Reference

### Methods
- `add_service()`: Add a service unit
- `enable_service()`: Enable a service
- `generate_all_units()`: Generate all systemd unit files
- `generate_default_target_link()`: Generate default target symlink

## Integration
Integrates with Boot, Network, Filesystem, and Security modules for service ordering.
