//! Dependency injection for loose coupling
//! Follows principles: Inversion of Control, Dependency Inversion, Loose Coupling

use std::collections::HashMap;

pub struct DependencyContainer {
    dependencies: HashMap<String, Box<dyn std::any::Any>>,
}

impl DependencyContainer {
    pub fn new() -> Self { Self { dependencies: HashMap::new() } }
    pub fn register<T: 'static>(&mut self, key: &str, value: T) { self.dependencies.insert(key.to_string(), Box::new(value)); }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = DependencyContainer::new(); } }
