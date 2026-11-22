//! SigmaOS locale operations module.
//!
//! This module provides actual locale management operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::locale_ops::*;
//!
//! let locale = LocaleOps::new();
//! if let Ok(current) = locale.get_current_locale() {
//!     println!("Current locale: {}", current);
//! }
//! ```

use std::fs;
use std::path::Path;

/// Errors that can occur in locale operations.
#[derive(Debug, Clone, PartialEq)]
pub enum LocaleError {
    /// Locale not found.
    LocaleNotFound(String),
    /// Failed to set locale.
    SetError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for LocaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocaleError::LocaleNotFound(name) => write!(f, "Locale '{}' not found", name),
            LocaleError::SetError(msg) => write!(f, "Failed to set locale: {}", msg),
            LocaleError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for LocaleError {}

/// Locale operations manager.
#[derive(Debug)]
pub struct LocaleOps;

impl LocaleOps {
    pub fn new() -> Self { Self }

    pub fn get_current_locale(&self) -> Result<String, LocaleError> {
        let path = Path::new("/etc/locale.conf");
        if !path.exists() {
            return Ok("en_US.UTF-8".to_string());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| LocaleError::IoError(format!("Failed to read locale.conf: {}", e)))?;

        for line in content.lines() {
            if line.starts_with("LANG=") {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() > 1 {
                    return Ok(parts[1].trim().to_string());
                }
            }
        }

        Ok("en_US.UTF-8".to_string())
    }

    pub fn list_available_locales(&self) -> Result<Vec<String>, LocaleError> {
        let path = Path::new("/etc/locale.gen");
        if !path.exists() {
            return Ok(vec!["en_US.UTF-8".to_string()]);
        }

        let content = fs::read_to_string(path)
            .map_err(|e| LocaleError::IoError(format!("Failed to read locale.gen: {}", e)))?;

        let mut locales = Vec::new();
        for line in content.lines() {
            if !line.starts_with('#') && line.contains("UTF-8") {
                let locale = line.split_whitespace().next().unwrap_or("");
                if !locale.is_empty() {
                    locales.push(locale.to_string());
                }
            }
        }

        Ok(locales)
    }
}

impl Default for LocaleOps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let locale = LocaleOps::new();
        let _ = locale;
    }

    #[test]
    fn test_get_current_locale() {
        let locale = LocaleOps::new();
        if let Ok(_) = locale.get_current_locale() {
            // Should not panic
        }
    }
}
