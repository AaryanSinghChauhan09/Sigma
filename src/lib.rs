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

/// Thermal monitoring module.
/// Provides actual thermal monitoring using /sys/class/thermal.
/// Follows Single Responsibility Principle: only manages thermal operations.
pub mod thermal;

/// Power status module.
/// Provides actual battery and power info using /sys/class/power_supply.
/// Follows Single Responsibility Principle: only manages power status.
pub mod power_status;

/// Mount operations module.
/// Provides actual mount operations using /proc/mounts and mount/umount.
/// Follows Single Responsibility Principle: only manages mount operations.
pub mod mount_ops;

/// Process tree module.
/// Provides actual process tree information using /proc.
/// Follows Single Responsibility Principle: only manages process tree.
pub mod proc_tree;

/// Network statistics module.
/// Provides actual network interface stats using /sys/class/net.
/// Follows Single Responsibility Principle: only manages network stats.
pub mod net_stats;

/// I/O statistics module.
/// Provides actual disk I/O stats using /proc/diskstats.
/// Follows Single Responsibility Principle: only manages I/O stats.
pub mod io_stats;

/// Timezone operations module.
/// Provides actual timezone operations using timedatectl.
/// Follows Single Responsibility Principle: only manages timezone operations.
pub mod timezone_ops;

/// Hostname operations module.
/// Provides actual hostname operations using hostnamectl.
/// Follows Single Responsibility Principle: only manages hostname operations.
pub mod hostname_ops;

/// Environment variables module.
/// Provides actual environment variable operations.
/// Follows Single Responsibility Principle: only manages environment variables.
pub mod env_ops;

/// Systemd operations module.
/// Provides actual systemd unit operations using systemctl.
/// Follows Single Responsibility Principle: only manages systemd operations.
pub mod systemd_ops;

/// DNS operations module.
/// Provides actual DNS configuration using resolv.conf and resolvectl.
/// Follows Single Responsibility Principle: only manages DNS operations.
pub mod dns_ops;

/// File watcher module.
/// Provides actual file/directory change monitoring.
/// Follows Single Responsibility Principle: only manages file watching.
pub mod file_watcher;

/// Resource monitor module.
/// Provides actual system resource monitoring using /proc and /sys.
/// Follows Single Responsibility Principle: only manages resource monitoring.
pub mod resource_monitor;

/// Cron operations module.
/// Provides actual cron job operations using crontab.
/// Follows Single Responsibility Principle: only manages cron operations.
pub mod cron_ops;

/// Timer operations module.
/// Provides actual systemd timer operations using systemctl.
/// Follows Single Responsibility Principle: only manages timer operations.
pub mod timer_ops;

/// Session operations module.
/// Provides actual user session operations using loginctl.
/// Follows Single Responsibility Principle: only manages session operations.
pub mod session_ops;

/// Firewall operations module.
/// Provides actual firewall operations using iptables and nftables.
/// Follows Single Responsibility Principle: only manages firewall operations.
pub mod firewall_ops;

/// Proxy operations module.
/// Provides actual proxy configuration operations.
/// Follows Single Responsibility Principle: only manages proxy operations.
pub mod proxy_ops;

/// Backup operations module.
/// Provides actual backup operations using rsync and tar.
/// Follows Single Responsibility Principle: only manages backup operations.
pub mod backup_ops;

/// User group operations module.

/// Certificate operations module.
/// Provides actual SSL/TLS certificate operations using openssl.
/// Follows Single Responsibility Principle: only manages certificate operations.
pub mod cert_ops;

/// Archive operations module.
/// Provides actual archive operations using tar and zip.
/// Follows Single Responsibility Principle: only manages archive operations.
pub mod archive_ops;

/// Log rotation module.
/// Provides actual log rotation operations using logrotate.
/// Follows Single Responsibility Principle: only manages log rotation.
pub mod logrotate_ops;

/// Disk space operations module.
/// Provides actual disk space operations using df and du.
/// Follows Single Responsibility Principle: only manages disk space operations.
pub mod disk_space_ops;

/// UEFI operations module.
/// Provides actual UEFI operations using efibootmgr.
/// Follows Single Responsibility Principle: only manages UEFI operations.
pub mod uefi_ops;

/// Repository operations module.
/// Provides actual package repository operations.
/// Follows Single Responsibility Principle: only manages repository operations.
pub mod repo_ops;

/// Scheduler operations module.
/// Provides actual CPU scheduler operations.
/// Follows Single Responsibility Principle: only manages scheduler operations.
pub mod scheduler_ops;

/// Swap operations module.
/// Provides actual swap management operations.
/// Follows Single Responsibility Principle: only manages swap operations.
pub mod swap_ops;

/// Keyboard operations module.
/// Provides actual keyboard and input device operations.
/// Follows Single Responsibility Principle: only manages keyboard operations.
pub mod keyboard_ops;

/// Locale operations module.
/// Provides actual locale management operations.
/// Follows Single Responsibility Principle: only manages locale operations.
pub mod locale_ops;

/// Display manager operations module.
/// Provides actual display manager operations.
/// Follows Single Responsibility Principle: only manages display operations.
pub mod display_ops;

/// Audio operations module.
/// Provides actual audio operations using PipeWire/PulseAudio.
/// Follows Single Responsibility Principle: only manages audio operations.
pub mod audio_ops;

/// Container operations module.
/// Provides actual container operations using Podman/Docker.
/// Follows Single Responsibility Principle: only manages container operations.
pub mod container_ops;

/// User group operations module.
/// Provides actual user and group operations using useradd/groupadd.
/// Follows Single Responsibility Principle: only manages user/group operations.
pub mod user_group_ops;

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
pub use thermal::{ThermalMonitor, TemperatureReadings, ThermalZone, ThermalError};
pub use power_status::{PowerStatus, BatteryInfo, BatteryStatus as PowerBatteryStatus, PowerSupplyInfo, PowerError};
pub use mount_ops::{MountOps, MountInfo, MountError};
pub use proc_tree::{ProcTree, ProcessNode, ProcessTree, ProcTreeError};
pub use net_stats::{NetStats as IfaceStats, InterfaceStats, NetStatsError};
pub use io_stats::{IoStats, DiskIoStats, IoStatsError};
pub use timezone_ops::{TimezoneOps, TimezoneError};
pub use hostname_ops::{HostnameOps, HostnameError};
pub use env_ops::{EnvOps, EnvError};
pub use systemd_ops::{SystemdOps, SystemStatus, UnitStatus, SystemdError};
pub use dns_ops::{DnsOps, DnsError};
pub use file_watcher::{FileWatcher, FileEvent, FileEventType, FileWatcherError};
pub use resource_monitor::{ResourceMonitor, ResourceSnapshot, ResourceError};
pub use cron_ops::{CronOps, CronJob as CronJobOp, CronError as CronOpError};
pub use timer_ops::{TimerOps, TimerInfo, TimerError};
pub use session_ops::{SessionOps, SessionInfo, SessionError};
pub use firewall_ops::{FirewallOps as FwManager, FirewallRule as FwRule, FirewallBackend as FwBackend, FirewallError as FwManagerError};
pub use proxy_ops::{ProxyOps, ProxyConfig, ProxyError};
pub use backup_ops::{BackupOps as RealBackupOps, BackupError as RealBackupError};
pub use user_group_ops::{UserGroupOps, UserInfo, GroupInfo, UserGroupError};
pub use cert_ops::{CertOps, CertInfo, CertError};
pub use archive_ops::{ArchiveOps, ArchiveError};
pub use logrotate_ops::{LogrotateOps, LogrotateError};
pub use disk_space_ops::{DiskSpaceOps, DiskSpace, DiskSpaceError};
pub use uefi_ops::{UefiOps, BootEntry as UefiBootEntry, UefiError as UefiManagerError};
pub use repo_ops::{RepoOps, RepoInfo, RepoError as RepoManagerError};
pub use scheduler_ops::{SchedulerOps as CpuScheduler, SchedulerPolicy, SchedulerError as CpuSchedulerError};
pub use swap_ops::{SwapOps, SwapInfo, SwapError as SwapManagerError};
pub use keyboard_ops::{KeyboardOps, KeyboardError as KeymapError};
pub use locale_ops::{LocaleOps, LocaleError as SystemLocaleError};
pub use display_ops::{DisplayOps, DisplayError as DisplayManagerError};
pub use audio_ops::{AudioOps, AudioSink, AudioError as AudioManagerError};
pub use container_ops::{ContainerOps as CtnManager, ContainerInfo, ContainerError as ContainerManagerError};
/// Certificate operations module.
/// Provides actual SSL/TLS certificate operations using openssl.
/// Follows Single Responsibility Principle: only manages certificate operations.

/// Convenience function to create a PersonaManager with fail-fast validation
pub fn create_persona_manager(initial: SystemPersona) -> PersonaManager {
    PersonaManager {
        current: SystemPersona::validate(initial),
    }
}
