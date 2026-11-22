//! Service discovery for finding available services
//! Follows principles: Discoverability, Distributed System Management, Dynamic Loadable Modules

use std::collections::HashMap;

pub struct DiscoveryOps {
    services: HashMap<String, String>,
}

impl DiscoveryOps {
    pub fn new() -> Self { Self { services: HashMap::new() } }
    pub fn register(&mut self, name: &str, endpoint: &str) { self.services.insert(name.to_string(), endpoint.to_string()); }
    pub fn discover(&self, name: &str) -> Option<&String> { self.services.get(name) }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = DiscoveryOps::new(); } }
