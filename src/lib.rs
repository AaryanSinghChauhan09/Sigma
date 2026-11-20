//! Sigma System Core Library
//! Principles: Single Responsibility, High Cohesion, Low Coupling, Defensive Programming

#![forbid(unsafe_code)]

/// Modular system adjustments module.
/// Handles CPU governors, sysctl memory tuning, and hardware optimization.
/// Follows Single Responsibility Principle: only manages system adjustment parameters.
pub mod adjustments;

/// Modular persona management module.
/// Enables sovereign desktop personas and overlay environments.
/// Follows Single Responsibility Principle: only manages persona state.
pub mod persona;

/// Public re-exports for convenient access.
pub use adjustments::{SystemAdjuster, GovernorMode};
pub use persona::{PersonaManager, SystemPersona};