//! Sigma System Core Library
//! Principles: Single Responsibility, High Cohesion, Low Coupling, Defensive Programming

pub mod adjustments;
pub mod persona;

pub use adjustments::{SystemAdjuster, GovernorMode};
pub use persona::{PersonaManager, SystemPersona};
