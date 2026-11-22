//! SigmaOS sensor operations module.
//!
//! This module provides actual sensor operations using /sys/class/hwmon.
//!
//! # Example
//!
//! ```
//! use sigma_system::sensor_ops::*;
//!
//! let sensor = SensorOps::new();
//! if let Ok(sensors) = sensor.list_sensors() {
//!     println!("Found {} sensors", sensors.len());
//! }
//! ```

use std::fs;
use std::path::Path;

/// Errors that can occur in sensor operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SensorError {
    /// Sensor not found.
    SensorNotFound(String),
    /// Failed to read sensor.
    ReadError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for SensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorError::SensorNotFound(name) => write!(f, "Sensor '{}' not found", name),
            SensorError::ReadError(msg) => write!(f, "Failed to read sensor: {}", msg),
            SensorError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for SensorError {}

/// Sensor information.
#[derive(Debug, Clone)]
pub struct SensorInfo {
    /// Sensor name.
    pub name: String,
    /// Sensor type.
    pub sensor_type: String,
    /// Current value.
    pub value: f64,
    /// Unit.
    pub unit: String,
}

/// Sensor operations manager.
#[derive(Debug)]
pub struct SensorOps;

impl SensorOps {
    pub fn new() -> Self { Self }

    pub fn list_sensors(&self) -> Result<Vec<SensorInfo>, SensorError> {
        let mut sensors = Vec::new();
        let hwmon_dir = Path::new("/sys/class/hwmon");

        if let Ok(entries) = fs::read_dir(hwmon_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                
                let sensor_type = if let Ok(name_file) = fs::read_to_string(entry.path().join("name")) {
                    name_file.trim().to_string()
                } else {
                    "unknown".to_string()
                };

                let temp_input = entry.path().join("temp1_input");
                if let Ok(temp_content) = fs::read_to_string(&temp_input) {
                    if let Ok(temp_val) = temp_content.trim().parse::<f64>() {
                        sensors.push(SensorInfo {
                            name: name_str.to_string(),
                            sensor_type,
                            value: temp_val / 1000.0,
                            unit: "C".to_string(),
                        });
                    }
                }
            }
        }

        Ok(sensors)
    }
}

impl Default for SensorOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sensor = SensorOps::new();
        let _ = sensor;
    }

    #[test]
    fn test_list_sensors() {
        let sensor = SensorOps::new();
        if let Ok(_) = sensor.list_sensors() {
            // Should not panic
        }
    }
}
