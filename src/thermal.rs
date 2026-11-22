//! SigmaOS thermal monitoring module.
//!
//! This module provides actual thermal monitoring using /sys/class/thermal.
//!
//! # Example
//!
//! ```
//! use sigma_system::thermal::*;
//!
//! let mut thermal = ThermalMonitor::new();
//! if let Ok(temps) = thermal.get_all_temperatures() {
//!     println!("CPU temp: {}°C", temps.cpu);
//! }
//! ```

use std::fs;
use std::path::Path;
use std::collections::BTreeMap;

/// Errors that can occur in thermal operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ThermalError {
    /// Thermal zone not found.
    ThermalZoneNotFound(String),
    /// Failed to read temperature.
    ReadError(String),
    /// Invalid temperature data.
    InvalidData(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ThermalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThermalError::ThermalZoneNotFound(zone) => write!(f, "Thermal zone '{}' not found", zone),
            ThermalError::ReadError(msg) => write!(f, "Failed to read temperature: {}", msg),
            ThermalError::InvalidData(msg) => write!(f, "Invalid temperature data: {}", msg),
            ThermalError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ThermalError {}

/// Temperature readings.
#[derive(Debug, Clone)]
pub struct TemperatureReadings {
    /// CPU temperature in Celsius.
    pub cpu: f64,
    /// GPU temperature in Celsius (if available).
    pub gpu: Option<f64>,
    /// NVMe temperature in Celsius (if available).
    pub nvme: Option<f64>,
    /// All thermal zones.
    pub zones: BTreeMap<String, f64>,
}

/// Thermal zone information.
#[derive(Debug, Clone)]
pub struct ThermalZone {
    /// Zone name.
    pub name: String,
    /// Temperature in Celsius.
    pub temperature: f64,
    /// Temperature type.
    pub zone_type: String,
}

/// Thermal monitor.
#[derive(Debug)]
pub struct ThermalMonitor {
    /// Thermal zones path.
    thermal_path: String,
    /// Cached temperatures.
    temperatures: Option<TemperatureReadings>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl ThermalMonitor {
    /// Create a new thermal monitor.
    pub fn new() -> Self {
        Self {
            thermal_path: "/sys/class/thermal".to_string(),
            temperatures: None,
            last_update: std::time::Instant::now(),
        }
    }

    /// Get all temperature readings.
    pub fn get_all_temperatures(&mut self) -> Result<&TemperatureReadings, ThermalError> {
        // Refresh if more than 1 second since last update
        if self.last_update.elapsed().as_secs() > 1 {
            self.refresh_temperatures()?;
        }
        self.temperatures.as_ref().ok_or_else(|| ThermalError::InvalidData("No temperature data".to_string()))
    }

    /// Refresh temperature readings.
    fn refresh_temperatures(&mut self) -> Result<(), ThermalError> {
        let mut zones = BTreeMap::new();
        let mut cpu_temp = 0.0;
        let mut gpu_temp = None;
        let mut nvme_temp = None;

        // Read thermal zones
        let thermal_dir = Path::new(&self.thermal_path);
        if !thermal_dir.exists() {
            return Err(ThermalError::IoError("Thermal sysfs not found".to_string()));
        }

        let entries = fs::read_dir(thermal_dir)
            .map_err(|e| ThermalError::IoError(format!("Failed to read thermal dir: {}", e)))?;

        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("thermal_zone") {
                    if let Ok(temp) = self.read_thermal_zone(&entry.path()) {
                        let zone_name = self.get_zone_name(&entry.path()).unwrap_or(name.clone());
                        
                        // Categorize temperatures
                        if zone_name.contains("cpu") || zone_name.contains("core") {
                            cpu_temp = temp;
                        } else if zone_name.contains("gpu") || zone_name.contains("video") {
                            gpu_temp = Some(temp);
                        } else if zone_name.contains("nvme") || zone_name.contains("ssd") {
                            nvme_temp = Some(temp);
                        }
                        
                        zones.insert(zone_name, temp);
                    }
                }
            }
        }

        // If no CPU temp found, use first available
        if cpu_temp == 0.0 && !zones.is_empty() {
            cpu_temp = zones.values().next().copied().unwrap_or(0.0);
        }

        self.temperatures = Some(TemperatureReadings {
            cpu: cpu_temp,
            gpu: gpu_temp,
            nvme: nvme_temp,
            zones,
        });

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Read temperature from a thermal zone.
    fn read_thermal_zone(&self, zone_path: &Path) -> Result<f64, ThermalError> {
        let temp_path = zone_path.join("temp");
        let temp_str = fs::read_to_string(&temp_path)
            .map_err(|e| ThermalError::IoError(format!("Failed to read temp: {}", e)))?;
        
        let temp_millicelsius: f64 = temp_str.trim().parse()
            .map_err(|e| ThermalError::InvalidData(format!("Invalid temp value: {}", e)))?;
        
        Ok(temp_millicelsius / 1000.0)
    }

    /// Get thermal zone name.
    fn get_zone_name(&self, zone_path: &Path) -> Option<String> {
        let type_path = zone_path.join("type");
        fs::read_to_string(&type_path)
            .ok()
            .map(|s| s.trim().to_string())
    }

    /// Get CPU temperature.
    pub fn get_cpu_temperature(&mut self) -> Result<f64, ThermalError> {
        let temps = self.get_all_temperatures()?;
        Ok(temps.cpu)
    }

    /// Get GPU temperature.
    pub fn get_gpu_temperature(&mut self) -> Result<Option<f64>, ThermalError> {
        let temps = self.get_all_temperatures()?;
        Ok(temps.gpu)
    }

    /// Get NVMe temperature.
    pub fn get_nvme_temperature(&mut self) -> Result<Option<f64>, ThermalError> {
        let temps = self.get_all_temperatures()?;
        Ok(temps.nvme)
    }

    /// Get all thermal zones.
    pub fn get_thermal_zones(&mut self) -> Result<Vec<ThermalZone>, ThermalError> {
        self.refresh_temperatures()?;
        let temps = self.temperatures.as_ref().unwrap();
        
        let mut zones = Vec::new();
        for (name, temp) in &temps.zones {
            zones.push(ThermalZone {
                name: name.clone(),
                temperature: *temp,
                zone_type: name.clone(),
            });
        }
        
        Ok(zones)
    }

    /// Get temperature status.
    pub fn get_temperature_status(&mut self) -> Result<String, ThermalError> {
        let temps = self.get_all_temperatures()?;
        
        let cpu_status = if temps.cpu > 80.0 {
            "CRITICAL"
        } else if temps.cpu > 70.0 {
            "HIGH"
        } else if temps.cpu > 60.0 {
            "WARM"
        } else {
            "NORMAL"
        };
        
        Ok(format!("CPU: {:.1}°C ({})", temps.cpu, cpu_status))
    }

    /// Check if temperature is critical.
    pub fn is_temperature_critical(&mut self) -> Result<bool, ThermalError> {
        let temps = self.get_all_temperatures()?;
        Ok(temps.cpu > 85.0 || temps.gpu.map_or(false, |g| g > 85.0))
    }

    /// Get thermal summary.
    pub fn get_thermal_summary(&mut self) -> Result<String, ThermalError> {
        let temps = self.get_all_temperatures()?;
        
        let mut summary = format!("CPU: {:.1}°C", temps.cpu);
        if let Some(gpu) = temps.gpu {
            summary.push_str(&format!(", GPU: {:.1}°C", gpu));
        }
        if let Some(nvme) = temps.nvme {
            summary.push_str(&format!(", NVMe: {:.1}°C", nvme));
        }
        summary.push_str(&format!(", Zones: {}", temps.zones.len()));
        
        Ok(summary)
    }
}

impl Default for ThermalMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let monitor = ThermalMonitor::new();
        assert_eq!(monitor.thermal_path, "/sys/class/thermal");
    }

    #[test]
    fn test_get_all_temperatures() {
        let mut monitor = ThermalMonitor::new();
        if let Ok(temps) = monitor.get_all_temperatures() {
            assert!(temps.cpu >= 0.0);
        }
    }

    #[test]
    fn test_get_cpu_temperature() {
        let mut monitor = ThermalMonitor::new();
        if let Ok(temp) = monitor.get_cpu_temperature() {
            assert!(temp >= 0.0);
        }
    }

    #[test]
    fn test_get_thermal_summary() {
        let mut monitor = ThermalMonitor::new();
        if let Ok(summary) = monitor.get_thermal_summary() {
            assert!(summary.contains("CPU"));
        }
    }
}
