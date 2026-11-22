//! SigmaOS proxy operations module (extended).
//!
//! This module provides actual proxy configuration via environment variables.
//!
//! # Example
//!
//! ```
//! use sigma_system::proxy_ops::*;
//!
//! let proxy = ProxyOps::new();
//! if let Ok(status) = proxy.set_proxy("http://proxy.example.com:8080") {
//!     println!("Proxy set: {}", status);
//! }
//! ```

use std::env;

/// Errors that can occur in proxy operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ProxyError {
    /// Set failed.
    SetError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyError::SetError(msg) => write!(f, "Failed to set proxy: {}", msg),
            ProxyError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ProxyError {}

/// Proxy operations manager.
#[derive(Debug)]
pub struct ProxyOps;

impl ProxyOps {
    pub fn new() -> Self { Self }

    pub fn set_proxy(&self, proxy_url: &str) -> Result<String, ProxyError> {
        env::set_var("http_proxy", proxy_url);
        env::set_var("https_proxy", proxy_url);
        env::set_var("HTTP_PROXY", proxy_url);
        env::set_var("HTTPS_PROXY", proxy_url);
        Ok("proxy_set".to_string())
    }

    pub fn clear_proxy(&self) -> Result<String, ProxyError> {
        env::remove_var("http_proxy");
        env::remove_var("https_proxy");
        env::remove_var("HTTP_PROXY");
        env::remove_var("HTTPS_PROXY");
        Ok("proxy_cleared".to_string())
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
        let proxy = ProxyOps::new();
        let _ = proxy;
    }

    #[test]
    fn test_set_proxy() {
        let proxy = ProxyOps::new();
        if let Ok(_) = proxy.set_proxy("http://test.proxy:8080") {
            // Should not panic
        }
    }

    #[test]
    fn test_clear_proxy() {
        let proxy = ProxyOps::new();
        if let Ok(_) = proxy.clear_proxy() {
            // Should not panic
        }
    }
}
