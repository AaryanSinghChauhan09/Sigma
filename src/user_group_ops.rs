//! SigmaOS user group operations module.
//!
//! This module provides actual user and group operations using useradd, groupadd, etc.
//!
//! # Example
//!
//! ```
//! use sigma_system::user_group_ops::*;
//!
//! let mut ug_ops = UserGroupOps::new();
//! if let Ok(users) = ug_ops.list_users() {
//!     println!("Found {} users", users.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in user/group operations.
#[derive(Debug, Clone, PartialEq)]
pub enum UserGroupError {
    /// User not found.
    UserNotFound(String),
    /// Group not found.
    GroupNotFound(String),
    /// Failed to add user.
    AddUserError(String),
    /// Failed to add group.
    AddGroupError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for UserGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserGroupError::UserNotFound(user) => write!(f, "User '{}' not found", user),
            UserGroupError::GroupNotFound(group) => write!(f, "Group '{}' not found", group),
            UserGroupError::AddUserError(msg) => write!(f, "Failed to add user: {}", msg),
            UserGroupError::AddGroupError(msg) => write!(f, "Failed to add group: {}", msg),
            UserGroupError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            UserGroupError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for UserGroupError {}

/// User information.
#[derive(Debug, Clone)]
pub struct UserInfo {
    /// Username.
    pub username: String,
    /// User ID.
    pub uid: u32,
    /// Group ID.
    pub gid: u32,
    /// Home directory.
    pub home: String,
    /// Shell.
    pub shell: String,
}

/// Group information.
#[derive(Debug, Clone)]
pub struct GroupInfo {
    /// Group name.
    pub groupname: String,
    /// Group ID.
    pub gid: u32,
    /// Members.
    pub members: Vec<String>,
}

/// User and group operations manager.
#[derive(Debug)]
pub struct UserGroupOps {
    /// Whether running as root.
    is_root: bool,
}

impl UserGroupOps {
    /// Create a new user/group operations manager.
    pub fn new() -> Self {
        let is_root = Command::new("id").arg("-u").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false);

        Self { is_root }
    }

    /// Check if running as root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// List all users.
    pub fn list_users(&self) -> Result<Vec<UserInfo>, UserGroupError> {
        let output = Command::new("getent").arg("passwd").output()
            .map_err(|e| UserGroupError::IoError(format!("Failed to get passwd: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let mut users = Vec::new();
        
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 7 {
                let uid = parts[2].parse().unwrap_or(0);
                let gid = parts[3].parse().unwrap_or(0);
                users.push(UserInfo {
                    username: parts[0].to_string(),
                    uid,
                    gid,
                    home: parts[5].to_string(),
                    shell: parts[6].to_string(),
                });
            }
        }
        
        Ok(users)
    }

    /// List all groups.
    pub fn list_groups(&self) -> Result<Vec<GroupInfo>, UserGroupError> {
        let output = Command::new("getent").arg("group").output()
            .map_err(|e| UserGroupError::IoError(format!("Failed to get group: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let mut groups = Vec::new();
        
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 4 {
                let gid = parts[2].parse().unwrap_or(0);
                let members: Vec<String> = if parts[3].is_empty() {
                    Vec::new()
                } else {
                    parts[3].split(',').map(|s| s.to_string()).collect()
                };
                groups.push(GroupInfo {
                    groupname: parts[0].to_string(),
                    gid,
                    members,
                });
            }
        }
        
        Ok(groups)
    }

    /// Get user info.
    pub fn get_user_info(&self, username: &str) -> Result<UserInfo, UserGroupError> {
        let output = Command::new("id").arg(username).output()
            .map_err(|e| UserGroupError::IoError(format!("Failed to get user info: {}", e)))?;
        
        if !output.status.success() {
            return Err(UserGroupError::UserNotFound(username.to_string()));
        }
        
        let content = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = content.split_whitespace().collect();
        
        let uid = parts[0].parse().unwrap_or(0);
        let gid = parts[1].parse().unwrap_or(0);
        
        Ok(UserInfo {
            username: username.to_string(),
            uid,
            gid,
            home: "/home/".to_string() + username,
            shell: "/bin/bash".to_string(),
        })
    }

    /// Get groups for a user.
    pub fn get_user_groups(&self, username: &str) -> Result<Vec<String>, UserGroupError> {
        let output = Command::new("groups").arg(username).output()
            .map_err(|e| UserGroupError::IoError(format!("Failed to get groups: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let groups: Vec<String> = content.split_whitespace()
            .skip(1)
            .map(|s| s.trim_end_matches(':').to_string())
            .collect();
        
        Ok(groups)
    }

    /// Add a user (requires root).
    pub fn add_user(&self, username: &str) -> Result<(), UserGroupError> {
        if !self.is_root {
            return Err(UserGroupError::PermissionDenied("Must be root to add user".to_string()));
        }

        let result = Command::new("useradd")
            .args(&["-m", "-s", "/bin/bash", username])
            .status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(UserGroupError::AddUserError("useradd command failed".to_string())),
            Err(e) => Err(UserGroupError::IoError(format!("Failed to run useradd: {}", e))),
        }
    }

    /// Add a group (requires root).
    pub fn add_group(&self, groupname: &str) -> Result<(), UserGroupError> {
        if !self.is_root {
            return Err(UserGroupError::PermissionDenied("Must be root to add group".to_string()));
        }

        let result = Command::new("groupadd").arg(groupname).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(UserGroupError::AddGroupError("groupadd command failed".to_string())),
            Err(e) => Err(UserGroupError::IoError(format!("Failed to run groupadd: {}", e))),
        }
    }

    /// Get user/group summary.
    pub fn get_summary(&self) -> Result<String, UserGroupError> {
        let users = self.list_users()?;
        let groups = self.list_groups()?;
        
        Ok(format!(
            "Total users: {}\nTotal groups: {}",
            users.len(),
            groups.len()
        ))
    }
}

impl Default for UserGroupOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ug_ops = UserGroupOps::new();
        let _ = ug_ops.is_root();
    }

    #[test]
    fn test_list_users() {
        let ug_ops = UserGroupOps::new();
        if let Ok(users) = ug_ops.list_users() {
            assert!(!users.is_empty());
        }
    }

    #[test]
    fn test_list_groups() {
        let ug_ops = UserGroupOps::new();
        if let Ok(groups) = ug_ops.list_groups() {
            assert!(!groups.is_empty());
        }
    }

    #[test]
    fn test_get_summary() {
        let ug_ops = UserGroupOps::new();
        if let Ok(summary) = ug_ops.get_summary() {
            assert!(!summary.is_empty());
        }
    }
}
