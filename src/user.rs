//! SigmaOS user and group management module.
//!
//! This module provides abstractions for user account management,
//! group management, and permission handling.
//!
//! # Example
//!
//! ```
//! use sigma_system::user::*;
//!
//! let mut user_mgr = UserManager::new();
//! let user = user_mgr.create_user("alice", "Alice Smith", "/home/alice", "/bin/bash").unwrap();
//! user_mgr.add_user_to_group("alice", "sudo").unwrap();
//! ```

use std::collections::{BTreeMap, BTreeSet};

/// Errors that can occur in user management.
#[derive(Debug, Clone, PartialEq)]
pub enum UserError {
    /// User already exists.
    UserExists(String),
    /// User not found.
    UserNotFound(String),
    /// Group already exists.
    GroupExists(String),
    /// Group not found.
    GroupNotFound(String),
    /// Invalid username.
    InvalidUsername(String),
    /// Invalid group name.
    InvalidGroupName(String),
    /// Permission denied.
    PermissionDenied(String),
    /// Invalid shell.
    InvalidShell(String),
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::UserExists(name) => write!(f, "User '{}' already exists", name),
            UserError::UserNotFound(name) => write!(f, "User '{}' not found", name),
            UserError::GroupExists(name) => write!(f, "Group '{}' already exists", name),
            UserError::GroupNotFound(name) => write!(f, "Group '{}' not found", name),
            UserError::InvalidUsername(name) => write!(f, "Invalid username '{}'", name),
            UserError::InvalidGroupName(name) => write!(f, "Invalid group name '{}'", name),
            UserError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            UserError::InvalidShell(shell) => write!(f, "Invalid shell '{}'", shell),
        }
    }
}

impl std::error::Error for UserError {}

/// User information.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    /// Username.
    pub username: String,
    /// Full name.
    pub full_name: String,
    /// Home directory.
    pub home_dir: String,
    /// Login shell.
    pub shell: String,
    /// User ID.
    pub uid: u32,
    /// Group ID.
    pub gid: u32,
    /// Groups the user belongs to.
    pub groups: BTreeSet<String>,
    /// Whether account is locked.
    pub locked: bool,
    /// Password hash (if set).
    pub password_hash: Option<String>,
}

impl User {
    /// Create a new user.
    pub fn new(
        username: &str,
        full_name: &str,
        home_dir: &str,
        shell: &str,
        uid: u32,
        gid: u32,
    ) -> Result<Self, UserError> {
        if username.is_empty() || username.len() > 32 {
            return Err(UserError::InvalidUsername(username.to_string()));
        }
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(UserError::InvalidUsername(username.to_string()));
        }
        if !["/bin/bash", "/bin/sh", "/bin/zsh", "/usr/bin/fish", "/bin/tcsh", "/bin/csh", "/bin/ksh"].contains(&shell) {
            return Err(UserError::InvalidShell(shell.to_string()));
        }
        Ok(Self {
            username: username.to_string(),
            full_name: full_name.to_string(),
            home_dir: home_dir.to_string(),
            shell: shell.to_string(),
            uid,
            gid,
            groups: BTreeSet::new(),
            locked: false,
            password_hash: None,
        })
    }

    /// Check if user is in a group.
    pub fn is_in_group(&self, group: &str) -> bool {
        self.groups.contains(group)
    }

    /// Add user to a group.
    pub fn add_to_group(&mut self, group: &str) {
        self.groups.insert(group.to_string());
    }

    /// Remove user from a group.
    pub fn remove_from_group(&mut self, group: &str) {
        self.groups.remove(group);
    }
}

/// Group information.
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    /// Group name.
    pub name: String,
    /// Group ID.
    pub gid: u32,
    /// Group description.
    pub description: String,
    /// Members of the group.
    pub members: BTreeSet<String>,
}

impl Group {
    /// Create a new group.
    pub fn new(name: &str, gid: u32, description: &str) -> Result<Self, UserError> {
        if name.is_empty() || name.len() > 32 {
            return Err(UserError::InvalidGroupName(name.to_string()));
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(UserError::InvalidGroupName(name.to_string()));
        }
        Ok(Self {
            name: name.to_string(),
            gid,
            description: description.to_string(),
            members: BTreeSet::new(),
        })
    }

    /// Add a member to the group.
    pub fn add_member(&mut self, username: &str) {
        self.members.insert(username.to_string());
    }

    /// Remove a member from the group.
    pub fn remove_member(&mut self, username: &str) {
        self.members.remove(username);
    }
}

/// User manager.
#[derive(Debug)]
pub struct UserManager {
    /// Users by username.
    users: BTreeMap<String, User>,
    /// Groups by name.
    groups: BTreeMap<String, Group>,
    /// Next UID.
    next_uid: u32,
    /// Next GID.
    next_gid: u32,
}

impl UserManager {
    /// Create a new user manager.
    pub fn new() -> Self {
        let mut mgr = Self {
            users: BTreeMap::new(),
            groups: BTreeMap::new(),
            next_uid: 1000,
            next_gid: 1000,
        };
        mgr.create_default_groups();
        mgr
    }

    fn create_default_groups(&mut self) {
        let default_groups = [
            ("users", "Regular users"),
            ("sudo", "Sudo users"),
            ("wheel", "Wheel group"),
            ("audio", "Audio devices"),
            ("video", "Video devices"),
            ("input", "Input devices"),
            ("disk", "Disk access"),
            ("network", "Network access"),
            ("plugdev", "Removable devices"),
            ("docker", "Docker users"),
            ("kvm", "KVM access"),
            ("libvirt", "Libvirt users"),
        ];
        for (name, desc) in default_groups {
            if let Ok(group) = Group::new(name, self.next_gid, desc) {
                self.next_gid += 1;
                self.groups.insert(name.to_string(), group);
            }
        }
    }

    /// Create a new user.
    pub fn create_user(
        &mut self,
        username: &str,
        full_name: &str,
        home_dir: &str,
        shell: &str,
    ) -> Result<User, UserError> {
        if self.users.contains_key(username) {
            return Err(UserError::UserExists(username.to_string()));
        }
        let uid = self.next_uid;
        self.next_uid += 1;
        let user = User::new(username, full_name, home_dir, shell, uid, 100)?;
        self.users.insert(username.to_string(), user.clone());
        Ok(user)
    }

    /// Delete a user.
    pub fn delete_user(&mut self, username: &str) -> Result<(), UserError> {
        if !self.users.contains_key(username) {
            return Err(UserError::UserNotFound(username.to_string()));
        }
        self.users.remove(username);
        Ok(())
    }

    /// Get a user by username.
    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.users.get(username)
    }

    /// Get all users.
    pub fn get_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    /// Create a new group.
    pub fn create_group(&mut self, name: &str, description: &str) -> Result<Group, UserError> {
        if self.groups.contains_key(name) {
            return Err(UserError::GroupExists(name.to_string()));
        }
        let gid = self.next_gid;
        self.next_gid += 1;
        let group = Group::new(name, gid, description)?;
        self.groups.insert(name.to_string(), group.clone());
        Ok(group)
    }

    /// Delete a group.
    pub fn delete_group(&mut self, name: &str) -> Result<(), UserError> {
        if !self.groups.contains_key(name) {
            return Err(UserError::GroupNotFound(name.to_string()));
        }
        self.groups.remove(name);
        Ok(())
    }

    /// Get a group by name.
    pub fn get_group(&self, name: &str) -> Option<&Group> {
        self.groups.get(name)
    }

    /// Get all groups.
    pub fn get_groups(&self) -> Vec<&Group> {
        self.groups.values().collect()
    }

    /// Add user to group.
    pub fn add_user_to_group(&mut self, username: &str, group_name: &str) -> Result<(), UserError> {
        if !self.users.contains_key(username) {
            return Err(UserError::UserNotFound(username.to_string()));
        }
        if !self.groups.contains_key(group_name) {
            return Err(UserError::GroupNotFound(group_name.to_string()));
        }
        if let Some(user) = self.users.get_mut(username) {
            user.add_to_group(group_name);
        }
        if let Some(group) = self.groups.get_mut(group_name) {
            group.add_member(username);
        }
        Ok(())
    }

    /// Remove user from group.
    pub fn remove_user_from_group(&mut self, username: &str, group_name: &str) -> Result<(), UserError> {
        if !self.users.contains_key(username) {
            return Err(UserError::UserNotFound(username.to_string()));
        }
        if !self.groups.contains_key(group_name) {
            return Err(UserError::GroupNotFound(group_name.to_string()));
        }
        if let Some(user) = self.users.get_mut(username) {
            user.remove_from_group(group_name);
        }
        if let Some(group) = self.groups.get_mut(group_name) {
            group.remove_member(username);
        }
        Ok(())
    }

    /// Check if user can perform an action (simplified permission check).
    pub fn can_user(&self, username: &str, permission: &str) -> bool {
        if let Some(user) = self.users.get(username) {
            // Check if user is in required group
            match permission {
                "sudo" => user.is_in_group("sudo") || user.is_in_group("wheel"),
                "docker" => user.is_in_group("docker"),
                "kvm" => user.is_in_group("kvm"),
                "libvirt" => user.is_in_group("libvirt"),
                "audio" => user.is_in_group("audio"),
                "video" => user.is_in_group("video"),
                "input" => user.is_in_group("input"),
                "disk" => user.is_in_group("disk"),
                _ => false,
            }
        } else {
            false
        }
    }

    /// Generate /etc/passwd content.
    pub fn generate_passwd(&self) -> String {
        let mut content = String::new();
        content.push_str("root:x:0:0:root:/root:/bin/bash\n");
        for user in self.users.values() {
            content.push_str(&format!(
                "{}:x:{}:{}:{}:{}:{}\n",
                user.username, user.uid, user.gid, user.full_name, user.home_dir, user.shell
            ));
        }
        content
    }

    /// Generate /etc/group content.
    pub fn generate_group(&self) -> String {
        let mut content = String::new();
        content.push_str("root:x:0:\n");
        for group in self.groups.values() {
            let members = group.members.iter().cloned().collect::<Vec<_>>().join(",");
            content.push_str(&format!("{}:x:{}:{}\n", group.name, group.gid, members));
        }
        content
    }

    /// Generate /etc/shadow content.
    pub fn generate_shadow(&self) -> String {
        let mut content = String::new();
        content.push_str("root:*:19000:0:99999:7:::\n");
        for user in self.users.values() {
            let password = user.password_hash.as_deref().unwrap_or("!");
            content.push_str(&format!("{}:{}:19000:0:99999:7:::\n", user.username, password));
        }
        content
    }
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let mut mgr = UserManager::new();
        let user = mgr.create_user("alice", "Alice Smith", "/home/alice", "/bin/bash").unwrap();
        assert_eq!(user.username, "alice");
        assert_eq!(user.uid, 1000);
    }

    #[test]
    fn test_create_duplicate_user() {
        let mut mgr = UserManager::new();
        mgr.create_user("alice", "Alice", "/home/alice", "/bin/bash").unwrap();
        let result = mgr.create_user("alice", "Alice2", "/home/alice2", "/bin/bash");
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_user() {
        let mut mgr = UserManager::new();
        mgr.create_user("alice", "Alice", "/home/alice", "/bin/bash").unwrap();
        mgr.delete_user("alice").unwrap();
        assert!(mgr.get_user("alice").is_none());
    }

    #[test]
    fn test_create_group() {
        let mut mgr = UserManager::new();
        let group = mgr.create_group("developers", "Developer group").unwrap();
        assert_eq!(group.name, "developers");
    }

    #[test]
    fn test_add_user_to_group() {
        let mut mgr = UserManager::new();
        mgr.create_user("alice", "Alice", "/home/alice", "/bin/bash").unwrap();
        // sudo is a default group, don't create it again
        mgr.add_user_to_group("alice", "sudo").unwrap();
        let user = mgr.get_user("alice").unwrap();
        assert!(user.is_in_group("sudo"));
    }

    #[test]
    fn test_can_user_sudo() {
        let mut mgr = UserManager::new();
        mgr.create_user("alice", "Alice", "/home/alice", "/bin/bash").unwrap();
        mgr.add_user_to_group("alice", "sudo").unwrap();
        assert!(mgr.can_user("alice", "sudo"));
        assert!(!mgr.can_user("alice", "docker"));
    }

    #[test]
    fn test_generate_passwd() {
        let mut mgr = UserManager::new();
        mgr.create_user("alice", "Alice", "/home/alice", "/bin/bash").unwrap();
        let passwd = mgr.generate_passwd();
        assert!(passwd.contains("alice"));
        assert!(passwd.contains("/home/alice"));
    }

    #[test]
    fn test_generate_group() {
        let mut mgr = UserManager::new();
        mgr.create_user("alice", "Alice", "/home/alice", "/bin/bash").unwrap();
        mgr.add_user_to_group("alice", "sudo").unwrap();
        let group = mgr.generate_group();
        assert!(group.contains("sudo"));
        assert!(group.contains("alice"));
    }

    #[test]
    fn test_default_groups() {
        let mgr = UserManager::new();
        assert!(mgr.get_group("sudo").is_some());
        assert!(mgr.get_group("docker").is_some());
        assert!(mgr.get_group("users").is_some());
    }

    #[test]
    fn test_invalid_username() {
        let mut mgr = UserManager::new();
        let result = mgr.create_user("user@name", "User", "/home/user", "/bin/bash");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_shell() {
        let mut mgr = UserManager::new();
        let result = mgr.create_user("alice", "Alice", "/home/alice", "/bin/invalid");
        assert!(result.is_err());
    }
}
