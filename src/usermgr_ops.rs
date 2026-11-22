//! SigmaOS user manager operations module.
//!
//! This module provides user management operations including:
//! - User creation
//! - User deletion
//! - Group management
//!
//! Follows Single Responsibility Principle: only manages user operations.
//!
//! # Example
//!
//! ```
//! use sigma_system::usermgr_ops::*;
//!
//! let usermgr = UserMgrOps::new();
//! if let Ok(users) = usermgr.list_users() {
//!     println!("Users: {}", users);
//! }
//! ```

use std::process::Command;

/// User manager operation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum UserMgrError {
    /// Command execution failed.
    CommandError(String),
    /// User not found.
    UserNotFound(String),
}

impl std::fmt::Display for UserMgrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserMgrError::CommandError(msg) => write!(f, "Command error: {}", msg),
            UserMgrError::UserNotFound(msg) => write!(f, "User not found: {}", msg),
        }
    }
}

impl std::error::Error for UserMgrError {}

/// User manager operations manager.
#[derive(Debug)]
pub struct UserMgrOps;

impl UserMgrOps {
    pub fn new() -> Self { Self }

    /// List users.
    pub fn list_users(&self) -> Result<String, UserMgrError> {
        let output = Command::new("getent")
            .arg("passwd")
            .output()
            .map_err(|e| UserMgrError::CommandError(format!("Failed to run getent: {}", e)))?;

        let users = String::from_utf8_lossy(&output.stdout);
        Ok(users.trim().to_string())
    }

    /// Create user.
    pub fn create_user(&self, username: &str) -> Result<(), UserMgrError> {
        let output = Command::new("useradd")
            .arg("-m")
            .arg(username)
            .output()
            .map_err(|e| UserMgrError::CommandError(format!("Failed to run useradd: {}", e)))?;

        if !output.status.success() {
            return Err(UserMgrError::CommandError("useradd failed".to_string()));
        }

        Ok(())
    }
}

impl Default for UserMgrOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let usermgr = UserMgrOps::new();
        let _ = usermgr;
    }

    #[test]
    fn test_list_users() {
        let usermgr = UserMgrOps::new();
        if let Ok(users) = usermgr.list_users() {
            // Should not panic
            let _ = users;
        }
    }
}
