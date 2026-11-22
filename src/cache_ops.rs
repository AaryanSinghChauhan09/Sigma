//! SigmaOS cache operations module.
//!
//! This module provides cache-conscious operations for efficient data access.
//!
//! Follows principles: Cache Conscious Design, Locality of Reference, Mechanical Sympathy
//!
//! # Example
//!
//! ```
//! use sigma_system::cache_ops::*;
//!
//! let cache = CacheOps::new();
//! cache.set("key", "value");
//! let value = cache.get("key");
//! ```

use std::collections::HashMap;
use std::sync::Mutex;

/// Cache operations manager.
#[derive(Debug)]
pub struct CacheOps {
    cache: Mutex<HashMap<String, String>>,
}

impl CacheOps {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Set value in cache.
    pub fn set(&self, key: &str, value: &str) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key.to_string(), value.to_string());
    }

    /// Get value from cache.
    pub fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }

    /// Remove value from cache.
    pub fn remove(&self, key: &str) {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(key);
    }

    /// Clear cache.
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}

impl Default for CacheOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cache = CacheOps::new();
        let _ = cache;
    }

    #[test]
    fn test_set_get() {
        let cache = CacheOps::new();
        cache.set("key", "value");
        assert_eq!(cache.get("key"), Some("value".to_string()));
    }
}
