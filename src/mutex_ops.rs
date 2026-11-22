//! Mutex operations for thread synchronization
//! Follows principles: Safe Concurrency, Race-Free Design, Deadlock Avoidance

use std::sync::Mutex;

pub struct MutexOps<T> {
    data: Mutex<T>,
}

impl<T> MutexOps<T> {
    pub fn new(data: T) -> Self { Self { data: Mutex::new(data) } }
    pub fn with_lock<F, R>(&self, f: F) -> R where F: FnOnce(&T) -> R {
        let data = self.data.lock().unwrap();
        f(&data)
    }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = MutexOps::new(42); } }
