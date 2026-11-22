//! Polymorphism for type flexibility
//! Follows principles: Polymorphism, Liskov Substitution, Interface Segregation

pub trait Animal { fn speak(&self) -> &str; }

pub struct Dog;
impl Animal for Dog { fn speak(&self) -> &str { "Woof" } }

pub struct Cat;
impl Animal for Cat { fn speak(&self) -> &str { "Meow" } }

#[cfg(test)] mod tests { use super::*; #[test] fn test_animals() { let _ = Dog; let _ = Cat; } }
