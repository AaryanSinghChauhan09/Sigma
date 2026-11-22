//! Abstraction for hiding implementation details
//! Follows principles: Abstraction, Information Hiding, Encapsulation

pub struct Abstraction {
    internal_state: i32,
}

impl Abstraction {
    pub fn new() -> Self { Self { internal_state: 0 } }
    pub fn operation(&self) -> String { "result".to_string() }
    pub fn state(&self) -> i32 { self.internal_state }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = Abstraction::new(); } }
