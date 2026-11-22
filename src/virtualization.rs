//! SigmaOS virtualization module.
//!
//! This module provides abstractions for virtual machines, containers,
//! and virtualization management.
//!
//! # Example
//!
//! ```
//! use sigma_system::virtualization::*;
//!
//! let mut virt_mgr = VirtualizationManager::new();
//! virt_mgr.create_vm("test-vm", "debian", 2048, 2).unwrap();
//! virt_mgr.start_vm("test-vm").unwrap();
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in virtualization operations.
#[derive(Debug, Clone, PartialEq)]
pub enum VirtualizationError {
    /// VM not found.
    VmNotFound(String),
    /// VM already exists.
    VmExists(String),
    /// Invalid VM configuration.
    InvalidConfig(String),
    /// Insufficient resources.
    InsufficientResources(String),
    /// Virtualization not available.
    NotAvailable(String),
}

impl std::fmt::Display for VirtualizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VirtualizationError::VmNotFound(name) => write!(f, "VM '{}' not found", name),
            VirtualizationError::VmExists(name) => write!(f, "VM '{}' already exists", name),
            VirtualizationError::InvalidConfig(msg) => write!(f, "Invalid config: {}", msg),
            VirtualizationError::InsufficientResources(msg) => write!(f, "Insufficient resources: {}", msg),
            VirtualizationError::NotAvailable(msg) => write!(f, "Virtualization not available: {}", msg),
        }
    }
}

impl std::error::Error for VirtualizationError {}

/// VM status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmStatus {
    /// VM is stopped.
    Stopped,
    /// VM is running.
    Running,
    /// VM is paused.
    Paused,
    /// VM is suspended.
    Suspended,
    /// VM is being created.
    Creating,
    /// VM is being destroyed.
    Destroying,
}

impl VmStatus {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            VmStatus::Stopped => "stopped",
            VmStatus::Running => "running",
            VmStatus::Paused => "paused",
            VmStatus::Suspended => "suspended",
            VmStatus::Creating => "creating",
            VmStatus::Destroying => "destroying",
        }
    }
}

/// Virtual machine configuration.
#[derive(Debug, Clone)]
pub struct VmConfig {
    /// VM name.
    pub name: String,
    /// OS type.
    pub os_type: String,
    /// Memory in MB.
    pub memory_mb: u64,
    /// CPU cores.
    pub cpu_cores: u32,
    /// Disk size in GB.
    pub disk_size_gb: u64,
    /// Network configuration.
    pub network_enabled: bool,
    /// GPU passthrough.
    pub gpu_passthrough: bool,
    /// Snapshot support.
    pub snapshots_enabled: bool,
}

impl VmConfig {
    /// Create a new VM configuration.
    pub fn new(name: &str, os_type: &str, memory_mb: u64, cpu_cores: u32, disk_size_gb: u64) -> Result<Self, VirtualizationError> {
        if name.is_empty() {
            return Err(VirtualizationError::InvalidConfig("VM name cannot be empty".to_string()));
        }
        if memory_mb == 0 {
            return Err(VirtualizationError::InvalidConfig("Memory must be greater than 0".to_string()));
        }
        if cpu_cores == 0 {
            return Err(VirtualizationError::InvalidConfig("CPU cores must be greater than 0".to_string()));
        }
        if disk_size_gb == 0 {
            return Err(VirtualizationError::InvalidConfig("Disk size must be greater than 0".to_string()));
        }
        
        Ok(Self {
            name: name.to_string(),
            os_type: os_type.to_string(),
            memory_mb,
            cpu_cores,
            disk_size_gb,
            network_enabled: true,
            gpu_passthrough: false,
            snapshots_enabled: true,
        })
    }
}

/// Virtual machine.
#[derive(Debug, Clone)]
pub struct Vm {
    /// VM configuration.
    pub config: VmConfig,
    /// VM status.
    pub status: VmStatus,
    /// VM ID.
    pub id: String,
    /// Created timestamp.
    pub created_at: i64,
    /// Last started timestamp.
    pub last_started: Option<i64>,
    /// IP address (if network enabled).
    pub ip_address: Option<String>,
    /// Disk path.
    pub disk_path: String,
}

impl Vm {
    /// Create a new VM.
    pub fn new(config: VmConfig) -> Self {
        let id = format!("vm-{}-{}", config.name, std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos());
        let disk_path = format!("/var/lib/sigmaos/vms/{}/disk.qcow2", config.name);
        Self {
            config,
            status: VmStatus::Stopped,
            id,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            last_started: None,
            ip_address: None,
            disk_path,
        }
    }

    /// Start the VM.
    pub fn start(&mut self) {
        self.status = VmStatus::Running;
        self.last_started = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64);
    }

    /// Stop the VM.
    pub fn stop(&mut self) {
        self.status = VmStatus::Stopped;
    }

    /// Pause the VM.
    pub fn pause(&mut self) {
        self.status = VmStatus::Paused;
    }

    /// Resume the VM.
    pub fn resume(&mut self) {
        self.status = VmStatus::Running;
    }

    /// Suspend the VM.
    pub fn suspend(&mut self) {
        self.status = VmStatus::Suspended;
    }
}

/// Container configuration.
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    /// Container name.
    pub name: String,
    /// Image name.
    pub image: String,
    /// Memory limit in MB.
    pub memory_limit_mb: u64,
    /// CPU limit.
    pub cpu_limit: f64,
    /// Port mappings.
    pub ports: Vec<(u16, u16)>,
    /// Volume mounts.
    pub volumes: Vec<(String, String)>,
    /// Environment variables.
    pub env_vars: BTreeMap<String, String>,
}

impl ContainerConfig {
    /// Create a new container configuration.
    pub fn new(name: &str, image: &str) -> Result<Self, VirtualizationError> {
        if name.is_empty() || image.is_empty() {
            return Err(VirtualizationError::InvalidConfig("Name and image cannot be empty".to_string()));
        }
        Ok(Self {
            name: name.to_string(),
            image: image.to_string(),
            memory_limit_mb: 512,
            cpu_limit: 1.0,
            ports: Vec::new(),
            volumes: Vec::new(),
            env_vars: BTreeMap::new(),
        })
    }
}

/// Virtualization manager.
#[derive(Debug)]
pub struct VirtualizationManager {
    /// VMs by name.
    vms: BTreeMap<String, Vm>,
    /// Container configs by name.
    containers: BTreeMap<String, ContainerConfig>,
    /// Virtualization backend.
    backend: String,
    /// Available memory in MB.
    available_memory_mb: u64,
    /// Available CPU cores.
    available_cpu_cores: u32,
}

impl VirtualizationManager {
    /// Create a new virtualization manager.
    pub fn new() -> Self {
        Self {
            vms: BTreeMap::new(),
            containers: BTreeMap::new(),
            backend: "kvm".to_string(),
            available_memory_mb: 8192,
            available_cpu_cores: 4,
        }
    }

    /// Set virtualization backend.
    pub fn set_backend(&mut self, backend: &str) {
        self.backend = backend.to_string();
    }

    /// Get virtualization backend.
    pub fn get_backend(&self) -> &str {
        &self.backend
    }

    /// Create a VM.
    pub fn create_vm(&mut self, name: &str, os_type: &str, memory_mb: u64, cpu_cores: u32) -> Result<Vm, VirtualizationError> {
        if self.vms.contains_key(name) {
            return Err(VirtualizationError::VmExists(name.to_string()));
        }
        
        // Check resources
        let total_memory: u64 = self.vms.values().map(|vm| vm.config.memory_mb).sum();
        if total_memory + memory_mb > self.available_memory_mb {
            return Err(VirtualizationError::InsufficientResources("Memory".to_string()));
        }
        
        let total_cpu: u32 = self.vms.values().map(|vm| vm.config.cpu_cores).sum();
        if total_cpu + cpu_cores > self.available_cpu_cores {
            return Err(VirtualizationError::InsufficientResources("CPU".to_string()));
        }
        
        let config = VmConfig::new(name, os_type, memory_mb, cpu_cores, 20)?;
        let vm = Vm::new(config);
        self.vms.insert(name.to_string(), vm.clone());
        Ok(vm)
    }

    /// Remove a VM.
    pub fn remove_vm(&mut self, name: &str) -> Result<(), VirtualizationError> {
        if !self.vms.contains_key(name) {
            return Err(VirtualizationError::VmNotFound(name.to_string()));
        }
        self.vms.remove(name);
        Ok(())
    }

    /// Get a VM.
    pub fn get_vm(&self, name: &str) -> Option<&Vm> {
        self.vms.get(name)
    }

    /// Get all VMs.
    pub fn get_vms(&self) -> Vec<&Vm> {
        self.vms.values().collect()
    }

    /// Start a VM.
    pub fn start_vm(&mut self, name: &str) -> Result<(), VirtualizationError> {
        if let Some(vm) = self.vms.get_mut(name) {
            if vm.status == VmStatus::Stopped {
                vm.start();
                Ok(())
            } else {
                Err(VirtualizationError::InvalidConfig(format!("VM is not stopped: {}", vm.status.as_str())))
            }
        } else {
            Err(VirtualizationError::VmNotFound(name.to_string()))
        }
    }

    /// Stop a VM.
    pub fn stop_vm(&mut self, name: &str) -> Result<(), VirtualizationError> {
        if let Some(vm) = self.vms.get_mut(name) {
            if vm.status == VmStatus::Running {
                vm.stop();
                Ok(())
            } else {
                Err(VirtualizationError::InvalidConfig(format!("VM is not running: {}", vm.status.as_str())))
            }
        } else {
            Err(VirtualizationError::VmNotFound(name.to_string()))
        }
    }

    /// Pause a VM.
    pub fn pause_vm(&mut self, name: &str) -> Result<(), VirtualizationError> {
        if let Some(vm) = self.vms.get_mut(name) {
            if vm.status == VmStatus::Running {
                vm.pause();
                Ok(())
            } else {
                Err(VirtualizationError::InvalidConfig(format!("VM is not running: {}", vm.status.as_str())))
            }
        } else {
            Err(VirtualizationError::VmNotFound(name.to_string()))
        }
    }

    /// Resume a VM.
    pub fn resume_vm(&mut self, name: &str) -> Result<(), VirtualizationError> {
        if let Some(vm) = self.vms.get_mut(name) {
            if vm.status == VmStatus::Paused {
                vm.resume();
                Ok(())
            } else {
                Err(VirtualizationError::InvalidConfig(format!("VM is not paused: {}", vm.status.as_str())))
            }
        } else {
            Err(VirtualizationError::VmNotFound(name.to_string()))
        }
    }

    /// Create a container config.
    pub fn create_container(&mut self, name: &str, image: &str) -> Result<ContainerConfig, VirtualizationError> {
        if self.containers.contains_key(name) {
            return Err(VirtualizationError::VmExists(name.to_string()));
        }
        let config = ContainerConfig::new(name, image)?;
        self.containers.insert(name.to_string(), config.clone());
        Ok(config)
    }

    /// Remove a container config.
    pub fn remove_container(&mut self, name: &str) -> Result<(), VirtualizationError> {
        if !self.containers.contains_key(name) {
            return Err(VirtualizationError::VmNotFound(name.to_string()));
        }
        self.containers.remove(name);
        Ok(())
    }

    /// Get a container config.
    pub fn get_container(&self, name: &str) -> Option<&ContainerConfig> {
        self.containers.get(name)
    }

    /// Get all container configs.
    pub fn get_containers(&self) -> Vec<&ContainerConfig> {
        self.containers.values().collect()
    }

    /// Generate libvirt XML for a VM.
    pub fn generate_libvirt_xml(&self, name: &str) -> Result<String, VirtualizationError> {
        let vm = self.vms.get(name).ok_or_else(|| VirtualizationError::VmNotFound(name.to_string()))?;
        
        let mut xml = String::new();
        xml.push_str("<domain type=\"kvm\">\n");
        xml.push_str(&format!("  <name>{}</name>\n", vm.config.name));
        xml.push_str(&format!("  <memory unit=\"MiB\">{}</memory>\n", vm.config.memory_mb));
        xml.push_str(&format!("  <vcpu>{}</vcpu>\n", vm.config.cpu_cores));
        xml.push_str("  <os>\n");
        xml.push_str("    <type arch=\"x86_64\" machine=\"pc\">hvm</type>\n");
        xml.push_str("    <boot dev=\"hd\"/>\n");
        xml.push_str("  </os>\n");
        xml.push_str("  <devices>\n");
        xml.push_str("    <disk type=\"file\" device=\"disk\">\n");
        xml.push_str(&format!("      <source file=\"{}\"/>\n", vm.disk_path));
        xml.push_str("      <target dev=\"vda\" bus=\"virtio\"/>\n");
        xml.push_str("    </disk>\n");
        xml.push_str("    <interface type=\"network\">\n");
        xml.push_str("      <source network=\"default\"/>\n");
        xml.push_str("      <model type=\"virtio\"/>\n");
        xml.push_str("    </interface>\n");
        xml.push_str("  </devices>\n");
        xml.push_str("</domain>\n");
        
        Ok(xml)
    }

    /// Generate QEMU command for a VM.
    pub fn generate_qemu_command(&self, name: &str) -> Result<String, VirtualizationError> {
        let vm = self.vms.get(name).ok_or_else(|| VirtualizationError::VmNotFound(name.to_string()))?;
        
        let mut cmd = String::new();
        cmd.push_str("qemu-system-x86_64 ");
        cmd.push_str(&format!("-name {} ", vm.config.name));
        cmd.push_str(&format!("-m {} ", vm.config.memory_mb));
        cmd.push_str(&format!("-smp {} ", vm.config.cpu_cores));
        cmd.push_str(&format!("-drive file={},format=qcow2 ", vm.disk_path));
        cmd.push_str("-netdev user,id=net0 ");
        cmd.push_str("-device virtio-net-pci,netdev=net0 ");
        cmd.push_str("-enable-kvm ");
        cmd.push_str("-display gtk ");
        
        Ok(cmd)
    }

    /// Set available resources.
    pub fn set_available_resources(&mut self, memory_mb: u64, cpu_cores: u32) {
        self.available_memory_mb = memory_mb;
        self.available_cpu_cores = cpu_cores;
    }
}

impl Default for VirtualizationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let mut mgr = VirtualizationManager::new();
        let vm = mgr.create_vm("test-vm", "debian", 2048, 2).unwrap();
        assert_eq!(vm.config.name, "test-vm");
        assert_eq!(vm.config.memory_mb, 2048);
        assert_eq!(vm.config.cpu_cores, 2);
    }

    #[test]
    fn test_create_duplicate_vm() {
        let mut mgr = VirtualizationManager::new();
        mgr.create_vm("test-vm", "debian", 2048, 2).unwrap();
        let result = mgr.create_vm("test-vm", "ubuntu", 1024, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_start_vm() {
        let mut mgr = VirtualizationManager::new();
        mgr.create_vm("test-vm", "debian", 2048, 2).unwrap();
        mgr.start_vm("test-vm").unwrap();
        assert_eq!(mgr.get_vm("test-vm").unwrap().status, VmStatus::Running);
    }

    #[test]
    fn test_stop_vm() {
        let mut mgr = VirtualizationManager::new();
        mgr.create_vm("test-vm", "debian", 2048, 2).unwrap();
        mgr.start_vm("test-vm").unwrap();
        mgr.stop_vm("test-vm").unwrap();
        assert_eq!(mgr.get_vm("test-vm").unwrap().status, VmStatus::Stopped);
    }

    #[test]
    fn test_pause_resume_vm() {
        let mut mgr = VirtualizationManager::new();
        mgr.create_vm("test-vm", "debian", 2048, 2).unwrap();
        mgr.start_vm("test-vm").unwrap();
        mgr.pause_vm("test-vm").unwrap();
        assert_eq!(mgr.get_vm("test-vm").unwrap().status, VmStatus::Paused);
        mgr.resume_vm("test-vm").unwrap();
        assert_eq!(mgr.get_vm("test-vm").unwrap().status, VmStatus::Running);
    }

    #[test]
    fn test_insufficient_resources() {
        let mut mgr = VirtualizationManager::new();
        mgr.set_available_resources(1024, 1);
        let result = mgr.create_vm("test-vm", "debian", 2048, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_libvirt_xml() {
        let mut mgr = VirtualizationManager::new();
        mgr.create_vm("test-vm", "debian", 2048, 2).unwrap();
        let xml = mgr.generate_libvirt_xml("test-vm").unwrap();
        assert!(xml.contains("<domain"));
        assert!(xml.contains("test-vm"));
    }

    #[test]
    fn test_generate_qemu_command() {
        let mut mgr = VirtualizationManager::new();
        mgr.create_vm("test-vm", "debian", 2048, 2).unwrap();
        let cmd = mgr.generate_qemu_command("test-vm").unwrap();
        assert!(cmd.contains("qemu-system-x86_64"));
        assert!(cmd.contains("test-vm"));
    }
}
