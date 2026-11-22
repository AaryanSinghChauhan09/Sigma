//! Security audit module inspired by Linux security and Arch Linux hardening.
//! Provides vulnerability scanning and compliance checking.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VulnerabilitySeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl VulnerabilitySeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditStatus {
    Passed,
    Failed,
    Warning,
    Skipped,
}

impl AuditStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Warning => "warning",
            Self::Skipped => "skipped",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditError {
    InvalidComponent(String),
    InvalidRule(String),
}

impl fmt::Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidComponent(msg) => write!(f, "invalid component: {}", msg),
            Self::InvalidRule(msg) => write!(f, "invalid rule: {}", msg),
        }
    }
}

impl std::error::Error for AuditError {}

#[derive(Debug, Clone)]
pub struct Vulnerability {
    pub id: String,
    pub component: String,
    pub severity: VulnerabilitySeverity,
    pub description: String,
    pub fixed_in: Option<String>,
    pub cvss_score: Option<f64>,
}

impl Vulnerability {
    pub fn new(id: impl Into<String>, component: impl Into<String>, severity: VulnerabilitySeverity) -> Self {
        Self {
            id: id.into(),
            component: component.into(),
            severity,
            description: String::new(),
            fixed_in: None,
            cvss_score: None,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_fix(mut self, version: impl Into<String>) -> Self {
        self.fixed_in = Some(version.into());
        self
    }

    pub fn with_cvss(mut self, score: f64) -> Self {
        self.cvss_score = Some(score);
        self
    }
}

#[derive(Debug, Clone)]
pub struct AuditRule {
    pub id: String,
    pub name: String,
    pub check: String,
    pub expected: String,
    pub severity: VulnerabilitySeverity,
}

impl AuditRule {
    pub fn new(id: impl Into<String>, name: impl Into<String>, check: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            check: check.into(),
            expected: String::new(),
            severity: VulnerabilitySeverity::Medium,
        }
    }

    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected = expected.into();
        self
    }

    pub fn with_severity(mut self, severity: VulnerabilitySeverity) -> Self {
        self.severity = severity;
        self
    }
}

#[derive(Debug, Clone)]
pub struct AuditResult {
    pub rule_id: String,
    pub status: AuditStatus,
    pub details: String,
    pub findings: Vec<Vulnerability>,
}

impl AuditResult {
    pub fn new(rule_id: impl Into<String>, status: AuditStatus, details: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            status,
            details: details.into(),
            findings: vec![],
        }
    }

    pub fn with_findings(mut self, findings: Vec<Vulnerability>) -> Self {
        self.findings = findings;
        self
    }
}

#[derive(Debug, Clone)]
pub struct AuditManager {
    pub rules: BTreeMap<String, AuditRule>,
    pub results: BTreeMap<String, AuditResult>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub auto_fix: bool,
}

impl Default for AuditManager {
    fn default() -> Self {
        Self {
            rules: BTreeMap::new(),
            results: BTreeMap::new(),
            vulnerabilities: vec![],
            auto_fix: false,
        }
    }
}

impl AuditManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_auto_fix(mut self) -> Self {
        self.auto_fix = true;
        self
    }

    pub fn add_rule(&mut self, rule: AuditRule) -> Result<(), AuditError> {
        let id = rule.id.clone();
        if id.trim().is_empty() {
            return Err(AuditError::InvalidRule("rule ID cannot be empty".to_string()));
        }
        self.rules.insert(id, rule);
        Ok(())
    }

    pub fn add_vulnerability(&mut self, vuln: Vulnerability) {
        self.vulnerabilities.push(vuln);
    }

    pub fn record_result(&mut self, result: AuditResult) {
        self.results.insert(result.rule_id.clone(), result);
    }

    pub fn get_rule(&self, id: &str) -> Option<&AuditRule> {
        self.rules.get(id)
    }

    pub fn get_result(&self, id: &str) -> Option<&AuditResult> {
        self.results.get(id)
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn vulnerability_count(&self) -> usize {
        self.vulnerabilities.len()
    }

    pub fn get_vulnerabilities_by_severity(&self, severity: VulnerabilitySeverity) -> Vec<&Vulnerability> {
        self.vulnerabilities.iter().filter(|v| v.severity == severity).collect()
    }

    pub fn get_critical_vulnerabilities(&self) -> Vec<&Vulnerability> {
        self.get_vulnerabilities_by_severity(VulnerabilitySeverity::Critical)
    }

    pub fn get_high_vulnerabilities(&self) -> Vec<&Vulnerability> {
        self.get_vulnerabilities_by_severity(VulnerabilitySeverity::High)
    }

    pub fn audit_summary(&self) -> String {
        let mut summary = String::new();
        
        summary.push_str("# SigmaOS Security Audit Summary\n");
        summary.push_str(&format!("# Rules: {}\n", self.rule_count()));
        summary.push_str(&format!("# Results: {}\n", self.results.len()));
        summary.push_str(&format!("# Vulnerabilities: {}\n\n", self.vulnerability_count()));
        
        summary.push_str("## By Severity\n");
        for severity in [VulnerabilitySeverity::Critical, VulnerabilitySeverity::High, VulnerabilitySeverity::Medium, VulnerabilitySeverity::Low] {
            let count = self.get_vulnerabilities_by_severity(severity).len();
            summary.push_str(&format!("{}: {}\n", severity.as_str(), count));
        }
        
        summary.push_str("\n## Results by Status\n");
        for status in [AuditStatus::Passed, AuditStatus::Failed, AuditStatus::Warning, AuditStatus::Skipped] {
            let count = self.results.values().filter(|r| r.status == status).count();
            summary.push_str(&format!("{}: {}\n", status.as_str(), count));
        }
        
        summary
    }

    pub fn generate_cis_benchmark_rules(&self) -> BTreeMap<String, AuditRule> {
        let mut rules = BTreeMap::new();
        
        rules.insert("cis-1.1.1".to_string(), AuditRule::new(
            "cis-1.1.1",
            "Ensure filesystem integrity is checked on boot",
            "check_fsck_passwd"
        ).with_severity(VulnerabilitySeverity::High));
        
        rules.insert("cis-1.2.1".to_string(), AuditRule::new(
            "cis-1.2.1",
            "Ensure filesystems are mounted with nodev option",
            "check_mount_nodev"
        ).with_severity(VulnerabilitySeverity::Medium));
        
        rules.insert("cis-1.3.1".to_string(), AuditRule::new(
            "cis-1.3.1",
            "Ensure setuid/setgid bits are restricted",
            "check_setuid_bits"
        ).with_severity(VulnerabilitySeverity::High));
        
        rules.insert("cis-1.4.1".to_string(), AuditRule::new(
            "cis-1.4.1",
            "Ensure core dumps are disabled",
            "check_core_dumps"
        ).with_severity(VulnerabilitySeverity::Medium));
        
        rules
    }

    pub fn generate_lynis_rules(&self) -> BTreeMap<String, AuditRule> {
        let mut rules = BTreeMap::new();
        
        rules.insert("lynis-0001".to_string(), AuditRule::new(
            "lynis-0001",
            "Check if SELinux/AppArmor is enabled",
            "check_mac_enabled"
        ).with_severity(VulnerabilitySeverity::High));
        
        rules.insert("lynis-0002".to_string(), AuditRule::new(
            "lynis-0002",
            "Check if firewall is enabled",
            "check_firewall_enabled"
        ).with_severity(VulnerabilitySeverity::Critical));
        
        rules.insert("lynis-0003".to_string(), AuditRule::new(
            "lynis-0003",
            "Check for open ports",
            "check_open_ports"
        ).with_severity(VulnerabilitySeverity::Medium));
        
        rules
    }

    pub fn compliance_score(&self) -> u8 {
        let total_rules = self.rule_count();
        if total_rules == 0 {
            return 0;
        }
        
        let passed = self.results.values()
            .filter(|r| r.status == AuditStatus::Passed)
            .count();
        
        ((passed * 100) / total_rules) as u8
    }

    pub fn generate_remediation_script(&self) -> String {
        let mut script = String::new();
        
        script.push_str("#!/bin/bash\n");
        script.push_str("# SigmaOS Security Remediation Script\n");
        script.push_str("# Generated by SigmaOS AuditManager\n\n");
        
        for vuln in &self.vulnerabilities {
            script.push_str(&format!("# Fix {}: {}\n", vuln.id, vuln.description));
            if let Some(ref fix) = vuln.fixed_in {
                script.push_str(&format!("# Upgrade {} to {}\n", vuln.component, fix));
            }
        }
        
        script
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vulnerability_severity_as_str() {
        assert_eq!(VulnerabilitySeverity::Critical.as_str(), "critical");
        assert_eq!(VulnerabilitySeverity::Low.as_str(), "low");
    }

    #[test]
    fn test_audit_status_as_str() {
        assert_eq!(AuditStatus::Passed.as_str(), "passed");
        assert_eq!(AuditStatus::Failed.as_str(), "failed");
    }

    #[test]
    fn test_vulnerability_new() {
        let vuln = Vulnerability::new("CVE-2024-0001", "openssl", VulnerabilitySeverity::High)
            .with_description("Buffer overflow in TLS handshake")
            .with_fix("3.0.12");
        assert_eq!(vuln.id, "CVE-2024-0001");
        assert_eq!(vuln.component, "openssl");
    }

    #[test]
    fn test_audit_rule_new() {
        let rule = AuditRule::new("rule-1", "Check firewall", "iptables -L")
            .with_expected("ALLOW")
            .with_severity(VulnerabilitySeverity::Critical);
        assert_eq!(rule.id, "rule-1");
        assert_eq!(rule.severity, VulnerabilitySeverity::Critical);
    }

    #[test]
    fn test_audit_result_new() {
        let result = AuditResult::new("rule-1", AuditStatus::Passed, "All checks passed");
        assert_eq!(result.rule_id, "rule-1");
        assert_eq!(result.status, AuditStatus::Passed);
    }

    #[test]
    fn test_audit_manager_new() {
        let mgr = AuditManager::new();
        assert_eq!(mgr.rule_count(), 0);
    }

    #[test]
    fn test_audit_manager_add_rule() {
        let mut mgr = AuditManager::new();
        let rule = AuditRule::new("rule-1", "Test Rule", "test check");
        mgr.add_rule(rule).unwrap();
        assert_eq!(mgr.rule_count(), 1);
    }

    #[test]
    fn test_audit_manager_add_rule_invalid() {
        let mut mgr = AuditManager::new();
        let rule = AuditRule::new("", "Test Rule", "test check");
        assert!(mgr.add_rule(rule).is_err());
    }

    #[test]
    fn test_audit_manager_add_vulnerability() {
        let mut mgr = AuditManager::new();
        let vuln = Vulnerability::new("CVE-2024-0001", "openssl", VulnerabilitySeverity::High);
        mgr.add_vulnerability(vuln);
        assert_eq!(mgr.vulnerability_count(), 1);
    }

    #[test]
    fn test_get_vulnerabilities_by_severity() {
        let mut mgr = AuditManager::new();
        mgr.add_vulnerability(Vulnerability::new("CVE-1", "comp1", VulnerabilitySeverity::High));
        mgr.add_vulnerability(Vulnerability::new("CVE-2", "comp2", VulnerabilitySeverity::Critical));
        assert_eq!(mgr.get_high_vulnerabilities().len(), 1);
        assert_eq!(mgr.get_critical_vulnerabilities().len(), 1);
    }

    #[test]
    fn test_audit_summary() {
        let mut mgr = AuditManager::new();
        mgr.add_rule(AuditRule::new("rule-1", "Test", "check"));
        mgr.record_result(AuditResult::new("rule-1", AuditStatus::Passed, "OK"));
        let summary = mgr.audit_summary();
        assert!(summary.contains("SigmaOS Security Audit Summary"));
        assert!(summary.contains("Rules: 1"));
    }

    #[test]
    fn test_generate_cis_benchmark_rules() {
        let mgr = AuditManager::new();
        let rules = mgr.generate_cis_benchmark_rules();
        assert!(rules.contains_key("cis-1.1.1"));
        assert!(rules.contains_key("cis-1.4.1"));
    }

    #[test]
    fn test_generate_lynis_rules() {
        let mgr = AuditManager::new();
        let rules = mgr.generate_lynis_rules();
        assert!(rules.contains_key("lynis-0001"));
        assert!(rules.contains_key("lynis-0003"));
    }

    #[test]
    fn test_compliance_score() {
        let mut mgr = AuditManager::new();
        mgr.add_rule(AuditRule::new("rule-1", "Test", "check"));
        mgr.record_result(AuditResult::new("rule-1", AuditStatus::Passed, "OK"));
        assert_eq!(mgr.compliance_score(), 100);
    }

    #[test]
    fn test_generate_remediation_script() {
        let mut mgr = AuditManager::new();
        let vuln = Vulnerability::new("CVE-2024-0001", "openssl", VulnerabilitySeverity::High)
            .with_fix("3.0.12");
        mgr.add_vulnerability(vuln);
        let script = mgr.generate_remediation_script();
        assert!(script.contains("CVE-2024-0001"));
        assert!(script.contains("openssl"));
    }
}
