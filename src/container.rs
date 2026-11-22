//! Container runtime module inspired by Podman and Docker.
//! Provides lightweight container runtime for virtualization.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerRuntime {
    Podman,
    Docker,
    Containers,
}

impl ContainerRuntime {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Podman => "podman",
            Self::Docker => "docker",
            Self::Containers => "containers",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerStatus {
    Running,
    Stopped,
    Paused,
    Creating,
    Dead,
}

impl ContainerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Stopped => "stopped",
            Self::Paused => "paused",
            Self::Creating => "creating",
            Self::Dead => "dead",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerError {
    InvalidImage(String),
    InvalidName(String),
    ResourceLimit(String),
    NetworkConfig(String),
}

impl fmt::Display for ContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidImage(msg) => write!(f, "invalid image: {}", msg),
            Self::InvalidName(msg) => write!(f, "invalid name: {}", msg),
            Self::ResourceLimit(msg) => write!(f, "resource limit error: {}", msg),
            Self::NetworkConfig(msg) => write!(f, "network config error: {}", msg),
        }
    }
}

impl std::error::Error for ContainerError {}

#[derive(Debug, Clone)]
pub struct ContainerSpec {
    pub name: String,
    pub image: String,
    pub command: Vec<String>,
    pub ports: Vec<u16>,
    pub volumes: BTreeMap<String, String>,
    pub environment: BTreeMap<String, String>,
    pub memory_limit: Option<u64>,
    pub cpu_limit: Option<f64>,
    pub network: String,
}

impl ContainerSpec {
    pub fn new(name: impl Into<String>, image: impl Into<String>) -> Result<Self, ContainerError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ContainerError::InvalidName("container name cannot be empty".to_string()));
        }
        let image = image.into();
        if image.trim().is_empty() {
            return Err(ContainerError::InvalidImage("container image cannot be empty".to_string()));
        }
        Ok(Self {
            name,
            image,
            command: vec![],
            ports: vec![],
            volumes: BTreeMap::new(),
            environment: BTreeMap::new(),
            memory_limit: None,
            cpu_limit: None,
            network: "bridge".to_string(),
        })
    }

    pub fn with_command(mut self, cmd: impl Into<String>) -> Self {
        self.command.push(cmd.into());
        self
    }

    pub fn with_port(mut self, port: u16) -> Result<Self, ContainerError> {
        if port == 0 || port > 65535 {
            return Err(ContainerError::InvalidName(format!("invalid port: {}", port)));
        }
        self.ports.push(port);
        Ok(self)
    }

    pub fn with_volume(mut self, host: impl Into<String>, container: impl Into<String>) -> Self {
        self.volumes.insert(host.into(), container.into());
        self
    }

    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }

    pub fn with_memory_limit(mut self, limit_mb: u64) -> Self {
        self.memory_limit = Some(limit_mb * 1024 * 1024);
        self
    }

    pub fn with_cpu_limit(mut self, cpus: f64) -> Self {
        self.cpu_limit = Some(cpus);
        self
    }

    pub fn with_network(mut self, network: impl Into<String>) -> Self {
        self.network = network.into();
        self
    }
}

#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub spec: ContainerSpec,
    pub status: ContainerStatus,
    pub created_at: u64,
}

impl Container {
    pub fn new(spec: ContainerSpec) -> Self {
        let id = format!("sigmaos-{}-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis(), &spec.name);
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            id,
            spec,
            status: ContainerStatus::Creating,
            created_at,
        }
    }

    pub fn start(&mut self) {
        self.status = ContainerStatus::Running;
    }

    pub fn stop(&mut self) {
        self.status = ContainerStatus::Stopped;
    }

    pub fn pause(&mut self) {
        self.status = ContainerStatus::Paused;
    }

    pub fn resume(&mut self) {
        self.status = ContainerStatus::Running;
    }
}

#[derive(Debug, Clone)]
pub struct ContainerManager {
    pub runtime: ContainerRuntime,
    pub containers: BTreeMap<String, Container>,
    pub images: BTreeMap<String, String>,
}

impl Default for ContainerManager {
    fn default() -> Self {
        Self {
            runtime: ContainerRuntime::Podman,
            containers: BTreeMap::new(),
            images: BTreeMap::new(),
        }
    }
}

impl ContainerManager {
    pub fn new(runtime: ContainerRuntime) -> Self {
        Self {
            runtime,
            containers: BTreeMap::new(),
            images: BTreeMap::new(),
        }
    }

    pub fn add_image(&mut self, name: impl Into<String>, digest: impl Into<String>) {
        self.images.insert(name.into(), digest.into());
    }

    pub fn create_container(&mut self, spec: ContainerSpec) -> Result<Container, ContainerError> {
        let container = Container::new(spec);
        let id = container.id.clone();
        self.containers.insert(id.clone(), container);
        Ok(self.containers.get(&id).unwrap().clone())
    }

    pub fn start_container(&mut self, id: &str) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(id)
            .ok_or_else(|| ContainerError::InvalidName(format!("container not found: {}", id)))?;
        container.start();
        Ok(())
    }

    pub fn stop_container(&mut self, id: &str) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(id)
            .ok_or_else(|| ContainerError::InvalidName(format!("container not found: {}", id)))?;
        container.stop();
        Ok(())
    }

    pub fn pause_container(&mut self, id: &str) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(id)
            .ok_or_else(|| ContainerError::InvalidName(format!("container not found: {}", id)))?;
        container.pause();
        Ok(())
    }

    pub fn resume_container(&mut self, id: &str) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(id)
            .ok_or_else(|| ContainerError::InvalidName(format!("container not found: {}", id)))?;
        container.resume();
        Ok(())
    }

    pub fn remove_container(&mut self, id: &str) -> Result<(), ContainerError> {
        self.containers.remove(id)
            .ok_or_else(|| ContainerError::InvalidName(format!("container not found: {}", id)))?;
        Ok(())
    }

    pub fn list_containers(&self) -> Vec<&Container> {
        self.containers.values().collect()
    }

    pub fn get_container(&self, id: &str) -> Option<&Container> {
        self.containers.get(id)
    }

    pub fn generate_podman_command(&self, container: &Container) -> String {
        let mut cmd = vec!["podman".to_string(), "run".to_string()];
        
        if let Some(mem) = container.spec.memory_limit {
            cmd.push("--memory".to_string());
            cmd.push(format!("{}m", mem / (1024 * 1024)));
        }
        
        if let Some(cpu) = container.spec.cpu_limit {
            cmd.push("--cpus".to_string());
            cmd.push(format!("{}", cpu));
        }
        
        for port in &container.spec.ports {
            cmd.push("-p".to_string());
            cmd.push(format!("{}:{}", port, port));
        }
        
        for (host, cont) in &container.spec.volumes {
            cmd.push("-v".to_string());
            cmd.push(format!("{}:{}", host, cont));
        }
        
        for (key, value) in &container.spec.environment {
            cmd.push("-e".to_string());
            cmd.push(format!("{}={}", key, value));
        }
        
        cmd.push("--network".to_string());
        cmd.push(container.spec.network.clone());
        
        cmd.push(container.spec.image.clone());
        
        for arg in &container.spec.command {
            cmd.push(arg.clone());
        }
        
        cmd.join(" ")
    }

    pub fn generate_docker_compose(&self) -> String {
        let mut compose = String::new();
        
        compose.push_str("version: '3.8'\nservices:\n");
        
        for container in self.containers.values() {
            compose.push_str(&format!("  {}:\n", container.spec.name));
            compose.push_str(&format!("    image: {}\n", container.spec.image));
            
            if !container.spec.command.is_empty() {
                compose.push_str("    command:\n");
                for cmd in &container.spec.command {
                    compose.push_str(&format!("      - {}\n", cmd));
                }
            }
            
            if !container.spec.ports.is_empty() {
                compose.push_str("    ports:\n");
                for port in &container.spec.ports {
                    compose.push_str(&format!("      - \"{}:{}\"\n", port, port));
                }
            }
            
            if !container.spec.environment.is_empty() {
                compose.push_str("    environment:\n");
                for (key, value) in &container.spec.environment {
                    compose.push_str(&format!("      {}: {}\n", key, value));
                }
            }
            
            if !container.spec.volumes.is_empty() {
                compose.push_str("    volumes:\n");
                for (host, cont) in &container.spec.volumes {
                    compose.push_str(&format!("      - \"{}:{}\"\n", host, cont));
                }
            }
            
            if let Some(mem) = container.spec.memory_limit {
                compose.push_str(&format!("    mem_limit: {}m\n", mem / (1024 * 1024)));
            }
            
            if let Some(cpu) = container.spec.cpu_limit {
                compose.push_str(&format!("    cpus: {}\n", cpu));
            }
        }
        
        compose
    }

    pub fn get_running_containers(&self) -> Vec<&Container> {
        self.containers.values()
            .filter(|c| c.status == ContainerStatus::Running)
            .collect()
    }

    pub fn container_count(&self) -> usize {
        self.containers.len()
    }

    pub fn image_count(&self) -> usize {
        self.images.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_runtime_as_str() {
        assert_eq!(ContainerRuntime::Podman.as_str(), "podman");
        assert_eq!(ContainerRuntime::Docker.as_str(), "docker");
    }

    #[test]
    fn test_container_status_as_str() {
        assert_eq!(ContainerStatus::Running.as_str(), "running");
        assert_eq!(ContainerStatus::Stopped.as_str(), "stopped");
    }

    #[test]
    fn test_container_spec_new() {
        let spec = ContainerSpec::new("test", "ubuntu:latest");
        assert!(spec.is_ok());
        assert_eq!(spec.unwrap().name, "test");
    }

    #[test]
    fn test_container_spec_new_invalid_name() {
        let spec = ContainerSpec::new("", "ubuntu:latest");
        assert!(spec.is_err());
    }

    #[test]
    fn test_container_spec_new_invalid_image() {
        let spec = ContainerSpec::new("test", "");
        assert!(spec.is_err());
    }

    #[test]
    fn test_container_spec_with_command() {
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap()
            .with_command("echo hello")
            .with_command("sleep 100");
        assert_eq!(spec.command.len(), 2);
    }

    #[test]
    fn test_container_spec_with_port() {
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap()
            .with_port(80)
            .unwrap();
        assert_eq!(spec.ports.len(), 1);
    }

    #[test]
    fn test_container_spec_with_port_invalid() {
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap()
            .with_port(0);
        assert!(spec.is_err());
    }

    #[test]
    fn test_container_spec_with_volume() {
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap()
            .with_volume("/host", "/container");
        assert_eq!(spec.volumes.len(), 1);
    }

    #[test]
    fn test_container_spec_with_memory_limit() {
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap()
            .with_memory_limit(512);
        assert_eq!(spec.memory_limit, Some(512 * 1024 * 1024));
    }

    #[test]
    fn test_container_spec_with_cpu_limit() {
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap()
            .with_cpu_limit(1.5);
        assert_eq!(spec.cpu_limit, Some(1.5));
    }

    #[test]
    fn test_container_manager_new() {
        let mgr = ContainerManager::new(ContainerRuntime::Podman);
        assert_eq!(mgr.runtime, ContainerRuntime::Podman);
    }

    #[test]
    fn test_container_manager_create_container() {
        let mut mgr = ContainerManager::default();
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap();
        let container = mgr.create_container(spec).unwrap();
        assert_eq!(container.spec.name, "test");
        assert_eq!(mgr.container_count(), 1);
    }

    #[test]
    fn test_container_manager_start_container() {
        let mut mgr = ContainerManager::default();
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap();
        let container = mgr.create_container(spec).unwrap();
        mgr.start_container(&container.id).unwrap();
        let updated = mgr.get_container(&container.id).unwrap();
        assert_eq!(updated.status, ContainerStatus::Running);
    }

    #[test]
    fn test_container_manager_stop_container() {
        let mut mgr = ContainerManager::default();
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap();
        let container = mgr.create_container(spec).unwrap();
        mgr.start_container(&container.id).unwrap();
        mgr.stop_container(&container.id).unwrap();
        let updated = mgr.get_container(&container.id).unwrap();
        assert_eq!(updated.status, ContainerStatus::Stopped);
    }

    #[test]
    fn test_container_manager_pause_container() {
        let mut mgr = ContainerManager::default();
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap();
        let container = mgr.create_container(spec).unwrap();
        mgr.start_container(&container.id).unwrap();
        mgr.pause_container(&container.id).unwrap();
        let updated = mgr.get_container(&container.id).unwrap();
        assert_eq!(updated.status, ContainerStatus::Paused);
    }

    #[test]
    fn test_container_manager_remove_container() {
        let mut mgr = ContainerManager::default();
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap();
        let container = mgr.create_container(spec).unwrap();
        mgr.remove_container(&container.id).unwrap();
        assert_eq!(mgr.container_count(), 0);
    }

    #[test]
    fn test_container_manager_remove_container_not_found() {
        let mut mgr = ContainerManager::default();
        let result = mgr.remove_container("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_podman_command() {
        let mut mgr = ContainerManager::default();
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap()
            .with_command("echo hello")
            .with_port(80)
            .unwrap();
        let container = mgr.create_container(spec).unwrap();
        let cmd = mgr.generate_podman_command(&container);
        assert!(cmd.contains("podman run"));
        assert!(cmd.contains("ubuntu:latest"));
    }

    #[test]
    fn test_generate_docker_compose() {
        let mut mgr = ContainerManager::default();
        let spec = ContainerSpec::new("test", "ubuntu:latest").unwrap();
        mgr.create_container(spec).unwrap();
        let compose = mgr.generate_docker_compose();
        assert!(compose.contains("version: '3.8'"));
        assert!(compose.contains("services:"));
    }
}
