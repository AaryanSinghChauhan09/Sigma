//! Sigma System Core Library
//! Principles: Single Responsibility, High Cohesion, Low Coupling, Defensive Programming,
//! Demeter's Law, Clear Abstraction, Fail-Fast System
//!
//! This library provides the core substrate for the SigmaOS ecosystem.
//! Inspired by Linux kernel mintsystem, Arch Linux modularity, and Omarchy Linux sovereignty.

#![forbid(unsafe_code)]

/// Modular system adjustments module.
/// Handles CPU governors, sysctl memory tuning, and hardware optimization.
/// Follows Single Responsibility Principle: only manages system adjustment parameters.
/// Applies Demeter's Law: internal state encapsulated, minimal public API.
pub mod adjustments;

/// Modular persona management module.
/// Enables sovereign desktop personas and overlay environments.
/// Follows Single Responsibility Principle: only manages persona state.
/// Applies Clear Abstraction: well-defined public interfaces for persona operations.
pub mod persona;

/// Memory management module.
/// Provides cgroups v2 control, memory pressure monitoring, and OOM prevention.
/// Follows Single Responsibility Principle: only manages memory subsystem.
pub mod memory;

/// Network management module.
/// Provides network configuration, firewall rules, and connection tracking.
/// Follows Single Responsibility Principle: only manages network subsystem.
pub mod network;

/// Security module.
/// Provides secure boot verification, TPM 2.0 integration, and integrity checks.
/// Follows Single Responsibility Principle: only manages security subsystem.
pub mod security;

/// Filesystem module.
/// Provides filesystem management, ZFS/Btrfs support, and snapshot operations.
/// Follows Single Responsibility Principle: only manages filesystem subsystem.
pub mod filesystem;

/// Power management module.
/// Provides power control, battery optimization, and energy efficiency tuning.
/// Follows Single Responsibility Principle: only manages power subsystem.
pub mod power;

/// Monitoring module.
/// Provides system metrics, performance monitoring, and health checks.
/// Follows Single Responsibility Principle: only manages monitoring subsystem.
pub mod monitoring;

/// Boot configuration module.
/// Provides bootloader configuration (GRUB, systemd-boot) and kernel parameters.
/// Follows Single Responsibility Principle: only manages boot subsystem.
pub mod boot;

/// Init system module.
/// Provides systemd service configuration and unit management.
/// Follows Single Responsibility Principle: only manages init subsystem.
pub mod init;

/// Container runtime module.
/// Provides lightweight container runtime for virtualization.
/// Follows Single Responsibility Principle: only manages container subsystem.
pub mod container;

/// Security audit module.
/// Provides vulnerability scanning and compliance checking.
/// Follows Single Responsibility Principle: only manages audit subsystem.
pub mod audit;

/// Package management module.
/// Provides dependency resolution and package management.
/// Follows Single Responsibility Principle: only manages package subsystem.
pub mod package;

/// Hardware detection module.
/// Provides hardware enumeration and driver management.
/// Follows Single Responsibility Principle: only manages hardware subsystem.
pub mod hardware;

/// Logging module.
/// Provides structured logging for all SigmaOS components.
/// Follows Single Responsibility Principle: only manages logging subsystem.
pub mod logging;

/// Time synchronization module.
/// Provides NTP configuration, timezone management, and clock sync.
/// Follows Single Responsibility Principle: only manages time subsystem.
pub mod time_sync;

/// User management module.
/// Provides user account management, group management, and permissions.
/// Follows Single Responsibility Principle: only manages user subsystem.
pub mod user;

/// Localization module.
/// Provides locale management, internationalization, and timezone support.
/// Follows Single Responsibility Principle: only manages locale subsystem.
pub mod locale;

/// Cron module.
/// Provides scheduled task management and cron job control.
/// Follows Single Responsibility Principle: only manages cron subsystem.
pub mod cron;

/// Backup module.
/// Provides backup, restore, and disaster recovery management.
/// Follows Single Responsibility Principle: only manages backup subsystem.
pub mod backup;

/// Update module.
/// Provides system update management and rolling release support.
/// Follows Single Responsibility Principle: only manages update subsystem.
pub mod update;

/// Desktop module.
/// Provides desktop environment, display server, and window manager management.
/// Follows Single Responsibility Principle: only manages desktop subsystem.
pub mod desktop;

/// Virtualization module.
/// Provides virtual machine and container management.
/// Follows Single Responsibility Principle: only manages virtualization subsystem.
pub mod virtualization;

/// Printing module.
/// Provides printer management and print queue control.
/// Follows Single Responsibility Principle: only manages printing subsystem.
pub mod printing;

/// Bluetooth module.
/// Provides Bluetooth device management and pairing.
/// Follows Single Responsibility Principle: only manages Bluetooth subsystem.
pub mod bluetooth;

/// USB module.
/// Provides USB device enumeration and management.
/// Follows Single Responsibility Principle: only manages USB subsystem.
pub mod usb;

/// Scheduler module.
/// Provides process scheduling, CPU affinity, and priority management.
/// Follows Single Responsibility Principle: only manages scheduler subsystem.
pub mod scheduler;

/// Process module.
/// Provides actual process management using /proc filesystem.
/// Follows Single Responsibility Principle: only manages process subsystem.
pub mod process;

/// Filesystem operations module.
/// Provides actual filesystem operations using Rust fs APIs.
/// Follows Single Responsibility Principle: only manages filesystem operations.
pub mod fs_ops;

/// Network operations module.
/// Provides actual network interface management using sysfs.
/// Follows Single Responsibility Principle: only manages network operations.
pub mod net_ops;

/// Memory operations module.
/// Provides actual memory information using /proc/meminfo.
/// Follows Single Responsibility Principle: only manages memory operations.
pub mod mem_ops;

/// CPU operations module.
/// Provides actual CPU information using /proc/cpuinfo and /proc/stat.
/// Follows Single Responsibility Principle: only manages CPU operations.
pub mod cpu_ops;

/// Package installation module.
/// Provides actual package installation using pacman/apt/dnf.
/// Follows Single Responsibility Principle: only manages package installation.
pub mod pkg_install;

/// Service management module.
/// Provides actual service control using systemctl.
/// Follows Single Responsibility Principle: only manages service operations.
pub mod service;

/// Kernel module management.
/// Provides actual kernel module control using modprobe/lsmod.
/// Follows Single Responsibility Principle: only manages kernel operations.
pub mod kernel;

/// Disk management module.
/// Provides actual disk management using lsblk, df, and parted.
/// Follows Single Responsibility Principle: only manages disk operations.
pub mod disk;

/// Log operations module.
/// Provides actual log management using journalctl.
/// Follows Single Responsibility Principle: only manages logging operations.
pub mod log_ops;

/// System information module.
/// Provides actual system info using /proc and /sys.
/// Follows Single Responsibility Principle: only manages system info.
pub mod sys_info;

/// Public re-exports for convenient access following Demeter's Law.
pub use adjustments::{SystemAdjuster, GovernorMode};
pub use persona::{SystemPersona, PersonaManager};
pub use memory::{MemoryManager, CgroupConfig, MemoryPressureLevel};
pub use network::{NetworkManager, FirewallRule, NetworkProtocol, ConnectionState};
pub use security::{SecurityManager, SecureBootState, TpmVersion, IntegrityCheck};
pub use filesystem::{FileSystemManager, FileSystemType, Snapshot, MountOption};
pub use power::{PowerManager, BatteryStatus, PowerProfile, TdpLevel};
pub use monitoring::{MonitoringManager, SystemMetric, MetricType, HealthStatus};
pub use boot::{BootManager, BootLoaderType, KernelParams, BootEntry};
pub use init::{InitManager, ServiceUnit, ServiceState, TargetType};
pub use container::{ContainerManager, ContainerSpec, Container, ContainerRuntime, ContainerStatus};
pub use audit::{AuditManager, Vulnerability, AuditRule, AuditResult, VulnerabilitySeverity, AuditStatus};
pub use package::{PackageManager, Package, PackageVersion, PackageError};
pub use hardware::{HardwareManager, DeviceInfo, DeviceType, DeviceStatus, HardwareError};
pub use logging::{Logger, LogEntry, LogLevel, LogError};
pub use time_sync::{TimeManager, NtpServer, TimeSource, TimeError};
pub use user::{UserManager, User, Group, UserError};
pub use locale::{LocaleManager, Locale, Timezone, Translation, LocaleError};
pub use cron::{CronManager, CronJob, CronError};
pub use backup::{BackupManager, Backup, BackupType, BackupStatus, BackupSchedule, BackupError};
pub use update::{UpdateManager, Update, UpdateType, UpdateStatus, UpdateError};
pub use desktop::{DesktopManager, DesktopSession, DesktopEnvironment, DisplayServer, WindowManager, DesktopError};
pub use virtualization::{VirtualizationManager, Vm, VmConfig, VmStatus, ContainerConfig, VirtualizationError};
pub use printing::{PrintManager, Printer, PrinterType, PrinterStatus, PrintJob, JobStatus, PrintError};
pub use bluetooth::{BluetoothManager, BluetoothDevice, BluetoothAdapter, BluetoothDeviceType, BluetoothDeviceStatus, BluetoothError};
pub use usb::{UsbManager, UsbDevice, UsbHub, UsbDeviceType, UsbDeviceStatus, UsbError};
pub use scheduler::{Scheduler, ProcessInfo, Priority, SchedulingPolicy, SchedulerError};
pub use process::{ProcessManager, ProcessInfo as ProcInfo, ProcessError};
pub use fs_ops::{FsOps, DiskUsage, FileInfo, FsError};
pub use net_ops::{NetOps, InterfaceInfo, InterfaceType, NetStats, NetError};
pub use mem_ops::{MemOps, MemoryInfo, MemoryPressure, MemError};
pub use cpu_ops::{CpuOps, CpuInfo, CpuUsage, CpuError};
pub use pkg_install::{PackageInstaller, PackageInfo, InstallError};
pub use service::{ServiceManager, ServiceInfo, ServiceStatus, ServiceError};
pub use kernel::{KernelManager, ModuleInfo, ModuleState, KernelInfo, KernelError};
pub use disk::{DiskManager, DiskInfo, PartitionInfo, FilesystemInfo, DiskError};
pub use log_ops::{LogOps, LogEntry as SysLogEntry, LogLevel as SysLogLevel, LogError as SysLogError};
pub use sys_info::{SysInfo, SystemInfo, SysInfoError};

/// Convenience function to create a PersonaManager with fail-fast validation
pub fn create_persona_manager(initial: SystemPersona) -> PersonaManager {
    PersonaManager {
        current: SystemPersona::validate(initial),
    }
}