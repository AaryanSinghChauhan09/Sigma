//! Init system module inspired by systemd.
//! Provides service configuration, unit management, and target dependencies.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Running,
    Stopped,
    Failed,
    Reloaded,
}

impl ServiceState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Stopped => "stopped",
            Self::Failed => "failed",
            Self::Reloaded => "reloaded",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    Basic,
    MultiUser,
    Graphical,
    Rescue,
    Emergency,
}

impl TargetType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic.target",
            Self::MultiUser => "multi-user.target",
            Self::Graphical => "graphical.target",
            Self::Rescue => "rescue.target",
            Self::Emergency => "emergency.target",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitError {
    InvalidServiceName(String),
    InvalidUnitFile(String),
    InvalidDependency(String),
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidServiceName(msg) => write!(f, "invalid service name: {}", msg),
            Self::InvalidUnitFile(msg) => write!(f, "invalid unit file: {}", msg),
            Self::InvalidDependency(msg) => write!(f, "invalid dependency: {}", msg),
        }
    }
}

impl std::error::Error for InitError {}

#[derive(Debug, Clone)]
pub struct ServiceUnit {
    pub name: String,
    pub description: String,
    pub exec_start: Vec<String>,
    pub exec_stop: Vec<String>,
    pub exec_reload: Vec<String>,
    pub restart: String,
    pub wanted_by: Vec<String>,
    pub after: Vec<String>,
    pub requires: Vec<String>,
    pub environment: BTreeMap<String, String>,
}

impl ServiceUnit {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            exec_start: vec![],
            exec_stop: vec![],
            exec_reload: vec![],
            restart: "on-failure".to_string(),
            wanted_by: vec![],
            after: vec![],
            requires: vec![],
            environment: BTreeMap::new(),
        }
    }

    pub fn with_exec_start(mut self, command: impl Into<String>) -> Self {
        self.exec_start.push(command.into());
        self
    }

    pub fn with_exec_stop(mut self, command: impl Into<String>) -> Self {
        self.exec_stop.push(command.into());
        self
    }

    pub fn with_wanted_by(mut self, target: impl Into<String>) -> Self {
        self.wanted_by.push(target.into());
        self
    }

    pub fn with_after(mut self, unit: impl Into<String>) -> Self {
        self.after.push(unit.into());
        self
    }

    pub fn with_requires(mut self, unit: impl Into<String>) -> Self {
        self.requires.push(unit.into());
        self
    }

    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }

    pub fn generate_systemd_unit(&self) -> String {
        let mut unit = String::new();
        
        unit.push_str("[Unit]\n");
        unit.push_str(&format!("Description={}\n", self.description));
        if !self.after.is_empty() {
            unit.push_str(&format!("After={}\n", self.after.join(" ")));
        }
        if !self.requires.is_empty() {
            unit.push_str(&format!("Requires={}\n", self.requires.join(" ")));
        }
        unit.push_str("\n");
        
        unit.push_str("[Service]\n");
        unit.push_str(&format!("Type=simple\n"));
        unit.push_str(&format!("Restart={}\n", self.restart));
        for (key, value) in &self.environment {
            unit.push_str(&format!("Environment=\"{}={}\"\n", key, value));
        }
        if !self.exec_start.is_empty() {
            unit.push_str(&format!("ExecStart={}\n", self.exec_start.join(" ")));
        }
        if !self.exec_stop.is_empty() {
            unit.push_str(&format!("ExecStop={}\n", self.exec_stop.join(" ")));
        }
        if !self.exec_reload.is_empty() {
            unit.push_str(&format!("ExecReload={}\n", self.exec_reload.join(" ")));
        }
        unit.push_str("\n");
        
        unit.push_str("[Install]\n");
        if !self.wanted_by.is_empty() {
            unit.push_str(&format!("WantedBy={}\n", self.wanted_by.join(" ")));
        }
        
        unit
    }
}

#[derive(Debug, Clone)]
pub struct InitManager {
    pub services: BTreeMap<String, ServiceUnit>,
    pub default_target: TargetType,
    pub enabled_services: Vec<String>,
}

impl Default for InitManager {
    fn default() -> Self {
        Self {
            services: BTreeMap::new(),
            default_target: TargetType::MultiUser,
            enabled_services: vec![],
        }
    }
}

impl InitManager {
    pub fn new(default_target: TargetType) -> Self {
        Self {
            services: BTreeMap::new(),
            default_target,
            enabled_services: vec![],
        }
    }

    pub fn add_service(&mut self, service: ServiceUnit) -> Result<(), InitError> {
        let name = service.name.clone();
        if name.trim().is_empty() {
            return Err(InitError::InvalidServiceName("service name cannot be empty".to_string()));
        }
        self.services.insert(name, service);
        Ok(())
    }

    pub fn enable_service(&mut self, name: &str) -> Result<(), InitError> {
        if !self.services.contains_key(name) {
            return Err(InitError::InvalidServiceName(format!("service {} not found", name)));
        }
        if !self.enabled_services.contains(&name.to_string()) {
            self.enabled_services.push(name.to_string());
        }
        Ok(())
    }

    pub fn disable_service(&mut self, name: &str) {
        self.enabled_services.retain(|s| s != name);
    }

    pub fn get_service(&self, name: &str) -> Option<&ServiceUnit> {
        self.services.get(name)
    }

    pub fn generate_all_units(&self) -> BTreeMap<String, String> {
        let mut units = BTreeMap::new();
        for (name, service) in &self.services {
            units.insert(format!("{}.service", name), service.generate_systemd_unit());
        }
        units
    }

    pub fn generate_default_target_link(&self) -> String {
        format!(
            "# Default target symlink\n\
             ln -sf /usr/lib/systemd/system/{} /etc/systemd/system/default.target",
            self.default_target.as_str()
        )
    }

    pub fn generate_service_enable_script(&self) -> String {
        let mut script = String::new();
        
        script.push_str("#!/bin/bash\n");
        script.push_str("# SigmaOS Service Enable Script\n\n");
        
        for service in &self.enabled_services {
            script.push_str(&format!(
                "ln -sf /usr/lib/systemd/system/{}.service /etc/systemd/system/multi-user.target.wants/\n",
                service
            ));
        }
        
        script.push_str("\nsystemctl daemon-reload\n");
        
        script
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_state_as_str() {
        assert_eq!(ServiceState::Running.as_str(), "running");
        assert_eq!(ServiceState::Stopped.as_str(), "stopped");
    }

    #[test]
    fn test_target_type_as_str() {
        assert_eq!(TargetType::Graphical.as_str(), "graphical.target");
        assert_eq!(TargetType::MultiUser.as_str(), "multi-user.target");
    }

    #[test]
    fn test_service_unit_new() {
        let unit = ServiceUnit::new("test", "Test Service");
        assert_eq!(unit.name, "test");
        assert_eq!(unit.description, "Test Service");
    }

    #[test]
    fn test_service_unit_with_exec_start() {
        let unit = ServiceUnit::new("test", "Test Service")
            .with_exec_start("/usr/bin/test");
        assert_eq!(unit.exec_start.len(), 1);
    }

    #[test]
    fn test_service_unit_generate_systemd_unit() {
        let unit = ServiceUnit::new("test", "Test Service")
            .with_exec_start("/usr/bin/test")
            .with_wanted_by("multi-user.target");
        let systemd_unit = unit.generate_systemd_unit();
        assert!(systemd_unit.contains("[Unit]"));
        assert!(systemd_unit.contains("[Service]"));
        assert!(systemd_unit.contains("[Install]"));
    }

    #[test]
    fn test_init_manager_new() {
        let mgr = InitManager::new(TargetType::Graphical);
        assert_eq!(mgr.default_target, TargetType::Graphical);
    }

    #[test]
    fn test_init_manager_add_service() {
        let mut mgr = InitManager::default();
        let unit = ServiceUnit::new("test", "Test Service");
        mgr.add_service(unit).unwrap();
        assert_eq!(mgr.services.len(), 1);
    }

    #[test]
    fn test_init_manager_add_service_invalid() {
        let mut mgr = InitManager::default();
        let unit = ServiceUnit::new("", "Test Service");
        assert!(mgr.add_service(unit).is_err());
    }

    #[test]
    fn test_init_manager_enable_service() {
        let mut mgr = InitManager::default();
        let unit = ServiceUnit::new("test", "Test Service");
        mgr.add_service(unit).unwrap();
        mgr.enable_service("test").unwrap();
        assert!(mgr.enabled_services.contains(&"test".to_string()));
    }

    #[test]
    fn test_generate_all_units() {
        let mut mgr = InitManager::default();
        let unit = ServiceUnit::new("test", "Test Service");
        mgr.add_service(unit).unwrap();
        let units = mgr.generate_all_units();
        assert!(units.contains_key("test.service"));
    }
}
