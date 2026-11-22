//! SigmaOS atomic operations module.
//!
//! This module provides atomic operations for thread-safe counters and flags.
//!
//! Follows principles: Atomic Operations, Race-Free Design, Safe Concurrency, Memory Safety
//!
//! # Example
//!
//! ```
//! use sigma_system::atomic_ops::*;
//!
//! let atomic = AtomicOps::new();
//! atomic.increment();
//! let value = atomic.get();
//! ```

use std::sync::atomic::{AtomicUsize, Ordering};

/// Atomic operations manager.
#[derive(Debug)]
pub struct AtomicOps {
    counter: AtomicUsize,
}

impl AtomicOps {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }

    /// Increment atomic counter.
    pub fn increment(&self) -> usize {
        self.counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Decrement atomic counter.
    pub fn decrement(&self) -> usize {
        self.counter.fetch_sub(1, Ordering::SeqCst)
    }

    /// Get atomic counter value.
    pub fn get(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }

    /// Set atomic counter value.
    pub fn set(&self, value: usize) {
        self.counter.store(value, Ordering::SeqCst);
    }
}

impl Default for AtomicOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let atomic = AtomicOps::new();
        let _ = atomic;
    }

    #[test]
    fn test_increment() {
        let atomic = AtomicOps::new();
        atomic.increment();
        assert_eq!(atomic.get(), 1);
    }
}
