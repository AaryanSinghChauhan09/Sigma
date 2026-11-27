//! SigmaOS event loop operations module.
//!
//! This module provides event loop operations for asynchronous event handling.
//!
//! Follows principles: Asynchronous I/O & Event Loops, Race-Free Design, Safe Concurrency
//!
//! # Example
//!
//! ```
//! use sigma_system::eventloop_ops::*;
//!
//! let eventloop = EventLoopOps::new();
//! eventloop.start();
//! assert!(eventloop.is_running());
//! eventloop.stop();
//! ```

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// Event loop operations manager.
#[derive(Debug)]
pub struct EventLoopOps {
    events: Arc<Mutex<VecDeque<String>>>,
    running: Arc<Mutex<bool>>,
}

impl EventLoopOps {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(VecDeque::new())),
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Add event to loop.
    pub fn add_event(&self, event: String) {
        let mut events = self.events.lock().unwrap();
        events.push_back(event);
    }

    /// Process next event.
    pub fn process_event(&self) -> Option<String> {
        let mut events = self.events.lock().unwrap();
        events.pop_front()
    }

    /// Start event loop.
    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        *running = true;
    }

    /// Stop event loop.
    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
    }

    /// Check if running.
    pub fn is_running(&self) -> bool {
        let running = self.running.lock().unwrap();
        *running
    }
}

impl Default for EventLoopOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let eventloop = EventLoopOps::new();
        let _ = eventloop;
    }

    #[test]
    fn test_add_event() {
        let eventloop = EventLoopOps::new();
        eventloop.add_event("test".to_string());
        assert_eq!(eventloop.process_event(), Some("test".to_string()));
    }
}
