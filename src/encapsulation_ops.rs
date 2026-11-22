//! Encapsulation for data protection
//! Follows principles: Encapsulation, Information Hiding, Abstraction

pub struct Encapsulated {
    private_field: String,
}

impl Encapsulated {
    pub fn new(value: &str) -> Self { Self { private_field: value.to_string() } }
    pub fn get_value(&self) -> &str { &self.private_field }
    pub fn set_value(&mut self, value: &str) { self.private_field = value.to_string(); }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = Encapsulated::new("test"); } }
