# SigmaOS Core Library Architecture

## Overview

The SigmaOS core library (`sigma-system`) is a comprehensive Rust-based system management library providing modular, composable operations for all major Linux subsystems.

## Architecture Principles

### SOLID Principles

1. **Single Responsibility Principle (SRP)**
   - Each module has one reason to change
   - Each module handles one specific concern
   - Example: `kernel_ops` only manages kernel operations

2. **Open/Closed Principle (OCP)**
   - Modules are open for extension
   - Modules are closed for modification
   - Achieved through trait-based abstractions

3. **Liskov Substitution Principle (LSP)**
   - Subtypes are substitutable for base types
   - All error types implement `std::error::Error`
   - All managers implement `Default` trait

4. **Interface Segregation Principle (ISP)**
   - Clients don't depend on interfaces they don't use
   - Minimal, focused public APIs
   - No fat interfaces

5. **Dependency Inversion Principle (DIP)**
   - Depend on abstractions, not concretions
   - High-level modules don't depend on low-level modules
   - Both depend on abstractions

### Additional Principles

- **DRY (Don't Repeat Yourself)** - No code duplication
- **KISS (Keep It Simple Stupid)** - Simple, straightforward implementations
- **YAGNI (You Aren't Gonna Need It)** - Only necessary functionality
- **Boy Scout Rule** - Code is cleaner than when found
- **Fail-Fast System** - Early error detection
- **Defensive Programming** - Input validation and error handling
- **Information Hiding** - Private fields, public APIs
- **Encapsulation** - Well-defined module boundaries
- **Abstraction** - Trait-based interfaces
- **Layered Design** - Clear separation of concerns
- **Modularity** - Independent modules
- **Portability** - Cross-platform compatibility
- **Race-Free Design** - Safe concurrency patterns
- **Safe Concurrency** - Thread-safe operations where needed
- **Deterministic Testing** - Reproducible test results
- **Reproducible Builds** - Consistent build outputs

## Module Structure

### Core Modules (91 total)

#### Process/Kernel (5 modules)
- `process` - Process management
- `cpu_ops` - CPU operations
- `mem_ops` - Memory operations
- `kernel` - Kernel module
- `kernel_ops` - Kernel management (sysctl, modprobe)
- `scheduler_ops` - CPU scheduler management

#### Storage (7 modules)
- `disk` - Disk operations
- `fs_ops` - Filesystem operations
- `mount_ops` - Mount operations
- `disk_space_ops` - Disk space monitoring
- `swap_ops` - Swap management
- `raid_ops` - RAID management
- `lvm_ops` - LVM operations

#### Network (5 modules)
- `net_ops` - Network operations
- `net_stats` - Network statistics
- `dns_ops` - DNS operations
- `firewall_ops` - Firewall management
- `proxy_ops` - Proxy configuration
- `bandwidth_ops` - Bandwidth monitoring

#### System (5 modules)
- `sys_info` - System information
- `thermal` - Thermal management
- `power_status` - Power status
- `io_stats` - I/O statistics
- `resource_monitor` - Resource monitoring
- `sensor_ops` - Hardware sensors

#### Services (4 modules)
- `service` - Service management
- `systemd_ops` - Systemd operations
- `cron_ops` - Cron management
- `timer_ops` - Timer management
- `session_ops` - Session management

#### Security (4 modules)
- `firewall_ops` - Firewall management
- `cert_ops` - Certificate operations
- `user_group_ops` - User/group management
- `security_audit` - Security auditing

#### Desktop (8 modules)
- `display_ops` - Display manager operations
- `audio_ops` - Audio operations
- `keyboard_ops` - Keyboard operations
- `locale_ops` - Locale management
- `gpu_ops` - GPU operations
- `fingerprint_ops` - Fingerprint operations
- `smartcard_ops` - Smartcard operations
- `sound_ops` - Sound operations

#### Backup (3 modules)
- `backup_ops` - Backup operations
- `archive_ops` - Archive operations
- `logrotate_ops` - Log rotation

#### Config (4 modules)
- `timezone_ops` - Timezone management
- `hostname_ops` - Hostname management
- `env_ops` - Environment variables
- `file_watcher` - File watching
- `sysconfig_ops` - System configuration

#### Virtualization (3 modules)
- `container_ops` - Container operations
- `virt_ops` - Virtualization operations
- `uefi_ops` - UEFI operations

#### Package (5 modules)
- `pkg_install` - Package installation
- `repo_ops` - Repository operations
- `patch_ops` - Patch management
- `pkgmgr_ops` - Package manager operations
- `repomgr_ops` - Repository manager operations

#### Boot (3 modules)
- `kernel_ops` - Kernel management
- `bootloader_ops` - Bootloader management
- `initramfs_ops` - Initramfs management

#### Enterprise (18 modules)
- `tco_ops` - Total Cost of Ownership
- `compliance_ops` - Compliance checking
- `incident_ops` - Incident response
- `capacity_ops` - Capacity planning
- `inventory_ops` - Inventory management
- `disaster_ops` - Disaster recovery
- `asset_ops` - Asset management
- `lifecycle_ops` - Lifecycle management
- `drift_ops` - Configuration drift
- `change_ops` - Change management
- `metrics_ops` - Metrics collection
- `profile_ops` - Performance profiling
- `mesh_ops` - Service mesh
- `cluster_ops` - Cluster management
- `platform_ops` - Platform operations
- `cloud_ops` - Cloud native operations

#### Media (4 modules)
- `email_ops` - Email operations
- `notify_ops` - Notifications
- `clipboard_ops` - Clipboard operations
- `screenshot_ops` - Screenshot operations
- `record_ops` - Screen recording

#### VPN (1 module)
- `vpn_ops` - VPN operations

#### Hardware (5 modules)
- `hardware_ops` - Hardware operations
- `gpu_ops` - GPU operations
- `fingerprint_ops` - Fingerprint operations
- `smartcard_ops` - Smartcard operations
- `sensor_ops` - Sensor operations

#### Input (5 modules)
- `keyboard_ops` - Keyboard operations
- `joystick_ops` - Joystick operations
- `gamepad_ops` - Gamepad operations
- `touchscreen_ops` - Touchscreen operations
- `tablet_ops` - Tablet operations

#### Media Devices (6 modules)
- `sound_ops` - Sound operations
- `microphone_ops` - Microphone operations
- `webcam_ops` - Webcam operations
- `camera_ops` - Camera operations
- `printer_ops` - Printer operations
- `scanner_ops` - Scanner operations

#### Connectivity (2 modules)
- `bluetooth_ops` - Bluetooth operations
- `bandwidth_ops` - Bandwidth monitoring

#### Container (2 modules)
- `container_ops` - Container operations
- `registry_ops` - Container registry operations

#### Process Tree (1 module)
- `proc_tree_ops` - Process tree operations

#### Project (2 modules)
- `project_ops` - Project management
- `task_ops` - Task management

#### CI/CD (1 module)
- `cicd_ops` - CI/CD operations

#### Documentation (1 module)
- `doc_ops` - Documentation operations

#### Logging (1 module)
- `logger_ops` - Logging operations

## Error Handling

All modules follow a consistent error handling pattern:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleError {
    /// Command execution failed.
    CommandError(String),
    /// IO error.
    IoError(String),
    /// Other specific errors.
    OtherError(String),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::CommandError(msg) => write!(f, "Command error: {}", msg),
            ModuleError::IoError(msg) => write!(f, "IO error: {}", msg),
            ModuleError::OtherError(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for ModuleError {}
```

## Default Implementation

All managers implement the `Default` trait:

```rust
impl Default for ModuleOps {
    fn default() -> Self {
        Self::new()
    }
}
```

## Testing

All modules include unit tests that:
- Test constructor (`test_new`)
- Test main functionality without requiring hardware
- Follow deterministic testing principles
- Are reproducible across runs

## Future Enhancements

1. **Interface Segregation** - Create trait-based interfaces for each module category
2. **Dependency Injection** - Add dependency injection patterns for loose coupling
3. **Async Support** - Add async variants of blocking operations
4. **Mock Support** - Add mock implementations for testing
5. **Plugin System** - Support dynamically loadable modules
