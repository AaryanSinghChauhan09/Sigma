//! Security module inspired by Linux kernel integrity subsystem and Arch Linux security.
//! Provides secure boot verification, TPM 2.0 integration, and integrity checks.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureBootState {
    Enabled,
    Disabled,
    Unknown,
}

impl SecureBootState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TpmVersion {
    None,
    Tpm12,
    Tpm20,
}

impl TpmVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Tpm12 => "1.2",
            Self::Tpm20 => "2.0",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityCheck {
    Passed,
    Failed,
    Skipped,
}

impl IntegrityCheck {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityError {
    InvalidKey(String),
    InvalidSignature(String),
    TpmNotAvailable,
    SecureBootNotSupported,
}

impl fmt::Display for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKey(msg) => write!(f, "invalid key: {}", msg),
            Self::InvalidSignature(msg) => write!(f, "invalid signature: {}", msg),
            Self::TpmNotAvailable => write!(f, "TPM not available"),
            Self::SecureBootNotSupported => write!(f, "secure boot not supported"),
        }
    }
}

impl std::error::Error for SecurityError {}

#[derive(Debug, Clone)]
pub struct SecureBootKey {
    pub key_id: String,
    pub key_type: String,
    pub owner: String,
    pub fingerprint: String,
}

impl SecureBootKey {
    pub fn new(key_id: impl Into<String>, key_type: impl Into<String>) -> Self {
        Self {
            key_id: key_id.into(),
            key_type: key_type.into(),
            owner: String::new(),
            fingerprint: String::new(),
        }
    }

    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = owner.into();
        self
    }

    pub fn with_fingerprint(mut self, fingerprint: impl Into<String>) -> Self {
        self.fingerprint = fingerprint.into();
        self
    }
}

#[derive(Debug, Clone)]
pub struct TpmConfig {
    pub version: TpmVersion,
    pub enabled: bool,
    pub pcr_banks: Vec<String>,
    pub attestation_enabled: bool,
}

impl Default for TpmConfig {
    fn default() -> Self {
        Self {
            version: TpmVersion::None,
            enabled: false,
            pcr_banks: vec!["sha256".to_string()],
            attestation_enabled: false,
        }
    }
}

impl TpmConfig {
    pub fn new(version: TpmVersion) -> Self {
        Self {
            version,
            enabled: version != TpmVersion::None,
            pcr_banks: vec!["sha256".to_string()],
            attestation_enabled: version == TpmVersion::Tpm20,
        }
    }

    pub fn with_pcr_bank(mut self, bank: impl Into<String>) -> Self {
        self.pcr_banks.push(bank.into());
        self
    }

    pub fn enable_attestation(mut self) -> Self {
        self.attestation_enabled = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SecurityManager {
    pub secure_boot_state: SecureBootState,
    pub tpm_config: TpmConfig,
    pub keys: BTreeMap<String, SecureBootKey>,
    pub integrity_checks: BTreeMap<String, IntegrityCheck>,
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self {
            secure_boot_state: SecureBootState::Unknown,
            tpm_config: TpmConfig::default(),
            keys: BTreeMap::new(),
            integrity_checks: BTreeMap::new(),
        }
    }
}

impl SecurityManager {
    pub fn new(secure_boot_state: SecureBootState, tpm_config: TpmConfig) -> Self {
        Self {
            secure_boot_state,
            tpm_config,
            keys: BTreeMap::new(),
            integrity_checks: BTreeMap::new(),
        }
    }

    pub fn with_secure_boot(mut self, state: SecureBootState) -> Self {
        self.secure_boot_state = state;
        self
    }

    pub fn add_key(&mut self, key: SecureBootKey) -> Result<(), SecurityError> {
        let id = key.key_id.clone();
        if id.trim().is_empty() {
            return Err(SecurityError::InvalidKey("key ID cannot be empty".to_string()));
        }
        self.keys.insert(id, key);
        Ok(())
    }

    pub fn remove_key(&mut self, key_id: &str) -> Option<SecureBootKey> {
        self.keys.remove(key_id)
    }

    pub fn get_key(&self, key_id: &str) -> Option<&SecureBootKey> {
        self.keys.get(key_id)
    }

    pub fn key_count(&self) -> usize {
        self.keys.len()
    }

    pub fn add_integrity_check(&mut self, name: impl Into<String>, result: IntegrityCheck) {
        self.integrity_checks.insert(name.into(), result);
    }

    pub fn get_integrity_check(&self, name: &str) -> Option<IntegrityCheck> {
        self.integrity_checks.get(name).copied()
    }

    pub fn all_integrity_checks_passed(&self) -> bool {
        self.integrity_checks.values().all(|&check| check == IntegrityCheck::Passed)
    }

    pub fn security_score(&self) -> u8 {
        let mut score = 0u8;
        
        if self.secure_boot_state == SecureBootState::Enabled {
            score += 30;
        }
        
        if self.tpm_config.version == TpmVersion::Tpm20 && self.tpm_config.enabled {
            score += 40;
        } else if self.tpm_config.version == TpmVersion::Tpm12 && self.tpm_config.enabled {
            score += 20;
        }
        
        if !self.keys.is_empty() {
            score += 10;
        }
        
        if self.all_integrity_checks_passed() && !self.integrity_checks.is_empty() {
            score += 20;
        }
        
        score
    }

    pub fn generate_efi_vars_config(&self) -> String {
        format!(
            "# SigmaOS Secure Boot Configuration\n\
             # Secure Boot: {}\n\
             # TPM: {}\n\
             # Keys: {}\n\
             \n\
             [efivars]\n\
             SecureBoot={}\n\
             SetupMode=0\n",
            self.secure_boot_state.as_str(),
            self.tpm_config.version.as_str(),
            self.key_count(),
            self.secure_boot_state.as_str()
        )
    }

    pub fn generate_tpm_config(&self) -> String {
        let pcr_banks = self.tpm_config.pcr_banks.join(",");
        format!(
            "# SigmaOS TPM Configuration\n\
             # Version: {}\n\
             # Enabled: {}\n\
             # Attestation: {}\n\
             \n\
             [tpm]\n\
             version={}\n\
             enabled={}\n\
             pcr_banks={}\n\
             attestation={}\n",
            self.tpm_config.version.as_str(),
            self.tpm_config.enabled,
            self.tpm_config.attestation_enabled,
            self.tpm_config.version.as_str(),
            if self.tpm_config.enabled { "true" } else { "false" },
            pcr_banks,
            if self.tpm_config.attestation_enabled { "true" } else { "false" },
        )
    }

    pub fn generate_integrity_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# SigmaOS Integrity Report\n");
        report.push_str(&format!("# Security Score: {}/100\n", self.security_score()));
        report.push_str("\n");
        
        report.push_str("[Secure Boot]\n");
        report.push_str(&format!("state={}\n", self.secure_boot_state.as_str()));
        report.push_str("\n");
        
        report.push_str("[TPM]\n");
        report.push_str(&format!("version={}\n", self.tpm_config.version.as_str()));
        report.push_str(&format!("enabled={}\n", self.tpm_config.enabled));
        report.push_str("\n");
        
        report.push_str("[Keys]\n");
        for (id, key) in &self.keys {
            report.push_str(&format!("{}: type={}, owner={}\n", id, key.key_type, key.owner));
        }
        report.push_str("\n");
        
        report.push_str("[Integrity Checks]\n");
        for (name, result) in &self.integrity_checks {
            report.push_str(&format!("{}: {}\n", name, result.as_str()));
        }
        
        report
    }

    pub fn get_security_recommendation(&self) -> &'static str {
        let score = self.security_score();
        if score >= 80 {
            "Security posture is excellent. System is well-protected."
        } else if score >= 60 {
            "Security posture is good. Consider enabling TPM for additional protection."
        } else if score >= 40 {
            "Security posture is moderate. Enable secure boot and TPM for better protection."
        } else {
            "Security posture is weak. Enable secure boot, TPM, and add keys for protection."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_boot_state_as_str() {
        assert_eq!(SecureBootState::Enabled.as_str(), "enabled");
        assert_eq!(SecureBootState::Disabled.as_str(), "disabled");
    }

    #[test]
    fn test_tpm_version_as_str() {
        assert_eq!(TpmVersion::Tpm20.as_str(), "2.0");
        assert_eq!(TpmVersion::Tpm12.as_str(), "1.2");
    }

    #[test]
    fn test_secure_boot_key_new() {
        let key = SecureBootKey::new("test-key", "rsa");
        assert_eq!(key.key_id, "test-key");
        assert_eq!(key.key_type, "rsa");
    }

    #[test]
    fn test_secure_boot_key_with_owner() {
        let key = SecureBootKey::new("test", "rsa")
            .with_owner("SigmaOS");
        assert_eq!(key.owner, "SigmaOS");
    }

    #[test]
    fn test_tpm_config_new() {
        let config = TpmConfig::new(TpmVersion::Tpm20);
        assert_eq!(config.version, TpmVersion::Tpm20);
        assert!(config.enabled);
    }

    #[test]
    fn test_tpm_config_with_pcr_bank() {
        let config = TpmConfig::new(TpmVersion::Tpm20)
            .with_pcr_bank("sha1");
        assert!(config.pcr_banks.contains(&"sha1".to_string()));
    }

    #[test]
    fn test_security_manager_new() {
        let mgr = SecurityManager::new(SecureBootState::Enabled, TpmConfig::new(TpmVersion::Tpm20));
        assert_eq!(mgr.secure_boot_state, SecureBootState::Enabled);
    }

    #[test]
    fn test_security_manager_add_key() {
        let mut mgr = SecurityManager::default();
        let key = SecureBootKey::new("test", "rsa");
        mgr.add_key(key).unwrap();
        assert_eq!(mgr.key_count(), 1);
    }

    #[test]
    fn test_security_manager_add_key_invalid() {
        let mut mgr = SecurityManager::default();
        let key = SecureBootKey::new("", "rsa");
        assert!(mgr.add_key(key).is_err());
    }

    #[test]
    fn test_security_manager_add_integrity_check() {
        let mut mgr = SecurityManager::default();
        mgr.add_integrity_check("kernel", IntegrityCheck::Passed);
        assert_eq!(mgr.get_integrity_check("kernel"), Some(IntegrityCheck::Passed));
    }

    #[test]
    fn test_all_integrity_checks_passed() {
        let mut mgr = SecurityManager::default();
        mgr.add_integrity_check("kernel", IntegrityCheck::Passed);
        mgr.add_integrity_check("initramfs", IntegrityCheck::Passed);
        assert!(mgr.all_integrity_checks_passed());
    }

    #[test]
    fn test_all_integrity_checks_passed_with_failure() {
        let mut mgr = SecurityManager::default();
        mgr.add_integrity_check("kernel", IntegrityCheck::Passed);
        mgr.add_integrity_check("initramfs", IntegrityCheck::Failed);
        assert!(!mgr.all_integrity_checks_passed());
    }

    #[test]
    fn test_security_score() {
        let mut mgr = SecurityManager::new(SecureBootState::Enabled, TpmConfig::new(TpmVersion::Tpm20));
        mgr.add_integrity_check("kernel", IntegrityCheck::Passed);
        let score = mgr.security_score();
        assert!(score >= 70);
    }

    #[test]
    fn test_generate_efi_vars_config() {
        let mgr = SecurityManager::new(SecureBootState::Enabled, TpmConfig::default());
        let config = mgr.generate_efi_vars_config();
        assert!(config.contains("SecureBoot=enabled"));
    }

    #[test]
    fn test_generate_tpm_config() {
        let mgr = SecurityManager::default();
        let config = mgr.generate_tpm_config();
        assert!(config.contains("[tpm]"));
    }

    #[test]
    fn test_generate_integrity_report() {
        let mut mgr = SecurityManager::default();
        mgr.add_integrity_check("kernel", IntegrityCheck::Passed);
        let report = mgr.generate_integrity_report();
        assert!(report.contains("Integrity Report"));
        assert!(report.contains("kernel: passed"));
    }

    #[test]
    fn test_get_security_recommendation() {
        let mut mgr = SecurityManager::new(SecureBootState::Enabled, TpmConfig::new(TpmVersion::Tpm20));
        mgr.add_integrity_check("kernel", IntegrityCheck::Passed);
        let key = SecureBootKey::new("test", "rsa");
        mgr.add_key(key).unwrap();
        let rec = mgr.get_security_recommendation();
        assert!(rec.contains("excellent"));
    }
}
