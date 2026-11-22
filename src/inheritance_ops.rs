//! Inheritance for code reuse (used appropriately)
//! Follows principles: Inheritance, Liskov Substitution, Code Reusability

pub trait Vehicle { fn speed(&self) -> u32; }

pub trait Car: Vehicle { fn doors(&self) -> u32; }

pub struct Sedan;
impl Vehicle for Sedan { fn speed(&self) -> u32 { 100 } }
impl Car for Sedan { fn doors(&self) -> u32 { 4 } }

#[cfg(test)] mod tests { use super::*; #[test] fn test_sedan() { let _ = Sedan; } }
