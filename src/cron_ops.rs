//! SigmaOS cron/scheduler operations module.
//!
//! This module provides actual cron job operations using crontab.
//!
//! # Example
//!
//! ```
//! use sigma_system::cron_ops::*;
//!
//! let mut cron_ops = CronOps::new();
//! if let Ok(jobs) = cron_ops.list_cron_jobs() {
//!     println!("Found {} cron jobs", jobs.len());
//! }
//! ```

use std::fs;
use std::process::Command;

/// Errors that can occur in cron operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CronError {
    /// Failed to read crontab.
    ReadError(String),
    /// Failed to write crontab.
    WriteError(String),
    /// Invalid cron expression.
    InvalidCron(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for CronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CronError::ReadError(msg) => write!(f, "Failed to read crontab: {}", msg),
            CronError::WriteError(msg) => write!(f, "Failed to write crontab: {}", msg),
            CronError::InvalidCron(expr) => write!(f, "Invalid cron expression: {}", expr),
            CronError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            CronError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for CronError {}

/// Cron job.
#[derive(Debug, Clone)]
pub struct CronJob {
    /// Minute (0-59).
    pub minute: String,
    /// Hour (0-23).
    pub hour: String,
    /// Day of month (1-31).
    pub day_of_month: String,
    /// Month (1-12).
    pub month: String,
    /// Day of week (0-7).
    pub day_of_week: String,
    /// Command to execute.
    pub command: String,
    /// Comment (if any).
    pub comment: Option<String>,
}

/// Cron operations manager.
#[derive(Debug)]
pub struct CronOps {
    /// Crontab path.
    crontab_path: String,
    /// Whether running as root.
    is_root: bool,
}

impl CronOps {
    /// Create a new cron operations manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self {
            crontab_path: "/var/spool/cron/crontabs".to_string(),
            is_root,
        }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// List cron jobs for current user.
    pub fn list_cron_jobs(&self) -> Result<Vec<CronJob>, CronError> {
        let output = Command::new("crontab").arg("-l").output();
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    let content = String::from_utf8_lossy(&output.stdout);
                    self.parse_crontab(&content)
                } else {
                    // No crontab exists
                    Ok(Vec::new())
                }
            }
            Err(_) => Ok(Vec::new()),
        }
    }

    /// List cron jobs for a specific user (requires root).
    pub fn list_user_cron_jobs(&self, user: &str) -> Result<Vec<CronJob>, CronError> {
        if !self.is_root {
            return Err(CronError::PermissionDenied("Must be root to list other user's crontab".to_string()));
        }

        let output = Command::new("crontab").args(&["-l", "-u", user]).output();
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    let content = String::from_utf8_lossy(&output.stdout);
                    self.parse_crontab(&content)
                } else {
                    Ok(Vec::new())
                }
            }
            Err(e) => Err(CronError::IoError(format!("Failed to run crontab: {}", e))),
        }
    }

    /// Parse crontab content.
    fn parse_crontab(&self, content: &str) -> Result<Vec<CronJob>, CronError> {
        let mut jobs = Vec::new();
        let mut current_comment: Option<String> = None;

        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines
            if line.is_empty() {
                continue;
            }
            
            // Skip environment variables
            if line.contains('=') && !line.contains(' ') {
                continue;
            }
            
            // Capture comments
            if line.starts_with('#') {
                current_comment = Some(line[1..].trim().to_string());
                continue;
            }
            
            // Parse cron job
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                jobs.push(CronJob {
                    minute: parts[0].to_string(),
                    hour: parts[1].to_string(),
                    day_of_month: parts[2].to_string(),
                    month: parts[3].to_string(),
                    day_of_week: parts[4].to_string(),
                    command: parts[5..].join(" "),
                    comment: current_comment.clone(),
                });
            }
            
            current_comment = None;
        }
        
        Ok(jobs)
    }

    /// Add a cron job.
    pub fn add_cron_job(&self, job: &CronJob) -> Result<(), CronError> {
        let mut jobs = self.list_cron_jobs()?;
        jobs.push(job.clone());
        self.write_crontab(&jobs)
    }

    /// Remove a cron job by command.
    pub fn remove_cron_job(&self, command: &str) -> Result<(), CronError> {
        let mut jobs = self.list_cron_jobs()?;
        jobs.retain(|j| j.command != command);
        self.write_crontab(&jobs)
    }

    /// Write crontab.
    fn write_crontab(&self, jobs: &[CronJob]) -> Result<(), CronError> {
        let mut content = String::new();
        
        for job in jobs {
            if let Some(comment) = &job.comment {
                content.push_str(&format!("# {}\n", comment));
            }
            content.push_str(&format!("{} {} {} {} {} {}\n",
                job.minute,
                job.hour,
                job.day_of_month,
                job.month,
                job.day_of_week,
                job.command
            ));
        }
        
        let result = Command::new("crontab")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();

        match result {
            Ok(mut child) => {
                if let Some(stdin) = child.stdin.as_mut() {
                    use std::io::Write;
                    stdin.write_all(content.as_bytes())
                        .map_err(|e| CronError::IoError(format!("Failed to write to crontab: {}", e)))?;
                }
                
                let status = child.wait()
                    .map_err(|e| CronError::IoError(format!("Failed to wait for crontab: {}", e)))?;
                
                if status.success() {
                    Ok(())
                } else {
                    Err(CronError::WriteError("crontab command failed".to_string()))
                }
            }
            Err(e) => Err(CronError::IoError(format!("Failed to run crontab: {}", e))),
        }
    }

    /// Validate cron expression.
    pub fn validate_cron_expr(&self, expr: &str) -> Result<(), CronError> {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() < 5 {
            return Err(CronError::InvalidCron("Cron expression must have at least 5 fields".to_string()));
        }
        
        // Validate minute (0-59)
        if !self.validate_field(parts[0], 0, 59) {
            return Err(CronError::InvalidCron(format!("Invalid minute: {}", parts[0])));
        }
        
        // Validate hour (0-23)
        if !self.validate_field(parts[1], 0, 23) {
            return Err(CronError::InvalidCron(format!("Invalid hour: {}", parts[1])));
        }
        
        // Validate day of month (1-31)
        if !self.validate_field(parts[2], 1, 31) {
            return Err(CronError::InvalidCron(format!("Invalid day of month: {}", parts[2])));
        }
        
        // Validate month (1-12)
        if !self.validate_field(parts[3], 1, 12) {
            return Err(CronError::InvalidCron(format!("Invalid month: {}", parts[3])));
        }
        
        // Validate day of week (0-7)
        if !self.validate_field(parts[4], 0, 7) {
            return Err(CronError::InvalidCron(format!("Invalid day of week: {}", parts[4])));
        }
        
        Ok(())
    }

    /// Validate a cron field.
    fn validate_field(&self, field: &str, min: u32, max: u32) -> bool {
        // Handle wildcard
        if field == "*" {
            return true;
        }
        
        // Handle ranges (e.g., 1-5)
        if field.contains('-') {
            let parts: Vec<&str> = field.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    return start >= min && end <= max && start <= end;
                }
            }
            return false;
        }
        
        // Handle lists (e.g., 1,2,3)
        if field.contains(',') {
            for part in field.split(',') {
                if let Ok(val) = part.trim().parse::<u32>() {
                    if val < min || val > max {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            return true;
        }
        
        // Handle single value
        if let Ok(val) = field.parse::<u32>() {
            return val >= min && val <= max;
        }
        
        false
    }

    /// Get cron summary.
    pub fn get_cron_summary(&self) -> Result<String, CronError> {
        let jobs = self.list_cron_jobs()?;
        
        Ok(format!(
            "Total cron jobs: {}\nCommands: {}",
            jobs.len(),
            jobs.iter().map(|j| j.command.clone()).collect::<Vec<_>>().join(", ")
        ))
    }
}

impl Default for CronOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cron_ops = CronOps::new();
        let _ = cron_ops.is_root();
    }

    #[test]
    fn test_list_cron_jobs() {
        let cron_ops = CronOps::new();
        if let Ok(jobs) = cron_ops.list_cron_jobs() {
            let _ = jobs;
        }
    }

    #[test]
    fn test_validate_cron_expr() {
        let cron_ops = CronOps::new();
        assert!(cron_ops.validate_cron_expr("* * * * * command").is_ok());
        assert!(cron_ops.validate_cron_expr("0 0 * * * command").is_ok());
        assert!(cron_ops.validate_cron_expr("invalid").is_err());
    }

    #[test]
    fn test_validate_field() {
        let cron_ops = CronOps::new();
        assert!(cron_ops.validate_field("*", 0, 59));
        assert!(cron_ops.validate_field("5", 0, 59));
        assert!(cron_ops.validate_field("1-5", 0, 59));
        assert!(cron_ops.validate_field("1,2,3", 0, 59));
        assert!(!cron_ops.validate_field("60", 0, 59));
    }
}
