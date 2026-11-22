//! Example: System optimization using SigmaOS library
//!
//! This example demonstrates how to use the SigmaOS library to optimize
//! a Linux system for performance, memory, and power efficiency.

use sigma_system::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SigmaOS System Optimization Example ===\n");

    // 1. CPU Governor Management
    println!("1. CPU Governor Management");
    let mut adjuster = SystemAdjuster::new();
    adjuster.set_governor(GovernorMode::Performance)?;
    println!("   Set CPU governor to: {}", adjuster.get_governor().as_str());
    println!("   Sysctl config:\n{}", adjuster.generate_sysctl_config());

    // 2. Memory Management
    println!("\n2. Memory Management");
    let mut memory_mgr = MemoryManager::new();
    memory_mgr.set_swappiness(10)?;
    memory_mgr.set_vfs_cache_pressure(50)?;
    println!("   Swappiness: {}", memory_mgr.get_swappiness());
    println!("   VFS cache pressure: {}", memory_mgr.get_vfs_cache_pressure());
    println!("   Memory pressure: {}", memory_mgr.current_pressure.as_str());

    // 3. Network Configuration
    println!("\n3. Network Configuration");
    let mut network_mgr = NetworkManager::new();
    network_mgr.add_firewall_rule(FirewallRule::new(
        NetworkProtocol::Tcp,
        "22",
        FirewallAction::Allow,
        "SSH access",
    )?)?;
    network_mgr.add_firewall_rule(FirewallRule::new(
        NetworkProtocol::Tcp,
        "80,443",
        FirewallAction::Allow,
        "HTTP/HTTPS",
    )?)?;
    println!("   Firewall rules: {}", network_mgr.firewall_rules.len());
    println!("   nftables config:\n{}", network_mgr.generate_nftables_config());

    // 4. Security Hardening
    println!("\n4. Security Hardening");
    let mut security_mgr = SecurityManager::new();
    security_mgr.enable_secure_boot()?;
    security_mgr.enable_tpm(TpmVersion::V2)?;
    security_mgr.enable_integrity_check()?;
    println!("   Secure Boot: {}", security_mgr.secure_boot_state.as_str());
    println!("   TPM: {}", security_mgr.tpm_version.as_ref().unwrap().as_str());
    println!("   Integrity check: {}", security_mgr.integrity_check_enabled);
    println!("   Security score: {}/100", security_mgr.calculate_security_score());

    // 5. Filesystem Optimization
    println!("\n5. Filesystem Optimization");
    let mut fs_mgr = FileSystemManager::new();
    fs_mgr.set_filesystem_type(FileSystemType::Btrfs)?;
    fs_mgr.enable_snapshots()?;
    fs_mgr.set_mount_options(vec![
        MountOption::new("compress", "zstd"),
        MountOption::new("ssd", "discard"),
        MountOption::new("noatime", ""),
    ])?;
    println!("   Filesystem: {}", fs_mgr.filesystem_type.as_str());
    println!("   Snapshots: {}", fs_mgr.snapshots_enabled);
    println!("   fstab entry:\n{}", fs_mgr.generate_fstab_entry());

    // 6. Power Management
    println!("\n6. Power Management");
    let mut power_mgr = PowerManager::new();
    power_mgr.set_power_profile(PowerProfile::Balanced)?;
    power_mgr.set_tdp_level(TdpLevel::Balanced)?;
    println!("   Power profile: {}", power_mgr.power_profile.as_str());
    println!("   TDP level: {}", power_mgr.tdp_level.as_str());
    println!("   Auto-optimization: {}", power_mgr.auto_optimization);

    // 7. Monitoring Setup
    println!("\n7. Monitoring Setup");
    let mut monitoring_mgr = MonitoringManager::new();
    monitoring_mgr.set_threshold(MetricType::CpuUsage, 80.0, 60.0)?;
    monitoring_mgr.set_threshold(MetricType::MemoryUsage, 85.0, 70.0)?;
    monitoring_mgr.set_threshold(MetricType::DiskUsage, 90.0, 75.0)?;
    println!("   Thresholds configured for CPU, memory, disk");
    println!("   Prometheus config:\n{}", monitoring_mgr.generate_prometheus_config());

    // 8. Generate Complete Configuration
    println!("\n8. Complete Configuration Summary");
    println!("   CPU Governor: {}", adjuster.get_governor().as_str());
    println!("   Memory Swappiness: {}", memory_mgr.get_swappiness());
    println!("   Firewall Rules: {}", network_mgr.firewall_rules.len());
    println!("   Security Score: {}/100", security_mgr.calculate_security_score());
    println!("   Filesystem: {}", fs_mgr.filesystem_type.as_str());
    println!("   Power Profile: {}", power_mgr.power_profile.as_str());
    println!("   Monitoring Thresholds: {}", monitoring_mgr.thresholds.len());

    println!("\n=== Optimization Complete ===");
    println!("\nTo apply these settings on a running system:");
    println!("1. Copy sysctl config to /etc/sysctl.d/99-sigmaos.conf");
    println!("2. Copy nftables rules to /etc/nftables.conf");
    println!("3. Copy fstab entry to /etc/fstab");
    println!("4. Enable SigmaOS services");
    println!("5. Reboot or reload services");

    Ok(())
}
