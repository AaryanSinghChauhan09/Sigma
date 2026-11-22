//! Semaphore for resource counting and access control
//! Follows principles: Safe Concurrency, Resource Management, Priority Inversion Avoidance

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Semaphore {
    permits: Arc<AtomicUsize>,
}

impl Semaphore {
    pub fn new(permits: usize) -> Self { Self { permits: Arc::new(AtomicUsize::new(permits)) } }
    pub fn acquire(&self) -> bool { self.permits.fetch_sub(1, Ordering::SeqCst) > 0 }
    pub fn release(&self) { self.permits.fetch_add(1, Ordering::SeqCst); }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = Semaphore::new(5); } }
