//! Read-write lock for concurrent read/write access
//! Follows principles: Safe Concurrency, Race-Free Design, Priority Inversion Avoidance

use std::sync::RwLock;

pub struct RwLockOps<T> {
    data: RwLock<T>,
}

impl<T> RwLockOps<T> {
    pub fn new(data: T) -> Self { Self { data: RwLock::new(data) } }
    pub fn read<F, R>(&self, f: F) -> R where F: FnOnce(&T) -> R {
        let data = self.data.read().unwrap();
        f(&data)
    }
    pub fn write<F, R>(&self, f: F) -> R where F: FnOnce(&mut T) -> R {
        let mut data = self.data.write().unwrap();
        f(&mut data)
    }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = RwLockOps::new(42); } }
