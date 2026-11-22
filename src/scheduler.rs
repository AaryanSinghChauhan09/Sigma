//! SigmaOS scheduler module.
//!
//! This module provides abstractions for process scheduling,
//! CPU affinity, and priority management.
//!
//! # Example
//!
//! ```
//! use sigma_system::scheduler::*;
//!
//! let mut scheduler = Scheduler::new();
//! scheduler.add_process(1234, "test").unwrap();
//! scheduler.set_process_priority(1234, Priority::High).unwrap();
//! scheduler.set_cpu_affinity(1234, &[0, 1]).unwrap();
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in scheduling operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerError {
    /// Process not found.
    ProcessNotFound(u32),
    /// Invalid priority.
    InvalidPriority(String),
    /// Invalid CPU affinity.
    InvalidAffinity(String),
    /// Permission denied.
    PermissionDenied(String),
}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchedulerError::ProcessNotFound(pid) => write!(f, "Process {} not found", pid),
            SchedulerError::InvalidPriority(p) => write!(f, "Invalid priority '{}'", p),
            SchedulerError::InvalidAffinity(a) => write!(f, "Invalid affinity '{}'", a),
            SchedulerError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
        }
    }
}

impl std::error::Error for SchedulerError {}

/// Process priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Realtime priority (highest).
    Realtime = -20,
    /// High priority.
    High = -10,
    /// Normal priority.
    Normal = 0,
    /// Low priority.
    Low = 10,
    /// Lowest priority.
    Lowest = 19,
}

impl Priority {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Realtime => "realtime",
            Priority::High => "high",
            Priority::Normal => "normal",
            Priority::Low => "low",
            Priority::Lowest => "lowest",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Result<Self, SchedulerError> {
        match s.to_lowercase().as_str() {
            "realtime" | "rt" => Ok(Priority::Realtime),
            "high" => Ok(Priority::High),
            "normal" | "default" => Ok(Priority::Normal),
            "low" => Ok(Priority::Low),
            "lowest" | "idle" => Ok(Priority::Lowest),
            _ => Err(SchedulerError::InvalidPriority(s.to_string())),
        }
    }

    /// Get numeric value.
    pub fn value(&self) -> i32 {
        *self as i32
    }
}

/// Scheduling policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingPolicy {
    /// Normal scheduling.
    Normal,
    /// Round-robin.
    RoundRobin,
    /// FIFO.
    Fifo,
    /// Batch.
    Batch,
    /// Idle.
    Idle,
}

impl SchedulingPolicy {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            SchedulingPolicy::Normal => "normal",
            SchedulingPolicy::RoundRobin => "rr",
            SchedulingPolicy::Fifo => "fifo",
            SchedulingPolicy::Batch => "batch",
            SchedulingPolicy::Idle => "idle",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Result<Self, SchedulerError> {
        match s.to_lowercase().as_str() {
            "normal" | "other" => Ok(SchedulingPolicy::Normal),
            "rr" | "round-robin" | "roundrobin" => Ok(SchedulingPolicy::RoundRobin),
            "fifo" => Ok(SchedulingPolicy::Fifo),
            "batch" => Ok(SchedulingPolicy::Batch),
            "idle" => Ok(SchedulingPolicy::Idle),
            _ => Err(SchedulerError::InvalidPriority(s.to_string())),
        }
    }
}

/// Process information.
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID.
    pub pid: u32,
    /// Process name.
    pub name: String,
    /// Current priority.
    pub priority: Priority,
    /// Scheduling policy.
    pub policy: SchedulingPolicy,
    /// CPU affinity mask.
    pub affinity: Vec<u32>,
    /// CPU usage percentage.
    pub cpu_usage: f64,
    /// Memory usage in MB.
    pub memory_mb: u64,
}

impl ProcessInfo {
    /// Create a new process info.
    pub fn new(pid: u32, name: &str) -> Self {
        Self {
            pid,
            name: name.to_string(),
            priority: Priority::Normal,
            policy: SchedulingPolicy::Normal,
            affinity: vec![0],
            cpu_usage: 0.0,
            memory_mb: 0,
        }
    }
}

/// Scheduler.
#[derive(Debug)]
pub struct Scheduler {
    /// Processes by PID.
    processes: BTreeMap<u32, ProcessInfo>,
    /// CPU count.
    cpu_count: u32,
    /// Scheduler enabled.
    enabled: bool,
}

impl Scheduler {
    /// Create a new scheduler.
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            cpu_count: 4,
            enabled: true,
        }
    }

    /// Set CPU count.
    pub fn set_cpu_count(&mut self, count: u32) {
        self.cpu_count = count;
    }

    /// Get CPU count.
    pub fn get_cpu_count(&self) -> u32 {
        self.cpu_count
    }

    /// Enable scheduler.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable scheduler.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if scheduler is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Add a process.
    pub fn add_process(&mut self, pid: u32, name: &str) -> Result<(), SchedulerError> {
        if self.processes.contains_key(&pid) {
            return Err(SchedulerError::ProcessNotFound(pid));
        }
        let process = ProcessInfo::new(pid, name);
        self.processes.insert(pid, process);
        Ok(())
    }

    /// Remove a process.
    pub fn remove_process(&mut self, pid: u32) -> Result<(), SchedulerError> {
        if self.processes.remove(&pid).is_some() {
            Ok(())
        } else {
            Err(SchedulerError::ProcessNotFound(pid))
        }
    }

    /// Get a process.
    pub fn get_process(&self, pid: u32) -> Option<&ProcessInfo> {
        self.processes.get(&pid)
    }

    /// Get all processes.
    pub fn get_processes(&self) -> Vec<&ProcessInfo> {
        self.processes.values().collect()
    }

    /// Set process priority.
    pub fn set_process_priority(&mut self, pid: u32, priority: Priority) -> Result<(), SchedulerError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.priority = priority;
            Ok(())
        } else {
            Err(SchedulerError::ProcessNotFound(pid))
        }
    }

    /// Set process scheduling policy.
    pub fn set_process_policy(&mut self, pid: u32, policy: SchedulingPolicy) -> Result<(), SchedulerError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.policy = policy;
            Ok(())
        } else {
            Err(SchedulerError::ProcessNotFound(pid))
        }
    }

    /// Set CPU affinity.
    pub fn set_cpu_affinity(&mut self, pid: u32, cpus: &[u32]) -> Result<(), SchedulerError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            // Validate CPU IDs
            for cpu in cpus {
                if *cpu >= self.cpu_count {
                    return Err(SchedulerError::InvalidAffinity(format!("CPU {} exceeds count {}", cpu, self.cpu_count)));
                }
            }
            process.affinity = cpus.to_vec();
            Ok(())
        } else {
            Err(SchedulerError::ProcessNotFound(pid))
        }
    }

    /// Update process stats.
    pub fn update_process_stats(&mut self, pid: u32, cpu_usage: f64, memory_mb: u64) -> Result<(), SchedulerError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.cpu_usage = cpu_usage;
            process.memory_mb = memory_mb;
            Ok(())
        } else {
            Err(SchedulerError::ProcessNotFound(pid))
        }
    }

    /// Get processes by priority.
    pub fn get_processes_by_priority(&self, priority: Priority) -> Vec<&ProcessInfo> {
        self.processes.values()
            .filter(|p| p.priority == priority)
            .collect()
    }

    /// Get high priority processes.
    pub fn get_high_priority_processes(&self) -> Vec<&ProcessInfo> {
        self.processes.values()
            .filter(|p| p.priority == Priority::High || p.priority == Priority::Realtime)
            .collect()
    }

    /// Generate scheduling configuration.
    pub fn generate_config(&self) -> String {
        let mut config = String::new();
        config.push_str("# SigmaOS Scheduler Configuration\n\n");
        config.push_str(&format!("CPU_COUNT={}\n", self.cpu_count));
        config.push_str(&format!("ENABLED={}\n", if self.enabled { "true" } else { "false" }));
        config.push_str("\n[Processes]\n");
        
        for process in self.processes.values() {
            config.push_str(&format!("[process:{}]\n", process.pid));
            config.push_str(&format!("name = {}\n", process.name));
            config.push_str(&format!("priority = {}\n", process.priority.as_str()));
            config.push_str(&format!("policy = {}\n", process.policy.as_str()));
            config.push_str(&format!("affinity = {:?}\n", process.affinity));
            config.push_str("\n");
        }
        
        config
    }

    /// Generate systemd service for scheduler.
    pub fn generate_systemd_service(&self) -> String {
        let mut service = String::new();
        service.push_str("[Unit]\n");
        service.push_str("Description=SigmaOS Scheduler Service\n");
        service.push_str("After=systemd-udevd.service\n\n");
        
        service.push_str("[Service]\n");
        service.push_str("Type=simple\n");
        service.push_str("ExecStart=/usr/bin/sigmaos-scheduler\n");
        service.push_str("Restart=always\n");
        service.push_str("RestartSec=5\n\n");
        
        service.push_str("[Install]\n");
        service.push_str("WantedBy=multi-user.target\n");
        
        service
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(1234, "test-process").unwrap();
        assert!(scheduler.get_process(1234).is_some());
    }

    #[test]
    fn test_set_process_priority() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(1234, "test-process").unwrap();
        scheduler.set_process_priority(1234, Priority::High).unwrap();
        assert_eq!(scheduler.get_process(1234).unwrap().priority, Priority::High);
    }

    #[test]
    fn test_set_cpu_affinity() {
        let mut scheduler = Scheduler::new();
        scheduler.set_cpu_count(4);
        scheduler.add_process(1234, "test-process").unwrap();
        scheduler.set_cpu_affinity(1234, &[0, 1]).unwrap();
        assert_eq!(scheduler.get_process(1234).unwrap().affinity, vec![0, 1]);
    }

    #[test]
    fn test_invalid_affinity() {
        let mut scheduler = Scheduler::new();
        scheduler.set_cpu_count(2);
        scheduler.add_process(1234, "test-process").unwrap();
        let result = scheduler.set_cpu_affinity(1234, &[0, 1, 2]);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_high_priority_processes() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(1234, "high-priority").unwrap();
        scheduler.set_process_priority(1234, Priority::High).unwrap();
        scheduler.add_process(1235, "normal-priority").unwrap();
        let high = scheduler.get_high_priority_processes();
        assert_eq!(high.len(), 1);
        assert_eq!(high[0].pid, 1234);
    }

    #[test]
    fn test_generate_config() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(1234, "test-process").unwrap();
        scheduler.set_process_priority(1234, Priority::High).unwrap();
        let config = scheduler.generate_config();
        assert!(config.contains("test-process"));
        assert!(config.contains("high"));
    }

    #[test]
    fn test_priority_parse() {
        assert_eq!(Priority::parse("high").unwrap(), Priority::High);
        assert_eq!(Priority::parse("normal").unwrap(), Priority::Normal);
        assert!(Priority::parse("invalid").is_err());
    }
}
