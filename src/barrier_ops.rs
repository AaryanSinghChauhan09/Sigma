//! Barrier for thread synchronization
//! Follows principles: Safe Concurrency, Memory Barriers, Race-Free Design

use std::sync::Barrier;

pub struct BarrierOps {
    barrier: Barrier,
}

impl BarrierOps {
    pub fn new(n: usize) -> Self { Self { barrier: Barrier::new(n) } }
    pub fn wait(&self) { self.barrier.wait(); }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = BarrierOps::new(2); } }
