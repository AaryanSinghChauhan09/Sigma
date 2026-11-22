# SigmaOS Time Synchronization Module

## Overview
NTP configuration, timezone management, and clock synchronization inspired by systemd-timesyncd and chrony.

## Features
- NTP server management
- Timezone configuration
- Sync interval and drift tolerance
- systemd-timesyncd, chrony, timedatectl config generation

## Usage

```rust
use sigma_system::{TimeManager, NtpServer};

let mut mgr = TimeManager::new("UTC").unwrap();
mgr.add_ntp_server(NtpServer::new("pool.ntp.org").unwrap()).unwrap();
let config = mgr.generate_timesyncd_config();
```

## API
- `TimeManager::new(timezone)`: Create manager
- `add_ntp_server()`: Add NTP server
- `set_sync_interval()`: Set sync interval
- `generate_timesyncd_config()`: systemd-timesyncd config
- `generate_chrony_config()`: chrony config
- `generate_timedatectl_commands()`: timedatectl commands
