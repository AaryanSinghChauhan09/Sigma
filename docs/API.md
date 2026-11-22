# SigmaOS API Reference

## Core Library (sigma-system)

The SigmaOS core library provides 100 modules for system management. All modules follow SOLID principles and provide consistent error handling.

## Module Categories

### Process/Kernel Modules

#### process
```rust
use sigma_system::process::*;

let process_mgr = ProcessManager::new();
let processes = process_mgr.list_processes()?;
```

#### cpu_ops
```rust
use sigma_system::cpu_ops::*;

let cpu = CpuOps::new();
let usage = cpu.get_cpu_usage()?;
```

#### mem_ops
```rust
use sigma_system::mem_ops::*;

let mem = MemOps::new();
let usage = mem.get_memory_usage()?;
```

#### kernel_ops
```rust
use sigma_system::kernel_ops::*;

let kernel = KernelOps::new();
let version = kernel.get_version()?;
```

### Storage Modules

#### disk
```rust
use sigma_system::disk::*;

let disk_mgr = DiskManager::new();
let disks = disk_mgr.list_disks()?;
```

#### fs_ops
```rust
use sigma_system::fs_ops::*;

let fs = FsOps::new();
let space = fs.get_disk_space("/")?;
```

### Network Modules

#### net_ops
```rust
use sigma_system::net_ops::*;

let net = NetOps::new();
let interfaces = net.list_interfaces()?;
```

#### netmgr_ops
```rust
use sigma_system::netmgr_ops::*;

let netmgr = NetMgrOps::new();
let connections = netmgr.list_connections()?;
```

### Service Modules

#### service
```rust
use sigma_system::service::*;

let service = ServiceManager::new();
let status = service.get_service_status("nginx")?;
```

#### servicemgr_ops
```rust
use sigma_system::servicemgr_ops::*;

let servicemgr = ServiceMgrOps::new();
servicemgr.start_service("nginx")?;
```

### Security Modules

#### security_audit
```rust
use sigma_system::security_audit::*;

let auditor = SecurityAuditor::new();
let report = auditor.audit_codebase("/path/to/code")?;
```

### Boot Modules

#### bootloader_ops
```rust
use sigma_system::bootloader_ops::*;

let bootloader = BootloaderOps::new();
bootloader.install_grub("/dev/sda")?;
```

#### initramfs_ops
```rust
use sigma_system::initramfs_ops::*;

let initramfs = InitramfsOps::new();
initramfs.generate()?;
```

### Package Modules

#### pkgmgr_ops
```rust
use sigma_system::pkgmgr_ops::*;

let pkgmgr = PkgMgrOps::new();
pkgmgr.install("nginx")?;
```

### Configuration Modules

#### sysconfig_ops
```rust
use sigma_system::sysconfig_ops::*;

let sysconfig = SysConfigOps::new();
sysconfig.set_config("key", "value")?;
```

### Backup Modules

#### backupmgr_ops
```rust
use sigma_system::backupmgr_ops::*;

let backupmgr = BackupMgrOps::new();
backupmgr.create_backup("/backup")?;
```

### Audit Modules

#### auditmgr_ops
```rust
use sigma_system::auditmgr_ops::*;

let auditmgr = AuditMgrOps::new();
let report = auditmgr.run_audit()?;
```

### Release Modules

#### releasemgr_ops
```rust
use sigma_system::releasemgr_ops::*;

let releasemgr = ReleaseMgrOps::new();
releasemgr.create_snapshot()?;
```

## Error Handling

All modules use consistent error handling:

```rust
pub enum ModuleError {
    CommandError(String),
    IoError(String),
    // Other specific errors
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::CommandError(msg) => write!(f, "Command error: {}", msg),
            ModuleError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ModuleError {}
```

## Default Implementation

All managers implement Default:

```rust
impl Default for ModuleOps {
    fn default() -> Self {
        Self::new()
    }
}
```

## Usage Pattern

```rust
use sigma_system::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create manager
    let cpu = CpuOps::new();
    
    // Use manager
    let usage = cpu.get_cpu_usage()?;
    println!("CPU Usage: {}", usage);
    
    Ok(())
}
```

## Module Count: 100

See [ARCHITECTURE.md](ARCHITECTURE.md) for complete module list.
