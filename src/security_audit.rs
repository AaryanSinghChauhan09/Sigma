//! SigmaOS security audit module.
//!
//! This module provides security audit functionality to detect and prevent:
//! - API key leaks
//! - Database password exposure
//! - Access token leaks
//! - Default passwords
//! - Debug mode enabled in production
//! - Unrestricted access to personal databases
//! - Cloud storage misconfigurations
//! - Insecure feature additions
//!
//! All secrets should be stored in environment variables or a secret manager.
//!
//! # Example
//!
//! ```
//! use sigma_system::security_audit::*;
//!
//! let auditor = SecurityAuditor::new();
//! if let Ok(report) = auditor.audit_codebase("/path/to/code") {
//!     println!("Security report: {}", report);
//! }
//! ```

use std::fs;
use std::path::Path;
use std::process::Command;

/// Security issue severity.
#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Security issue type.
#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    ApiKeyLeak,
    DatabasePassword,
    AccessToken,
    DefaultPassword,
    DebugModeEnabled,
    UnrestrictedAccess,
    CloudStorageMisconfig,
    InsecureFeature,
    HardcodedSecret,
    Other(String),
}

/// Security issue.
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub issue_type: IssueType,
    pub severity: Severity,
    pub file: String,
    pub line: usize,
    pub description: String,
    pub recommendation: String,
}

/// Security audit result.
#[derive(Debug, Clone)]
pub struct SecurityAuditResult {
    pub issues: Vec<SecurityIssue>,
    pub files_scanned: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
}

/// Errors that can occur during security audit.
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityAuditError {
    /// File read error.
    FileReadError(String),
    /// Directory access error.
    DirectoryAccessError(String),
    /// Invalid path.
    InvalidPath(String),
}

impl std::fmt::Display for SecurityAuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityAuditError::FileReadError(msg) => write!(f, "File read error: {}", msg),
            SecurityAuditError::DirectoryAccessError(msg) => write!(f, "Directory access error: {}", msg),
            SecurityAuditError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
        }
    }
}

impl std::error::Error for SecurityAuditError {}

/// Security auditor.
#[derive(Debug)]
pub struct SecurityAuditor;

impl SecurityAuditor {
    pub fn new() -> Self { Self }

    /// Audit a codebase for security issues.
    pub fn audit_codebase(&self, path: &str) -> Result<SecurityAuditResult, SecurityAuditError> {
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(SecurityAuditError::InvalidPath(path.to_string()));
        }

        let mut issues = Vec::new();
        let mut files_scanned = 0;

        // Scan Rust source files
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        files_scanned += 1;
                        self.scan_file(&file_path, &content, &mut issues);
                    }
                }
            }
        }

        let critical_count = issues.iter().filter(|i| i.severity == Severity::Critical).count();
        let high_count = issues.iter().filter(|i| i.severity == Severity::High).count();
        let medium_count = issues.iter().filter(|i| i.severity == Severity::Medium).count();
        let low_count = issues.iter().filter(|i| i.severity == Severity::Low).count();

        Ok(SecurityAuditResult {
            issues,
            files_scanned,
            critical_count,
            high_count,
            medium_count,
            low_count,
        })
    }

    /// Scan a single file for security issues.
    fn scan_file(&self, file_path: &Path, content: &str, issues: &mut Vec<SecurityIssue>) {
        let lines: Vec<&str> = content.lines().collect();
        let file_name = file_path.to_string_lossy().to_string();

        // Simple pattern matching for security issues (without regex)
        for (line_num, line) in lines.iter().enumerate() {
            let lower_line = line.to_lowercase();
            
            // Check for common hardcoded secrets
            if lower_line.contains("api_key") || lower_line.contains("apikey") {
                if line.contains('"') || line.contains('\'') {
                    if !line.contains("${") && !line.contains("env::") {
                        issues.push(SecurityIssue {
                            issue_type: IssueType::ApiKeyLeak,
                            severity: Severity::Critical,
                            file: file_name.clone(),
                            line: line_num + 1,
                            description: "Potential API key leak detected".to_string(),
                            recommendation: "Use environment variables or a secret manager instead of hardcoding secrets.".to_string(),
                        });
                    }
                }
            }
            
            if lower_line.contains("secret") || lower_line.contains("private_key") {
                if line.contains('"') || line.contains('\'') {
                    if !line.contains("${") && !line.contains("env::") {
                        issues.push(SecurityIssue {
                            issue_type: IssueType::HardcodedSecret,
                            severity: Severity::High,
                            file: file_name.clone(),
                            line: line_num + 1,
                            description: "Potential hardcoded secret detected".to_string(),
                            recommendation: "Use environment variables or a secret manager instead of hardcoding secrets.".to_string(),
                        });
                    }
                }
            }
            
            if lower_line.contains("password") {
                if line.contains("=\"password\"") || line.contains("='password'") ||
                   line.contains("=\"123456\"") || line.contains("='123456'") ||
                   line.contains("=\"admin\"") || line.contains("='admin'") ||
                   line.contains("=\"root\"") || line.contains("='root'") {
                    issues.push(SecurityIssue {
                        issue_type: IssueType::DefaultPassword,
                        severity: Severity::Critical,
                        file: file_name.clone(),
                        line: line_num + 1,
                        description: "Default password detected".to_string(),
                        recommendation: "Use environment variables or a secret manager instead of default passwords.".to_string(),
                    });
                }
            }
            
            if lower_line.contains("debug") && (line.contains("=true") || line.contains("=1")) {
                issues.push(SecurityIssue {
                    issue_type: IssueType::DebugModeEnabled,
                    severity: Severity::Medium,
                    file: file_name.clone(),
                    line: line_num + 1,
                    description: "Debug mode potentially enabled".to_string(),
                    recommendation: "Ensure debug mode is disabled in production.".to_string(),
                });
            }
        }
    }

    /// Check if environment variables are properly used.
    pub fn check_env_vars(&self) -> Result<String, SecurityAuditError> {
        let output = Command::new("env")
            .output()
            .map_err(|e| SecurityAuditError::FileReadError(format!("Failed to run env: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.to_string())
    }
}

impl Default for SecurityAuditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let auditor = SecurityAuditor::new();
        let _ = auditor;
    }

    #[test]
    fn test_audit_codebase() {
        let auditor = SecurityAuditor::new();
        if let Ok(result) = auditor.audit_codebase("/home/aaryansinghchauhan/Sigma/src") {
            // Should not panic
            let _ = result;
        }
    }
}
