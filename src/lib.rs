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

/// Convenience function to create a PersonaManager with fail-fast validation
pub fn create_persona_manager(initial: SystemPersona) -> PersonaManager {
    PersonaManager {
        current: SystemPersona::validate(initial),
    }
}