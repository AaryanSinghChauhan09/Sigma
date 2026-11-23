# SigmaOS

SigmaOS is a sovereign Linux distribution with a comprehensive Rust-based system management library and suite of command-line tools. Built with modular architecture and community-oriented development, SigmaOS provides a complete operating system ecosystem with programmatic system management.

## What Is SigmaOS?

SigmaOS consists of two main components:

1. **System Library** - A comprehensive Rust library (600+ tests, 159 modules) for system management
2. **Distribution Tools** - ISO build, installer, and system management tools

## Features

### System Library (sigma-system)

159 modules covering all major Linux subsystems:

- **Process/Kernel**: process, cpu_ops, mem_ops, kernel, scheduler_ops
- **Storage**: disk, fs_ops, mount_ops, disk_space_ops, swap_ops, raid_ops, lvm_ops
- **Network**: net_ops, net_stats, dns_ops, firewall_ops, proxy_ops
- **System**: sys_info, thermal, power_status, io_stats, resource_monitor, sensor_ops
- **Services**: service, systemd_ops, cron_ops, timer_ops, session_ops
- **Security**: firewall_ops, cert_ops, user_group_ops, security_audit
- **Desktop**: display_ops, audio_ops, keyboard_ops, locale_ops, gpu_ops, fingerprint_ops, smartcard_ops, sound_ops
- **Backup**: backup_ops, archive_ops, logrotate_ops
- **Config**: timezone_ops, hostname_ops, env_ops, file_watcher
- **Virtualization**: container_ops, virt_ops, uefi_ops
- **Package**: pkg_install, repo_ops, patch_ops, pkgmgr_ops, repomgr_ops
- **Boot**: kernel_ops, bootloader_ops, initramfs_ops
- **Config**: sysconfig_ops
- **Enterprise**: tco_ops, compliance_ops, incident_ops, capacity_ops, inventory_ops, disaster_ops, asset_ops, lifecycle_ops, drift_ops, change_ops, metrics_ops, profile_ops, mesh_ops, cluster_ops, platform_ops, cloud_ops
- **Media**: email_ops, notify_ops, clipboard_ops, screenshot_ops, record_ops
- **VPN**: vpn_ops
- **Hardware**: hardware_ops, gpu_ops, fingerprint_ops, smartcard_ops, sensor_ops
- **Input**: keyboard_ops, joystick_ops, gamepad_ops, touchscreen_ops, tablet_ops
- **Media Devices**: sound_ops, microphone_ops, webcam_ops, camera_ops, printer_ops, scanner_ops
- **Connectivity**: bluetooth_ops, bandwidth_ops, netmgr_ops
- **Container**: container_ops, registry_ops
- **Process Tree**: proc_tree_ops
- **Project**: project_ops, task_ops
- **CI/CD**: cicd_ops
- **Documentation**: doc_ops
- **Logging**: logger_ops
- **Services**: servicemgr_ops
- **System Update**: sysupdate_ops
- **Storage Manager**: storagemgr_ops
- **User Manager**: usermgr_ops
- **Log Manager**: logmgr_ops
- **Backup Manager**: backupmgr_ops
- **Audit Manager**: auditmgr_ops
- **Release Manager**: releasemgr_ops

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

- **build-iso-archiso.sh** - Uses archiso for proper ISO creation
- **build-iso-improved.sh** - Alternative with mkarchiso support
- **install-sigmaos.sh** - Text-based installer

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
# On Linux-based systems
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

- **Base**: Linux packages
- **Kernel**: Linux
- **Init System**: systemd
- **Package Manager**: sigma-pkg
- **Bootloader**: GRUB (UEFI)
- **Library**: Rust (sigma-system crate)
- **Configuration**: /etc/sigmaos/

## Comparison with Other Distros

| Feature | SigmaOS | Other Distros |
|---------|---------|---------------|
| Base | Linux packages | Various |
| ISO Build | archiso-based | Various |
| Installer | Text-based | Various |
| CLI Tool | Yes (sigmaos) | Varies |
| System Library | Rust (159 modules) | None |
| Config Tools | sigmaos-config | Manual |
| Release Management | sigmaos-release | Manual |
| Performance Profiles | Yes | Manual |
| Desktop Support | Full docs | Varies |
| First Boot Wizard | Yes | No |

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

- **Library**: 159 modules, 600+ unit tests, 100+ doctests
- **Tools**: 14 system tools
- **Packages**: 3 metapackages (base, kde, gnome)
- **Config**: pacman.conf, sysctl, GRUB, KDE configs
- **Scripts**: ISO build (improved), installer (improved), testing, pipeline
- **Documentation**: Comprehensive guides
- **Version**: 0.5.0

## ⚠️ Important: SigmaOS is NOT Yet Bootable

SigmaOS has complete infrastructure but requires a **Linux system** to build and test the actual ISO:

```bash
# On a Linux system:
git clone https://github.com/AaryanSinghChauhan09/Sigma
cd Sigma
./scripts/sigmaos-pipeline full
# This will build library, packages, ISO, and test it
```

See [BOOTABLE_PLAN.md](BOOTABLE_PLAN.md) for detailed steps to make SigmaOS bootable.

## Technical Details

| Feature | SigmaOS |
|---------|---------|
| Base | Arch Linux packages |
| ISO Build | archiso via `scripts/build-iso-container.sh` (Docker) |
| Bootable ISO | ✅ Builds; live-boot verification in progress |
| Installer | `scripts/install-sigmaos.sh` |
| CLI Tool | Yes (14 tools) |
| System Library | Rust (159 modules, enterprise ops) |
| Config Tools | sigmaos-config |
| Release Management | sigmaos-release |
| Automation | sigmaos-pipeline |

**The canonical bootable ISO / live desktop lives in [SigmaOS-ISO](https://github.com/AaryanSinghChauhan09/SigmaOS-ISO)** — an Arch-based live system with Hyprland + Waybar and published releases. This repo holds the component libraries, CLI tools, and ISO build scripts; `scripts/build-iso-archiso.sh` produces a bootable archiso image (BIOS + UEFI) verified in QEMU.

## License

GPL-3.0-or-later

## Contact

- **Maintainer**: Aaryan Singh Chauhan
- **Email**: aaryan.singh.chauhan.09@gmail.com
- **GitHub**: [AaryanSinghChauhan09/Sigma](https://github.com/AaryanSinghChauhan09/Sigma)
