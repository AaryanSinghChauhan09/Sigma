//! SigmaOS backup and restore module.
//!
//! This module provides abstractions for system backup, file backup,
//! and disaster recovery.
//!
//! # Example
//!
//! ```
//! use sigma_system::backup::*;
//!
//! let mut backup_mgr = BackupManager::new();
//! backup_mgr.create_backup("system", BackupType::Full, "/", "/backup/system").unwrap();
//! backup_mgr.start_backup("system").unwrap();
//! backup_mgr.complete_backup("system", 1024, 100).unwrap();
//! ```

use std::collections::BTreeMap;
use std::path::PathBuf;

/// Errors that can occur in backup operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BackupError {
    /// Backup not found.
    BackupNotFound(String),
    /// Backup already exists.
    BackupExists(String),
    /// Invalid backup path.
    InvalidPath(String),
    /// Backup failed.
    BackupFailed(String),
    /// Restore failed.
    RestoreFailed(String),
    /// Insufficient space.
    InsufficientSpace(String),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::BackupNotFound(name) => write!(f, "Backup '{}' not found", name),
            BackupError::BackupExists(name) => write!(f, "Backup '{}' already exists", name),
            BackupError::InvalidPath(path) => write!(f, "Invalid path '{}'", path),
            BackupError::BackupFailed(msg) => write!(f, "Backup failed: {}", msg),
            BackupError::RestoreFailed(msg) => write!(f, "Restore failed: {}", msg),
            BackupError::InsufficientSpace(msg) => write!(f, "Insufficient space: {}", msg),
        }
    }
}

impl std::error::Error for BackupError {}

/// Backup type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BackupType {
    /// Full system backup.
    Full,
    /// Incremental backup.
    Incremental,
    /// Differential backup.
    Differential,
    /// File backup.
    File,
    /// Configuration backup.
    Config,
}

impl BackupType {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            BackupType::Full => "full",
            BackupType::Incremental => "incremental",
            BackupType::Differential => "differential",
            BackupType::File => "file",
            BackupType::Config => "config",
        }
    }
}

/// Backup status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupStatus {
    /// Backup is in progress.
    InProgress,
    /// Backup completed successfully.
    Completed,
    /// Backup failed.
    Failed,
    /// Backup is scheduled.
    Scheduled,
}

/// Backup information.
#[derive(Debug, Clone)]
pub struct Backup {
    /// Backup name.
    pub name: String,
    /// Backup type.
    pub backup_type: BackupType,
    /// Source path.
    pub source_path: PathBuf,
    /// Destination path.
    pub dest_path: PathBuf,
    /// Backup status.
    pub status: BackupStatus,
    /// Backup timestamp.
    pub timestamp: i64,
    /// Backup size in bytes.
    pub size: u64,
    /// Number of files backed up.
    pub file_count: u64,
    /// Whether backup is compressed.
    pub compressed: bool,
    /// Encryption status.
    pub encrypted: bool,
}

impl Backup {
    /// Create a new backup.
    pub fn new(name: &str, backup_type: BackupType, source: &str, dest: &str) -> Result<Self, BackupError> {
        let source_path = PathBuf::from(source);
        let dest_path = PathBuf::from(dest);
        
        if source.is_empty() || dest.is_empty() {
            return Err(BackupError::InvalidPath(format!("{} -> {}", source, dest)));
        }
        
        Ok(Self {
            name: name.to_string(),
            backup_type,
            source_path,
            dest_path,
            status: BackupStatus::Scheduled,
            timestamp: 0,
            size: 0,
            file_count: 0,
            compressed: true,
            encrypted: false,
        })
    }

    /// Mark backup as in progress.
    pub fn start(&mut self) {
        self.status = BackupStatus::InProgress;
        self.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
    }

    /// Mark backup as completed.
    pub fn complete(&mut self, size: u64, file_count: u64) {
        self.status = BackupStatus::Completed;
        self.size = size;
        self.file_count = file_count;
    }

    /// Mark backup as failed.
    pub fn fail(&mut self) {
        self.status = BackupStatus::Failed;
    }
}

/// Backup schedule.
#[derive(Debug, Clone, PartialEq)]
pub struct BackupSchedule {
    /// Schedule name.
    pub name: String,
    /// Cron expression.
    pub cron_expression: String,
    /// Backup type.
    pub backup_type: BackupType,
    /// Retention days.
    pub retention_days: u32,
    /// Whether schedule is enabled.
    pub enabled: bool,
}

impl BackupSchedule {
    /// Create a new backup schedule.
    pub fn new(name: &str, cron: &str, backup_type: BackupType, retention_days: u32) -> Self {
        Self {
            name: name.to_string(),
            cron_expression: cron.to_string(),
            backup_type,
            retention_days,
            enabled: true,
        }
    }
}

/// Backup manager.
#[derive(Debug)]
pub struct BackupManager {
    /// Backups by name.
    backups: BTreeMap<String, Backup>,
    /// Schedules by name.
    schedules: BTreeMap<String, BackupSchedule>,
    /// Backup directory.
    backup_dir: PathBuf,
}

impl BackupManager {
    /// Create a new backup manager.
    pub fn new() -> Self {
        Self {
            backups: BTreeMap::new(),
            schedules: BTreeMap::new(),
            backup_dir: PathBuf::from("/var/backups/sigmaos"),
        }
    }

    /// Set backup directory.
    pub fn set_backup_dir(&mut self, dir: &str) {
        self.backup_dir = PathBuf::from(dir);
    }

    /// Get backup directory.
    pub fn get_backup_dir(&self) -> &PathBuf {
        &self.backup_dir
    }

    /// Create a backup job.
    pub fn create_backup(&mut self, name: &str, backup_type: BackupType, source: &str, dest: &str) -> Result<Backup, BackupError> {
        if self.backups.contains_key(name) {
            return Err(BackupError::BackupExists(name.to_string()));
        }
        let backup = Backup::new(name, backup_type, source, dest)?;
        self.backups.insert(name.to_string(), backup.clone());
        Ok(backup)
    }

    /// Remove a backup job.
    pub fn remove_backup(&mut self, name: &str) -> Result<(), BackupError> {
        if !self.backups.contains_key(name) {
            return Err(BackupError::BackupNotFound(name.to_string()));
        }
        self.backups.remove(name);
        Ok(())
    }

    /// Get a backup job.
    pub fn get_backup(&self, name: &str) -> Option<&Backup> {
        self.backups.get(name)
    }

    /// Get all backup jobs.
    pub fn get_backups(&self) -> Vec<&Backup> {
        self.backups.values().collect()
    }

    /// Start a backup.
    pub fn start_backup(&mut self, name: &str) -> Result<(), BackupError> {
        if let Some(backup) = self.backups.get_mut(name) {
            backup.start();
            Ok(())
        } else {
            Err(BackupError::BackupNotFound(name.to_string()))
        }
    }

    /// Complete a backup.
    pub fn complete_backup(&mut self, name: &str, size: u64, file_count: u64) -> Result<(), BackupError> {
        if let Some(backup) = self.backups.get_mut(name) {
            backup.complete(size, file_count);
            Ok(())
        } else {
            Err(BackupError::BackupNotFound(name.to_string()))
        }
    }

    /// Fail a backup.
    pub fn fail_backup(&mut self, name: &str) -> Result<(), BackupError> {
        if let Some(backup) = self.backups.get_mut(name) {
            backup.fail();
            Ok(())
        } else {
            Err(BackupError::BackupNotFound(name.to_string()))
        }
    }

    /// Add a backup schedule.
    pub fn add_schedule(&mut self, name: &str, cron: &str, backup_type: BackupType, retention_days: u32) {
        let schedule = BackupSchedule::new(name, cron, backup_type, retention_days);
        self.schedules.insert(name.to_string(), schedule);
    }

    /// Remove a backup schedule.
    pub fn remove_schedule(&mut self, name: &str) -> Result<(), BackupError> {
        if !self.schedules.contains_key(name) {
            return Err(BackupError::BackupNotFound(name.to_string()));
        }
        self.schedules.remove(name);
        Ok(())
    }

    /// Get all schedules.
    pub fn get_schedules(&self) -> Vec<&BackupSchedule> {
        self.schedules.values().collect()
    }

    /// Generate backup script.
    pub fn generate_backup_script(&self) -> String {
        let mut script = String::new();
        script.push_str("#!/bin/bash\n");
        script.push_str("# SigmaOS Backup Script\n\n");
        script.push_str(&format!("BACKUP_DIR=\"{}\"\n", self.backup_dir.display()));
        script.push_str("mkdir -p \"$BACKUP_DIR\"\n\n");
        
        for backup in self.backups.values() {
            if backup.status == BackupStatus::Scheduled || backup.status == BackupStatus::Completed {
                script.push_str(&format!("# Backup: {}\n", backup.name));
                script.push_str(&format!("tar -czf \"$BACKUP_DIR/{}.tar.gz\" -C \"{}\" .\n", backup.name, backup.source_path.display()));
                script.push_str("\n");
            }
        }
        
        script
    }

    /// Generate restore script.
    pub fn generate_restore_script(&self) -> String {
        let mut script = String::new();
        script.push_str("#!/bin/bash\n");
        script.push_str("# SigmaOS Restore Script\n\n");
        script.push_str(&format!("BACKUP_DIR=\"{}\"\n", self.backup_dir.display()));
        script.push_str("\n");
        
        for backup in self.backups.values() {
            if backup.status == BackupStatus::Completed {
                script.push_str(&format!("# Restore: {}\n", backup.name));
                script.push_str(&format!("tar -xzf \"$BACKUP_DIR/{}.tar.gz\" -C \"{}\"\n", backup.name, backup.dest_path.display()));
                script.push_str("\n");
            }
        }
        
        script
    }

    /// Add default backup jobs.
    pub fn add_default_backups(&mut self) {
        // System backup
        if let Ok(backup) = self.create_backup("system", BackupType::Full, "/", "/var/backups/system") {
            let _ = backup;
        }
        
        // Config backup
        if let Ok(backup) = self.create_backup("config", BackupType::Config, "/etc", "/var/backups/config") {
            let _ = backup;
        }
        
        // Home backup
        if let Ok(backup) = self.create_backup("home", BackupType::Incremental, "/home", "/var/backups/home") {
            let _ = backup;
        }
        
        // Add schedules
        self.add_schedule("daily-system", "0 2 * * *", BackupType::Incremental, 7);
        self.add_schedule("weekly-config", "0 3 * * 0", BackupType::Config, 30);
        self.add_schedule("monthly-full", "0 4 1 * *", BackupType::Full, 365);
    }
}

impl Default for BackupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_backup() {
        let mut mgr = BackupManager::new();
        let backup = mgr.create_backup("system", BackupType::Full, "/", "/backup/system").unwrap();
        assert_eq!(backup.name, "system");
        assert_eq!(backup.backup_type, BackupType::Full);
    }

    #[test]
    fn test_create_duplicate_backup() {
        let mut mgr = BackupManager::new();
        mgr.create_backup("system", BackupType::Full, "/", "/backup/system").unwrap();
        let result = mgr.create_backup("system", BackupType::Full, "/", "/backup/system2");
        assert!(result.is_err());
    }

    #[test]
    fn test_start_backup() {
        let mut mgr = BackupManager::new();
        mgr.create_backup("system", BackupType::Full, "/", "/backup/system").unwrap();
        mgr.start_backup("system").unwrap();
        assert_eq!(mgr.get_backup("system").unwrap().status, BackupStatus::InProgress);
    }

    #[test]
    fn test_complete_backup() {
        let mut mgr = BackupManager::new();
        mgr.create_backup("system", BackupType::Full, "/", "/backup/system").unwrap();
        mgr.start_backup("system").unwrap();
        mgr.complete_backup("system", 1024, 100).unwrap();
        let backup = mgr.get_backup("system").unwrap();
        assert_eq!(backup.status, BackupStatus::Completed);
        assert_eq!(backup.size, 1024);
        assert_eq!(backup.file_count, 100);
    }

    #[test]
    fn test_fail_backup() {
        let mut mgr = BackupManager::new();
        mgr.create_backup("system", BackupType::Full, "/", "/backup/system").unwrap();
        mgr.start_backup("system").unwrap();
        mgr.fail_backup("system").unwrap();
        assert_eq!(mgr.get_backup("system").unwrap().status, BackupStatus::Failed);
    }

    #[test]
    fn test_add_schedule() {
        let mut mgr = BackupManager::new();
        mgr.add_schedule("daily", "0 2 * * *", BackupType::Incremental, 7);
        assert!(mgr.get_schedules().len() == 1);
    }

    #[test]
    fn test_generate_backup_script() {
        let mut mgr = BackupManager::new();
        mgr.create_backup("system", BackupType::Full, "/", "/backup/system").unwrap();
        mgr.start_backup("system").unwrap();
        mgr.complete_backup("system", 1024, 100).unwrap();
        let script = mgr.generate_backup_script();
        assert!(script.contains("tar -czf"));
        assert!(script.contains("system"));
    }

    #[test]
    fn test_default_backups() {
        let mut mgr = BackupManager::new();
        mgr.add_default_backups();
        assert!(mgr.get_backup("system").is_some());
        assert!(mgr.get_backup("config").is_some());
        assert!(mgr.get_backup("home").is_some());
    }

    #[test]
    fn test_backup_type_as_str() {
        assert_eq!(BackupType::Full.as_str(), "full");
        assert_eq!(BackupType::Incremental.as_str(), "incremental");
    }
}
