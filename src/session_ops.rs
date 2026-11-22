//! SigmaOS user session operations module.
//!
//! This module provides actual user session operations using loginctl.
//!
//! # Example
//!
//! ```
//! use sigma_system::session_ops::*;
//!
//! let mut session_ops = SessionOps::new();
//! if let Ok(sessions) = session_ops.list_sessions() {
//!     println!("Found {} sessions", sessions.len());
//! }
//! ```

use std::process::Command;

/// Errors that can occur in session operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionError {
    /// Session not found.
    SessionNotFound(String),
    /// Failed to get session info.
    InfoError(String),
    /// Permission denied.
    PermissionDenied(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::SessionNotFound(id) => write!(f, "Session '{}' not found", id),
            SessionError::InfoError(msg) => write!(f, "Failed to get session info: {}", msg),
            SessionError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            SessionError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for SessionError {}

/// Session information.
#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// Session ID.
    pub id: String,
    /// User name.
    pub user: String,
    /// Seat (if any).
    pub seat: String,
    /// TTY (if any).
    pub tty: String,
    /// Session type.
    pub session_type: String,
    /// Session state.
    pub state: String,
    /// Idle hint.
    pub idle_hint: bool,
    /// Locked hint.
    pub locked_hint: bool,
}

/// User session operations manager.
#[derive(Debug)]
pub struct SessionOps {
    /// Whether running as root.
    is_root: bool,
}

impl SessionOps {
    /// Create a new session operations manager.
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

    /// List all sessions.
    pub fn list_sessions(&self) -> Result<Vec<SessionInfo>, SessionError> {
        let output = Command::new("loginctl").arg("list-sessions").arg("--no-legend").output()
            .map_err(|e| SessionError::IoError(format!("Failed to list sessions: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let mut sessions = Vec::new();
        
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let id = parts[0].to_string();
                let user = parts[1].to_string();
                
                if let Ok(session) = self.get_session_info(&id) {
                    sessions.push(session);
                }
            }
        }
        
        Ok(sessions)
    }

    /// Get session info.
    pub fn get_session_info(&self, id: &str) -> Result<SessionInfo, SessionError> {
        let output = Command::new("loginctl").args(&["show-session", id]).output()
            .map_err(|e| SessionError::IoError(format!("Failed to get session info: {}", e)))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        
        let mut session_info = SessionInfo {
            id: id.to_string(),
            user: "unknown".to_string(),
            seat: String::new(),
            tty: String::new(),
            session_type: "unknown".to_string(),
            state: "unknown".to_string(),
            idle_hint: false,
            locked_hint: false,
        };
        
        for line in content.lines() {
            if line.starts_with("User=") {
                session_info.user = line[5..].trim().to_string();
            } else if line.starts_with("Seat=") {
                session_info.seat = line[5..].trim().to_string();
            } else if line.starts_with("TTY=") {
                session_info.tty = line[4..].trim().to_string();
            } else if line.starts_with("Type=") {
                session_info.session_type = line[5..].trim().to_string();
            } else if line.starts_with("State=") {
                session_info.state = line[6..].trim().to_string();
            } else if line.starts_with("IdleHint=") {
                session_info.idle_hint = line[9..].trim() == "yes";
            } else if line.starts_with("LockedHint=") {
                session_info.locked_hint = line[11..].trim() == "yes";
            }
        }
        
        Ok(session_info)
    }

    /// Get current session.
    pub fn get_current_session(&self) -> Result<Option<SessionInfo>, SessionError> {
        let output = Command::new("loginctl").arg("show-session").output()
            .map_err(|e| SessionError::IoError(format!("Failed to get current session: {}", e)))?;
        
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.starts_with("Id=") {
                    let id = line[3..].trim().to_string();
                    return Ok(Some(self.get_session_info(&id)?));
                }
            }
        }
        
        Ok(None)
    }

    /// Terminate a session.
    pub fn terminate_session(&self, id: &str) -> Result<(), SessionError> {
        if !self.is_root {
            return Err(SessionError::PermissionDenied("Must be root to terminate session".to_string()));
        }

        let result = Command::new("loginctl").args(&["terminate-session", id]).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(SessionError::InfoError("loginctl terminate-session failed".to_string())),
            Err(e) => Err(SessionError::IoError(format!("Failed to run loginctl: {}", e))),
        }
    }

    /// Lock a session.
    pub fn lock_session(&self, id: &str) -> Result<(), SessionError> {
        let result = Command::new("loginctl").args(&["lock-session", id]).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(SessionError::InfoError("loginctl lock-session failed".to_string())),
            Err(e) => Err(SessionError::IoError(format!("Failed to run loginctl: {}", e))),
        }
    }

    /// Unlock a session.
    pub fn unlock_session(&self, id: &str) -> Result<(), SessionError> {
        let result = Command::new("loginctl").args(&["unlock-session", id]).status();

        match result {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(SessionError::InfoError("loginctl unlock-session failed".to_string())),
            Err(e) => Err(SessionError::IoError(format!("Failed to run loginctl: {}", e))),
        }
    }

    /// Get active sessions count.
    pub fn get_active_sessions_count(&self) -> Result<usize, SessionError> {
        let sessions = self.list_sessions()?;
        Ok(sessions.iter().filter(|s| s.state == "active").count())
    }

    /// Get session summary.
    pub fn get_session_summary(&self) -> Result<String, SessionError> {
        let sessions = self.list_sessions()?;
        let active_count = self.get_active_sessions_count()?;
        
        Ok(format!(
            "Total sessions: {}\nActive sessions: {}",
            sessions.len(),
            active_count
        ))
    }
}

impl Default for SessionOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let session_ops = SessionOps::new();
        let _ = session_ops.is_root();
    }

    #[test]
    fn test_list_sessions() {
        let session_ops = SessionOps::new();
        if let Ok(sessions) = session_ops.list_sessions() {
            let _ = sessions;
        }
    }

    #[test]
    fn test_get_current_session() {
        let session_ops = SessionOps::new();
        if let Ok(session) = session_ops.get_current_session() {
            let _ = session;
        }
    }

    #[test]
    fn test_get_session_summary() {
        let session_ops = SessionOps::new();
        if let Ok(summary) = session_ops.get_session_summary() {
            assert!(!summary.is_empty());
        }
    }
}
