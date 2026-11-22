use clap::{Parser, Subcommand};
use sigma_system::*;
use anyhow::Result;

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { detailed } => cmd_info(detailed),
        Commands::Process { action } => cmd_process(action),
        Commands::Service { action } => cmd_service(action),
        Commands::Network { action } => cmd_network(action),
        Commands::Disk { action } => cmd_disk(action),
        Commands::Monitor { action } => cmd_monitor(action),
    }
}

fn cmd_info(detailed: bool) -> Result<()> {
    let sys_info = SysInfo::new();
    
    println!("=== SigmaOS System Information ===");
    
    if let Ok(info) = sys_info.get_system_info() {
        println!("Hostname: {}", info.hostname);
        println!("OS: {}", info.os);
        println!("Kernel: {}", info.kernel);
        println!("Architecture: {}", info.arch);
    }
    
    if detailed {
        if let Ok(uptime) = sys_info.get_uptime() {
            println!("Uptime: {} seconds", uptime);
        }
        
        if let Ok(memory) = sys_info.get_memory_usage() {
            println!("Memory: {} / {} MB", memory.used_mb, memory.total_mb);
        }
    }
    
    Ok(())
}

fn cmd_process(action: ProcessCommands) -> Result<()> {
    let proc_mgr = ProcessManager::new();
    
    match action {
        ProcessCommands::List => {
            if let Ok(processes) = proc_mgr.list_processes() {
                println!("PID\tName\tCPU%\tMEM%");
                for proc in processes {
                    println!("{}\t{}\t{:.1}\t{:.1}", 
                        proc.pid, proc.name, proc.cpu_percent, proc.mem_percent);
                }
            }
        }
        ProcessCommands::Info { pid } => {
            if let Ok(info) = proc_mgr.get_process_info(pid) {
                println!("Process {} - {}", pid, info.name);
                println!("State: {}", info.state);
                println!("CPU: {:.1}%", info.cpu_percent);
                println!("Memory: {:.1}%", info.mem_percent);
            }
        }
    }
    Ok(())
}

fn cmd_service(action: ServiceCommands) -> Result<()> {
    let svc_mgr = ServiceOps::new();
    
    match action {
        ServiceCommands::List => {
            if let Ok(services) = svc_mgr.list_services() {
                println!("Service\t\tStatus");
                for svc in services {
                    println!("{}\t\t{}", svc.name, svc.status);
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
                println!("{}: {}", name, status);
            }
        }
    }
    Ok(())
}

fn cmd_network(action: NetworkCommands) -> Result<()> {
    let net_mgr = NetOps::new();
    
    match action {
        NetworkCommands::List => {
            if let Ok(ifaces) = net_mgr.list_interfaces() {
                println!("Interface\t\tStatus\t\tIP");
                for iface in ifaces {
                    println!("{}\t\t{}\t\t{}", iface.name, iface.status, iface.ip);
                }
            }
        }
        NetworkCommands::Info { iface } => {
            if let Ok(info) = net_mgr.get_interface_info(&iface) {
                println!("Interface: {}", info.name);
                println!("Status: {}", info.status);
                println!("IP: {}", info.ip);
                println!("MAC: {}", info.mac);
            }
        }
    }
    Ok(())
}

fn cmd_disk(action: DiskCommands) -> Result<()> {
    let disk_mgr = DiskOps::new();
    
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
                println!("Available: {} GB", space.available_bytes / 1024 / 1024 / 1024);
                println!("Usage: {:.1}%", space.used_percent);
            }
        }
        DiskCommands::Mounts => {
            let mount_mgr = MountOps::new();
            if let Ok(mounts) = mount_mgr.list_mounts() {
                println!("Device\t\tMount Point\t\tType");
                for mount in mounts {
                    println!("{}\t\t{}\t\t{}", mount.device, mount.mount_point, mount.fs_type);
                }
            }
        }
    }
    Ok(())
}

fn cmd_monitor(action: MonitorCommands) -> Result<()> {
    match action {
        MonitorCommands::Cpu => {
            let cpu_mgr = CpuOps::new();
            if let Ok(info) = cpu_mgr.get_cpu_info() {
                println!("CPU Model: {}", info.model);
                println!("Cores: {}", info.cores);
                println!("Usage: {:.1}%", info.usage_percent);
            }
        }
        MonitorCommands::Memory => {
            let mem_mgr = MemOps::new();
            if let Ok(info) = mem_mgr.get_memory_info() {
                println!("Total: {} MB", info.total_mb);
                println!("Used: {} MB", info.used_mb);
                println!("Free: {} MB", info.free_mb);
                println!("Usage: {:.1}%", info.usage_percent);
            }
        }
        MonitorCommands::Thermal => {
            let thermal_mgr = ThermalOps::new();
            if let Ok(zones) = thermal_mgr.list_thermal_zones() {
                println!("Zone\t\tTemperature");
                for zone in zones {
                    println!("{}\t\t{} C", zone.name, zone.temperature);
                }
            }
        }
    }
    Ok(())
}
