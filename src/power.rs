//! Power management inspired by Linux kernel power management and Arch Linux power saving.
//! Provides power control, battery optimization, and energy efficiency tuning.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    Unknown,
}

impl BatteryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Charging => "charging",
            Self::Discharging => "discharging",
            Self::Full => "full",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

impl PowerProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Balanced => "balanced",
            Self::PowerSaver => "power-saver",
        }
    }

    pub fn cpu_governor(&self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Balanced => "schedutil",
            Self::PowerSaver => "powersave",
        }
    }

    pub fn recommended_tdp_multiplier(&self) -> f32 {
        match self {
            Self::Performance => 1.0,
            Self::Balanced => 0.8,
            Self::PowerSaver => 0.6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TdpLevel {
    T15,
    T30,
    T45,
    T60,
    T90,
    Custom(u16),
}

impl TdpLevel {
    pub fn watts(&self) -> u16 {
        match self {
            Self::T15 => 15,
            Self::T30 => 30,
            Self::T45 => 45,
            Self::T60 => 60,
            Self::T90 => 90,
            Self::Custom(w) => *w,
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Self::Custom(w) => format!("{}W", w),
            _ => format!("{}W", self.watts()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerError {
    InvalidPercentage(String),
    InvalidTdp(String),
    BatteryNotAvailable,
}

impl fmt::Display for PowerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPercentage(msg) => write!(f, "invalid percentage: {}", msg),
            Self::InvalidTdp(msg) => write!(f, "invalid TDP: {}", msg),
            Self::BatteryNotAvailable => write!(f, "battery not available"),
        }
    }
}

impl std::error::Error for PowerError {}

#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub capacity: u8,
    pub status: BatteryStatus,
    pub energy_full: u32,
    pub energy_now: u32,
    pub voltage: u32,
}

impl BatteryInfo {
    pub fn new(capacity: u8, status: BatteryStatus) -> Result<Self, PowerError> {
        if capacity > 100 {
            return Err(PowerError::InvalidPercentage("capacity must be 0-100".to_string()));
        }
        Ok(Self {
            capacity,
            status,
            energy_full: 50000,
            energy_now: (capacity as u32 * 500),
            voltage: 12000,
        })
    }

    pub fn with_energy(mut self, full: u32, now: u32) -> Self {
        self.energy_full = full;
        self.energy_now = now;
        self
    }

    pub fn percentage(&self) -> u8 {
        if self.energy_full == 0 {
            return 0;
        }
        ((self.energy_now * 100) / self.energy_full) as u8
    }

    pub fn time_remaining_minutes(&self) -> Option<u32> {
        if self.status != BatteryStatus::Discharging || self.energy_now == 0 {
            return None;
        }
        let drain_rate = 10000;
        Some(self.energy_now / drain_rate)
    }
}

#[derive(Debug, Clone)]
pub struct PowerManager {
    pub profile: PowerProfile,
    pub tdp_level: TdpLevel,
    pub battery_info: Option<BatteryInfo>,
    pub auto_brightness: bool,
    pub bluetooth_enabled: bool,
    pub wifi_power_save: bool,
}

impl Default for PowerManager {
    fn default() -> Self {
        Self {
            profile: PowerProfile::Balanced,
            tdp_level: TdpLevel::T45,
            battery_info: None,
            auto_brightness: true,
            bluetooth_enabled: true,
            wifi_power_save: false,
        }
    }
}

impl PowerManager {
    pub fn new(profile: PowerProfile, tdp_level: TdpLevel) -> Self {
        Self {
            profile,
            tdp_level,
            battery_info: None,
            auto_brightness: true,
            bluetooth_enabled: true,
            wifi_power_save: false,
        }
    }

    pub fn with_battery(mut self, battery: BatteryInfo) -> Self {
        self.battery_info = Some(battery);
        self
    }

    pub fn set_profile(&mut self, profile: PowerProfile) {
        self.profile = profile;
    }

    pub fn set_tdp_level(&mut self, tdp: TdpLevel) -> Result<(), PowerError> {
        if tdp.watts() == 0 {
            return Err(PowerError::InvalidTdp("TDP cannot be 0W".to_string()));
        }
        self.tdp_level = tdp;
        Ok(())
    }

    pub fn enable_wifi_power_save(&mut self) {
        self.wifi_power_save = true;
    }

    pub fn disable_wifi_power_save(&mut self) {
        self.wifi_power_save = false;
    }

    pub fn enable_bluetooth(&mut self) {
        self.bluetooth_enabled = true;
    }

    pub fn disable_bluetooth(&mut self) {
        self.bluetooth_enabled = false;
    }

    pub fn auto_optimize(&mut self) {
        if let Some(ref battery) = self.battery_info {
            if battery.capacity < 20 {
                self.profile = PowerProfile::PowerSaver;
                self.wifi_power_save = true;
                self.bluetooth_enabled = false;
            } else if battery.capacity < 50 {
                self.profile = PowerProfile::Balanced;
                self.wifi_power_save = true;
            } else {
                self.profile = PowerProfile::Balanced;
            }
        }
    }

    pub fn generate_tlp_config(&self) -> String {
        format!(
            "# SigmaOS TLP Configuration\n\
             # Profile: {}\n\
             # TDP: {}\n\
             \n\
             TLP_ENABLE=1\n\
             TLP_DEFAULT_MODE={}\n\
             CPU_MAX_PERF_ON_BAT={}\n\
             CPU_BOOST_ON_BAT={}\n\
             WIFI_PWR_ON_BAT={}\n\
             BLUETOOTH_DEV_ON_BAT={}\n",
            self.profile.as_str(),
            self.tdp_level.as_str(),
            match self.profile {
                PowerProfile::Performance => "BAT",
                PowerProfile::Balanced => "AC",
                PowerProfile::PowerSaver => "BAT",
            },
            if self.profile == PowerProfile::PowerSaver { "30" } else { "100" },
            if self.profile == PowerProfile::PowerSaver { "0" } else { "1" },
            if self.wifi_power_save { "on" } else { "off" },
            if self.bluetooth_enabled { "on" } else { "off" },
        )
    }

    pub fn generate_power_profiles_daemon_config(&self) -> String {
        format!(
            "# SigmaOS Power Profiles Daemon Configuration\n\
             # Profile: {}\n\
             \n\
             [Performance]\n\
             CPU governor = {}\n\
             \n\
             [Balanced]\n\
             CPU governor = {}\n\
             \n\
             [Power-Saver]\n\
             CPU governor = {}\n",
            self.profile.as_str(),
            PowerProfile::Performance.cpu_governor(),
            PowerProfile::Balanced.cpu_governor(),
            PowerProfile::PowerSaver.cpu_governor(),
        )
    }

    pub fn generate_throttled_config(&self) -> String {
        let tdp = self.tdp_level.watts();
        format!(
            "# SigmaOS Throttled Configuration\n\
             # TDP: {}W\n\
             \n\
             # General\n\
             SOFT_TEMP_LIMIT=80\n\
             HARD_TEMP_LIMIT=85\n\
             \n\
             # Power limits\n\
             TDP={}W\n",
            tdp, tdp
        )
    }

    pub fn get_battery_status_summary(&self) -> String {
        let tdp = self.tdp_level.watts();
        let profile = self.profile.as_str();
        if let Some(ref battery) = self.battery_info {
            format!(
                "Battery: {}% ({}) | TDP: {}W | Profile: {}",
                battery.capacity,
                battery.status.as_str(),
                tdp,
                profile
            )
        } else {
            format!("AC Power | TDP: {}W | Profile: {}", tdp, profile)
        }
    }

    pub fn estimated_battery_life(&self) -> Option<String> {
        self.battery_info.as_ref()?.time_remaining_minutes().map(|mins| {
            if mins >= 60 {
                format!("{}h {}m", mins / 60, mins % 60)
            } else {
                format!("{}m", mins)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_status_as_str() {
        assert_eq!(BatteryStatus::Charging.as_str(), "charging");
        assert_eq!(BatteryStatus::Discharging.as_str(), "discharging");
    }

    #[test]
    fn test_power_profile_as_str() {
        assert_eq!(PowerProfile::Performance.as_str(), "performance");
        assert_eq!(PowerProfile::PowerSaver.as_str(), "power-saver");
    }

    #[test]
    fn test_power_profile_cpu_governor() {
        assert_eq!(PowerProfile::Performance.cpu_governor(), "performance");
        assert_eq!(PowerProfile::Balanced.cpu_governor(), "schedutil");
    }

    #[test]
    fn test_tdp_level_watts() {
        assert_eq!(TdpLevel::T15.watts(), 15);
        assert_eq!(TdpLevel::T45.watts(), 45);
        assert_eq!(TdpLevel::Custom(65).watts(), 65);
    }

    #[test]
    fn test_battery_info_new_valid() {
        let battery = BatteryInfo::new(50, BatteryStatus::Discharging);
        assert!(battery.is_ok());
        assert_eq!(battery.unwrap().capacity, 50);
    }

    #[test]
    fn test_battery_info_new_invalid() {
        let battery = BatteryInfo::new(150, BatteryStatus::Discharging);
        assert!(battery.is_err());
    }

    #[test]
    fn test_battery_info_percentage() {
        let battery = BatteryInfo::new(50, BatteryStatus::Discharging).unwrap()
            .with_energy(50000, 25000);
        assert_eq!(battery.percentage(), 50);
    }

    #[test]
    fn test_power_manager_new() {
        let mgr = PowerManager::new(PowerProfile::Performance, TdpLevel::T60);
        assert_eq!(mgr.profile, PowerProfile::Performance);
        assert_eq!(mgr.tdp_level, TdpLevel::T60);
    }

    #[test]
    fn test_power_manager_set_profile() {
        let mut mgr = PowerManager::default();
        mgr.set_profile(PowerProfile::PowerSaver);
        assert_eq!(mgr.profile, PowerProfile::PowerSaver);
    }

    #[test]
    fn test_power_manager_set_tdp_valid() {
        let mut mgr = PowerManager::default();
        mgr.set_tdp_level(TdpLevel::T30).unwrap();
        assert_eq!(mgr.tdp_level, TdpLevel::T30);
    }

    #[test]
    fn test_power_manager_set_tdp_invalid() {
        let mut mgr = PowerManager::default();
        assert!(mgr.set_tdp_level(TdpLevel::Custom(0)).is_err());
    }

    #[test]
    fn test_power_manager_auto_optimize() {
        let battery = BatteryInfo::new(15, BatteryStatus::Discharging).unwrap();
        let mut mgr = PowerManager::default().with_battery(battery);
        mgr.auto_optimize();
        assert_eq!(mgr.profile, PowerProfile::PowerSaver);
        assert!(mgr.wifi_power_save);
        assert!(!mgr.bluetooth_enabled);
    }

    #[test]
    fn test_generate_tlp_config() {
        let mgr = PowerManager::default();
        let config = mgr.generate_tlp_config();
        assert!(config.contains("TLP_ENABLE=1"));
    }

    #[test]
    fn test_generate_power_profiles_daemon_config() {
        let mgr = PowerManager::default();
        let config = mgr.generate_power_profiles_daemon_config();
        assert!(config.contains("Performance"));
        assert!(config.contains("Balanced"));
    }

    #[test]
    fn test_get_battery_status_summary() {
        let battery = BatteryInfo::new(75, BatteryStatus::Charging).unwrap();
        let mgr = PowerManager::default().with_battery(battery);
        let summary = mgr.get_battery_status_summary();
        assert!(summary.contains("75%"));
        assert!(summary.contains("charging"));
    }
}
