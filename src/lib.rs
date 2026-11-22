//! Sigma System Core Library
//! Principles: Single Responsibility, High Cohesion, Low Coupling, Defensive Programming,
//! Demeter's Law, Clear Abstraction, Fail-Fast System
//!
//! This library provides the core substrate for the SigmaOS ecosystem.
//! Built with modular architecture, sovereign design, and community-oriented development.

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

/// Sound operations module.
/// Provides actual sound operations via pactl.
/// Follows Single Responsibility Principle: only manages sound operations.
pub mod sound_ops;

/// Bandwidth operations module.
/// Provides actual bandwidth monitoring via ifstat.
/// Follows Single Responsibility Principle: only manages bandwidth operations.
pub mod bandwidth_ops;

/// Process tree operations module.
/// Provides actual process tree operations via pstree.
/// Follows Single Responsibility Principle: only manages process tree operations.
pub mod proc_tree_ops;

/// Container registry operations module.
/// Provides actual container registry operations via docker.
/// Follows Single Responsibility Principle: only manages container registry operations.
pub mod registry_ops;

/// Hardware operations module.
/// Provides actual hardware operations via lspci.
/// Follows Single Responsibility Principle: only manages hardware operations.
pub mod hardware_ops;

/// Bluetooth operations module.
/// Provides actual Bluetooth operations via bluetoothctl.
/// Follows Single Responsibility Principle: only manages Bluetooth operations.
pub mod bluetooth_ops;

/// Webcam operations module.
/// Provides actual webcam operations via v4l2-ctl.
/// Follows Single Responsibility Principle: only manages webcam operations.
pub mod webcam_ops;

/// Printer operations module.
/// Provides actual printer operations via lp.
/// Follows Single Responsibility Principle: only manages printer operations.
pub mod printer_ops;

/// Microphone operations module.
/// Provides actual microphone operations via arecord.
/// Follows Single Responsibility Principle: only manages microphone operations.
pub mod microphone_ops;

/// Joystick operations module.
/// Provides actual joystick operations via jstest.
/// Follows Single Responsibility Principle: only manages joystick operations.
pub mod joystick_ops;

/// Gamepad operations module.
/// Provides actual gamepad operations via evtest.
/// Follows Single Responsibility Principle: only manages gamepad operations.
pub mod gamepad_ops;

/// Touchscreen operations module.
/// Provides actual touchscreen operations via xinput.
/// Follows Single Responsibility Principle: only manages touchscreen operations.
pub mod touchscreen_ops;

/// Tablet operations module.
/// Provides actual tablet operations via xsetwacom.
/// Follows Single Responsibility Principle: only manages tablet operations.
pub mod tablet_ops;

/// Scanner operations module.
/// Provides actual scanner operations via scanimage.
/// Follows Single Responsibility Principle: only manages scanner operations.
pub mod scanner_ops;

/// Camera operations module.
/// Provides actual camera operations via fswebcam.
/// Follows Single Responsibility Principle: only manages camera operations.
pub mod camera_ops;

/// Project management operations module.
/// Provides actual project management operations via git.
/// Follows Single Responsibility Principle: only manages project operations.
pub mod project_ops;

/// Task management operations module.
/// Provides actual task management operations via task.
/// Follows Single Responsibility Principle: only manages task operations.
pub mod task_ops;

/// CI/CD operations module.
/// Provides actual CI/CD operations via make.
/// Follows Single Responsibility Principle: only manages CI/CD operations.
pub mod cicd_ops;

/// Documentation operations module.
/// Provides actual documentation operations via pandoc.
/// Follows Single Responsibility Principle: only manages documentation operations.
pub mod doc_ops;

/// Logger operations module.
/// Provides actual logging operations via logger.
/// Follows Single Responsibility Principle: only manages logging operations.
pub mod logger_ops;

/// Security audit module.
/// Provides security audit functionality to detect API key leaks, hardcoded secrets, etc.
/// Follows Single Responsibility Principle: only manages security audit operations.
pub mod security_audit;

/// Kernel operations module.
/// Provides kernel management operations via sysctl, modprobe, uname.
/// Follows Single Responsibility Principle: only manages kernel operations.
pub mod kernel_ops;

/// Bootloader operations module.
/// Provides bootloader management operations via grub, efibootmgr.
/// Follows Single Responsibility Principle: only manages bootloader operations.
pub mod bootloader_ops;

/// Initramfs operations module.
/// Provides initramfs management operations via mkinitcpio.
/// Follows Single Responsibility Principle: only manages initramfs operations.
pub mod initramfs_ops;

/// Package manager operations module.
/// Provides package manager operations via pacman.
/// Follows Single Responsibility Principle: only manages package operations.
pub mod pkgmgr_ops;

/// Repository manager operations module.
/// Provides repository management operations via pacman.
/// Follows Single Responsibility Principle: only manages repository operations.
pub mod repomgr_ops;

/// System configuration operations module.
/// Provides system configuration operations via sigmaos-config.
/// Follows Single Responsibility Principle: only manages configuration operations.
pub mod sysconfig_ops;

/// Service manager operations module.
/// Provides service management operations via systemctl.
/// Follows Single Responsibility Principle: only manages service operations.
pub mod servicemgr_ops;

/// Network manager operations module.
/// Provides network management operations via nmcli.
/// Follows Single Responsibility Principle: only manages network operations.
pub mod netmgr_ops;

/// System update operations module.
/// Provides system update operations via sigmaos-update.
/// Follows Single Responsibility Principle: only manages update operations.
pub mod sysupdate_ops;

/// Storage manager operations module.
/// Provides storage management operations via lsblk, parted.
/// Follows Single Responsibility Principle: only manages storage operations.
pub mod storagemgr_ops;

/// User manager operations module.
/// Provides user management operations via useradd, getent.
/// Follows Single Responsibility Principle: only manages user operations.
pub mod usermgr_ops;

/// Log manager operations module.
/// Provides log management operations via logrotate, journalctl.
/// Follows Single Responsibility Principle: only manages log operations.
pub mod logmgr_ops;

/// Backup manager operations module.
/// Provides backup management operations via rsync.
/// Follows Single Responsibility Principle: only manages backup operations.
pub mod backupmgr_ops;

/// Audit manager operations module.
/// Provides audit operations via sigmaos-audit.
/// Follows Single Responsibility Principle: only manages audit operations.
pub mod auditmgr_ops;

/// Release manager operations module.
/// Provides release management operations via sigmaos-release.
/// Follows Single Responsibility Principle: only manages release operations.
pub mod releasemgr_ops;

/// Async operations module.
/// Provides async I/O operations using tokio.
/// Follows principles: Asynchronous I/O & Event Loops, Race-Free Design, Safe Concurrency.
pub mod async_ops;

/// Atomic operations module.
/// Provides atomic operations for thread-safe counters.
/// Follows principles: Atomic Operations, Race-Free Design, Safe Concurrency.
pub mod atomic_ops;

/// Cache operations module.
/// Provides cache-conscious operations for efficient data access.
/// Follows principles: Cache Conscious Design, Locality of Reference.
pub mod cache_ops;

/// Cross-platform operations module.
/// Provides cross-platform operations for Linux, macOS, Windows.
/// Follows principles: Cross Platform Compatibility, Portability.
pub mod crossplatform_ops;

/// Event loop operations module.
/// Provides event loop operations for async event handling.
/// Follows principles: Asynchronous I/O & Event Loops, Race-Free Design.
pub mod eventloop_ops;

/// Instrumentation module.
/// Provides instrumentation for monitoring and logging system events.
/// Follows principles: Instrumentation, Log Everything.
pub mod instrumentation_ops;

/// Strategy pattern module.
/// Implements Strategy design pattern for interchangeable algorithms.
/// Follows principles: Design Pattern, Composition Over Inheritance.
pub mod strategy_ops;

/// Observer pattern module.
/// Implements Observer design pattern for event notification.
/// Follows principles: Design Pattern, Loose Coupling.
pub mod observer_ops;
/// Factory pattern module.
/// Implements Factory design pattern for object creation.
pub mod factory_ops;
/// Thread pool module for efficient task execution
pub mod pool_ops;
/// Resource pool for managing reusable resources
pub mod resource_pool_ops;
/// Connection pool for database/network connections
pub mod connection_pool_ops;
/// Mutex operations for thread synchronization
pub mod mutex_ops;
/// Read-write lock for concurrent access
pub mod rwlock_ops;
/// Barrier for thread synchronization
pub mod barrier_ops;
/// Channel for message passing between threads
pub mod channel_ops;
/// Semaphore for resource counting and access control
pub mod semaphore_ops;
/// Condition variable for thread coordination
pub mod condition_ops;
/// Interrupt handling operations
pub mod interrupt_ops;
/// Comprehensive error handling operations
pub mod error_ops;
/// Refactoring operations for code improvement
pub mod refactor_ops;
/// Spec-driven development operations
pub mod spec_ops;

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

/// Virtualization operations module.
/// Provides actual virtualization operations using libvirt/virsh.
/// Follows Single Responsibility Principle: only manages virtualization operations.
pub mod virt_ops;

/// Printing operations module.
/// Provides actual printing operations using CUPS.
/// Follows Single Responsibility Principle: only manages printing operations.
pub mod print_ops;

/// Time synchronization operations module.
/// Provides actual time synchronization operations using NTP/timedatectl.
/// Follows Single Responsibility Principle: only manages time sync operations.
pub mod timesync_ops;

/// Camera operations module.
/// Provides actual camera operations using v4l2.
/// Follows Single Responsibility Principle: only manages camera operations.
pub mod camera_ops;

/// Project management operations module.
/// Provides actual project management operations via git.
/// Follows Single Responsibility Principle: only manages project operations.
pub mod project_ops;

/// Task management operations module.
/// Provides actual task management operations via task.
/// Follows Single Responsibility Principle: only manages task operations.
pub mod task_ops;

/// CI/CD operations module.
/// Provides actual CI/CD operations via make.
/// Follows Single Responsibility Principle: only manages CI/CD operations.
pub mod cicd_ops;

/// Documentation operations module.
/// Provides actual documentation operations via pandoc.
/// Follows Single Responsibility Principle: only manages documentation operations.
pub mod doc_ops;

/// Logger operations module.
/// Provides actual logging operations via logger.
/// Follows Single Responsibility Principle: only manages logging operations.
pub mod logger_ops;

/// Security audit module.
/// Provides security audit functionality to detect API key leaks, hardcoded secrets, etc.
/// Follows Single Responsibility Principle: only manages security audit operations.
pub mod security_audit;

/// Kernel operations module.
/// Provides kernel management operations via sysctl, modprobe, uname.
/// Follows Single Responsibility Principle: only manages kernel operations.
pub mod kernel_ops;

/// Bootloader operations module.
/// Provides bootloader management operations via grub, efibootmgr.
/// Follows Single Responsibility Principle: only manages bootloader operations.
pub mod bootloader_ops;

/// Initramfs operations module.
/// Provides initramfs management operations via mkinitcpio.
/// Follows Single Responsibility Principle: only manages initramfs operations.
pub mod initramfs_ops;

/// Package manager operations module.
/// Provides package manager operations via pacman.
/// Follows Single Responsibility Principle: only manages package operations.
pub mod pkgmgr_ops;

/// Repository manager operations module.
/// Provides repository management operations via pacman.
/// Follows Single Responsibility Principle: only manages repository operations.
pub mod repomgr_ops;

/// System configuration operations module.
/// Provides system configuration operations via sigmaos-config.
/// Follows Single Responsibility Principle: only manages configuration operations.
pub mod sysconfig_ops;

/// Service manager operations module.
/// Provides service management operations via systemctl.
/// Follows Single Responsibility Principle: only manages service operations.
pub mod servicemgr_ops;

/// Network manager operations module.
/// Provides network management operations via nmcli.
/// Follows Single Responsibility Principle: only manages network operations.
pub mod netmgr_ops;

/// System update operations module.
/// Provides system update operations via sigmaos-update.
/// Follows Single Responsibility Principle: only manages update operations.
pub mod sysupdate_ops;

/// Storage manager operations module.
/// Provides storage management operations via lsblk, parted.
/// Follows Single Responsibility Principle: only manages storage operations.
pub mod storagemgr_ops;

/// User manager operations module.
/// Provides user management operations via useradd, getent.
/// Follows Single Responsibility Principle: only manages user operations.
pub mod usermgr_ops;

/// Log manager operations module.
/// Provides log management operations via logrotate, journalctl.
/// Follows Single Responsibility Principle: only manages log operations.
pub mod logmgr_ops;

/// Backup manager operations module.
/// Provides backup management operations via rsync.
/// Follows Single Responsibility Principle: only manages backup operations.
pub mod backupmgr_ops;

/// Audit manager operations module.
/// Provides audit operations via sigmaos-audit.
/// Follows Single Responsibility Principle: only manages audit operations.
pub mod auditmgr_ops;

/// Release manager operations module.
/// Provides release management operations via sigmaos-release.
/// Follows Single Responsibility Principle: only manages release operations.
pub mod releasemgr_ops;

/// Async operations module.
/// Provides async I/O operations using tokio.
/// Follows principles: Asynchronous I/O & Event Loops, Race-Free Design, Safe Concurrency.
pub mod async_ops;

/// Atomic operations module.
/// Provides atomic operations for thread-safe counters.
/// Follows principles: Atomic Operations, Race-Free Design, Safe Concurrency.
pub mod atomic_ops;

/// Cache operations module.
/// Provides cache-conscious operations for efficient data access.
/// Follows principles: Cache Conscious Design, Locality of Reference.
pub mod cache_ops;

/// Cross-platform operations module.
/// Provides cross-platform operations for Linux, macOS, Windows.
/// Follows principles: Cross Platform Compatibility, Portability.
pub mod crossplatform_ops;

/// Event loop operations module.
/// Provides event loop operations for async event handling.
/// Follows principles: Asynchronous I/O & Event Loops, Race-Free Design.
pub mod eventloop_ops;

/// Instrumentation module.
/// Provides instrumentation for monitoring and logging system events.
/// Follows principles: Instrumentation, Log Everything.
pub mod instrumentation_ops;

/// Strategy pattern module.
/// Implements Strategy design pattern for interchangeable algorithms.
/// Follows principles: Design Pattern, Composition Over Inheritance.
pub mod strategy_ops;

/// Observer pattern module.
/// Implements Observer design pattern for event notification.
/// Follows principles: Design Pattern, Loose Coupling.
pub mod observer_ops;
/// Factory pattern module.
/// Implements Factory design pattern for object creation.
pub mod factory_ops;
/// Thread pool module for efficient task execution
pub mod pool_ops;
/// Resource pool for managing reusable resources
pub mod resource_pool_ops;
/// Connection pool for database/network connections
pub mod connection_pool_ops;
/// Mutex operations for thread synchronization
pub mod mutex_ops;
/// Read-write lock for concurrent access
pub mod rwlock_ops;
/// Barrier for thread synchronization
pub mod barrier_ops;
/// Channel for message passing between threads
pub mod channel_ops;
/// Semaphore for resource counting and access control
pub mod semaphore_ops;
/// Condition variable for thread coordination
pub mod condition_ops;
/// Interrupt handling operations
pub mod interrupt_ops;
/// Comprehensive error handling operations
pub mod error_ops;
/// Refactoring operations for code improvement
pub mod refactor_ops;
/// Spec-driven development operations
pub mod spec_ops;

/// Sensor operations module.
/// Provides actual sensor operations using /sys/class/hwmon.
/// Follows Single Responsibility Principle: only manages sensor operations.
pub mod sensor_ops;

/// RAID operations module.
/// Provides actual RAID operations using mdadm.
/// Follows Single Responsibility Principle: only manages RAID operations.
pub mod raid_ops;

/// LVM operations module.
/// Provides actual LVM operations using lvm commands.
/// Follows Single Responsibility Principle: only manages LVM operations.
pub mod lvm_ops;

/// GPU operations module.
/// Provides actual GPU operations using nvidia-smi/AMD tools.
/// Follows Single Responsibility Principle: only manages GPU operations.
pub mod gpu_ops;

/// Fingerprint operations module.
/// Provides actual fingerprint operations using fprintd.
/// Follows Single Responsibility Principle: only manages fingerprint operations.
pub mod fingerprint_ops;

/// Smartcard operations module.
/// Provides actual smartcard operations using pcscd.
/// Follows Single Responsibility Principle: only manages smartcard operations.
pub mod smartcard_ops;

/// TCO operations module.
/// Provides actual TCO (Total Cost of Ownership) operations.
/// Follows Single Responsibility Principle: only manages TCO operations.
pub mod tco_ops;

/// Compliance operations module.
/// Provides actual compliance operations.
/// Follows Single Responsibility Principle: only manages compliance operations.
pub mod compliance_ops;

/// Incident response operations module.
/// Provides actual incident response operations.
/// Follows Single Responsibility Principle: only manages incident response operations.
pub mod incident_ops;

/// Capacity planning operations module.
/// Provides actual capacity planning operations.
/// Follows Single Responsibility Principle: only manages capacity planning operations.
pub mod capacity_ops;

/// Inventory operations module.
/// Provides actual inventory operations.
/// Follows Single Responsibility Principle: only manages inventory operations.
pub mod inventory_ops;

/// Disaster recovery operations module.
/// Provides actual disaster recovery operations.
/// Follows Single Responsibility Principle: only manages disaster recovery operations.
pub mod disaster_ops;

/// Asset management operations module.
/// Provides actual asset management operations.
/// Follows Single Responsibility Principle: only manages asset management operations.
pub mod asset_ops;

/// Patch management operations module.
/// Provides actual patch management operations.
/// Follows Single Responsibility Principle: only manages patch management operations.
pub mod patch_ops;

/// Lifecycle management operations module.
/// Provides actual lifecycle management operations.
/// Follows Single Responsibility Principle: only manages lifecycle management operations.
pub mod lifecycle_ops;

/// Configuration drift operations module.
/// Provides actual configuration drift detection operations.
/// Follows Single Responsibility Principle: only manages configuration drift operations.
pub mod drift_ops;

/// Change management operations module.
/// Provides actual change management operations.
/// Follows Single Responsibility Principle: only manages change management operations.
pub mod change_ops;

/// Metrics collection operations module.
/// Provides actual metrics collection operations.
/// Follows Single Responsibility Principle: only manages metrics collection operations.
pub mod metrics_ops;

/// Performance profiling operations module.
/// Provides actual performance profiling operations.
/// Follows Single Responsibility Principle: only manages performance profiling operations.
pub mod profile_ops;

/// Service mesh operations module.
/// Provides actual service mesh operations.
/// Follows Single Responsibility Principle: only manages service mesh operations.
pub mod mesh_ops;

/// Multi-cluster operations module.
/// Provides actual multi-cluster operations.
/// Follows Single Responsibility Principle: only manages multi-cluster operations.
pub mod cluster_ops;

/// Cross-platform operations module.
/// Provides actual cross-platform operations.
/// Follows Single Responsibility Principle: only manages cross-platform operations.
pub mod platform_ops;

/// Cloud native operations module.
/// Provides actual cloud native operations.
/// Follows Single Responsibility Principle: only manages cloud native operations.
pub mod cloud_ops;

/// Email operations module.
/// Provides actual email operations via sendmail/postfix.
/// Follows Single Responsibility Principle: only manages email operations.
pub mod email_ops;

/// Notification operations module.
/// Provides actual notification operations via notify-send.
/// Follows Single Responsibility Principle: only manages notification operations.
pub mod notify_ops;

/// VPN operations module.
/// Provides actual VPN operations via NetworkManager.
/// Follows Single Responsibility Principle: only manages VPN operations.
pub mod vpn_ops;

/// Clipboard operations module.
/// Provides actual clipboard operations via xclip/wl-copy.
/// Follows Single Responsibility Principle: only manages clipboard operations.
pub mod clipboard_ops;

/// Screenshot operations module.
/// Provides actual screenshot operations via grim/slop.
/// Follows Single Responsibility Principle: only manages screenshot operations.
pub mod screenshot_ops;

/// Screen recording operations module.
/// Provides actual screen recording operations via wf-recorder.
/// Follows Single Responsibility Principle: only manages screen recording operations.
pub mod record_ops;

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
pub use file_watcher::{FileWatcherOps, FileWatcherError};
pub use sound_ops::{SoundOps, SoundError as SoundManagerError};
pub use bandwidth_ops::{BandwidthOps, BandwidthError as BandwidthManagerError};
pub use proc_tree_ops::{ProcTreeOps, ProcTreeError as ProcTreeManagerError};
pub use registry_ops::{RegistryOps, RegistryError as RegistryManagerError};
pub use hardware_ops::{HardwareOps, HardwareError as HardwareManagerError};
pub use bluetooth_ops::{BluetoothOps, BluetoothError as BluetoothManagerError};
pub use webcam_ops::{WebcamOps, WebcamError as WebcamManagerError};
pub use printer_ops::{PrinterOps, PrinterError as PrinterManagerError};
pub use microphone_ops::{MicrophoneOps, MicrophoneError as MicrophoneManagerError};
pub use joystick_ops::{JoystickOps, JoystickError as JoystickManagerError};
pub use gamepad_ops::{GamepadOps, GamepadError as GamepadManagerError};
pub use touchscreen_ops::{TouchscreenOps, TouchscreenError as TouchscreenManagerError};
pub use tablet_ops::{TabletOps, TabletError as TabletManagerError};
pub use scanner_ops::{ScannerOps, ScannerError as ScannerManagerError};
pub use camera_ops::{CameraOps, CameraError as CameraManagerError};
pub use project_ops::{ProjectOps, ProjectError as ProjectManagerError};
pub use task_ops::{TaskOps, TaskError as TaskManagerError};
pub use cicd_ops::{CicdOps, CicdError as CicdManagerError};
pub use doc_ops::{DocOps, DocError as DocManagerError};
pub use logger_ops::{LoggerOps, LoggerError as LogManagerError};
pub use security_audit::{SecurityAuditor, SecurityAuditResult, SecurityIssue, SecurityAuditError};
pub use kernel_ops::{KernelOps, KernelError as KernelManagerError};
pub use bootloader_ops::{BootloaderOps, BootloaderError as BootloaderManagerError};
pub use initramfs_ops::{InitramfsOps, InitramfsError as InitramfsManagerError};
pub use pkgmgr_ops::{PkgMgrOps, PkgMgrError as PkgManagerError};
pub use repomgr_ops::{RepoMgrOps, RepoMgrError as RepoManagerError};
pub use sysconfig_ops::{SysConfigOps, SysConfigError as SysConfigManagerError};
pub use servicemgr_ops::{ServiceMgrOps, ServiceMgrError as ServiceManagerError};
pub use netmgr_ops::{NetMgrOps, NetMgrError as NetworkManagerError};
pub use sysupdate_ops::{SysUpdateOps, SysUpdateError as SystemUpdateError};
pub use storagemgr_ops::{StorageMgrOps, StorageMgrError as StorageManagerError};
pub use usermgr_ops::{UserMgrOps, UserMgrError as UserManagerError};
pub use logmgr_ops::{LogMgrOps, LogMgrError as LogManagerError};
pub use backupmgr_ops::{BackupMgrOps, BackupMgrError as BackupManagerError};
pub use auditmgr_ops::{AuditMgrOps, AuditMgrError as AuditManagerError};
pub use releasemgr_ops::{ReleaseMgrOps, ReleaseMgrError as ReleaseManagerError};
pub use async_ops::{AsyncOps, AsyncError as AsyncManagerError};
pub use atomic_ops::{AtomicOps};
pub use resource_monitor::{ResourceMonitor, ResourceSnapshot, ResourceError};
pub use cron_ops::{CronOps, CronJob as CronJobOp, CronError as CronOpError};
pub use timer_ops::{TimerOps, TimerInfo, TimerError};
pub use session_ops::{SessionOps, SessionInfo, SessionError};
pub use firewall_ops::{FirewallOps as FwManager, FirewallRule as FwRule, FirewallBackend as FwBackend, FirewallError as FwManagerError};
pub use proxy_ops::{ProxyOps, ProxyError};
pub use backup_ops::{BackupOps as RealBackupOps, BackupError as RealBackupError};
pub use user_group_ops::{UserGroupOps, UserInfo, GroupInfo, UserGroupError};
pub use cert_ops::{CertOps, CertInfo, CertError};
pub use archive_ops::{ArchiveOps, ArchiveError};
pub use logrotate_ops::{LogrotateOps, LogrotateError};
pub use disk_space_ops::{DiskSpaceOps, DiskSpace, DiskSpaceError};
pub use uefi_ops::{UefiOps, BootEntry as UefiBootEntry, UefiError as UefiManagerError};
pub use repo_ops::{RepoOps, RepoInfo, RepoError as RepoManagerError};
pub use scheduler_ops::{SchedulerOps as CpuScheduler, SchedulerError as CpuSchedulerError};
pub use swap_ops::{SwapOps, SwapInfo, SwapError as SwapManagerError};
pub use keyboard_ops::{KeyboardOps, KeyboardError as KeymapError};
pub use locale_ops::{LocaleOps, LocaleError as SystemLocaleError};
pub use display_ops::{DisplayOps, DisplayError as DisplayManagerError};
pub use audio_ops::{AudioOps, AudioSink, AudioError as AudioManagerError};
pub use container_ops::{ContainerOps as CtnManager, ContainerInfo, ContainerError as ContainerManagerError};
pub use virt_ops::{VirtOps as VmManager, VmInfo, VirtError as VirtManagerError};
pub use print_ops::{PrintOps, PrinterInfo, PrintError as PrintManagerError};
pub use timesync_ops::{TimeSyncOps, TimeSyncError as NtpSyncError};
pub use camera_ops::{CameraOps, CameraInfo, CameraError as CameraManagerError};
pub use project_ops::{ProjectOps, ProjectError as ProjectManagerError};
pub use task_ops::{TaskOps, TaskError as TaskManagerError};
pub use cicd_ops::{CicdOps, CicdError as CicdManagerError};
pub use doc_ops::{DocOps, DocError as DocManagerError};
pub use logger_ops::{LoggerOps, LoggerError as LogManagerError};
pub use security_audit::{SecurityAuditor, SecurityAuditResult, SecurityIssue, SecurityAuditError};
pub use kernel_ops::{KernelOps, KernelError as KernelManagerError};
pub use bootloader_ops::{BootloaderOps, BootloaderError as BootloaderManagerError};
pub use initramfs_ops::{InitramfsOps, InitramfsError as InitramfsManagerError};
pub use pkgmgr_ops::{PkgMgrOps, PkgMgrError as PkgManagerError};
pub use repomgr_ops::{RepoMgrOps, RepoMgrError as RepoManagerError};
pub use sysconfig_ops::{SysConfigOps, SysConfigError as SysConfigManagerError};
pub use servicemgr_ops::{ServiceMgrOps, ServiceMgrError as ServiceManagerError};
pub use netmgr_ops::{NetMgrOps, NetMgrError as NetworkManagerError};
pub use sysupdate_ops::{SysUpdateOps, SysUpdateError as SystemUpdateError};
pub use storagemgr_ops::{StorageMgrOps, StorageMgrError as StorageManagerError};
pub use usermgr_ops::{UserMgrOps, UserMgrError as UserManagerError};
pub use logmgr_ops::{LogMgrOps, LogMgrError as LogManagerError};
pub use backupmgr_ops::{BackupMgrOps, BackupMgrError as BackupManagerError};
pub use auditmgr_ops::{AuditMgrOps, AuditMgrError as AuditManagerError};
pub use releasemgr_ops::{ReleaseMgrOps, ReleaseMgrError as ReleaseManagerError};
pub use async_ops::{AsyncOps, AsyncError as AsyncManagerError};
pub use atomic_ops::{AtomicOps};
pub use sensor_ops::{SensorOps, SensorInfo, SensorError as SensorManagerError};
pub use raid_ops::{RaidOps, RaidInfo, RaidError as RaidManagerError};
pub use lvm_ops::{LvmOps, VolumeGroup, LvmError as LvmManagerError};
pub use gpu_ops::{GpuOps, GpuInfo, GpuError as GpuManagerError};
pub use fingerprint_ops::{FingerprintOps, FingerprintDevice, FingerprintError as FpManagerError};
pub use smartcard_ops::{SmartcardOps, SmartcardInfo, SmartcardError as ScManagerError};
pub use tco_ops::{TcoOps, TcoError as TcoManagerError};
pub use compliance_ops::{ComplianceOps, ComplianceError as ComplianceManagerError};
pub use incident_ops::{IncidentOps, IncidentError as IncidentManagerError};
pub use capacity_ops::{CapacityOps, CapacityError as CapacityManagerError};
pub use inventory_ops::{InventoryOps, InventoryItem, InventoryError as InventoryManagerError};
pub use disaster_ops::{DisasterOps, DisasterError as DisasterManagerError};
pub use asset_ops::{AssetOps, Asset, AssetError as AssetManagerError};
pub use patch_ops::{PatchOps, Patch, PatchError as PatchManagerError};
pub use lifecycle_ops::{LifecycleOps, LifecycleError as LifecycleManagerError};
pub use drift_ops::{DriftOps, DriftError as DriftManagerError};
pub use change_ops::{ChangeOps, ChangeError as ChangeManagerError};
pub use metrics_ops::{MetricsOps, MetricsError as MetricsManagerError};
pub use profile_ops::{ProfileOps, ProfileError as ProfileManagerError};
pub use mesh_ops::{MeshOps, MeshError as MeshManagerError};
pub use cluster_ops::{ClusterOps, ClusterError as ClusterManagerError};
pub use platform_ops::{PlatformOps, PlatformError as PlatformManagerError};
pub use cloud_ops::{CloudOps, CloudError as CloudManagerError};
pub use email_ops::{EmailOps, EmailError as EmailManagerError};
pub use notify_ops::{NotifyOps, NotifyError as NotifyManagerError};
pub use vpn_ops::{VpnOps, VpnError as VpnManagerError};
pub use clipboard_ops::{ClipboardOps, ClipboardError as ClipboardManagerError};
pub use screenshot_ops::{ScreenshotOps, ScreenshotError as ScreenshotManagerError};
pub use record_ops::{RecordOps, RecordError as RecordManagerError};
/// Certificate operations module.
/// Provides actual SSL/TLS certificate operations using openssl.
/// Follows Single Responsibility Principle: only manages certificate operations.

/// Convenience function to create a PersonaManager with fail-fast validation
pub fn create_persona_manager(initial: SystemPersona) -> PersonaManager {
    PersonaManager {
        current: SystemPersona::validate(initial),
    }
}
