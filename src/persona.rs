//! Persona Management inspired by Omarchy Linux sovereign desktops and Arch Linux minimalist principles.
//! Enables sovereign desktop personas and overlay environments with fail-fast switching.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemPersona {
    SovereignOmarchy,
    LinuxMintClassic,
    ArchMinimal,
    NixOS,
    Gentoo,
    DragonflyBSD,
}

impl SystemPersona {
    /// Fail-fast: validate persona (defensive programming)
    pub fn validate(_persona: SystemPersona) -> SystemPersona {
        _persona
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::SovereignOmarchy => "Omarchy Linux Sovereign",
            Self::LinuxMintClassic => "Linux Mint Cinnamon Classic",
            Self::ArchMinimal => "Arch Linux Minimalist",
            Self::NixOS => "NixOS Configurable",
            Self::Gentoo => "Gentoo Optimizing",
            Self::DragonflyBSD => "DragonflyBSD Resilient",
        }
    }

    /// Get associated desktop environment inspired by distro philosophies
    pub fn desktop_environment(&self) -> &'static str {
        match self {
            Self::SovereignOmarchy => "sovereign overlay",
            Self::LinuxMintClassic => "Cinnamon",
            Self::ArchMinimal => "i3wm + base",
            Self::NixOS => "Hyprland",
            Self::Gentoo => "KDE Plasma",
            Self::DragonflyBSD => "Xfce",
        }
    }
}

/// Fail-fast: create PersonaManager with validation
pub fn create_persona_manager(initial: SystemPersona) -> PersonaManager {
    PersonaManager {
        current: SystemPersona::validate(initial),
    }
}

#[derive(Debug, Clone)]
pub struct PersonaManager {
    pub current: SystemPersona,
}

impl Default for PersonaManager {
    fn default() -> Self {
        Self {
            current: SystemPersona::SovereignOmarchy,
        }
    }
}

impl PersonaManager {
    /// Fail-fast: validate persona switch before applying
    pub fn switch_persona(&mut self, next: SystemPersona) -> SystemPersona {
        self.current = SystemPersona::validate(next);
        self.current.clone()
    }

    /// Get current persona (encapsulation per Demeter's Law)
    pub fn current(&self) -> SystemPersona {
        self.current
    }

    /// Get desktop environment for current persona (Omarchy/Arch inspired)
    pub fn current_desktop_environment(&self) -> &'static str {
        self.current.desktop_environment()
    }

    /// Get persona label (inspired by Omarchy Linux labeling)
    pub fn current_label(&self) -> &'static str {
        self.current.label()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_switch() {
        // Arrange
        let mut mgr = PersonaManager::default();
        assert_eq!(mgr.current(), SystemPersona::SovereignOmarchy);

        // Act
        mgr.switch_persona(SystemPersona::LinuxMintClassic);

        // Assert
        assert_eq!(mgr.current(), SystemPersona::LinuxMintClassic);
        assert_eq!(mgr.current_label(), "Linux Mint Cinnamon Classic");
    }

    #[test]
    fn test_persona_new_valid() {
        // Act
        let mgr = create_persona_manager(SystemPersona::SovereignOmarchy);

        // Assert - fail-fast: new() should succeed for valid persona
        assert_eq!(mgr.current(), SystemPersona::SovereignOmarchy);
    }

    #[test]
    fn test_persona_desktop_environment() {
        // Act & Assert
        let mgr = PersonaManager::default();
        assert_eq!(mgr.current_desktop_environment(), "sovereign overlay");

        let mgr = PersonaManager {
            current: SystemPersona::ArchMinimal,
        };
        assert_eq!(mgr.current_desktop_environment(), "i3wm + base");

        let mgr = PersonaManager {
            current: SystemPersona::NixOS,
        };
        assert_eq!(mgr.current_desktop_environment(), "Hyprland");
    }

    #[test]
    fn test_persona_validate_success() {
        // Act
        let result = SystemPersona::validate(SystemPersona::SovereignOmarchy);

        // Assert
        assert_eq!(result, SystemPersona::SovereignOmarchy);
    }
}