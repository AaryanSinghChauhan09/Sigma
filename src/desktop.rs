//! SigmaOS desktop environment module.
//!
//! This module provides abstractions for desktop environment management,
//! window managers, display servers, and GUI applications.
//!
//! # Example
//!
//! ```
//! use sigma_system::desktop::*;
//!
//! let mut desktop_mgr = DesktopManager::new();
//! desktop_mgr.set_desktop_environment("gnome").unwrap();
//! desktop_mgr.set_display_server("wayland").unwrap();
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in desktop management.
#[derive(Debug, Clone, PartialEq)]
pub enum DesktopError {
    /// Invalid desktop environment.
    InvalidDesktopEnvironment(String),
    /// Invalid display server.
    InvalidDisplayServer(String),
    /// Invalid window manager.
    InvalidWindowManager(String),
    /// Desktop environment not found.
    DesktopEnvironmentNotFound(String),
}

impl std::fmt::Display for DesktopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DesktopError::InvalidDesktopEnvironment(de) => write!(f, "Invalid desktop environment '{}'", de),
            DesktopError::InvalidDisplayServer(ds) => write!(f, "Invalid display server '{}'", ds),
            DesktopError::InvalidWindowManager(wm) => write!(f, "Invalid window manager '{}'", wm),
            DesktopError::DesktopEnvironmentNotFound(de) => write!(f, "Desktop environment '{}' not found", de),
        }
    }
}

impl std::error::Error for DesktopError {}

/// Desktop environment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DesktopEnvironment {
    /// GNOME desktop environment.
    Gnome,
    /// KDE Plasma desktop environment.
    Kde,
    /// XFCE desktop environment.
    Xfce,
    /// LXDE desktop environment.
    Lxde,
    /// LXQt desktop environment.
    Lxqt,
    /// Cinnamon desktop environment.
    Cinnamon,
    /// MATE desktop environment.
    Mate,
    /// Budgie desktop environment.
    Budgie,
    /// Pantheon desktop environment.
    Pantheon,
    /// Sway (Wayland compositor).
    Sway,
    /// i3 (X11 window manager).
    I3,
    /// Custom/Minimal.
    Custom,
}

impl DesktopEnvironment {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopEnvironment::Gnome => "gnome",
            DesktopEnvironment::Kde => "kde",
            DesktopEnvironment::Xfce => "xfce",
            DesktopEnvironment::Lxde => "lxde",
            DesktopEnvironment::Lxqt => "lxqt",
            DesktopEnvironment::Cinnamon => "cinnamon",
            DesktopEnvironment::Mate => "mate",
            DesktopEnvironment::Budgie => "budgie",
            DesktopEnvironment::Pantheon => "pantheon",
            DesktopEnvironment::Sway => "sway",
            DesktopEnvironment::I3 => "i3",
            DesktopEnvironment::Custom => "custom",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Result<Self, DesktopError> {
        match s.to_lowercase().as_str() {
            "gnome" => Ok(DesktopEnvironment::Gnome),
            "kde" | "plasma" => Ok(DesktopEnvironment::Kde),
            "xfce" => Ok(DesktopEnvironment::Xfce),
            "lxde" => Ok(DesktopEnvironment::Lxde),
            "lxqt" => Ok(DesktopEnvironment::Lxqt),
            "cinnamon" => Ok(DesktopEnvironment::Cinnamon),
            "mate" => Ok(DesktopEnvironment::Mate),
            "budgie" => Ok(DesktopEnvironment::Budgie),
            "pantheon" => Ok(DesktopEnvironment::Pantheon),
            "sway" => Ok(DesktopEnvironment::Sway),
            "i3" => Ok(DesktopEnvironment::I3),
            "custom" | "minimal" => Ok(DesktopEnvironment::Custom),
            _ => Err(DesktopError::InvalidDesktopEnvironment(s.to_string())),
        }
    }
}

/// Display server type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayServer {
    /// X11 display server.
    X11,
    /// Wayland display server.
    Wayland,
    /// Both X11 and Wayland.
    Both,
}

impl DisplayServer {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            DisplayServer::X11 => "x11",
            DisplayServer::Wayland => "wayland",
            DisplayServer::Both => "both",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Result<Self, DesktopError> {
        match s.to_lowercase().as_str() {
            "x11" | "xorg" => Ok(DisplayServer::X11),
            "wayland" => Ok(DisplayServer::Wayland),
            "both" | "x11-wayland" => Ok(DisplayServer::Both),
            _ => Err(DesktopError::InvalidDisplayServer(s.to_string())),
        }
    }
}

/// Window manager type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowManager {
    /// GNOME Shell.
    GnomeShell,
    /// KDE KWin.
    KWin,
    /// XFCE Xfwm4.
    Xfwm4,
    /// Openbox.
    Openbox,
    /// i3 window manager.
    I3,
    /// Sway window manager.
    Sway,
    /// Custom window manager.
    Custom,
}

impl WindowManager {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            WindowManager::GnomeShell => "gnome-shell",
            WindowManager::KWin => "kwin",
            WindowManager::Xfwm4 => "xfwm4",
            WindowManager::Openbox => "openbox",
            WindowManager::I3 => "i3",
            WindowManager::Sway => "sway",
            WindowManager::Custom => "custom",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Result<Self, DesktopError> {
        match s.to_lowercase().as_str() {
            "gnome-shell" | "gnome" => Ok(WindowManager::GnomeShell),
            "kwin" | "kde" => Ok(WindowManager::KWin),
            "xfwm4" | "xfce" => Ok(WindowManager::Xfwm4),
            "openbox" => Ok(WindowManager::Openbox),
            "i3" => Ok(WindowManager::I3),
            "sway" => Ok(WindowManager::Sway),
            "custom" => Ok(WindowManager::Custom),
            _ => Err(DesktopError::InvalidWindowManager(s.to_string())),
        }
    }
}

/// Desktop session.
#[derive(Debug, Clone)]
pub struct DesktopSession {
    /// Session name.
    pub name: String,
    /// Desktop environment.
    pub desktop_environment: DesktopEnvironment,
    /// Display server.
    pub display_server: DisplayServer,
    /// Window manager.
    pub window_manager: WindowManager,
    /// Session command.
    pub command: String,
    /// Whether session is default.
    pub is_default: bool,
}

impl DesktopSession {
    /// Create a new desktop session.
    pub fn new(
        name: &str,
        de: DesktopEnvironment,
        ds: DisplayServer,
        wm: WindowManager,
        command: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            desktop_environment: de,
            display_server: ds,
            window_manager: wm,
            command: command.to_string(),
            is_default: false,
        }
    }
}

/// Desktop manager.
#[derive(Debug)]
pub struct DesktopManager {
    /// Available sessions.
    sessions: BTreeMap<String, DesktopSession>,
    /// Current session.
    current_session: Option<String>,
    /// Display manager (login screen).
    display_manager: String,
    /// Autologin enabled.
    autologin: bool,
    /// Autologin user.
    autologin_user: Option<String>,
}

impl DesktopManager {
    /// Create a new desktop manager.
    pub fn new() -> Self {
        let mut mgr = Self {
            sessions: BTreeMap::new(),
            current_session: None,
            display_manager: "gdm".to_string(),
            autologin: false,
            autologin_user: None,
        };
        mgr.add_default_sessions();
        mgr
    }

    fn add_default_sessions(&mut self) {
        let sessions = [
            ("gnome", DesktopEnvironment::Gnome, DisplayServer::Wayland, WindowManager::GnomeShell, "gnome-session"),
            ("kde", DesktopEnvironment::Kde, DisplayServer::Wayland, WindowManager::KWin, "startplasma-wayland"),
            ("xfce", DesktopEnvironment::Xfce, DisplayServer::X11, WindowManager::Xfwm4, "startxfce4"),
            ("sway", DesktopEnvironment::Sway, DisplayServer::Wayland, WindowManager::Sway, "sway"),
            ("i3", DesktopEnvironment::I3, DisplayServer::X11, WindowManager::I3, "i3"),
        ];
        
        for (name, de, ds, wm, cmd) in sessions {
            let session = DesktopSession::new(name, de, ds, wm, cmd);
            self.sessions.insert(name.to_string(), session);
        }
        
        // Set default session
        self.current_session = Some("gnome".to_string());
        if let Some(session) = self.sessions.get_mut("gnome") {
            session.is_default = true;
        }
    }

    /// Set the current desktop environment.
    pub fn set_desktop_environment(&mut self, de_name: &str) -> Result<(), DesktopError> {
        let de = DesktopEnvironment::parse(de_name)?;
        // Find session with this DE
        for (name, session) in self.sessions.iter_mut() {
            if session.desktop_environment == de {
                session.is_default = true;
                self.current_session = Some(name.clone());
                return Ok(());
            }
        }
        Err(DesktopError::DesktopEnvironmentNotFound(de_name.to_string()))
    }

    /// Set the display server.
    pub fn set_display_server(&mut self, ds_name: &str) -> Result<(), DesktopError> {
        let ds = DisplayServer::parse(ds_name)?;
        // Update current session
        if let Some(session_name) = &self.current_session {
            if let Some(session) = self.sessions.get_mut(session_name) {
                session.display_server = ds;
                return Ok(());
            }
        }
        Err(DesktopError::InvalidDisplayServer(ds_name.to_string()))
    }

    /// Set the window manager.
    pub fn set_window_manager(&mut self, wm_name: &str) -> Result<(), DesktopError> {
        let wm = WindowManager::parse(wm_name)?;
        // Update current session
        if let Some(session_name) = &self.current_session {
            if let Some(session) = self.sessions.get_mut(session_name) {
                session.window_manager = wm;
                return Ok(());
            }
        }
        Err(DesktopError::InvalidWindowManager(wm_name.to_string()))
    }

    /// Get the current session.
    pub fn get_current_session(&self) -> Option<&DesktopSession> {
        self.current_session.as_ref().and_then(|name| self.sessions.get(name))
    }

    /// Get all sessions.
    pub fn get_sessions(&self) -> Vec<&DesktopSession> {
        self.sessions.values().collect()
    }

    /// Add a custom session.
    pub fn add_session(&mut self, session: DesktopSession) {
        self.sessions.insert(session.name.clone(), session);
    }

    /// Remove a session.
    pub fn remove_session(&mut self, name: &str) -> Result<(), DesktopError> {
        if self.sessions.contains_key(name) {
            self.sessions.remove(name);
            Ok(())
        } else {
            Err(DesktopError::DesktopEnvironmentNotFound(name.to_string()))
        }
    }

    /// Set display manager.
    pub fn set_display_manager(&mut self, dm: &str) {
        self.display_manager = dm.to_string();
    }

    /// Get display manager.
    pub fn get_display_manager(&self) -> &str {
        &self.display_manager
    }

    /// Enable autologin.
    pub fn enable_autologin(&mut self, user: &str) {
        self.autologin = true;
        self.autologin_user = Some(user.to_string());
    }

    /// Disable autologin.
    pub fn disable_autologin(&mut self) {
        self.autologin = false;
        self.autologin_user = None;
    }

    /// Generate display manager configuration.
    pub fn generate_display_manager_config(&self) -> String {
        let mut config = String::new();
        config.push_str(&format!("# Display Manager: {}\n", self.display_manager));
        config.push_str(&format!("DisplayManager={}\n", self.display_manager));
        
        if self.autologin {
            if let Some(user) = &self.autologin_user {
                config.push_str(&format!("AutomaticLoginEnable=true\n"));
                config.push_str(&format!("AutomaticLogin={}\n", user));
            }
        }
        
        config
    }

    /// Generate session configuration.
    pub fn generate_session_config(&self) -> String {
        let mut config = String::new();
        config.push_str("[Desktop]\n");
        
        if let Some(session) = self.get_current_session() {
            config.push_str(&format!("Session={}\n", session.name));
            config.push_str(&format!("DesktopEnvironment={}\n", session.desktop_environment.as_str()));
            config.push_str(&format!("DisplayServer={}\n", session.display_server.as_str()));
            config.push_str(&format!("WindowManager={}\n", session.window_manager.as_str()));
        }
        
        config
    }

    /// Generate X11 configuration.
    pub fn generate_x11_config(&self) -> String {
        let mut config = String::new();
        config.push_str("Section \"ServerLayout\"\n");
        config.push_str("    Identifier     \"Layout0\"\n");
        config.push_str("    Screen      0  \"Screen0\" 0 0\n");
        config.push_str("EndSection\n\n");
        
        config.push_str("Section \"Screen\"\n");
        config.push_str("    Identifier     \"Screen0\"\n");
        config.push_str("    Device         \"Card0\"\n");
        config.push_str("    DefaultDepth    24\n");
        config.push_str("    SubSection     \"Display\"\n");
        config.push_str("        Depth       24\n");
        config.push_str("    EndSubSection\n");
        config.push_str("EndSection\n\n");
        
        config.push_str("Section \"Device\"\n");
        config.push_str("    Identifier     \"Card0\"\n");
        config.push_str("    Driver         \"modesetting\"\n");
        config.push_str("EndSection\n");
        
        config
    }

    /// Generate Wayland configuration.
    pub fn generate_wayland_config(&self) -> String {
        let mut config = String::new();
        config.push_str("# Wayland Configuration\n");
        config.push_str(&format!("WLR_BACKENDS=drm\n"));
        config.push_str(&format!("XDG_SESSION_TYPE=wayland\n"));
        
        if let Some(session) = self.get_current_session() {
            config.push_str(&format!("XDG_CURRENT_DESKTOP={}\n", session.desktop_environment.as_str()));
        }
        
        config
    }
}

impl Default for DesktopManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_desktop_environment() {
        let mut mgr = DesktopManager::new();
        mgr.set_desktop_environment("kde").unwrap();
        assert_eq!(mgr.get_current_session().unwrap().desktop_environment, DesktopEnvironment::Kde);
    }

    #[test]
    fn test_set_display_server() {
        let mut mgr = DesktopManager::new();
        mgr.set_display_server("x11").unwrap();
        assert_eq!(mgr.get_current_session().unwrap().display_server, DisplayServer::X11);
    }

    #[test]
    fn test_set_window_manager() {
        let mut mgr = DesktopManager::new();
        mgr.set_window_manager("i3").unwrap();
        assert_eq!(mgr.get_current_session().unwrap().window_manager, WindowManager::I3);
    }

    #[test]
    fn test_get_sessions() {
        let mgr = DesktopManager::new();
        let sessions = mgr.get_sessions();
        assert!(!sessions.is_empty());
    }

    #[test]
    fn test_add_session() {
        let mut mgr = DesktopManager::new();
        let session = DesktopSession::new("custom", DesktopEnvironment::Custom, DisplayServer::X11, WindowManager::Custom, "custom-session");
        mgr.add_session(session);
        assert!(mgr.get_sessions().len() > 5);
    }

    #[test]
    fn test_generate_display_manager_config() {
        let mut mgr = DesktopManager::new();
        mgr.enable_autologin("user");
        let config = mgr.generate_display_manager_config();
        assert!(config.contains("AutomaticLogin"));
        assert!(config.contains("user"));
    }

    #[test]
    fn test_generate_session_config() {
        let mgr = DesktopManager::new();
        let config = mgr.generate_session_config();
        assert!(config.contains("Session="));
        assert!(config.contains("DesktopEnvironment="));
    }

    #[test]
    fn test_desktop_environment_parse() {
        assert_eq!(DesktopEnvironment::parse("gnome").unwrap(), DesktopEnvironment::Gnome);
        assert_eq!(DesktopEnvironment::parse("kde").unwrap(), DesktopEnvironment::Kde);
        assert!(DesktopEnvironment::parse("invalid").is_err());
    }
}
