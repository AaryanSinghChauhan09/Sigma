# SigmaOS Boot Configuration Module

## Overview
The boot configuration module provides bootloader configuration (GRUB, systemd-boot) and kernel parameter management. It is inspired by GRUB and systemd-boot, essential for the SigmaOS ISO boot process.

## Features
- **Bootloader Support**: GRUB, systemd-boot, and Limine
- **Kernel Parameters**: Manage kernel command-line parameters
- **Boot Entries**: Configure multiple boot entries with different options
- **Configuration Generation**: Generate GRUB and systemd-boot configs
- **Default Management**: Set default boot entry and timeout

## Usage

```rust
use sigma_system::{BootManager, BootLoaderType, BootEntry, KernelParams};

// Create a boot manager
let mut mgr = BootManager::new(BootLoaderType::SystemdBoot)
    .with_timeout(5)
    .unwrap();

// Add boot entries
let main_entry = BootEntry::new("sigmaos", "SigmaOS 0.3.0", "/vmlinuz-linux", "/initramfs-linux.img")
    .with_params(KernelParams::new().with_quiet(true).with_splash(true))
    .set_default();
mgr.add_entry(main_entry).unwrap();

let fallback = BootEntry::new("sigmaos-fallback", "SigmaOS Fallback", "/vmlinuz-linux", "/initramfs-linux-fallback.img");
mgr.add_entry(fallback).unwrap();

// Generate configurations
let grub_config = mgr.generate_grub_config();
let systemd_config = mgr.generate_systemd_boot_config();
let cmdline = mgr.generate_kernel_cmdline();
```

## Configuration

### BootLoaderType
- `Grub`: GRUB bootloader (legacy and UEFI)
- `SystemdBoot`: systemd-boot (UEFI only)
- `Limine`: Limine bootloader

### KernelParams
- `quiet`: Suppress kernel messages
- `splash`: Show splash screen
- `nomodeset`: Disable kernel mode setting
- `single`: Boot to single-user mode
- `custom`: Custom kernel parameters

### BootEntry
- `id`: Entry identifier
- `title`: Display title
- `kernel`: Path to kernel image
- `initrd`: Path to initramfs
- `params`: Kernel parameters
- `is_default`: Set as default entry

## API Reference

### Methods
- `add_entry()`: Add a boot entry
- `set_default()`: Set default boot entry
- `generate_grub_config()`: Generate GRUB configuration
- `generate_systemd_boot_config()`: Generate systemd-boot config
- `generate_kernel_cmdline()`: Generate kernel command line

## Examples

### GRUB Configuration
```rust
let mut mgr = BootManager::new(BootLoaderType::Grub)
    .with_timeout(5)
    .unwrap();

let entry = BootEntry::new("sigmaos", "SigmaOS", "/boot/vmlinuz", "/boot/initramfs")
    .with_params(KernelParams::new()
        .with_quiet(true)
        .with_splash(true)
        .with_custom(" mitigations=off"));
mgr.add_entry(entry).unwrap();

let grub_cfg = mgr.generate_grub_config();
std::fs::write("/boot/grub/grub.cfg", grub_cfg)?;
```

### systemd-boot Configuration
```rust
let mut mgr = BootManager::new(BootLoaderType::SystemdBoot)
    .with_timeout(3)
    .unwrap();

let entry = BootEntry::new("sigmaos", "SigmaOS", "/vmlinuz-linux", "/initramfs-linux.img");
mgr.add_entry(entry).unwrap();

let loader_conf = mgr.generate_systemd_boot_config();
std::fs::write("/boot/loader/loader.conf", &loader_conf.lines().take(3).collect::<Vec<_>>().join("\n"))?;
```

### Kernel Parameters
```rust
// Standard parameters
let params = KernelParams::new()
    .with_quiet(true)
    .with_splash(true);

// Recovery mode
let recovery = KernelParams::new()
    .with_quiet(false)
    .with_single(true);

// Custom parameters
let custom = KernelParams::new()
    .with_custom("rd.lvm.lv=vg/root")
    .with_custom("rd.lvm.lv=vg/swap");
```

## Boot Configuration Files

### GRUB Configuration Structure
```
set timeout=5
set default=sigmaos

menuentry "SigmaOS 0.3.0" {
    linux /boot/vmlinuz-linux quiet splash
    initrd /boot/initramfs-linux.img
}
```

### systemd-boot Configuration Structure
```
# loader.conf
default sigmaos
timeout 5
editor 0

# entries/sigmaos.conf
title SigmaOS 0.3.0
linux /vmlinuz-linux
initrd /initramfs-linux.img
options quiet splash
```

## Integration with Other Modules
The boot module integrates with:
- **Security module**: Secure boot verification
- **Filesystem module**: Root filesystem mounting
- **Init module**: Default target selection
- **ISO build script**: Generates boot configurations for ISO

## Principles Applied
- Single Responsibility: Only manages boot subsystem
- Fail-Fast: Validates kernel and initrd paths
- Clear Abstraction: Well-defined public API
- AAA Testing: All tests follow Arrange-Act-Assert pattern
