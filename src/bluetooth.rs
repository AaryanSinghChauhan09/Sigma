//! SigmaOS Bluetooth management module.
//!
//! This module provides abstractions for Bluetooth device management,
//! pairing, and connection control.
//!
//! # Example
//!
//! ```
//! use sigma_system::bluetooth::*;
//!
//! let mut bt_mgr = BluetoothManager::new();
//! bt_mgr.enable_bluetooth().unwrap();
//! bt_mgr.scan_devices().unwrap();
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in Bluetooth operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BluetoothError {
    /// Bluetooth not available.
    NotAvailable(String),
    /// Device not found.
    DeviceNotFound(String),
    /// Device already paired.
    DevicePaired(String),
    /// Pairing failed.
    PairingFailed(String),
    /// Connection failed.
    ConnectionFailed(String),
    /// Invalid device address.
    InvalidAddress(String),
}

impl std::fmt::Display for BluetoothError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BluetoothError::NotAvailable(msg) => write!(f, "Bluetooth not available: {}", msg),
            BluetoothError::DeviceNotFound(addr) => write!(f, "Device '{}' not found", addr),
            BluetoothError::DevicePaired(addr) => write!(f, "Device '{}' already paired", addr),
            BluetoothError::PairingFailed(addr) => write!(f, "Pairing failed for '{}'", addr),
            BluetoothError::ConnectionFailed(addr) => write!(f, "Connection failed for '{}'", addr),
            BluetoothError::InvalidAddress(addr) => write!(f, "Invalid device address '{}'", addr),
        }
    }
}

impl std::error::Error for BluetoothError {}

/// Bluetooth device type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BluetoothDeviceType {
    /// Mouse.
    Mouse,
    /// Keyboard.
    Keyboard,
    /// Headphones.
    Headphones,
    /// Speaker.
    Speaker,
    /// Phone.
    Phone,
    /// Tablet.
    Tablet,
    /// Computer.
    Computer,
    /// Watch.
    Watch,
    /// Tracker.
    Tracker,
    /// Unknown.
    Unknown,
}

impl BluetoothDeviceType {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            BluetoothDeviceType::Mouse => "mouse",
            BluetoothDeviceType::Keyboard => "keyboard",
            BluetoothDeviceType::Headphones => "headphones",
            BluetoothDeviceType::Speaker => "speaker",
            BluetoothDeviceType::Phone => "phone",
            BluetoothDeviceType::Tablet => "tablet",
            BluetoothDeviceType::Computer => "computer",
            BluetoothDeviceType::Watch => "watch",
            BluetoothDeviceType::Tracker => "tracker",
            BluetoothDeviceType::Unknown => "unknown",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Result<Self, BluetoothError> {
        match s.to_lowercase().as_str() {
            "mouse" => Ok(BluetoothDeviceType::Mouse),
            "keyboard" => Ok(BluetoothDeviceType::Keyboard),
            "headphones" | "headset" => Ok(BluetoothDeviceType::Headphones),
            "speaker" => Ok(BluetoothDeviceType::Speaker),
            "phone" => Ok(BluetoothDeviceType::Phone),
            "tablet" => Ok(BluetoothDeviceType::Tablet),
            "computer" | "pc" => Ok(BluetoothDeviceType::Computer),
            "watch" => Ok(BluetoothDeviceType::Watch),
            "tracker" => Ok(BluetoothDeviceType::Tracker),
            "unknown" => Ok(BluetoothDeviceType::Unknown),
            _ => Err(BluetoothError::InvalidAddress(s.to_string())),
        }
    }
}

/// Bluetooth device status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothDeviceStatus {
    /// Device is disconnected.
    Disconnected,
    /// Device is connecting.
    Connecting,
    /// Device is connected.
    Connected,
    /// Device is paired.
    Paired,
    /// Device is trusted.
    Trusted,
    /// Device is blocked.
    Blocked,
}

impl BluetoothDeviceStatus {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            BluetoothDeviceStatus::Disconnected => "disconnected",
            BluetoothDeviceStatus::Connecting => "connecting",
            BluetoothDeviceStatus::Connected => "connected",
            BluetoothDeviceStatus::Paired => "paired",
            BluetoothDeviceStatus::Trusted => "trusted",
            BluetoothDeviceStatus::Blocked => "blocked",
        }
    }
}

/// Bluetooth device.
#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    /// Device MAC address.
    pub address: String,
    /// Device name.
    pub name: String,
    /// Device type.
    pub device_type: BluetoothDeviceType,
    /// Device status.
    pub status: BluetoothDeviceStatus,
    /// RSSI (signal strength).
    pub rssi: i32,
    /// Whether device is discoverable.
    pub discoverable: bool,
    /// Whether device is paired.
    pub paired: bool,
    /// Whether device is trusted.
    pub trusted: bool,
}

impl BluetoothDevice {
    /// Create a new Bluetooth device.
    pub fn new(address: &str, name: &str, device_type: BluetoothDeviceType) -> Result<Self, BluetoothError> {
        // Validate MAC address format (simplified)
        if !address.contains(':') || address.len() != 17 {
            return Err(BluetoothError::InvalidAddress(address.to_string()));
        }
        
        Ok(Self {
            address: address.to_string(),
            name: name.to_string(),
            device_type,
            status: BluetoothDeviceStatus::Disconnected,
            rssi: -100,
            discoverable: false,
            paired: false,
            trusted: false,
        })
    }

    /// Set device status.
    pub fn set_status(&mut self, status: BluetoothDeviceStatus) {
        self.status = status;
    }

    /// Pair with device.
    pub fn pair(&mut self) -> Result<(), BluetoothError> {
        if self.paired {
            return Err(BluetoothError::DevicePaired(self.address.clone()));
        }
        self.paired = true;
        self.status = BluetoothDeviceStatus::Paired;
        Ok(())
    }

    /// Connect to device.
    pub fn connect(&mut self) -> Result<(), BluetoothError> {
        if !self.paired {
            return Err(BluetoothError::PairingFailed(self.address.clone()));
        }
        self.status = BluetoothDeviceStatus::Connected;
        Ok(())
    }

    /// Disconnect from device.
    pub fn disconnect(&mut self) {
        self.status = BluetoothDeviceStatus::Disconnected;
    }

    /// Trust device.
    pub fn trust(&mut self) {
        self.trusted = true;
        self.status = BluetoothDeviceStatus::Trusted;
    }

    /// Block device.
    pub fn block(&mut self) {
        self.status = BluetoothDeviceStatus::Blocked;
    }
}

/// Bluetooth adapter.
#[derive(Debug, Clone)]
pub struct BluetoothAdapter {
    /// Adapter name.
    pub name: String,
    /// Adapter MAC address.
    pub address: String,
    /// Whether adapter is powered on.
    pub powered: bool,
    /// Whether adapter is discoverable.
    pub discoverable: bool,
    /// Whether adapter is discoverable timeout.
    pub discoverable_timeout: u32,
}

impl BluetoothAdapter {
    /// Create a new Bluetooth adapter.
    pub fn new(name: &str, address: &str) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
            powered: false,
            discoverable: false,
            discoverable_timeout: 180,
        }
    }

    /// Power on adapter.
    pub fn power_on(&mut self) {
        self.powered = true;
    }

    /// Power off adapter.
    pub fn power_off(&mut self) {
        self.powered = false;
        self.discoverable = false;
    }

    /// Set discoverable.
    pub fn set_discoverable(&mut self, discoverable: bool) {
        self.discoverable = discoverable;
    }

    /// Set discoverable timeout.
    pub fn set_discoverable_timeout(&mut self, timeout: u32) {
        self.discoverable_timeout = timeout;
    }
}

/// Bluetooth manager.
#[derive(Debug)]
pub struct BluetoothManager {
    /// Bluetooth adapter.
    adapter: Option<BluetoothAdapter>,
    /// Paired devices by address.
    devices: BTreeMap<String, BluetoothDevice>,
    /// Discovered devices by address.
    discovered: BTreeMap<String, BluetoothDevice>,
    /// Bluetooth enabled.
    enabled: bool,
}

impl BluetoothManager {
    /// Create a new Bluetooth manager.
    pub fn new() -> Self {
        Self {
            adapter: None,
            devices: BTreeMap::new(),
            discovered: BTreeMap::new(),
            enabled: false,
        }
    }

    /// Enable Bluetooth.
    pub fn enable_bluetooth(&mut self) -> Result<(), BluetoothError> {
        self.enabled = true;
        // Create default adapter if none exists
        if self.adapter.is_none() {
            self.adapter = Some(BluetoothAdapter::new("hci0", "00:11:22:33:44:55"));
        }
        if let Some(adapter) = &mut self.adapter {
            adapter.power_on();
        }
        Ok(())
    }

    /// Disable Bluetooth.
    pub fn disable_bluetooth(&mut self) {
        self.enabled = false;
        if let Some(adapter) = &mut self.adapter {
            adapter.power_off();
        }
    }

    /// Check if Bluetooth is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get adapter.
    pub fn get_adapter(&self) -> Option<&BluetoothAdapter> {
        self.adapter.as_ref()
    }

    /// Set adapter.
    pub fn set_adapter(&mut self, adapter: BluetoothAdapter) {
        self.adapter = Some(adapter);
    }

    /// Scan for devices.
    pub fn scan_devices(&mut self) -> Result<Vec<&BluetoothDevice>, BluetoothError> {
        if !self.enabled {
            return Err(BluetoothError::NotAvailable("Bluetooth is disabled".to_string()));
        }
        // Simulate scan - in real implementation, this would use D-Bus/BlueZ
        // For now, return empty list
        Ok(self.discovered.values().collect())
    }

    /// Add a discovered device.
    pub fn add_discovered_device(&mut self, device: BluetoothDevice) {
        self.discovered.insert(device.address.clone(), device);
    }

    /// Pair with a device.
    pub fn pair_device(&mut self, address: &str) -> Result<(), BluetoothError> {
        if let Some(device) = self.discovered.get_mut(address) {
            device.pair()?;
            // Move to paired devices
            let device = device.clone();
            self.devices.insert(address.to_string(), device);
            self.discovered.remove(address);
            Ok(())
        } else if let Some(device) = self.devices.get_mut(address) {
            device.pair()
        } else {
            Err(BluetoothError::DeviceNotFound(address.to_string()))
        }
    }

    /// Connect to a device.
    pub fn connect_device(&mut self, address: &str) -> Result<(), BluetoothError> {
        if let Some(device) = self.devices.get_mut(address) {
            device.connect()
        } else {
            Err(BluetoothError::DeviceNotFound(address.to_string()))
        }
    }

    /// Disconnect from a device.
    pub fn disconnect_device(&mut self, address: &str) -> Result<(), BluetoothError> {
        if let Some(device) = self.devices.get_mut(address) {
            device.disconnect();
            Ok(())
        } else {
            Err(BluetoothError::DeviceNotFound(address.to_string()))
        }
    }

    /// Get a paired device.
    pub fn get_device(&self, address: &str) -> Option<&BluetoothDevice> {
        self.devices.get(address)
    }

    /// Get all paired devices.
    pub fn get_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices.values().collect()
    }

    /// Get all discovered devices.
    pub fn get_discovered_devices(&self) -> Vec<&BluetoothDevice> {
        self.discovered.values().collect()
    }

    /// Remove a paired device.
    pub fn remove_device(&mut self, address: &str) -> Result<(), BluetoothError> {
        if self.devices.remove(address).is_some() {
            Ok(())
        } else {
            Err(BluetoothError::DeviceNotFound(address.to_string()))
        }
    }

    /// Generate BlueZ configuration.
    pub fn generate_bluez_config(&self) -> String {
        let mut config = String::new();
        config.push_str("# SigmaOS BlueZ Configuration\n\n");
        
        if let Some(adapter) = &self.adapter {
            config.push_str("[Adapter]\n");
            config.push_str(&format!("Name={}\n", adapter.name));
            config.push_str(&format!("Address={}\n", adapter.address));
            config.push_str(&format!("Powered={}\n", if adapter.powered { "on" } else { "off" }));
            config.push_str(&format!("Discoverable={}\n", if adapter.discoverable { "on" } else { "off" }));
            config.push_str(&format!("DiscoverableTimeout={}\n", adapter.discoverable_timeout));
            config.push_str("\n");
        }
        
        for device in self.devices.values() {
            config.push_str(&format!("[Device {}]\n", device.address));
            config.push_str(&format!("Name={}\n", device.name));
            config.push_str(&format!("Type={}\n", device.device_type.as_str()));
            config.push_str(&format!("Status={}\n", device.status.as_str()));
            config.push_str(&format!("Paired={}\n", if device.paired { "yes" } else { "no" }));
            config.push_str(&format!("Trusted={}\n", if device.trusted { "yes" } else { "no" }));
            config.push_str("\n");
        }
        
        config
    }

    /// Generate udev rules for Bluetooth.
    pub fn generate_udev_rules(&self) -> String {
        let mut rules = String::new();
        rules.push_str("# SigmaOS Bluetooth udev rules\n\n");
        rules.push_str("SUBSYSTEM==\"bluetooth\", ACTION==\"add\", RUN+=\"/usr/bin/sigmaos-bluetooth-init\"\n");
        rules.push_str("SUBSYSTEM==\"input\", KERNEL==\"event*\", ATTRS{name}==\"*Bluetooth*\", SYMLINK+=\"input/bluetooth/%k\"\n");
        rules
    }
}

impl Default for BluetoothManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enable_bluetooth() {
        let mut mgr = BluetoothManager::new();
        mgr.enable_bluetooth().unwrap();
        assert!(mgr.is_enabled());
        assert!(mgr.get_adapter().is_some());
    }

    #[test]
    fn test_disable_bluetooth() {
        let mut mgr = BluetoothManager::new();
        mgr.enable_bluetooth().unwrap();
        mgr.disable_bluetooth();
        assert!(!mgr.is_enabled());
    }

    #[test]
    fn test_add_discovered_device() {
        let mut mgr = BluetoothManager::new();
        mgr.enable_bluetooth().unwrap();
        let device = BluetoothDevice::new("00:11:22:33:44:55", "Test Device", BluetoothDeviceType::Headphones).unwrap();
        mgr.add_discovered_device(device);
        assert_eq!(mgr.get_discovered_devices().len(), 1);
    }

    #[test]
    fn test_pair_device() {
        let mut mgr = BluetoothManager::new();
        mgr.enable_bluetooth().unwrap();
        let device = BluetoothDevice::new("00:11:22:33:44:55", "Test Device", BluetoothDeviceType::Headphones).unwrap();
        mgr.add_discovered_device(device);
        mgr.pair_device("00:11:22:33:44:55").unwrap();
        assert!(mgr.get_device("00:11:22:33:44:55").is_some());
    }

    #[test]
    fn test_connect_device() {
        let mut mgr = BluetoothManager::new();
        mgr.enable_bluetooth().unwrap();
        let device = BluetoothDevice::new("00:11:22:33:44:55", "Test Device", BluetoothDeviceType::Headphones).unwrap();
        mgr.add_discovered_device(device);
        mgr.pair_device("00:11:22:33:44:55").unwrap();
        mgr.connect_device("00:11:22:33:44:55").unwrap();
        assert_eq!(mgr.get_device("00:11:22:33:44:55").unwrap().status, BluetoothDeviceStatus::Connected);
    }

    #[test]
    fn test_disconnect_device() {
        let mut mgr = BluetoothManager::new();
        mgr.enable_bluetooth().unwrap();
        let device = BluetoothDevice::new("00:11:22:33:44:55", "Test Device", BluetoothDeviceType::Headphones).unwrap();
        mgr.add_discovered_device(device);
        mgr.pair_device("00:11:22:33:44:55").unwrap();
        mgr.connect_device("00:11:22:33:44:55").unwrap();
        mgr.disconnect_device("00:11:22:33:44:55").unwrap();
        assert_eq!(mgr.get_device("00:11:22:33:44:55").unwrap().status, BluetoothDeviceStatus::Disconnected);
    }

    #[test]
    fn test_generate_bluez_config() {
        let mut mgr = BluetoothManager::new();
        mgr.enable_bluetooth().unwrap();
        let device = BluetoothDevice::new("00:11:22:33:44:55", "Test Device", BluetoothDeviceType::Headphones).unwrap();
        mgr.add_discovered_device(device);
        mgr.pair_device("00:11:22:33:44:55").unwrap();
        let config = mgr.generate_bluez_config();
        assert!(config.contains("[Adapter]"));
        assert!(config.contains("00:11:22:33:44:55"));
    }

    #[test]
    fn test_device_type_parse() {
        assert_eq!(BluetoothDeviceType::parse("mouse").unwrap(), BluetoothDeviceType::Mouse);
        assert_eq!(BluetoothDeviceType::parse("headphones").unwrap(), BluetoothDeviceType::Headphones);
        assert!(BluetoothDeviceType::parse("invalid").is_err());
    }
}
