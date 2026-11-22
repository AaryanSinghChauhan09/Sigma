//! SigmaOS environment variables module.
//!
//! This module provides actual environment variable operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::env_ops::*;
//!
//! let env_ops = EnvOps::new();
//! if let Ok(path) = env_ops.get_var("PATH") {
//!     println!("PATH: {}", path);
//! }
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in environment operations.
#[derive(Debug, Clone, PartialEq)]
pub enum EnvError {
    /// Variable not found.
    VarNotFound(String),
    /// Failed to set variable.
    SetError(String),
    /// Invalid variable name.
    InvalidVarName(String),
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::VarNotFound(name) => write!(f, "Variable '{}' not found", name),
            EnvError::SetError(msg) => write!(f, "Failed to set variable: {}", msg),
            EnvError::InvalidVarName(name) => write!(f, "Invalid variable name: {}", name),
        }
    }
}

impl std::error::Error for EnvError {}

/// Environment variables manager.
#[derive(Debug)]
pub struct EnvOps {
    /// Cached environment variables.
    env_vars: BTreeMap<String, String>,
}

impl EnvOps {
    /// Create a new environment operations manager.
    pub fn new() -> Self {
        let mut env_vars = BTreeMap::new();
        
        // Populate with current environment
        for (key, value) in std::env::vars() {
            env_vars.insert(key, value);
        }
        
        Self { env_vars }
    }

    /// Get environment variable.
    pub fn get_var(&self, name: &str) -> Result<String, EnvError> {
        self.env_vars.get(name)
            .cloned()
            .ok_or_else(|| EnvError::VarNotFound(name.to_string()))
    }

    /// Get environment variable with default.
    pub fn get_var_or_default(&self, name: &str, default: &str) -> String {
        self.env_vars.get(name)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Set environment variable (for current process only).
    pub fn set_var(&mut self, name: &str, value: &str) -> Result<(), EnvError> {
        self.validate_var_name(name)?;
        std::env::set_var(name, value);
        self.env_vars.insert(name.to_string(), value.to_string());
        Ok(())
    }

    /// Remove environment variable (for current process only).
    pub fn remove_var(&mut self, name: &str) -> Result<(), EnvError> {
        std::env::remove_var(name);
        self.env_vars.remove(name);
        Ok(())
    }

    /// Validate variable name.
    fn validate_var_name(&self, name: &str) -> Result<(), EnvError> {
        if name.is_empty() {
            return Err(EnvError::InvalidVarName("Variable name cannot be empty".to_string()));
        }

        // First character must be alphabetic or underscore
        let first_char = name.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return Err(EnvError::InvalidVarName("Variable name must start with letter or underscore".to_string()));
        }

        // Rest must be alphanumeric or underscore
        for c in name.chars() {
            if !c.is_alphanumeric() && c != '_' {
                return Err(EnvError::InvalidVarName(format!("Invalid character '{}' in variable name", c)));
            }
        }

        Ok(())
    }

    /// Get all environment variables.
    pub fn get_all_vars(&self) -> BTreeMap<String, String> {
        self.env_vars.clone()
    }

    /// Get environment variables with prefix.
    pub fn get_vars_with_prefix(&self, prefix: &str) -> BTreeMap<String, String> {
        self.env_vars.iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Get PATH variable as vector.
    pub fn get_path(&self) -> Result<Vec<String>, EnvError> {
        let path = self.get_var("PATH")?;
        Ok(path.split(':').map(|s| s.to_string()).collect())
    }

    /// Add directory to PATH.
    pub fn add_to_path(&mut self, dir: &str) -> Result<(), EnvError> {
        let mut current_path = self.get_path().unwrap_or_default();
        
        // Check if already in PATH
        if current_path.contains(&dir.to_string()) {
            return Ok(());
        }
        
        current_path.insert(0, dir.to_string());
        let new_path = current_path.join(":");
        self.set_var("PATH", &new_path)
    }

    /// Get home directory.
    pub fn get_home(&self) -> Result<String, EnvError> {
        self.get_var("HOME")
    }

    /// Get user.
    pub fn get_user(&self) -> Result<String, EnvError> {
        self.get_var("USER")
    }

    /// Get shell.
    pub fn get_shell(&self) -> Result<String, EnvError> {
        self.get_var("SHELL")
    }

    /// Get language.
    pub fn get_lang(&self) -> Result<String, EnvError> {
        self.get_var("LANG")
    }

    /// Get terminal.
    pub fn get_term(&self) -> Result<String, EnvError> {
        self.get_var("TERM")
    }

    /// Get editor.
    pub fn get_editor(&self) -> Result<String, EnvError> {
        let editor = self.get_var_or_default("EDITOR", "vi");
        if editor != "vi" {
            return Ok(editor);
        }
        let visual = self.get_var_or_default("VISUAL", "vi");
        if visual != "vi" {
            return Ok(visual);
        }
        Ok("vi".to_string())
    }

    /// Get pager.
    pub fn get_pager(&self) -> Result<String, EnvError> {
        Ok(self.get_var_or_default("PAGER", "less"))
    }

    /// Expand environment variables in a string.
    pub fn expand(&self, s: &str) -> String {
        let mut result = s.to_string();
        
        for (key, value) in &self.env_vars {
            result = result.replace(&format!("${}", key), value);
            result = result.replace(&format!("${{{}}}", key), value);
        }
        
        result
    }

    /// Get environment summary.
    pub fn get_env_summary(&self) -> String {
        let mut summary = String::new();
        
        if let Ok(home) = self.get_home() {
            summary.push_str(&format!("HOME: {}\n", home));
        }
        if let Ok(user) = self.get_user() {
            summary.push_str(&format!("USER: {}\n", user));
        }
        if let Ok(shell) = self.get_shell() {
            summary.push_str(&format!("SHELL: {}\n", shell));
        }
        if let Ok(lang) = self.get_lang() {
            summary.push_str(&format!("LANG: {}\n", lang));
        }
        if let Ok(term) = self.get_term() {
            summary.push_str(&format!("TERM: {}\n", term));
        }
        
        summary.push_str(&format!("Total variables: {}", self.env_vars.len()));
        
        summary
    }
}

impl Default for EnvOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let env_ops = EnvOps::new();
        assert!(!env_ops.env_vars.is_empty());
    }

    #[test]
    fn test_get_var() {
        let env_ops = EnvOps::new();
        if let Ok(path) = env_ops.get_var("PATH") {
            assert!(!path.is_empty());
        }
    }

    #[test]
    fn test_set_var() {
        let mut env_ops = EnvOps::new();
        let result = env_ops.set_var("SIGMAOS_TEST", "test_value");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_var_name() {
        let env_ops = EnvOps::new();
        assert!(env_ops.validate_var_name("VALID_NAME").is_ok());
        assert!(env_ops.validate_var_name("valid_name").is_ok());
        assert!(env_ops.validate_var_name("_valid").is_ok());
        assert!(env_ops.validate_var_name("1invalid").is_err());
        assert!(env_ops.validate_var_name("invalid-name").is_err());
    }
}
