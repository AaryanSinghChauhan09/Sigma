//! Boot configuration inspired by GRUB and systemd-boot.
//! Provides bootloader configuration and kernel parameter management.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootLoaderType {
    Grub,
    SystemdBoot,
    Limine,
}

impl BootLoaderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Grub => "grub",
            Self::SystemdBoot => "systemd-boot",
            Self::Limine => "limine",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootError {
    InvalidKernel(String),
    InvalidInitrd(String),
    InvalidTimeout(String),
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKernel(msg) => write!(f, "invalid kernel: {}", msg),
            Self::InvalidInitrd(msg) => write!(f, "invalid initrd: {}", msg),
            Self::InvalidTimeout(msg) => write!(f, "invalid timeout: {}", msg),
        }
    }
}

impl std::error::Error for BootError {}

#[derive(Debug, Clone)]
pub struct KernelParams {
    pub quiet: bool,
    pub splash: bool,
    pub nomodeset: bool,
    pub single: bool,
    pub custom: Vec<String>,
}

impl Default for KernelParams {
    fn default() -> Self {
        Self {
            quiet: true,
            splash: true,
            nomodeset: false,
            single: false,
            custom: vec![],
        }
    }
}

impl KernelParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    pub fn with_splash(mut self, splash: bool) -> Self {
        self.splash = splash;
        self
    }

    pub fn with_custom(mut self, param: impl Into<String>) -> Self {
        self.custom.push(param.into());
        self
    }

    pub fn to_string(&self) -> String {
        let mut params = Vec::new();
        if self.quiet { params.push("quiet".to_string()); }
        if self.splash { params.push("splash".to_string()); }
        if self.nomodeset { params.push("nomodeset".to_string()); }
        if self.single { params.push("single".to_string()); }
        params.extend(self.custom.clone());
        params.join(" ")
    }
}

#[derive(Debug, Clone)]
pub struct BootEntry {
    pub id: String,
    pub title: String,
    pub kernel: String,
    pub initrd: String,
    pub params: KernelParams,
    pub is_default: bool,
}

impl BootEntry {
    pub fn new(id: impl Into<String>, title: impl Into<String>, kernel: impl Into<String>, initrd: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            kernel: kernel.into(),
            initrd: initrd.into(),
            params: KernelParams::default(),
            is_default: false,
        }
    }

    pub fn with_params(mut self, params: KernelParams) -> Self {
        self.params = params;
        self
    }

    pub fn set_default(mut self) -> Self {
        self.is_default = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct BootManager {
    pub bootloader: BootLoaderType,
    pub timeout: u32,
    pub entries: BTreeMap<String, BootEntry>,
    pub default_entry: Option<String>,
}

impl Default for BootManager {
    fn default() -> Self {
        Self {
            bootloader: BootLoaderType::SystemdBoot,
            timeout: 5,
            entries: BTreeMap::new(),
            default_entry: None,
        }
    }
}

impl BootManager {
    pub fn new(bootloader: BootLoaderType) -> Self {
        Self {
            bootloader,
            timeout: 5,
            entries: BTreeMap::new(),
            default_entry: None,
        }
    }

    pub fn with_timeout(mut self, timeout: u32) -> Result<Self, BootError> {
        if timeout > 60 {
            return Err(BootError::InvalidTimeout("timeout must be <= 60 seconds".to_string()));
        }
        self.timeout = timeout;
        Ok(self)
    }

    pub fn add_entry(&mut self, entry: BootEntry) -> Result<(), BootError> {
        let id = entry.id.clone();
        if entry.kernel.trim().is_empty() {
            return Err(BootError::InvalidKernel("kernel path cannot be empty".to_string()));
        }
        if entry.initrd.trim().is_empty() {
            return Err(BootError::InvalidInitrd("initrd path cannot be empty".to_string()));
        }
        if entry.is_default {
            self.default_entry = Some(id.clone());
        }
        self.entries.insert(id, entry);
        Ok(())
    }

    pub fn set_default(&mut self, id: &str) -> Option<BootEntry> {
        self.default_entry = Some(id.to_string());
        self.entries.get(id).cloned()
    }

    pub fn generate_grub_config(&self) -> String {
        let mut config = String::new();
        
        config.push_str("# SigmaOS GRUB Configuration\n");
        config.push_str(&format!("set timeout={}\n", self.timeout));
        config.push_str(&format!("set default={}\n", 
            self.default_entry.as_deref().unwrap_or("0")
        ));
        config.push_str("\n");
        
        for (_id, entry) in &self.entries {
            config.push_str(&format!("menuentry \"{}\" {{\n", entry.title));
            config.push_str(&format!("    linux {} {}\n", entry.kernel, entry.params.to_string()));
            config.push_str(&format!("    initrd {}\n", entry.initrd));
            config.push_str("}\n\n");
        }
        
        config
    }

    pub fn generate_systemd_boot_config(&self) -> String {
        let mut config = String::new();
        
        config.push_str("# SigmaOS systemd-boot Configuration\n");
        config.push_str("# loader.conf\n");
        config.push_str(&format!("default {}\n", 
            self.default_entry.as_deref().unwrap_or("sigmaos")
        ));
        config.push_str(&format!("timeout {}\n", self.timeout));
        config.push_str("editor 0\n\n");
        
        for (id, entry) in &self.entries {
            config.push_str(&format!("# loader/entries/{}.conf\n", id));
            config.push_str(&format!("title {}\n", entry.title));
            config.push_str(&format!("linux {}\n", entry.kernel));
            config.push_str(&format!("initrd {}\n", entry.initrd));
            config.push_str(&format!("options {}\n\n", entry.params.to_string()));
        }
        
        config
    }

    pub fn generate_kernel_cmdline(&self) -> String {
        if let Some(default_id) = &self.default_entry {
            if let Some(entry) = self.entries.get(default_id) {
                return entry.params.to_string();
            }
        }
        KernelParams::default().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_loader_type_as_str() {
        assert_eq!(BootLoaderType::Grub.as_str(), "grub");
        assert_eq!(BootLoaderType::SystemdBoot.as_str(), "systemd-boot");
    }

    #[test]
    fn test_kernel_params_to_string() {
        let params = KernelParams::new()
            .with_quiet(true)
            .with_splash(true);
        assert!(params.to_string().contains("quiet"));
        assert!(params.to_string().contains("splash"));
    }

    #[test]
    fn test_boot_entry_new() {
        let entry = BootEntry::new("test", "Test OS", "/vmlinuz", "/initrd");
        assert_eq!(entry.id, "test");
        assert_eq!(entry.title, "Test OS");
    }

    #[test]
    fn test_boot_manager_new() {
        let mgr = BootManager::new(BootLoaderType::Grub);
        assert_eq!(mgr.bootloader, BootLoaderType::Grub);
    }

    #[test]
    fn test_boot_manager_add_entry() {
        let mut mgr = BootManager::default();
        let entry = BootEntry::new("sigmaos", "SigmaOS", "/vmlinuz", "/initrd");
        mgr.add_entry(entry).unwrap();
        assert_eq!(mgr.entries.len(), 1);
    }

    #[test]
    fn test_boot_manager_add_entry_invalid() {
        let mut mgr = BootManager::default();
        let entry = BootEntry::new("test", "Test", "", "/initrd");
        assert!(mgr.add_entry(entry).is_err());
    }

    #[test]
    fn test_generate_grub_config() {
        let mut mgr = BootManager::new(BootLoaderType::Grub);
        let entry = BootEntry::new("sigmaos", "SigmaOS", "/vmlinuz", "/initrd");
        mgr.add_entry(entry).unwrap();
        let config = mgr.generate_grub_config();
        assert!(config.contains("menuentry"));
    }

    #[test]
    fn test_generate_systemd_boot_config() {
        let mut mgr = BootManager::new(BootLoaderType::SystemdBoot);
        let entry = BootEntry::new("sigmaos", "SigmaOS", "/vmlinuz", "/initrd");
        mgr.add_entry(entry).unwrap();
        let config = mgr.generate_systemd_boot_config();
        assert!(config.contains("title"));
    }
}
