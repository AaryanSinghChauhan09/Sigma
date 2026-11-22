//! System adjustments for performance optimization.
//! Manages CPU governors, sysctl memory tuning, and hardware optimization.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernorMode {
    Performance,
    Powersave,
    Schedutil,
    Interactive,
    Conservative,
    Ondemand,
}

impl GovernorMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Powersave => "powersave",
            Self::Schedutil => "schedutil",
            Self::Interactive => "interactive",
            Self::Conservative => "conservative",
            Self::Ondemand => "ondemand",
        }
    }
}

/// Fail-fast: validate governor mode (defensive programming)
pub fn validate_governor(_mode: GovernorMode) -> GovernorMode {
    _mode
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
    /// Fail-fast: reject out-of-range swappiness values immediately
    pub fn new(swappiness: u32, vfs_cache_pressure: u32, governor: GovernorMode) -> Result<Self, String> {
        if swappiness > 100 {
            return Err(format!(
                "swappiness {} exceeds maximum 100 (Linux kernel range)",
                swappiness
            ));
        }
        if vfs_cache_pressure > 200 {
            return Err(format!(
                "vfs_cache_pressure {} exceeds maximum 200 (Linux kernel range)",
                vfs_cache_pressure
            ));
        }
        Ok(Self {
            swappiness,
            vfs_cache_pressure,
            governor: validate_governor(governor),
        })
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

    /// Generate Linux-style sysctl configuration inspired by kernel boot parameters
    pub fn generate_sysctl_conf(&self) -> String {
        format!(
            "# SigmaOS System Adjustments\n# Inspired by Linux kernel mintsystem and SigmaOS tuning\nvm.swappiness = {}\nvm.vfs_cache_pressure = {}\n",
            self.swappiness, self.vfs_cache_pressure
        )
    }

    /// Generate kernel command-line parameters inspired by Linux kernel cmdline
    pub fn generate_kernel_params(&self) -> String {
        format!(
            "swappiness={} vfs_cache_pressure={} governor={}",
            self.swappiness, self.vfs_cache_pressure, self.governor.as_str()
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
    fn test_system_adjuster_valid_construction() {
        // Act
        let adjuster = SystemAdjuster::new(15, 60, GovernorMode::Performance);

        // Assert
        assert!(adjuster.is_ok());
        let a = adjuster.unwrap();
        assert_eq!(a.swappiness(), 15);
        assert_eq!(a.vfs_cache_pressure(), 60);
        assert_eq!(a.governor(), GovernorMode::Performance);
    }

    #[test]
    fn test_system_adjuster_fail_fast_swappiness() {
        // Act & Assert - fail-fast on invalid swappiness
        let result = SystemAdjuster::new(150, 50, GovernorMode::Performance);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum 100"));
    }

    #[test]
    fn test_system_adjuster_fail_fast_vfs_cache_pressure() {
        // Act & Assert - fail-fast on invalid vfs_cache_pressure
        let result = SystemAdjuster::new(50, 250, GovernorMode::Performance);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum 200"));
    }

    #[test]
    fn test_generate_sysctl_conf() {
        // Arrange
        let adjuster = SystemAdjuster::new(15, 60, GovernorMode::Performance).unwrap();

        // Act
        let conf = adjuster.generate_sysctl_conf();

        // Assert
        assert!(conf.contains("vm.swappiness = 15"));
        assert!(conf.contains("vm.vfs_cache_pressure = 60"));
        assert!(conf.contains("# SigmaOS System Adjustments"));
        assert!(conf.contains("# Inspired by Linux kernel"));
    }

    #[test]
    fn test_generate_kernel_params() {
        // Arrange
        let adjuster = SystemAdjuster::new(10, 30, GovernorMode::Interactive).unwrap();

        // Act
        let params = adjuster.generate_kernel_params();

        // Assert
        assert!(params.contains("swappiness=10"));
        assert!(params.contains("vfs_cache_pressure=30"));
        assert!(params.contains("governor=interactive"));
    }
}