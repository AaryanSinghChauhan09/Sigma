//! Resource pool for managing reusable resources
//! Follows principles: Object Pooling, Resource Management, Efficiency

use std::collections::VecDeque;

pub struct ResourcePool<T> {
    available: VecDeque<T>,
}

impl<T> ResourcePool<T> {
    pub fn new() -> Self { Self { available: VecDeque::new() } }
    pub fn add(&mut self, resource: T) { self.available.push_back(resource); }
    pub fn get(&mut self) -> Option<T> { self.available.pop_front() }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = ResourcePool::<i32>::new(); } }
