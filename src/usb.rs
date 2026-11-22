//! SigmaOS USB device management module.
//!
//! This module provides abstractions for USB device enumeration,
//! management, and configuration.
//!
//! # Example
//!
//! ```
//! use sigma_system::usb::*;
//!
//! let mut usb_mgr = UsbManager::new();
//! usb_mgr.scan_devices().unwrap();
//! usb_mgr.get_devices();
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in USB operations.
#[derive(Debug, Clone, PartialEq)]
pub enum UsbError {
    /// USB subsystem not available.
    NotAvailable(String),
    /// Device not found.
    DeviceNotFound(String),
    /// Device already connected.
    DeviceConnected(String),
    /// Device already disconnected.
    DeviceDisconnected(String),
    /// Invalid device ID.
    InvalidDeviceId(String),
    /// Permission denied.
    PermissionDenied(String),
}

impl std::fmt::Display for UsbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UsbError::NotAvailable(msg) => write!(f, "USB not available: {}", msg),
            UsbError::DeviceNotFound(id) => write!(f, "Device '{}' not found", id),
            UsbError::DeviceConnected(id) => write!(f, "Device '{}' already connected", id),
            UsbError::DeviceDisconnected(id) => write!(f, "Device '{}' already disconnected", id),
            UsbError::InvalidDeviceId(id) => write!(f, "Invalid device ID '{}'", id),
            UsbError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
        }
    }
}

impl std::error::Error for UsbError {}

/// USB device type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UsbDeviceType {
    /// Mass storage device.
    MassStorage,
    /// HID device (keyboard, mouse).
    Hid,
    /// Audio device.
    Audio,
    /// Video device.
    Video,
    /// Network adapter.
    Network,
    /// Bluetooth adapter.
    Bluetooth,
    /// Serial adapter.
    Serial,
    /// Printer.
    Printer,
    /// Camera.
    Camera,
    /// Unknown device.
    Unknown,
}

impl UsbDeviceType {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            UsbDeviceType::MassStorage => "mass-storage",
            UsbDeviceType::Hid => "hid",
            UsbDeviceType::Audio => "audio",
            UsbDeviceType::Video => "video",
            UsbDeviceType::Network => "network",
            UsbDeviceType::Bluetooth => "bluetooth",
            UsbDeviceType::Serial => "serial",
            UsbDeviceType::Printer => "printer",
            UsbDeviceType::Camera => "camera",
            UsbDeviceType::Unknown => "unknown",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Result<Self, UsbError> {
        match s.to_lowercase().as_str() {
            "mass-storage" | "storage" | "disk" => Ok(UsbDeviceType::MassStorage),
            "hid" | "keyboard" | "mouse" => Ok(UsbDeviceType::Hid),
            "audio" | "sound" => Ok(UsbDeviceType::Audio),
            "video" | "webcam" => Ok(UsbDeviceType::Video),
            "network" | "ethernet" | "wifi" => Ok(UsbDeviceType::Network),
            "bluetooth" | "bt" => Ok(UsbDeviceType::Bluetooth),
            "serial" | "com" => Ok(UsbDeviceType::Serial),
            "printer" => Ok(UsbDeviceType::Printer),
            "camera" => Ok(UsbDeviceType::Camera),
            "unknown" => Ok(UsbDeviceType::Unknown),
            _ => Err(UsbError::InvalidDeviceId(s.to_string())),
        }
    }
}

/// USB device status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbDeviceStatus {
    /// Device is connected.
    Connected,
    /// Device is disconnected.
    Disconnected,
    /// Device is suspended.
    Suspended,
    /// Device is error.
    Error,
}

impl UsbDeviceStatus {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            UsbDeviceStatus::Connected => "connected",
            UsbDeviceStatus::Disconnected => "disconnected",
            UsbDeviceStatus::Suspended => "suspended",
            UsbDeviceStatus::Error => "error",
        }
    }
}

/// USB device information.
#[derive(Debug, Clone)]
pub struct UsbDevice {
    /// Device ID (bus/device).
    pub id: String,
    /// Device path.
    pub path: String,
    /// Vendor ID.
    pub vendor_id: String,
    /// Product ID.
    pub product_id: String,
    /// Device type.
    pub device_type: UsbDeviceType,
    /// Device status.
    pub status: UsbDeviceStatus,
    /// Device name.
    pub name: String,
    /// Manufacturer.
    pub manufacturer: String,
    /// Serial number.
    pub serial_number: String,
    /// Whether device is authorized.
    pub authorized: bool,
}

impl UsbDevice {
    /// Create a new USB device.
    pub fn new(id: &str, path: &str, vendor_id: &str, product_id: &str) -> Result<Self, UsbError> {
        if id.is_empty() || path.is_empty() {
            return Err(UsbError::InvalidDeviceId(format!("{}:{}", id, path)));
        }
        
        Ok(Self {
            id: id.to_string(),
            path: path.to_string(),
            vendor_id: vendor_id.to_string(),
            product_id: product_id.to_string(),
            device_type: UsbDeviceType::Unknown,
            status: UsbDeviceStatus::Connected,
            name: String::new(),
            manufacturer: String::new(),
            serial_number: String::new(),
            authorized: true,
        })
    }

    /// Set device type.
    pub fn set_device_type(&mut self, device_type: UsbDeviceType) {
        self.device_type = device_type;
    }

    /// Set device status.
    pub fn set_status(&mut self, status: UsbDeviceStatus) {
        self.status = status;
    }

    /// Authorize device.
    pub fn authorize(&mut self) {
        self.authorized = true;
    }

    /// Deauthorize device.
    pub fn deauthorize(&mut self) {
        self.authorized = false;
    }
}

/// USB hub.
#[derive(Debug, Clone)]
pub struct UsbHub {
    /// Hub ID.
    pub id: String,
    /// Hub path.
    pub path: String,
    /// Number of ports.
    pub ports: u8,
    /// Whether hub is powered.
    pub powered: bool,
}

impl UsbHub {
    /// Create a new USB hub.
    pub fn new(id: &str, path: &str, ports: u8) -> Self {
        Self {
            id: id.to_string(),
            path: path.to_string(),
            ports,
            powered: true,
        }
    }

    /// Power on hub.
    pub fn power_on(&mut self) {
        self.powered = true;
    }

    /// Power off hub.
    pub fn power_off(&mut self) {
        self.powered = false;
    }
}

/// USB manager.
#[derive(Debug)]
pub struct UsbManager {
    /// USB devices by ID.
    devices: BTreeMap<String, UsbDevice>,
    /// USB hubs by ID.
    hubs: BTreeMap<String, UsbHub>,
    /// USB subsystem enabled.
    enabled: bool,
    /// Auto-mount enabled.
    auto_mount: bool,
}

impl UsbManager {
    /// Create a new USB manager.
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            hubs: BTreeMap::new(),
            enabled: true,
            auto_mount: true,
        }
    }

    /// Enable USB subsystem.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable USB subsystem.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if USB is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set auto-mount.
    pub fn set_auto_mount(&mut self, enabled: bool) {
        self.auto_mount = enabled;
    }

    /// Check if auto-mount is enabled.
    pub fn is_auto_mount(&self) -> bool {
        self.auto_mount
    }

    /// Scan for USB devices.
    pub fn scan_devices(&mut self) -> Result<Vec<&UsbDevice>, UsbError> {
        if !self.enabled {
            return Err(UsbError::NotAvailable("USB subsystem is disabled".to_string()));
        }
        // Simulate scan - in real implementation, this would use sysfs
        Ok(self.devices.values().collect())
    }

    /// Add a USB device.
    pub fn add_device(&mut self, device: UsbDevice) {
        self.devices.insert(device.id.clone(), device);
    }

    /// Remove a USB device.
    pub fn remove_device(&mut self, id: &str) -> Result<(), UsbError> {
        if self.devices.remove(id).is_some() {
            Ok(())
        } else {
            Err(UsbError::DeviceNotFound(id.to_string()))
        }
    }

    /// Get a USB device.
    pub fn get_device(&self, id: &str) -> Option<&UsbDevice> {
        self.devices.get(id)
    }

    /// Get all USB devices.
    pub fn get_devices(&self) -> Vec<&UsbDevice> {
        self.devices.values().collect()
    }

    /// Get devices by type.
    pub fn get_devices_by_type(&self, device_type: UsbDeviceType) -> Vec<&UsbDevice> {
        self.devices.values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    /// Add a USB hub.
    pub fn add_hub(&mut self, hub: UsbHub) {
        self.hubs.insert(hub.id.clone(), hub);
    }

    /// Remove a USB hub.
    pub fn remove_hub(&mut self, id: &str) -> Result<(), UsbError> {
        if self.hubs.remove(id).is_some() {
            Ok(())
        } else {
            Err(UsbError::DeviceNotFound(id.to_string()))
        }
    }

    /// Get a USB hub.
    pub fn get_hub(&self, id: &str) -> Option<&UsbHub> {
        self.hubs.get(id)
    }

    /// Get all USB hubs.
    pub fn get_hubs(&self) -> Vec<&UsbHub> {
        self.hubs.values().collect()
    }

    /// Authorize a device.
    pub fn authorize_device(&mut self, id: &str) -> Result<(), UsbError> {
        if let Some(device) = self.devices.get_mut(id) {
            device.authorize();
            Ok(())
        } else {
            Err(UsbError::DeviceNotFound(id.to_string()))
        }
    }

    /// Deauthorize a device.
    pub fn deauthorize_device(&mut self, id: &str) -> Result<(), UsbError> {
        if let Some(device) = self.devices.get_mut(id) {
            device.deauthorize();
            Ok(())
        } else {
            Err(UsbError::DeviceNotFound(id.to_string()))
        }
    }

    /// Suspend a device.
    pub fn suspend_device(&mut self, id: &str) -> Result<(), UsbError> {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_status(UsbDeviceStatus::Suspended);
            Ok(())
        } else {
            Err(UsbError::DeviceNotFound(id.to_string()))
        }
    }

    /// Resume a device.
    pub fn resume_device(&mut self, id: &str) -> Result<(), UsbError> {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_status(UsbDeviceStatus::Connected);
            Ok(())
        } else {
            Err(UsbError::DeviceNotFound(id.to_string()))
        }
    }

    /// Generate udev rules for USB.
    pub fn generate_udev_rules(&self) -> String {
        let mut rules = String::new();
        rules.push_str("# SigmaOS USB udev rules\n\n");
        rules.push_str("SUBSYSTEM==\"usb\", ACTION==\"add\", RUN+=\"/usr/bin/sigmaos-usb-init\"\n");
        rules.push_str("SUBSYSTEM==\"block\", KERNEL==\"sd*\", ATTRS{vendor}==\"*\", RUN+=\"/usr/bin/sigmaos-usb-mount\"\n");
        rules
    }

    /// Generate fstab entries for USB devices.
    pub fn generate_fstab_entries(&self) -> String {
        let mut entries = String::new();
        entries.push_str("# SigmaOS USB fstab entries\n\n");
        
        for device in self.devices.values() {
            if device.device_type == UsbDeviceType::MassStorage && device.authorized {
                entries.push_str(&format!("# USB Device: {} ({})\n", device.name, device.id));
                entries.push_str(&format!("UUID=usb-{} /mnt/usb-{} auto noauto,user,rw 0 0\n", device.id, device.id));
            }
        }
        
        entries
    }

    /// Add default USB configuration.
    pub fn add_default_config(&mut self) {
        // Add common USB hubs
        let hub1 = UsbHub::new("usb1", "/dev/bus/usb/001", 4);
        let hub2 = UsbHub::new("usb2", "/dev/bus/usb/002", 4);
        self.add_hub(hub1);
        self.add_hub(hub2);
    }
}

impl Default for UsbManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_devices() {
        let mut mgr = UsbManager::new();
        let devices = mgr.scan_devices().unwrap();
        assert!(devices.is_empty());
    }

    #[test]
    fn test_add_device() {
        let mut mgr = UsbManager::new();
        let device = UsbDevice::new("1-1", "/dev/bus/usb/001/001", "1234", "5678").unwrap();
        mgr.add_device(device);
        assert_eq!(mgr.get_devices().len(), 1);
    }

    #[test]
    fn test_remove_device() {
        let mut mgr = UsbManager::new();
        let device = UsbDevice::new("1-1", "/dev/bus/usb/001/001", "1234", "5678").unwrap();
        mgr.add_device(device);
        mgr.remove_device("1-1").unwrap();
        assert!(mgr.get_device("1-1").is_none());
    }

    #[test]
    fn test_get_devices_by_type() {
        let mut mgr = UsbManager::new();
        let mut device1 = UsbDevice::new("1-1", "/dev/bus/usb/001/001", "1234", "5678").unwrap();
        device1.set_device_type(UsbDeviceType::MassStorage);
        let mut device2 = UsbDevice::new("1-2", "/dev/bus/usb/001/002", "1234", "5679").unwrap();
        device2.set_device_type(UsbDeviceType::Hid);
        mgr.add_device(device1);
        mgr.add_device(device2);
        let storage_devices = mgr.get_devices_by_type(UsbDeviceType::MassStorage);
        assert_eq!(storage_devices.len(), 1);
    }

    #[test]
    fn test_authorize_device() {
        let mut mgr = UsbManager::new();
        let device = UsbDevice::new("1-1", "/dev/bus/usb/001/001", "1234", "5678").unwrap();
        mgr.add_device(device);
        mgr.deauthorize_device("1-1").unwrap();
        assert!(!mgr.get_device("1-1").unwrap().authorized);
        mgr.authorize_device("1-1").unwrap();
        assert!(mgr.get_device("1-1").unwrap().authorized);
    }

    #[test]
    fn test_suspend_resume_device() {
        let mut mgr = UsbManager::new();
        let device = UsbDevice::new("1-1", "/dev/bus/usb/001/001", "1234", "5678").unwrap();
        mgr.add_device(device);
        mgr.suspend_device("1-1").unwrap();
        assert_eq!(mgr.get_device("1-1").unwrap().status, UsbDeviceStatus::Suspended);
        mgr.resume_device("1-1").unwrap();
        assert_eq!(mgr.get_device("1-1").unwrap().status, UsbDeviceStatus::Connected);
    }

    #[test]
    fn test_generate_udev_rules() {
        let mgr = UsbManager::new();
        let rules = mgr.generate_udev_rules();
        assert!(rules.contains("SUBSYSTEM"));
        assert!(rules.contains("usb"));
    }

    #[test]
    fn test_device_type_parse() {
        assert_eq!(UsbDeviceType::parse("mass-storage").unwrap(), UsbDeviceType::MassStorage);
        assert_eq!(UsbDeviceType::parse("hid").unwrap(), UsbDeviceType::Hid);
        assert!(UsbDeviceType::parse("invalid").is_err());
    }
}
