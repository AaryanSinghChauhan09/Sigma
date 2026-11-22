//! SigmaOS cron and scheduled task management module.
//!
//! This module provides abstractions for cron jobs, scheduled tasks,
//! and periodic system maintenance.
//!
//! # Example
//!
//! ```
//! use sigma_system::cron::*;
//!
//! let mut cron_mgr = CronManager::new();
//! cron_mgr.add_job("backup", "0 2 * * *", "sigmaos-backup").unwrap();
//! cron_mgr.add_job("cleanup", "0 3 * * 0", "sigmaos-cleanup").unwrap();
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in cron management.
#[derive(Debug, Clone, PartialEq)]
pub enum CronError {
    /// Invalid cron expression.
    InvalidCronExpression(String),
    /// Job already exists.
    JobExists(String),
    /// Job not found.
    JobNotFound(String),
    /// Invalid command.
    InvalidCommand(String),
}

impl std::fmt::Display for CronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CronError::InvalidCronExpression(expr) => write!(f, "Invalid cron expression '{}'", expr),
            CronError::JobExists(name) => write!(f, "Job '{}' already exists", name),
            CronError::JobNotFound(name) => write!(f, "Job '{}' not found", name),
            CronError::InvalidCommand(cmd) => write!(f, "Invalid command '{}'", cmd),
        }
    }
}

impl std::error::Error for CronError {}

/// Cron job.
#[derive(Debug, Clone, PartialEq)]
pub struct CronJob {
    /// Job name.
    pub name: String,
    /// Cron expression.
    pub schedule: String,
    /// Command to execute.
    pub command: String,
    /// Whether job is enabled.
    pub enabled: bool,
    /// Last run timestamp.
    pub last_run: Option<i64>,
    /// Next run timestamp.
    pub next_run: Option<i64>,
}

impl CronJob {
    /// Create a new cron job.
    pub fn new(name: &str, schedule: &str, command: &str) -> Result<Self, CronError> {
        if name.is_empty() {
            return Err(CronError::InvalidCommand(name.to_string()));
        }
        if command.is_empty() {
            return Err(CronError::InvalidCommand(command.to_string()));
        }
        if !Self::validate_cron(schedule) {
            return Err(CronError::InvalidCronExpression(schedule.to_string()));
        }
        Ok(Self {
            name: name.to_string(),
            schedule: schedule.to_string(),
            command: command.to_string(),
            enabled: true,
            last_run: None,
            next_run: None,
        })
    }

    /// Validate cron expression (simplified).
    fn validate_cron(expr: &str) -> bool {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() != 5 {
            return false;
        }
        // Simplified validation - check if parts are valid cron fields
        for part in parts {
            if part.is_empty() {
                return false;
            }
            // Check for invalid characters
            if part.contains('x') || part.contains('z') {
                return false;
            }
        }
        true
    }
}

/// Cron manager.
#[derive(Debug)]
pub struct CronManager {
    /// Jobs by name.
    jobs: BTreeMap<String, CronJob>,
}

impl CronManager {
    /// Create a new cron manager.
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
        }
    }

    /// Add a cron job.
    pub fn add_job(&mut self, name: &str, schedule: &str, command: &str) -> Result<CronJob, CronError> {
        if self.jobs.contains_key(name) {
            return Err(CronError::JobExists(name.to_string()));
        }
        let job = CronJob::new(name, schedule, command)?;
        self.jobs.insert(name.to_string(), job.clone());
        Ok(job)
    }

    /// Remove a cron job.
    pub fn remove_job(&mut self, name: &str) -> Result<(), CronError> {
        if !self.jobs.contains_key(name) {
            return Err(CronError::JobNotFound(name.to_string()));
        }
        self.jobs.remove(name);
        Ok(())
    }

    /// Get a cron job.
    pub fn get_job(&self, name: &str) -> Option<&CronJob> {
        self.jobs.get(name)
    }

    /// Get all cron jobs.
    pub fn get_jobs(&self) -> Vec<&CronJob> {
        self.jobs.values().collect()
    }

    /// Enable a job.
    pub fn enable_job(&mut self, name: &str) -> Result<(), CronError> {
        if let Some(job) = self.jobs.get_mut(name) {
            job.enabled = true;
            Ok(())
        } else {
            Err(CronError::JobNotFound(name.to_string()))
        }
    }

    /// Disable a job.
    pub fn disable_job(&mut self, name: &str) -> Result<(), CronError> {
        if let Some(job) = self.jobs.get_mut(name) {
            job.enabled = false;
            Ok(())
        } else {
            Err(CronError::JobNotFound(name.to_string()))
        }
    }

    /// Update job schedule.
    pub fn update_schedule(&mut self, name: &str, schedule: &str) -> Result<(), CronError> {
        if !CronJob::validate_cron(schedule) {
            return Err(CronError::InvalidCronExpression(schedule.to_string()));
        }
        if let Some(job) = self.jobs.get_mut(name) {
            job.schedule = schedule.to_string();
            Ok(())
        } else {
            Err(CronError::JobNotFound(name.to_string()))
        }
    }

    /// Update job command.
    pub fn update_command(&mut self, name: &str, command: &str) -> Result<(), CronError> {
        if command.is_empty() {
            return Err(CronError::InvalidCommand(command.to_string()));
        }
        if let Some(job) = self.jobs.get_mut(name) {
            job.command = command.to_string();
            Ok(())
        } else {
            Err(CronError::JobNotFound(name.to_string()))
        }
    }

    /// Mark job as run.
    pub fn mark_run(&mut self, name: &str, timestamp: i64) -> Result<(), CronError> {
        if let Some(job) = self.jobs.get_mut(name) {
            job.last_run = Some(timestamp);
            job.next_run = Some(timestamp + 3600); // Simplified: next run in 1 hour
            Ok(())
        } else {
            Err(CronError::JobNotFound(name.to_string()))
        }
    }

    /// Get enabled jobs.
    pub fn get_enabled_jobs(&self) -> Vec<&CronJob> {
        self.jobs.values().filter(|j| j.enabled).collect()
    }

    /// Generate crontab content.
    pub fn generate_crontab(&self) -> String {
        let mut content = String::new();
        content.push_str("# SigmaOS Cron Jobs\n");
        content.push_str("# Generated by SigmaOS Cron Manager\n\n");
        
        for job in self.jobs.values() {
            if job.enabled {
                let comment = format!("# {}", job.name);
                content.push_str(&format!("{} {} {}\n", job.schedule, job.command, comment));
            }
        }
        content
    }

    /// Generate systemd timer units.
    pub fn generate_systemd_timers(&self) -> String {
        let mut content = String::new();
        for job in self.jobs.values() {
            if job.enabled {
                content.push_str(&format!("[Unit]\nDescription={}\n\n[Timer]\nOnCalendar={}\nPersistent=true\n\n[Install]\nWantedBy=timers.target\n\n", job.name, job.schedule));
            }
        }
        content
    }

    /// Add default system jobs.
    pub fn add_default_jobs(&mut self) {
        let defaults = [
            ("daily-update", "0 2 * * *", "sigmaos-update-check"),
            ("weekly-cleanup", "0 3 * * 0", "sigmaos-cleanup"),
            ("monthly-backup", "0 4 1 * *", "sigmaos-backup"),
            ("hourly-monitor", "0 * * * *", "sigmaos-monitor-check"),
            ("daily-logrotate", "0 0 * * *", "sigmaos-logrotate"),
        ];
        for (name, schedule, command) in defaults {
            if let Ok(job) = CronJob::new(name, schedule, command) {
                self.jobs.insert(name.to_string(), job);
            }
        }
    }
}

impl Default for CronManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_job() {
        let mut mgr = CronManager::new();
        let job = mgr.add_job("backup", "0 2 * * *", "sigmaos-backup").unwrap();
        assert_eq!(job.name, "backup");
        assert_eq!(job.schedule, "0 2 * * *");
    }

    #[test]
    fn test_add_duplicate_job() {
        let mut mgr = CronManager::new();
        mgr.add_job("backup", "0 2 * * *", "sigmaos-backup").unwrap();
        let result = mgr.add_job("backup", "0 3 * * *", "sigmaos-backup");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_job() {
        let mut mgr = CronManager::new();
        mgr.add_job("backup", "0 2 * * *", "sigmaos-backup").unwrap();
        mgr.remove_job("backup").unwrap();
        assert!(mgr.get_job("backup").is_none());
    }

    #[test]
    fn test_enable_disable_job() {
        let mut mgr = CronManager::new();
        mgr.add_job("backup", "0 2 * * *", "sigmaos-backup").unwrap();
        mgr.disable_job("backup").unwrap();
        assert!(!mgr.get_job("backup").unwrap().enabled);
        mgr.enable_job("backup").unwrap();
        assert!(mgr.get_job("backup").unwrap().enabled);
    }

    #[test]
    fn test_update_schedule() {
        let mut mgr = CronManager::new();
        mgr.add_job("backup", "0 2 * * *", "sigmaos-backup").unwrap();
        mgr.update_schedule("backup", "0 3 * * *").unwrap();
        assert_eq!(mgr.get_job("backup").unwrap().schedule, "0 3 * * *");
    }

    #[test]
    fn test_invalid_cron() {
        let mut mgr = CronManager::new();
        let result = mgr.add_job("backup", "invalid", "sigmaos-backup");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_crontab() {
        let mut mgr = CronManager::new();
        mgr.add_job("backup", "0 2 * * *", "sigmaos-backup").unwrap();
        mgr.add_job("cleanup", "0 3 * * 0", "sigmaos-cleanup").unwrap();
        let crontab = mgr.generate_crontab();
        assert!(crontab.contains("sigmaos-backup"));
        assert!(crontab.contains("sigmaos-cleanup"));
    }

    #[test]
    fn test_generate_systemd_timers() {
        let mut mgr = CronManager::new();
        mgr.add_job("backup", "0 2 * * *", "sigmaos-backup").unwrap();
        let timers = mgr.generate_systemd_timers();
        assert!(timers.contains("OnCalendar"));
        assert!(timers.contains("backup"));
    }

    #[test]
    fn test_default_jobs() {
        let mut mgr = CronManager::new();
        mgr.add_default_jobs();
        assert!(mgr.get_job("daily-update").is_some());
        assert!(mgr.get_job("weekly-cleanup").is_some());
    }
}
