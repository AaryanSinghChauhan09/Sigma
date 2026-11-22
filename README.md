# SigmaOS

SigmaOS is a Linux distribution built on Arch Linux with a comprehensive Rust-based system management library and suite of command-line tools. Inspired by Arch Linux and Omarchy Linux, SigmaOS provides a complete operating system ecosystem with programmatic system management.

## What Is SigmaOS?

SigmaOS consists of two main components:

1. **System Library** - A comprehensive Rust library (521+ tests, 75 modules) for system management
2. **Distribution Tools** - ISO build, installer, and system management tools

## Features

### System Library (sigma-system)

75 modules covering all major Linux subsystems:

- **Process/Kernel**: process, cpu_ops, mem_ops, kernel, scheduler_ops
- **Storage**: disk, fs_ops, mount_ops, disk_space_ops, swap_ops, raid_ops, lvm_ops
- **Network**: net_ops, net_stats, dns_ops, firewall_ops, proxy_ops
- **System**: sys_info, thermal, power_status, io_stats, resource_monitor, sensor_ops
- **Services**: service, systemd_ops, cron_ops, timer_ops, session_ops
- **Security**: firewall_ops, cert_ops, user_group_ops
- **Desktop**: display_ops, audio_ops, keyboard_ops, locale_ops, gpu_ops, fingerprint_ops, smartcard_ops, sound_ops
- **Backup**: backup_ops, archive_ops, logrotate_ops
- **Config**: timezone_ops, hostname_ops, env_ops, file_watcher
- **Virtualization**: container_ops, virt_ops, uefi_ops
- **Package**: pkg_install, repo_ops, patch_ops
- **Enterprise**: tco_ops, compliance_ops, incident_ops, capacity_ops, inventory_ops, disaster_ops, asset_ops, lifecycle_ops, drift_ops, change_ops, metrics_ops, profile_ops, mesh_ops, cluster_ops, platform_ops, cloud_ops
- **Media**: email_ops, notify_ops, clipboard_ops, screenshot_ops, record_ops
- **VPN**: vpn_ops

### Distribution Tools (14 tools)

**Core Management:**
- sigmaos - Main CLI tool
- sigmaos-config - System configuration
- sigmaos-service - Service manager

**Package Management:**
- sigmaos-pkg - Package manager wrapper
- sigmaos-build-pkg - Package build system
- sigmaos-mirror - Mirror configuration

**Updates & Releases:**
- sigmaos-update - System updates
- sigmaos-release - Release snapshots
- sigmaos-news - News feed

**Backup & Audit:**
- sigmaos-backup - Backup/restore
- sigmaos-audit - Security audit

**Installation:**
- sigmaos-firstboot - First-boot wizard
- sigmaos-chroot - Chroot tool

**Security:**
- sigmaos-keyring - GPG keyring

### ISO Build & Installation

- **build-iso-archiso.sh** - Uses Arch's archiso for proper ISO creation
- **build-iso-improved.sh** - Alternative with mkarchiso support
- **install-sigmaos.sh** - Text-based installer inspired by archinstall

## Quick Start

### Using the Library

```bash
cargo add sigma-system
```

```rust
use sigma_system::*;

fn main() {
    // System information
    let sys_info = SysInfo::new();
    let info = sys_info.get_system_info()?;
    println!("Hostname: {}", info.hostname);
    
    // Process management
    let proc_mgr = ProcessManager::new();
    let processes = proc_mgr.list_processes()?;
    
    // Service management
    let svc_mgr = ServiceOps::new();
    svc_mgr.start_service("NetworkManager")?;
}
```

### Using the CLI

```bash
# System information
sigmaos info --detailed

# Process management
sigmaos process list

# Service management
sigmaos service start NetworkManager

# Network operations
sigmaos network list

# Disk operations
sigmaos disk usage /

# System monitoring
sigmaos monitor cpu
```

### Building an ISO

```bash
# On Arch-based systems
./scripts/build-iso-archiso.sh

# Alternative method
./scripts/build-iso-improved.sh
```

### Installing SigmaOS

1. Boot the ISO
2. Run: `sudo /root/install-sigmaos.sh`
3. Follow the prompts
4. Reboot into your new system

### First Boot Setup

After installation, run the first boot wizard:

```bash
sudo sigmaos-firstboot
```

This will guide you through:
- Hostname configuration
- Locale and timezone
- User management
- Performance profile selection
- Desktop environment installation
- Service enablement

## System Tools

### sigmaos-config

```bash
sudo sigmaos-config init                    # Initialize configuration
sudo sigmaos-config apply performance       # Apply performance profile
sudo sigmaos-config apply desktop kde      # Apply desktop configuration
sigmaos-config status                       # Show system status
sigmaos-config report                       # Generate system report
```

### sigmaos-pkg

```bash
sudo sigmaos-pkg init                       # Initialize SigmaOS repository
sudo sigmaos-pkg add package.pkg.tar.zst   # Add package to repository
sigmaos-pkg list                            # List SigmaOS packages
sudo sigmaos-pkg install mypackage         # Install from SigmaOS repo
```

### sigmaos-service

```bash
sudo sigmaos-service init                   # Initialize service configuration
sudo sigmaos-service profile enable core   # Enable service profile
sigmaos-service profile list                # List service profiles
sigmaos-service status                      # Show service status
sudo sigmaos-service enable NetworkManager  # Enable service
```

### sigmaos-release

```bash
sudo sigmaos-release create                 # Create release snapshot
sigmaos-release list                        # List releases
sudo sigmaos-release set-current sigmaos-20250101
sigmaos-release rollback sigmaos-20250101  # Rollback to previous release
```

### sigmaos-update

```bash
sudo sigmaos-update full                    # Full system update
sudo sigmaos-update security               # Security-focused update
sudo sigmaos-update kernel                 # Kernel and bootloader update
sigmaos-update check                       # Check for updates
```

## Documentation

- [Installation Guide](INSTALL.md) - ISO build and installation instructions
- [Desktop Guide](DESKTOP.md) - Desktop environment setup
- [Tools Guide](TOOLS.md) - System tools documentation

## Architecture

SigmaOS is built on:

- **Base**: Arch Linux packages
- **Kernel**: Linux (same as Arch)
- **Init System**: systemd
- **Package Manager**: pacman (with SigmaOS extensions)
- **Bootloader**: GRUB (UEFI)
- **Library**: Rust (sigma-system crate)
- **Configuration**: /etc/sigmaos/

## Comparison with Other Distros

| Feature | SigmaOS | Arch | Omarchy |
|---------|---------|------|---------|
| Base | Arch packages | Arch | Arch |
| ISO Build | archiso-based | archiso | Custom |
| Installer | Text-based (archinstall-style) | Manual | Custom |
| CLI Tool | Yes (sigmaos) | pacman | Custom |
| System Library | Rust (54 modules) | None | None |
| Config Tools | sigmaos-config | Manual | sigma-config |
| Release Management | sigmaos-release | Manual | Manual |
| Performance Profiles | Yes | Manual | Yes |
| Desktop Support | Full docs | Manual | KDE/GNOME |
| First Boot Wizard | Yes | No | No |

## Development

### Building the CLI

```bash
cd cli
cargo build --release
sudo cp target/release/sigmaos /usr/local/bin/
```

### Running Tests

```bash
cargo test
```

### Installing Tools

```bash
sudo cp scripts/sigmaos-* /usr/local/bin/
sudo chmod +x /usr/local/bin/sigmaos-*
```

## Project Status

- **Library**: 75 modules, 521 unit tests, 89 doctests
- **Tools**: 14 system tools
- **Packages**: 3 metapackages (base, kde, gnome)
- **Config**: pacman.conf, sysctl, GRUB, KDE configs
- **Scripts**: ISO build (improved), installer (improved), testing, pipeline
- **Documentation**: Comprehensive guides
- **Version**: 0.5.0

## ⚠️ Important: SigmaOS is NOT Yet Bootable

SigmaOS has complete infrastructure but requires an **Arch Linux system** to build and test the actual ISO:

```bash
# On an Arch system:
git clone https://github.com/AaryanSinghChauhan09/Sigma
cd Sigma
./scripts/sigmaos-pipeline full
# This will build library, packages, ISO, and test it
```

See [BOOTABLE_PLAN.md](BOOTABLE_PLAN.md) for detailed steps to make SigmaOS bootable.

## Comparison with Arch/Omarchy

| Feature | SigmaOS | Arch | Omarchy |
|---------|---------|------|---------|
| Base | Arch packages | Arch | Arch |
| ISO Build | archiso scripts (improved, not executed) | archiso | Custom |
| Bootable ISO | ❌ Not built | ✅ Available | ✅ Available |
| Installer | Text-based (improved, not tested) | Manual | Custom |
| CLI Tool | Yes (14 tools) | pacman | Custom |
| System Library | Rust (72 modules, enterprise ops) | None | None |
| Config Tools | sigmaos-config | Manual | sigma-config |
| Release Management | sigmaos-release | Manual | Manual |
| Automation | sigmaos-pipeline | Manual | Manual |

**SigmaOS has better tooling than minimal Arch, but without building/testing the ISO, it remains infrastructure, not a bootable OS.**

## License

GPL-3.0-or-later

## Contact

- **Maintainer**: Aaryan Singh Chauhan
- **Email**: aaryan.singh.chauhan.09@gmail.com
- **GitHub**: [AaryanSinghChauhan09/Sigma](https://github.com/AaryanSinghChauhan09/Sigma)
