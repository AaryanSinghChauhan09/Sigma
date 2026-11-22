//! Sigma System Core Library
//! Principles: Single Responsibility, High Cohesion, Low Coupling, Defensive Programming,
//! Demeter's Law, Clear Abstraction, Fail-Fast System
//!
//! This library provides the core substrate for the SigmaOS ecosystem.
//! Inspired by Linux kernel mintsystem, Arch Linux modularity, and Omarchy Linux sovereignty.

#![forbid(unsafe_code)]

/// Modular system adjustments module.
/// Handles CPU governors, sysctl memory tuning, and hardware optimization.
/// Follows Single Responsibility Principle: only manages system adjustment parameters.
/// Applies Demeter's Law: internal state encapsulated, minimal public API.
pub mod adjustments;

/// Modular persona management module.
/// Enables sovereign desktop personas and overlay environments.
/// Follows Single Responsibility Principle: only manages persona state.
/// Applies Clear Abstraction: well-defined public interfaces for persona operations.
pub mod persona;

/// Public re-exports for convenient access following Demeter's Law.
pub use adjustments::{SystemAdjuster, GovernorMode};
pub use persona::{SystemPersona, PersonaManager};

/// Convenience function to create a PersonaManager with fail-fast validation
pub fn create_persona_manager(initial: SystemPersona) -> PersonaManager {
    PersonaManager {
        current: SystemPersona::validate(initial),
    }
}