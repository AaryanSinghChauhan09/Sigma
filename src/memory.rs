//! Memory management inspired by Linux kernel cgroups v2 and SigmaOS optimization.
//! Provides cgroups v2 control, memory pressure monitoring, and OOM prevention.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressureLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl MemoryPressureLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    pub fn from_percentage(pct: u32) -> Self {
        if pct < 50 {
            Self::Low
        } else if pct < 70 {
            Self::Medium
        } else if pct < 90 {
            Self::High
        } else {
            Self::Critical
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    InvalidLimit(String),
    InvalidPercentage(String),
    CgroupPathEmpty,
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimit(msg) => write!(f, "invalid memory limit: {}", msg),
            Self::InvalidPercentage(msg) => write!(f, "invalid percentage: {}", msg),
            Self::CgroupPathEmpty => write!(f, "cgroup path cannot be empty"),
        }
    }
}

impl std::error::Error for MemoryError {}

#[derive(Debug, Clone)]
pub struct CgroupConfig {
    pub name: String,
    pub memory_limit_bytes: u64,
    pub memory_swap_limit_bytes: u64,
    pub oom_kill_disable: bool,
    pub memory_pressure_notification: bool,
}

impl CgroupConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            memory_limit_bytes: 0,
            memory_swap_limit_bytes: 0,
            oom_kill_disable: false,
            memory_pressure_notification: true,
        }
    }

    pub fn with_memory_limit(mut self, bytes: u64) -> Result<Self, MemoryError> {
        if bytes > 0 && bytes < 1024 * 1024 {
            return Err(MemoryError::InvalidLimit(
                "memory limit must be at least 1MB".to_string()
            ));
        }
        self.memory_limit_bytes = bytes;
        Ok(self)
    }

    pub fn with_swap_limit(mut self, bytes: u64) -> Result<Self, MemoryError> {
        self.memory_swap_limit_bytes = bytes;
        Ok(self)
    }

    pub fn disable_oom_kill(mut self) -> Self {
        self.oom_kill_disable = true;
        self
    }

    pub fn enable_pressure_notification(mut self) -> Self {
        self.memory_pressure_notification = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct MemoryManager {
    pub cgroup_path: String,
    pub config: CgroupConfig,
    pub current_pressure: MemoryPressureLevel,
    pub total_memory_mb: u64,
    pub available_memory_mb: u64,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self {
            cgroup_path: "/sys/fs/cgroup/sigmaos".to_string(),
            config: CgroupConfig::new("sigmaos-default"),
            current_pressure: MemoryPressureLevel::Low,
            total_memory_mb: 8192,
            available_memory_mb: 8192,
        }
    }
}

impl MemoryManager {
    pub fn new(cgroup_path: impl Into<String>, config: CgroupConfig) -> Result<Self, MemoryError> {
        let path = cgroup_path.into();
        if path.trim().is_empty() {
            return Err(MemoryError::CgroupPathEmpty);
        }
        Ok(Self {
            cgroup_path: path,
            config,
            current_pressure: MemoryPressureLevel::Low,
            total_memory_mb: 8192,
            available_memory_mb: 8192,
        })
    }

    pub fn memory_usage_percentage(&self) -> u32 {
        if self.total_memory_mb == 0 {
            return 0;
        }
        let used = self.total_memory_mb.saturating_sub(self.available_memory_mb);
        ((used * 100) / self.total_memory_mb) as u32
    }

    pub fn update_pressure(&mut self) {
        let pct = self.memory_usage_percentage();
        self.current_pressure = MemoryPressureLevel::from_percentage(pct);
    }

    pub fn generate_cgroup_memory_config(&self) -> String {
        format!(
            "# SigmaOS cgroups v2 Memory Configuration\n# Path: {}\n# cgroup: {}\n\
             memory.max={}\n\
             memory.swap.max={}\n\
             memory.oom.group={}\n\
             memory.pressure_notification={}\n",
            self.cgroup_path,
            self.config.name,
            if self.config.memory_limit_bytes > 0 {
                self.config.memory_limit_bytes.to_string()
            } else {
                "max".to_string()
            },
            if self.config.memory_swap_limit_bytes > 0 {
                self.config.memory_swap_limit_bytes.to_string()
            } else {
                "max".to_string()
            },
            if self.config.oom_kill_disable { "0" } else { "1" },
            if self.config.memory_pressure_notification { "1" } else { "0" },
        )
    }

    pub fn generate_sysctl_memory_tuning(&self) -> String {
        let usage_pct = self.memory_usage_percentage();
        let vm_swappiness = if usage_pct > 80 { 10 } else if usage_pct > 60 { 30 } else { 60 };
        let min_free_kbytes = (self.total_memory_mb * 1024) / 100;
        
        format!(
            "# SigmaOS Memory Tuning\n# Current pressure: {} ({}% usage)\n\
             vm.swappiness={}\n\
             vm.vfs_cache_pressure={}\n\
             vm.min_free_kbytes={}\n\
             vm.watermark_scale_factor={}\n",
            self.current_pressure.as_str(),
            usage_pct,
            vm_swappiness,
            if usage_pct > 80 { 100 } else { 50 },
            min_free_kbytes,
            if usage_pct > 80 { 200 } else { 10 },
        )
    }

    pub fn should_trigger_oom_prevention(&self) -> bool {
        self.current_pressure == MemoryPressureLevel::Critical
    }

    pub fn get_recommendation(&self) -> &'static str {
        match self.current_pressure {
            MemoryPressureLevel::Low => "System memory is healthy. No action needed.",
            MemoryPressureLevel::Medium => "Consider closing unused applications.",
            MemoryPressureLevel::High => "Close unused applications and consider increasing swap.",
            MemoryPressureLevel::Critical => "URGENT: Free memory immediately or system may become unstable.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pressure_from_percentage() {
        assert_eq!(MemoryPressureLevel::from_percentage(30), MemoryPressureLevel::Low);
        assert_eq!(MemoryPressureLevel::from_percentage(60), MemoryPressureLevel::Medium);
        assert_eq!(MemoryPressureLevel::from_percentage(80), MemoryPressureLevel::High);
        assert_eq!(MemoryPressureLevel::from_percentage(95), MemoryPressureLevel::Critical);
    }

    #[test]
    fn test_cgroup_config_default() {
        let config = CgroupConfig::new("test");
        assert_eq!(config.name, "test");
        assert_eq!(config.memory_limit_bytes, 0);
        assert!(!config.oom_kill_disable);
    }

    #[test]
    fn test_cgroup_config_with_memory_limit_valid() {
        let config = CgroupConfig::new("test")
            .with_memory_limit(1024 * 1024)
            .unwrap();
        assert_eq!(config.memory_limit_bytes, 1024 * 1024);
    }

    #[test]
    fn test_cgroup_config_with_memory_limit_invalid() {
        let result = CgroupConfig::new("test")
            .with_memory_limit(512 * 1024);
        assert!(result.is_err());
    }

    #[test]
    fn test_memory_manager_new_valid() {
        let config = CgroupConfig::new("test");
        let mgr = MemoryManager::new("/sys/fs/cgroup/test", config);
        assert!(mgr.is_ok());
    }

    #[test]
    fn test_memory_manager_new_invalid_path() {
        let config = CgroupConfig::new("test");
        let result = MemoryManager::new("", config);
        assert!(result.is_err());
    }

    #[test]
    fn test_memory_usage_percentage() {
        let mut mgr = MemoryManager::default();
        mgr.total_memory_mb = 1000;
        mgr.available_memory_mb = 500;
        assert_eq!(mgr.memory_usage_percentage(), 50);
    }

    #[test]
    fn test_update_pressure() {
        let mut mgr = MemoryManager::default();
        mgr.total_memory_mb = 1000;
        mgr.available_memory_mb = 200;
        mgr.update_pressure();
        assert_eq!(mgr.current_pressure, MemoryPressureLevel::High);
    }

    #[test]
    fn test_should_trigger_oom_prevention() {
        let mut mgr = MemoryManager::default();
        mgr.total_memory_mb = 1000;
        mgr.available_memory_mb = 50;
        mgr.update_pressure();
        assert!(mgr.should_trigger_oom_prevention());
    }

    #[test]
    fn test_generate_cgroup_memory_config() {
        let config = CgroupConfig::new("test").with_memory_limit(1024 * 1024 * 1024).unwrap();
        let mgr = MemoryManager::new("/sys/fs/cgroup/test", config).unwrap();
        let conf = mgr.generate_cgroup_memory_config();
        assert!(conf.contains("memory.max=1073741824"));
        assert!(conf.contains("cgroup: test"));
    }

    #[test]
    fn test_generate_sysctl_memory_tuning() {
        let mut mgr = MemoryManager::default();
        mgr.total_memory_mb = 1000;
        mgr.available_memory_mb = 200;
        mgr.update_pressure();
        let tuning = mgr.generate_sysctl_memory_tuning();
        assert!(tuning.contains("vm.swappiness="));
        assert!(tuning.contains(mgr.current_pressure.as_str()));
    }

    #[test]
    fn test_get_recommendation() {
        let mut mgr = MemoryManager::default();
        mgr.total_memory_mb = 1000;
        mgr.available_memory_mb = 100;
        mgr.update_pressure();
        assert!(mgr.get_recommendation().contains("URGENT"));
    }
}
