# SigmaOS Hardware Detection Module

## Overview
Hardware enumeration and driver management inspired by Arch Linux hwinfo.

## Features
- Device detection (CPU, GPU, RAM, Storage, Network, Audio, Input, USB, PCI)
- Driver management and blacklisting
- lspci/lsusb output generation
- Hardware summary reports

## Usage

```rust
use sigma_system::{HardwareManager, DeviceInfo, DeviceType, DeviceStatus};

let mut mgr = HardwareManager::new();
mgr.detect_device(DeviceInfo::new("cpu0", DeviceType::Cpu, "Intel", "Core i7")
    .with_driver("intel")
    .with_status(DeviceStatus::Active)).unwrap();
let summary = mgr.get_hardware_summary();
```

## API
- `detect_device()`: Register device
- `get_devices_by_type()`: Filter by type
- `get_active_devices()`: Active devices only
- `get_devices_needing_drivers()`: Find devices without drivers
- `add_driver()`: Register available drivers
- `generate_lspci_output()`: lspci-style output
