//! SigmaOS proxy operations module.
//!
//! This module provides actual proxy configuration operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::proxy_ops::*;
//!
//! let mut proxy_ops = ProxyOps::new();
//! if let Ok(config) = proxy_ops.get_proxy_config() {
//!     println!("HTTP proxy: {}", config.http_proxy);
//! }
//! ```

use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

/// Errors that can occur in proxy operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ProxyError {
    /// Failed to read proxy config.
    ReadError(String),
    /// Failed to set proxy.
    SetError(String),
    /// Invalid proxy URL.
    InvalidProxy(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyError::ReadError(msg) => write!(f, "Failed to read proxy config: {}", msg),
            ProxyError::SetError(msg) => write!(f, "Failed to set proxy: {}", msg),
            ProxyError::InvalidProxy(url) => write!(f, "Invalid proxy URL: {}", url),
            ProxyError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ProxyError {}

/// Proxy configuration.
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// HTTP proxy URL.
    pub http_proxy: String,
    /// HTTPS proxy URL.
    pub https_proxy: String,
    /// FTP proxy URL.
    pub ftp_proxy: String,
    /// No proxy list.
    pub no_proxy: String,
    /// SOCKS proxy URL.
    pub socks_proxy: String,
}

/// Proxy operations manager.
#[derive(Debug)]
pub struct ProxyOps {
    /// Environment variable names.
    env_vars: BTreeMap<String, String>,
}

impl ProxyOps {
    /// Create a new proxy operations manager.
    pub fn new() -> Self {
        let mut env_vars = BTreeMap::new();
        
        for (key, value) in std::env::vars() {
            if key.to_lowercase().contains("proxy") {
                env_vars.insert(key, value);
            }
        }
        
        Self { env_vars }
    }

    /// Get proxy configuration.
    pub fn get_proxy_config(&self) -> Result<ProxyConfig, ProxyError> {
        Ok(ProxyConfig {
            http_proxy: self.get_env_or_default("HTTP_PROXY", ""),
            https_proxy: self.get_env_or_default("HTTPS_PROXY", ""),
            ftp_proxy: self.get_env_or_default("FTP_PROXY", ""),
            no_proxy: self.get_env_or_default("NO_PROXY", ""),
            socks_proxy: self.get_env_or_default("SOCKS_PROXY", ""),
        })
    }

    /// Get environment variable or default.
    fn get_env_or_default(&self, key: &str, default: &str) -> String {
        self.env_vars.get(key)
            .or_else(|| self.env_vars.get(&key.to_lowercase()))
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Set HTTP proxy.
    pub fn set_http_proxy(&self, url: &str) -> Result<(), ProxyError> {
        self.validate_proxy_url(url)?;
        std::env::set_var("HTTP_PROXY", url);
        Ok(())
    }

    /// Set HTTPS proxy.
    pub fn set_https_proxy(&self, url: &str) -> Result<(), ProxyError> {
        self.validate_proxy_url(url)?;
        std::env::set_var("HTTPS_PROXY", url);
        Ok(())
    }

    /// Set FTP proxy.
    pub fn set_ftp_proxy(&self, url: &str) -> Result<(), ProxyError> {
        self.validate_proxy_url(url)?;
        std::env::set_var("FTP_PROXY", url);
        Ok(())
    }

    /// Set no proxy list.
    pub fn set_no_proxy(&self, list: &str) -> Result<(), ProxyError> {
        std::env::set_var("NO_PROXY", list);
        Ok(())
    }

    /// Validate proxy URL.
    fn validate_proxy_url(&self, url: &str) -> Result<(), ProxyError> {
        if url.is_empty() {
            return Ok(());
        }

        if !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with("socks://") && !url.starts_with("socks5://") {
            return Err(ProxyError::InvalidProxy("Proxy URL must start with http://, https://, socks://, or socks5://".to_string()));
        }

        Ok(())
    }

    /// Clear all proxy settings.
    pub fn clear_proxies(&self) -> Result<(), ProxyError> {
        std::env::remove_var("HTTP_PROXY");
        std::env::remove_var("HTTPS_PROXY");
        std::env::remove_var("FTP_PROXY");
        std::env::remove_var("NO_PROXY");
        std::env::remove_var("SOCKS_PROXY");
        std::env::remove_var("ALL_PROXY");
        std::env::remove_var("all_proxy");
        Ok(())
    }

    /// Get proxy summary.
    pub fn get_proxy_summary(&self) -> Result<String, ProxyError> {
        let config = self.get_proxy_config()?;
        
        let mut summary = String::new();
        if !config.http_proxy.is_empty() {
            summary.push_str(&format!("HTTP: {} ", config.http_proxy));
        }
        if !config.https_proxy.is_empty() {
            summary.push_str(&format!("HTTPS: {} ", config.https_proxy));
        }
        if !config.ftp_proxy.is_empty() {
            summary.push_str(&format!("FTP: {} ", config.ftp_proxy));
        }
        if !config.socks_proxy.is_empty() {
            summary.push_str(&format!("SOCKS: {} ", config.socks_proxy));
        }
        if !config.no_proxy.is_empty() {
            summary.push_str(&format!("NO_PROXY: {}", config.no_proxy));
        }
        
        if summary.is_empty() {
            summary = "No proxy configured".to_string();
        }
        
        Ok(summary)
    }
}

impl Default for ProxyOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let proxy_ops = ProxyOps::new();
        let _ = proxy_ops.env_vars;
    }

    #[test]
    fn test_get_proxy_config() {
        let proxy_ops = ProxyOps::new();
        if let Ok(config) = proxy_ops.get_proxy_config() {
            let _ = config;
        }
    }

    #[test]
    fn test_validate_proxy_url() {
        let proxy_ops = ProxyOps::new();
        assert!(proxy_ops.validate_proxy_url("http://proxy.example.com:8080").is_ok());
        assert!(proxy_ops.validate_proxy_url("https://proxy.example.com:8080").is_ok());
        assert!(proxy_ops.validate_proxy_url("invalid").is_err());
    }

    #[test]
    fn test_clear_proxies() {
        let proxy_ops = ProxyOps::new();
        if let Ok(_) = proxy_ops.clear_proxies() {
            // Should not panic
        }
    }
}
