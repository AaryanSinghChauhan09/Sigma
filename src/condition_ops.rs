//! Condition variable for thread coordination
//! Follows principles: Safe Concurrency, Deadlock Avoidance, Thread Coordination

use std::sync::{Arc, Mutex, Condvar};

pub struct ConditionOps {
    data: Arc<(Mutex<bool>, Condvar)>,
}

impl ConditionOps {
    pub fn new() -> Self { Self { data: Arc::new((Mutex::new(false), Condvar::new())) } }
    pub fn wait(&self) { let (lock, cvar) = &*self.data; let _guard = cvar.wait(lock.lock().unwrap()); }
    pub fn notify(&self) { let (_, cvar) = &*self.data; cvar.notify_one(); }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = ConditionOps::new(); } }
