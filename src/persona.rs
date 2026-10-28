//! Persona Management inspired by Omarchy Linux and Linux Mint
//! Enables sovereign desktop personas and overlay environments.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemPersona {
    SovereignOmarchy,
    LinuxMintClassic,
    ArchMinimal,
}

impl SystemPersona {
    pub fn label(&self) -> &'static str {
        match self {
            Self::SovereignOmarchy => "Omarchy Linux Sovereign",
            Self::LinuxMintClassic => "Linux Mint Cinnamon Classic",
            Self::ArchMinimal => "Arch Linux Minimalist",
        }
    }
}

pub struct PersonaManager {
    current: SystemPersona,
}

impl Default for PersonaManager {
    fn default() -> Self {
        Self {
            current: SystemPersona::SovereignOmarchy,
        }
    }
}

impl PersonaManager {
    pub fn new(initial: SystemPersona) -> Self {
        Self { current: initial }
    }

    pub fn current(&self) -> SystemPersona {
        self.current
    }

    pub fn switch_persona(&mut self, next: SystemPersona) {
        self.current = next;
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
        assert_eq!(mgr.current().label(), "Linux Mint Cinnamon Classic");
    }
}
