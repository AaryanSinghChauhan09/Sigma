//! Hardware detection and configuration module inspired by Arch Linux hwinfo.
//! Provides hardware enumeration, driver management, and device configuration.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Cpu,
    Gpu,
    Ram,
    Storage,
    Network,
    Audio,
    Input,
    Usb,
    Pci,
    Smbus,
}

impl DeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Gpu => "gpu",
            Self::Ram => "ram",
            Self::Storage => "storage",
            Self::Network => "network",
            Self::Audio => "audio",
            Self::Input => "input",
            Self::Usb => "usb",
            Self::Pci => "pci",
            Self::Smbus => "smbus",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareError {
    InvalidDevice(String),
    DriverNotFound(String),
    UnsupportedDevice(String),
}

impl fmt::Display for HardwareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDevice(msg) => write!(f, "invalid device: {}", msg),
            Self::DriverNotFound(msg) => write!(f, "driver not found: {}", msg),
            Self::UnsupportedDevice(msg) => write!(f, "unsupported device: {}", msg),
        }
    }
}

impl std::error::Error for HardwareError {}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: String,
    pub device_type: DeviceType,
    pub vendor: String,
    pub model: String,
    pub driver: String,
    pub status: DeviceStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    Active,
    Inactive,
    Error,
    Unknown,
}

impl DeviceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Error => "error",
            Self::Unknown => "unknown",
        }
    }
}

impl DeviceInfo {
    pub fn new(id: impl Into<String>, device_type: DeviceType, vendor: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            device_type,
            vendor: vendor.into(),
            model: model.into(),
            driver: String::new(),
            status: DeviceStatus::Unknown,
        }
    }

    pub fn with_driver(mut self, driver: impl Into<String>) -> Self {
        self.driver = driver.into();
        self
    }

    pub fn with_status(mut self, status: DeviceStatus) -> Self {
        self.status = status;
        self
    }

    pub fn is_active(&self) -> bool {
        self.status == DeviceStatus::Active
    }

    pub fn needs_driver(&self) -> bool {
        self.driver.is_empty() && self.status == DeviceStatus::Unknown
    }
}

#[derive(Debug, Clone)]
pub struct HardwareManager {
    pub devices: BTreeMap<String, DeviceInfo>,
    pub drivers: BTreeMap<String, Vec<String>>,
    pub blacklisted: Vec<String>,
}

impl Default for HardwareManager {
    fn default() -> Self {
        Self {
            devices: BTreeMap::new(),
            drivers: BTreeMap::new(),
            blacklisted: vec![],
        }
    }
}

impl HardwareManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn detect_device(&mut self, device: DeviceInfo) -> Result<(), HardwareError> {
        let id = device.id.clone();
        if id.trim().is_empty() {
            return Err(HardwareError::InvalidDevice("device ID cannot be empty".to_string()));
        }
        self.devices.insert(id, device);
        Ok(())
    }

    pub fn get_device(&self, id: &str) -> Option<&DeviceInfo> {
        self.devices.get(id)
    }

    pub fn get_devices_by_type(&self, device_type: DeviceType) -> Vec<&DeviceInfo> {
        self.devices.values().filter(|d| d.device_type == device_type).collect()
    }

    pub fn get_active_devices(&self) -> Vec<&DeviceInfo> {
        self.devices.values().filter(|d| d.status == DeviceStatus::Active).collect()
    }

    pub fn get_devices_needing_drivers(&self) -> Vec<&DeviceInfo> {
        self.devices.values().filter(|d| d.needs_driver()).collect()
    }

    pub fn add_driver(&mut self, device_type: DeviceType, driver: impl Into<String>) {
        self.drivers.entry(device_type.as_str().to_string()).or_insert_with(Vec::new).push(driver.into());
    }

    pub fn get_drivers(&self, device_type: DeviceType) -> Vec<&String> {
        self.drivers.get(device_type.as_str()).map(|v| v.iter().collect()).unwrap_or_default()
    }

    pub fn blacklist_device(&mut self, device_id: &str) {
        if !self.blacklisted.contains(&device_id.to_string()) {
            self.blacklisted.push(device_id.to_string());
        }
    }

    pub fn is_blacklisted(&self, device_id: &str) -> bool {
        self.blacklisted.contains(&device_id.to_string())
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn active_device_count(&self) -> usize {
        self.get_active_devices().len()
    }

    pub fn generate_lspci_output(&self) -> String {
        let mut output = String::new();
        
        for device in self.devices.values().filter(|d| d.device_type == DeviceType::Pci) {
            output.push_str(&format!("{}: {} {} {} [{}]\n", 
                device.id, 
                device.vendor, 
                device.model, 
                device.driver, 
                device.status.as_str()
            ));
        }
        
        output
    }

    pub fn generate_lsusb_output(&self) -> String {
        let mut output = String::new();
        
        for device in self.devices.values().filter(|d| d.device_type == DeviceType::Usb) {
            output.push_str(&format!("{}: {} {} {} [{}]\n", 
                device.id, 
                device.vendor, 
                device.model, 
                device.driver, 
                device.status.as_str()
            ));
        }
        
        output
    }

    pub fn generate_dmesg_grep(&self, pattern: &str) -> String {
        let mut output = String::new();
        
        for device in self.devices.values() {
            if device.model.contains(pattern) || device.vendor.contains(pattern) {
                output.push_str(&format!("[{}] {} {}: {}\n", 
                    device.device_type.as_str(),
                    device.vendor, 
                    device.model, 
                    device.status.as_str()
                ));
            }
        }
        
        output
    }

    pub fn get_hardware_summary(&self) -> String {
        let mut summary = String::new();
        
        summary.push_str("# SigmaOS Hardware Summary\n");
        summary.push_str(&format!("# Total Devices: {}\n", self.device_count()));
        summary.push_str(&format!("# Active Devices: {}\n", self.active_device_count()));
        summary.push_str(&format!("# Devices Needing Drivers: {}\n\n", self.get_devices_needing_drivers().len()));
        
        for device_type in [DeviceType::Cpu, DeviceType::Gpu, DeviceType::Ram, DeviceType::Storage, DeviceType::Network, DeviceType::Audio] {
            let count = self.get_devices_by_type(device_type).len();
            summary.push_str(&format!("{}: {}\n", device_type.as_str(), count));
        }
        
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_type_as_str() {
        assert_eq!(DeviceType::Cpu.as_str(), "cpu");
        assert_eq!(DeviceType::Gpu.as_str(), "gpu");
    }

    #[test]
    fn test_device_status_as_str() {
        assert_eq!(DeviceStatus::Active.as_str(), "active");
        assert_eq!(DeviceStatus::Error.as_str(), "error");
    }

    #[test]
    fn test_device_info_new() {
        let device = DeviceInfo::new("cpu0", DeviceType::Cpu, "Intel", "Core i7");
        assert_eq!(device.id, "cpu0");
        assert_eq!(device.device_type, DeviceType::Cpu);
    }

    #[test]
    fn test_device_info_with_driver() {
        let device = DeviceInfo::new("gpu0", DeviceType::Gpu, "NVIDIA", "RTX 3080")
            .with_driver("nvidia")
            .with_status(DeviceStatus::Active);
        assert_eq!(device.driver, "nvidia");
        assert!(device.is_active());
    }

    #[test]
    fn test_device_info_needs_driver() {
        let device = DeviceInfo::new("wifi0", DeviceType::Network, "Intel", "AX200");
        assert!(device.needs_driver());
    }

    #[test]
    fn test_hardware_manager_new() {
        let mgr = HardwareManager::new();
        assert_eq!(mgr.device_count(), 0);
    }

    #[test]
    fn test_hardware_manager_detect_device() {
        let mut mgr = HardwareManager::new();
        let device = DeviceInfo::new("cpu0", DeviceType::Cpu, "Intel", "Core i7");
        mgr.detect_device(device).unwrap();
        assert_eq!(mgr.device_count(), 1);
    }

    #[test]
    fn test_hardware_manager_detect_device_invalid() {
        let mut mgr = HardwareManager::new();
        let device = DeviceInfo::new("", DeviceType::Cpu, "Intel", "Core i7");
        assert!(mgr.detect_device(device).is_err());
    }

    #[test]
    fn test_get_devices_by_type() {
        let mut mgr = HardwareManager::new();
        mgr.detect_device(DeviceInfo::new("cpu0", DeviceType::Cpu, "Intel", "Core i7")).unwrap();
        mgr.detect_device(DeviceInfo::new("cpu1", DeviceType::Cpu, "Intel", "Core i5")).unwrap();
        mgr.detect_device(DeviceInfo::new("gpu0", DeviceType::Gpu, "NVIDIA", "RTX 3080")).unwrap();
        
        let cpus = mgr.get_devices_by_type(DeviceType::Cpu);
        assert_eq!(cpus.len(), 2);
    }

    #[test]
    fn test_get_active_devices() {
        let mut mgr = HardwareManager::new();
        mgr.detect_device(DeviceInfo::new("cpu0", DeviceType::Cpu, "Intel", "Core i7")
            .with_status(DeviceStatus::Active)).unwrap();
        mgr.detect_device(DeviceInfo::new("gpu0", DeviceType::Gpu, "NVIDIA", "RTX 3080")
            .with_status(DeviceStatus::Inactive)).unwrap();
        
        let active = mgr.get_active_devices();
        assert_eq!(active.len(), 1);
    }

    #[test]
    fn test_get_devices_needing_drivers() {
        let mut mgr = HardwareManager::new();
        mgr.detect_device(DeviceInfo::new("wifi0", DeviceType::Network, "Intel", "AX200")).unwrap();
        mgr.detect_device(DeviceInfo::new("gpu0", DeviceType::Gpu, "NVIDIA", "RTX 3080")
            .with_driver("nvidia")).unwrap();
        
        let needing = mgr.get_devices_needing_drivers();
        assert_eq!(needing.len(), 1);
        assert_eq!(needing[0].id, "wifi0");
    }

    #[test]
    fn test_add_driver() {
        let mut mgr = HardwareManager::new();
        mgr.add_driver(DeviceType::Network, "iwlwifi");
        mgr.add_driver(DeviceType::Network, "ath9k");
        
        let drivers = mgr.get_drivers(DeviceType::Network);
        assert_eq!(drivers.len(), 2);
    }

    #[test]
    fn test_blacklist_device() {
        let mut mgr = HardwareManager::new();
        mgr.blacklist_device("bad_device");
        assert!(mgr.is_blacklisted("bad_device"));
        assert!(!mgr.is_blacklisted("good_device"));
    }

    #[test]
    fn test_generate_lspci_output() {
        let mut mgr = HardwareManager::new();
        mgr.detect_device(DeviceInfo::new("00:00.0", DeviceType::Pci, "Intel", "Host Bridge")
            .with_driver("intel_agp")).unwrap();
        
        let output = mgr.generate_lspci_output();
        assert!(output.contains("00:00.0"));
        assert!(output.contains("intel_agp"));
    }

    #[test]
    fn test_get_hardware_summary() {
        let mut mgr = HardwareManager::new();
        mgr.detect_device(DeviceInfo::new("cpu0", DeviceType::Cpu, "Intel", "Core i7")).unwrap();
        mgr.detect_device(DeviceInfo::new("gpu0", DeviceType::Gpu, "NVIDIA", "RTX 3080")).unwrap();
        
        let summary = mgr.get_hardware_summary();
        assert!(summary.contains("Total Devices: 2"));
        assert!(summary.contains("cpu: 1"));
        assert!(summary.contains("gpu: 1"));
    }
}
