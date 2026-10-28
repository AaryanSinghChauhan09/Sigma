//! System adjustments inspired by Linux Mint mintsystem
//! Manages CPU governors, sysctl memory tuning, and hardware optimization.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernorMode {
    Performance,
    Powersave,
    Schedutil,
}

impl GovernorMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Powersave => "powersave",
            Self::Schedutil => "schedutil",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemAdjuster {
    swappiness: u32,
    vfs_cache_pressure: u32,
    governor: GovernorMode,
}

impl Default for SystemAdjuster {
    fn default() -> Self {
        Self {
            swappiness: 10,
            vfs_cache_pressure: 50,
            governor: GovernorMode::Schedutil,
        }
    }
}

impl SystemAdjuster {
    pub fn new(swappiness: u32, vfs_cache_pressure: u32, governor: GovernorMode) -> Self {
        Self {
            swappiness: swappiness.min(100),
            vfs_cache_pressure: vfs_cache_pressure.min(200),
            governor,
        }
    }

    pub fn swappiness(&self) -> u32 {
        self.swappiness
    }

    pub fn vfs_cache_pressure(&self) -> u32 {
        self.vfs_cache_pressure
    }

    pub fn governor(&self) -> GovernorMode {
        self.governor
    }

    pub fn generate_sysctl_conf(&self) -> String {
        format!(
            "# SigmaOS System Adjustments (mintsystem inspiration)\nvm.swappiness = {}\nvm.vfs_cache_pressure = {}\n",
            self.swappiness, self.vfs_cache_pressure
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_adjuster_defaults() {
        // Arrange
        let adjuster = SystemAdjuster::default();

        // Act & Assert
        assert_eq!(adjuster.swappiness(), 10);
        assert_eq!(adjuster.vfs_cache_pressure(), 50);
        assert_eq!(adjuster.governor(), GovernorMode::Schedutil);
    }

    #[test]
    fn test_generate_sysctl_conf() {
        // Arrange
        let adjuster = SystemAdjuster::new(15, 60, GovernorMode::Performance);

        // Act
        let conf = adjuster.generate_sysctl_conf();

        // Assert
        assert!(conf.contains("vm.swappiness = 15"));
        assert!(conf.contains("vm.vfs_cache_pressure = 60"));
    }
}
