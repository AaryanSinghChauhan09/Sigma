//! SigmaOS battery and power status module.
//!
//! This module provides actual battery and power information using /sys/class/power_supply.
//!
//! # Example
//!
//! ```
//! use sigma_system::power_status::*;
//!
//! let mut power = PowerStatus::new();
//! if let Ok(battery) = power.get_battery_info() {
//!     println!("Battery: {}%", battery.level);
//! }
//! ```

use std::fs;
use std::path::Path;
use std::collections::BTreeMap;

/// Errors that can occur in power operations.
#[derive(Debug, Clone, PartialEq)]
pub enum PowerError {
    /// Power supply not found.
    PowerSupplyNotFound(String),
    /// Failed to read power info.
    ReadError(String),
    /// Invalid power data.
    InvalidData(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for PowerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerError::PowerSupplyNotFound(name) => write!(f, "Power supply '{}' not found", name),
            PowerError::ReadError(msg) => write!(f, "Failed to read power info: {}", msg),
            PowerError::InvalidData(msg) => write!(f, "Invalid power data: {}", msg),
            PowerError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for PowerError {}

/// Battery information.
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    /// Battery name.
    pub name: String,
    /// Battery level percentage (0-100).
    pub level: u32,
    /// Battery status (charging, discharging, full, etc.).
    pub status: BatteryStatus,
    /// Battery capacity in µAh.
    pub capacity: u64,
    /// Battery voltage in µV.
    pub voltage: u64,
    /// Battery current in µA.
    pub current: u64,
    /// Battery temperature in tenths of degree Celsius.
    pub temperature: f64,
    /// Time to empty in seconds (if discharging).
    pub time_to_empty: Option<u64>,
    /// Time to full in seconds (if charging).
    pub time_to_full: Option<u64>,
    /// Battery technology.
    pub technology: String,
    /// Battery model.
    pub model: String,
}

/// Battery status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    /// Battery is charging.
    Charging,
    /// Battery is discharging.
    Discharging,
    /// Battery is full.
    Full,
    /// Battery is not charging.
    NotCharging,
    /// Battery is unknown.
    Unknown,
}

impl BatteryStatus {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            BatteryStatus::Charging => "charging",
            BatteryStatus::Discharging => "discharging",
            BatteryStatus::Full => "full",
            BatteryStatus::NotCharging => "not_charging",
            BatteryStatus::Unknown => "unknown",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "charging" => BatteryStatus::Charging,
            "discharging" => BatteryStatus::Discharging,
            "full" => BatteryStatus::Full,
            "not charging" | "not_charging" => BatteryStatus::NotCharging,
            _ => BatteryStatus::Unknown,
        }
    }
}

/// Power supply information.
#[derive(Debug, Clone)]
pub struct PowerSupplyInfo {
    /// Supply name.
    pub name: String,
    /// Supply type (battery, AC, USB, etc.).
    pub supply_type: String,
    /// Whether supply is online.
    pub online: bool,
    /// Supply status.
    pub status: String,
}

/// Power status manager.
#[derive(Debug)]
pub struct PowerStatus {
    /// Power supplies path.
    power_path: String,
    /// Cached battery info.
    battery_info: Option<BatteryInfo>,
    /// Cached power supplies.
    power_supplies: BTreeMap<String, PowerSupplyInfo>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl PowerStatus {
    /// Create a new power status manager.
    pub fn new() -> Self {
        Self {
            power_path: "/sys/class/power_supply".to_string(),
            battery_info: None,
            power_supplies: BTreeMap::new(),
            last_update: std::time::Instant::now(),
        }
    }

    /// Get battery information.
    pub fn get_battery_info(&mut self) -> Result<&BatteryInfo, PowerError> {
        self.refresh_power_info()?;
        self.battery_info.as_ref().ok_or_else(|| PowerError::PowerSupplyNotFound("battery".to_string()))
    }

    /// Get all power supplies.
    pub fn get_power_supplies(&mut self) -> Result<Vec<&PowerSupplyInfo>, PowerError> {
        self.refresh_power_info()?;
        Ok(self.power_supplies.values().collect())
    }

    /// Get AC adapter status.
    pub fn get_ac_status(&mut self) -> Result<Option<bool>, PowerError> {
        self.refresh_power_info()?;
        for supply in self.power_supplies.values() {
            if supply.supply_type == "Mains" || supply.name.contains("AC") {
                return Ok(Some(supply.online));
            }
        }
        Ok(None)
    }

    /// Check if on battery power.
    pub fn is_on_battery(&mut self) -> Result<bool, PowerError> {
        self.refresh_power_info()?;
        for supply in self.power_supplies.values() {
            if supply.supply_type == "Battery" {
                return Ok(!supply.online);
            }
        }
        Ok(false)
    }

    /// Get battery level percentage.
    pub fn get_battery_level(&mut self) -> Result<Option<u32>, PowerError> {
        self.refresh_power_info()?;
        Ok(self.battery_info.as_ref().map(|b| b.level))
    }

    /// Get battery status.
    pub fn get_battery_status(&mut self) -> Result<Option<BatteryStatus>, PowerError> {
        self.refresh_power_info()?;
        Ok(self.battery_info.as_ref().map(|b| b.status))
    }

    /// Check if battery is low (below 20%).
    pub fn is_battery_low(&mut self) -> Result<bool, PowerError> {
        self.refresh_power_info()?;
        Ok(self.battery_info.as_ref().map_or(false, |b| b.level < 20))
    }

    /// Check if battery is critical (below 10%).
    pub fn is_battery_critical(&mut self) -> Result<bool, PowerError> {
        self.refresh_power_info()?;
        Ok(self.battery_info.as_ref().map_or(false, |b| b.level < 10))
    }

    /// Get power summary.
    pub fn get_power_summary(&mut self) -> Result<String, PowerError> {
        self.refresh_power_info()?;
        
        let mut summary = String::new();
        
        if let Some(battery) = &self.battery_info {
            summary.push_str(&format!("Battery: {}% ({})", battery.level, battery.status.as_str()));
            if battery.temperature > 0.0 {
                summary.push_str(&format!(", {:.1}°C", battery.temperature / 10.0));
            }
        } else {
            summary.push_str("No battery detected");
        }
        
        if let Some(ac_online) = self.get_ac_status()? {
            summary.push_str(&format!(", AC: {}", if ac_online { "connected" } else { "disconnected" }));
        }
        
        Ok(summary)
    }

    /// Refresh power info from sysfs.
    fn refresh_power_info(&mut self) -> Result<(), PowerError> {
        // Only refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() < 1 {
            return Ok(());
        }

        self.power_supplies.clear();
        self.battery_info = None;

        let power_dir = Path::new(&self.power_path);
        if !power_dir.exists() {
            return Ok(()); // No power supply info available
        }

        let entries = fs::read_dir(power_dir)
            .map_err(|e| PowerError::IoError(format!("Failed to read power dir: {}", e)))?;

        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                let supply_path = entry.path();
                
                // Read supply type
                let type_path = supply_path.join("type");
                let supply_type = fs::read_to_string(&type_path)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|_| "Unknown".to_string());
                
                // Read online status
                let online_path = supply_path.join("online");
                let online = fs::read_to_string(&online_path)
                    .map(|s| s.trim() == "1")
                    .unwrap_or(false);
                
                // Read status
                let status_path = supply_path.join("status");
                let status = fs::read_to_string(&status_path)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|_| "Unknown".to_string());
                
                self.power_supplies.insert(name.clone(), PowerSupplyInfo {
                    name: name.clone(),
                    supply_type: supply_type.clone(),
                    online,
                    status: status.clone(),
                });
                
                // If it's a battery, read detailed info
                if supply_type == "Battery" {
                    self.battery_info = Some(self.read_battery_info(&name, &supply_path)?);
                }
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Read detailed battery info.
    fn read_battery_info(&self, name: &str, supply_path: &Path) -> Result<BatteryInfo, PowerError> {
        // Read capacity
        let capacity = self.read_sysfs_value(supply_path, "capacity")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0) as u64;
        
        // Read status
        let status_str = self.read_sysfs_value(supply_path, "status")
            .unwrap_or_else(|| "Unknown".to_string());
        let status = BatteryStatus::parse(&status_str);
        
        // Read voltage
        let voltage = self.read_sysfs_value(supply_path, "voltage_now")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        
        // Read current
        let current = self.read_sysfs_value(supply_path, "current_now")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        
        // Read temperature
        let temperature = self.read_sysfs_value(supply_path, "temp")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        
        // Read time to empty
        let time_to_empty = self.read_sysfs_value(supply_path, "time_to_empty")
            .and_then(|s| s.parse::<u64>().ok());
        
        // Read time to full
        let time_to_full = self.read_sysfs_value(supply_path, "time_to_full")
            .and_then(|s| s.parse::<u64>().ok());
        
        // Read technology
        let technology = self.read_sysfs_value(supply_path, "technology")
            .unwrap_or_else(|| "Unknown".to_string());
        
        // Read model
        let model = self.read_sysfs_value(supply_path, "model_name")
            .unwrap_or_else(|| "Unknown".to_string());
        
        Ok(BatteryInfo {
            name: name.to_string(),
            level: capacity as u32,
            status,
            capacity,
            voltage,
            current,
            temperature,
            time_to_empty,
            time_to_full,
            technology,
            model,
        })
    }

    /// Read a sysfs value.
    fn read_sysfs_value(&self, path: &Path, filename: &str) -> Option<String> {
        let file_path = path.join(filename);
        fs::read_to_string(&file_path)
            .ok()
            .map(|s| s.trim().to_string())
    }
}

impl Default for PowerStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let power = PowerStatus::new();
        assert_eq!(power.power_path, "/sys/class/power_supply");
    }

    #[test]
    fn test_get_battery_info() {
        let mut power = PowerStatus::new();
        if let Ok(battery) = power.get_battery_info() {
            assert!(battery.level <= 100);
        }
    }

    #[test]
    fn test_battery_status_parse() {
        assert_eq!(BatteryStatus::parse("Charging"), BatteryStatus::Charging);
        assert_eq!(BatteryStatus::parse("Discharging"), BatteryStatus::Discharging);
        assert_eq!(BatteryStatus::parse("Full"), BatteryStatus::Full);
        assert_eq!(BatteryStatus::parse("Unknown"), BatteryStatus::Unknown);
    }

    #[test]
    fn test_get_power_summary() {
        let mut power = PowerStatus::new();
        if let Ok(summary) = power.get_power_summary() {
            assert!(!summary.is_empty());
        }
    }
}
