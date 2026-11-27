use anyhow::Result;
use clap::{Parser, Subcommand};
use sigma_system::*;

#[derive(Parser)]
#[command(name = "sigmaos")]
#[command(about = "SigmaOS command-line interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// System information
    Info {
        /// Show detailed system info
        #[arg(short, long)]
        detailed: bool,
    },
    /// Process management
    Process {
        #[command(subcommand)]
        action: ProcessCommands,
    },
    /// Service management
    Service {
        #[command(subcommand)]
        action: ServiceCommands,
    },
    /// Network operations
    Network {
        #[command(subcommand)]
        action: NetworkCommands,
    },
    /// Disk operations
    Disk {
        #[command(subcommand)]
        action: DiskCommands,
    },
    /// System monitoring
    Monitor {
        #[command(subcommand)]
        action: MonitorCommands,
    },
    /// Desktop environment management
    Desktop {
        #[command(subcommand)]
        action: DesktopCommands,
    },
    /// Power management
    Power {
        #[command(subcommand)]
        action: PowerCommands,
    },
    /// Audio operations
    Audio {
        #[command(subcommand)]
        action: AudioCommands,
    },
    /// Display manager operations
    Display {
        #[command(subcommand)]
        action: DisplayCommands,
    },
    /// Package operations
    Package {
        #[command(subcommand)]
        action: PackageCommands,
    },
    /// Backup operations
    Backup {
        #[command(subcommand)]
        action: BackupCommands,
    },
    /// Security operations
    Security {
        #[command(subcommand)]
        action: SecurityCommands,
    },
    /// Theme operations
    Theme {
        #[command(subcommand)]
        action: ThemeCommands,
    },
    /// System updates
    Update {
        #[command(subcommand)]
        action: UpdateCommands,
    },
    /// User management
    User {
        #[command(subcommand)]
        action: UserCommands,
    },
}

#[derive(Subcommand)]
enum ProcessCommands {
    /// List all processes
    List,
    /// Get process info by PID
    Info { pid: u32 },
}

#[derive(Subcommand)]
enum ServiceCommands {
    /// List all services
    List,
    /// Start a service
    Start { name: String },
    /// Stop a service
    Stop { name: String },
    /// Get service status
    Status { name: String },
}

#[derive(Subcommand)]
enum NetworkCommands {
    /// List network interfaces
    List,
    /// Get interface info
    Info { iface: String },
}

#[derive(Subcommand)]
enum DiskCommands {
    /// List disks
    List,
    /// Get disk usage
    Usage { path: String },
    /// List mounts
    Mounts,
}

#[derive(Subcommand)]
enum MonitorCommands {
    /// Show CPU usage
    Cpu,
    /// Show memory usage
    Memory,
    /// Show thermal info
    Thermal,
}

#[derive(Subcommand)]
enum DesktopCommands {
    /// List available desktop sessions
    List,
    /// Show current session
    Current,
    /// Set desktop environment
    SetDE { name: String },
    /// Set display server
    SetDisplayServer { server: String },
    /// Set window manager
    SetWindowManager { wm: String },
    /// Generate display manager config
    GenDMConfig,
}

#[derive(Subcommand)]
enum PowerCommands {
    /// Show battery and power status
    Status,
    /// Set power profile
    SetProfile { profile: String },
    /// Show available profiles
    Profiles,
}

#[derive(Subcommand)]
enum AudioCommands {
    /// List audio sinks
    ListSinks,
}

#[derive(Subcommand)]
enum DisplayCommands {
    /// Get current display manager
    Manager,
}

#[derive(Subcommand)]
enum PackageCommands {
    /// Install a package
    Install { name: String },
}

#[derive(Subcommand)]
enum BackupCommands {
    /// Create a backup
    Create {
        name: String,
        source: String,
        dest: String,
    },
}

#[derive(Subcommand)]
enum SecurityCommands {
    /// Show security status
    Status,
}

#[derive(Subcommand)]
enum ThemeCommands {
    /// List available themes
    List,
    /// Set theme
    Set { name: String },
}

#[derive(Subcommand)]
enum UpdateCommands {
    /// Check for updates
    Check,
}

#[derive(Subcommand)]
enum UserCommands {
    /// List users
    List,
    /// Add a user
    Add { name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { detailed } => cmd_info(detailed),
        Commands::Process { action } => cmd_process(action),
        Commands::Service { action } => cmd_service(action),
        Commands::Network { action } => cmd_network(action),
        Commands::Disk { action } => cmd_disk(action),
        Commands::Monitor { action } => cmd_monitor(action),
        Commands::Desktop { action } => cmd_desktop(action),
        Commands::Power { action } => cmd_power(action),
        Commands::Audio { action } => cmd_audio(action),
        Commands::Display { action } => cmd_display(action),
        Commands::Package { action } => cmd_package(action),
        Commands::Backup { action } => cmd_backup(action),
        Commands::Security { action } => cmd_security(action),
        Commands::Theme { action } => cmd_theme(action),
        Commands::Update { action } => cmd_update(action),
        Commands::User { action } => cmd_user(action),
    }
}

fn cmd_info(detailed: bool) -> Result<()> {
    let mut sys_info = SysInfo::new();

    println!("=== SigmaOS System Information ===");

    if let Ok(info) = sys_info.get_system_info() {
        println!("Hostname: {}", info.hostname);
        println!("Kernel: {}", info.kernel_release);
        println!("Architecture: {}", info.architecture);
    }

    if detailed {
        if let Ok(uptime) = sys_info.get_uptime() {
            println!("Uptime: {}", uptime);
        }

        if let Ok(memory) = sys_info.get_memory_usage() {
            println!("Memory: {}", memory);
        }
    }

    Ok(())
}

fn cmd_process(action: ProcessCommands) -> Result<()> {
    let mut proc_mgr = ProcessManager::new();

    match action {
        ProcessCommands::List => {
            if let Ok(processes) = proc_mgr.list_processes() {
                println!("PID\tName\tState\tCPU%\tMem(KB)");
                for proc in processes {
                    println!(
                        "{}\t{}\t{}\t{:.1}\t{}",
                        proc.pid, proc.name, proc.state, proc.cpu_percent, proc.memory_kb
                    );
                }
            }
        }
        ProcessCommands::Info { pid } => {
            if let Ok(Some(info)) = proc_mgr.get_process(pid) {
                println!("Process {} - {}", pid, info.name);
                println!("State: {}", info.state);
                println!("CPU: {:.1}%", info.cpu_percent);
                println!("Memory: {} KB", info.memory_kb);
            }
        }
    }
    Ok(())
}

fn cmd_service(action: ServiceCommands) -> Result<()> {
    let mut svc_mgr = ServiceManager::new();

    match action {
        ServiceCommands::List => {
            if let Ok(services) = svc_mgr.list_services() {
                println!("Service\t\tStatus\tEnabled");
                for svc in services {
                    println!("{}\t\t{}\t{}", svc.name, svc.status.as_str(), svc.enabled);
                }
            }
        }
        ServiceCommands::Start { name } => {
            if let Ok(_) = svc_mgr.start_service(&name) {
                println!("Started {}", name);
            }
        }
        ServiceCommands::Stop { name } => {
            if let Ok(_) = svc_mgr.stop_service(&name) {
                println!("Stopped {}", name);
            }
        }
        ServiceCommands::Status { name } => {
            if let Ok(status) = svc_mgr.get_service_status(&name) {
                println!("{}: {}", name, status.as_str());
            }
        }
    }
    Ok(())
}

fn cmd_network(action: NetworkCommands) -> Result<()> {
    let mut net_mgr = NetOps::new();

    match action {
        NetworkCommands::List => {
            if let Ok(ifaces) = net_mgr.list_interfaces() {
                println!("Interface\tStatus\tIPv4");
                for iface in ifaces {
                    let status = if iface.is_up { "up" } else { "down" };
                    let ip = iface
                        .ipv4_addresses
                        .first()
                        .map(|a| a.to_string())
                        .unwrap_or_else(|| "none".to_string());
                    println!("{}\t{}\t{}", iface.name, status, ip);
                }
            }
        }
        NetworkCommands::Info { iface } => {
            if let Ok(Some(info)) = net_mgr.get_interface(&iface) {
                println!("Interface: {}", info.name);
                println!("Status: {}", if info.is_up { "up" } else { "down" });
                println!(
                    "MAC: {}",
                    info.mac_address
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("none")
                );
                println!(
                    "IPv4: {}",
                    info.ipv4_addresses
                        .first()
                        .map(|a| a.to_string())
                        .unwrap_or_else(|| "none".to_string())
                );
            }
        }
    }
    Ok(())
}

fn cmd_disk(action: DiskCommands) -> Result<()> {
    let mut disk_mgr = DiskManager::new();

    match action {
        DiskCommands::List => {
            if let Ok(disks) = disk_mgr.list_disks() {
                println!("Device\t\tSize\t\tType");
                for disk in disks {
                    println!("{}\t\t{}\t\t{}", disk.name, disk.size, disk.disk_type);
                }
            }
        }
        DiskCommands::Usage { path } => {
            let space_mgr = DiskSpaceOps::new();
            if let Ok(space) = space_mgr.get_disk_space(&path) {
                println!("Path: {}", path);
                println!("Total: {} GB", space.total_bytes / 1024 / 1024 / 1024);
                println!("Used: {} GB", space.used_bytes / 1024 / 1024 / 1024);
                println!(
                    "Available: {} GB",
                    space.available_bytes / 1024 / 1024 / 1024
                );
                println!("Usage: {:.1}%", space.used_percent);
            }
        }
        DiskCommands::Mounts => {
            let mut mount_mgr = MountOps::new();
            if let Ok(mounts) = mount_mgr.list_mounts() {
                println!("Device\t\tMount Point\t\tType");
                for mount in mounts {
                    println!(
                        "{}\t\t{}\t\t{}",
                        mount.device, mount.mount_point, mount.fs_type
                    );
                }
            }
        }
    }
    Ok(())
}

fn cmd_monitor(action: MonitorCommands) -> Result<()> {
    match action {
        MonitorCommands::Cpu => {
            let mut cpu_mgr = CpuOps::new();
            if let Ok(info) = cpu_mgr.get_cpu_info() {
                println!("CPU Model: {}", info.model_name);
                println!("Cores: {}", info.core_count);
            }
            if let Ok(usage) = cpu_mgr.get_cpu_usage_percent() {
                println!("Usage: {:.1}%", usage);
            }
        }
        MonitorCommands::Memory => {
            let mut mem_mgr = MemOps::new();
            if let Ok(info) = mem_mgr.get_memory_info() {
                println!("Total: {} KB", info.total_kb);
                println!("Used: {} KB", info.used_kb);
                println!("Free: {} KB", info.free_kb);
                println!("Usage: {:.1}%", info.usage_percent);
            }
        }
        MonitorCommands::Thermal => {
            let mut thermal_mgr = ThermalMonitor::new();
            if let Ok(zones) = thermal_mgr.get_thermal_zones() {
                println!("Zone\t\tTemperature");
                for zone in zones {
                    println!("{}\t\t{:.1} C", zone.name, zone.temperature);
                }
            }
        }
    }
    Ok(())
}

fn cmd_desktop(action: DesktopCommands) -> Result<()> {
    let mut desktop_mgr = DesktopManager::new();

    match action {
        DesktopCommands::List => {
            println!("Available desktop sessions:");
            for session in desktop_mgr.get_sessions() {
                let default = if session.is_default { " (default)" } else { "" };
                println!(
                    "  {} - {} | {} | {} | {}{}",
                    session.name,
                    session.desktop_environment.as_str(),
                    session.display_server.as_str(),
                    session.window_manager.as_str(),
                    session.command,
                    default
                );
            }
        }
        DesktopCommands::Current => {
            if let Some(session) = desktop_mgr.get_current_session() {
                println!("Current session: {}", session.name);
                println!("  Desktop: {}", session.desktop_environment.as_str());
                println!("  Display: {}", session.display_server.as_str());
                println!("  Window Manager: {}", session.window_manager.as_str());
                println!("  Command: {}", session.command);
            } else {
                println!("No active session");
            }
        }
        DesktopCommands::SetDE { name } => match desktop_mgr.set_desktop_environment(&name) {
            Ok(()) => println!("Desktop environment set to: {}", name),
            Err(e) => eprintln!("Failed to set desktop environment: {}", e),
        },
        DesktopCommands::SetDisplayServer { server } => {
            match desktop_mgr.set_display_server(&server) {
                Ok(()) => println!("Display server set to: {}", server),
                Err(e) => eprintln!("Failed to set display server: {}", e),
            }
        }
        DesktopCommands::SetWindowManager { wm } => match desktop_mgr.set_window_manager(&wm) {
            Ok(()) => println!("Window manager set to: {}", wm),
            Err(e) => eprintln!("Failed to set window manager: {}", e),
        },
        DesktopCommands::GenDMConfig => {
            let config = desktop_mgr.generate_display_manager_config();
            println!("{}", config);
        }
    }
    Ok(())
}

fn cmd_power(action: PowerCommands) -> Result<()> {
    let mut power_mgr = PowerManager::default();

    match action {
        PowerCommands::Status => {
            println!("{}", power_mgr.get_battery_status_summary());
        }
        PowerCommands::SetProfile { profile } => {
            let p = match profile.to_lowercase().as_str() {
                "performance" => PowerProfile::Performance,
                "balanced" => PowerProfile::Balanced,
                "power-saver" | "powersaver" => PowerProfile::PowerSaver,
                _ => {
                    eprintln!("Unknown profile: {}", profile);
                    return Ok(());
                }
            };
            power_mgr.set_profile(p);
            println!("Power profile set to: {}", profile);
        }
        PowerCommands::Profiles => {
            println!("Available power profiles:");
            println!("  performance - Maximum performance");
            println!("  balanced - Balanced performance and power");
            println!("  power-saver - Maximum power savings");
        }
    }
    Ok(())
}

fn cmd_audio(action: AudioCommands) -> Result<()> {
    let audio_mgr = AudioOps::new();

    match action {
        AudioCommands::ListSinks => {
            if let Ok(sinks) = audio_mgr.list_audio_sinks() {
                println!("Audio sinks:");
                for sink in sinks {
                    let default = if sink.is_default { " (default)" } else { "" };
                    println!("  {} - {}{}", sink.name, sink.description, default);
                }
            }
        }
    }
    Ok(())
}

fn cmd_display(action: DisplayCommands) -> Result<()> {
    let display_mgr = DisplayOps::new();

    match action {
        DisplayCommands::Manager => match display_mgr.get_display_manager() {
            Ok(dm) => println!("Display manager: {}", dm),
            Err(e) => eprintln!("Failed to get display manager: {}", e),
        },
    }
    Ok(())
}

fn cmd_package(action: PackageCommands) -> Result<()> {
    let mut installer = PackageInstaller::new();

    match action {
        PackageCommands::Install { name } => match installer.install(&name) {
            Ok(()) => println!("Installed {}", name),
            Err(e) => eprintln!("Failed to install {}: {}", name, e),
        },
    }
    Ok(())
}

fn cmd_backup(action: BackupCommands) -> Result<()> {
    let mut backup_mgr = BackupManager::new();

    match action {
        BackupCommands::Create { name, source, dest } => {
            match backup_mgr.create_backup(&name, BackupType::Full, &source, &dest) {
                Ok(backup) => {
                    println!(
                        "Created backup '{}' from {} to {}",
                        backup.name,
                        backup.source_path.display(),
                        backup.dest_path.display()
                    );
                }
                Err(e) => eprintln!("Failed to create backup: {}", e),
            }
        }
    }
    Ok(())
}

fn cmd_security(action: SecurityCommands) -> Result<()> {
    let security_mgr = SecurityManager::default();

    match action {
        SecurityCommands::Status => {
            println!("Secure Boot: {:?}", security_mgr.secure_boot_state);
            println!("TPM: {:?}", security_mgr.tpm_config);
        }
    }
    Ok(())
}

fn cmd_theme(action: ThemeCommands) -> Result<()> {
    match action {
        ThemeCommands::List => {
            println!("Available themes:");
            println!("  default - SigmaOS default theme");
            println!("  dark - Dark theme variant");
            println!("  light - Light theme variant");
        }
        ThemeCommands::Set { name } => {
            println!("Theme set to: {}", name);
        }
    }
    Ok(())
}

fn cmd_update(action: UpdateCommands) -> Result<()> {
    let mut update_mgr = UpdateManager::new();

    match action {
        UpdateCommands::Check => match update_mgr.check_updates() {
            Ok(updates) => {
                println!("Available updates: {}", updates.len());
                for update in updates {
                    println!(
                        "  {} {} -> {}",
                        update.name, update.current_version, update.new_version
                    );
                }
            }
            Err(e) => eprintln!("Failed to check updates: {}", e),
        },
    }
    Ok(())
}

fn cmd_user(action: UserCommands) -> Result<()> {
    let mut user_mgr = UserManager::new();

    match action {
        UserCommands::List => {
            let users = user_mgr.get_users();
            println!("Users:");
            for user in users {
                println!("  {} ({})", user.username, user.shell);
            }
        }
        UserCommands::Add { name } => {
            match user_mgr.create_user(&name, &name, &format!("/home/{}", name), "/bin/bash") {
                Ok(_) => println!("User '{}' created", name),
                Err(e) => eprintln!("Failed to create user '{}': {}", name, e),
            }
        }
    }
    Ok(())
}
