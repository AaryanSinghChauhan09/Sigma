//! Interrupt handling operations
//! Follows principles: Interrupt Handling Discipline, Low Level Control, Mechanical Sympathy

use std::process::Command;

pub struct InterruptOps;

impl InterruptOps {
    pub fn new() -> Self { Self }
    pub fn send_signal(&self, pid: u32, signal: i32) -> bool {
        Command::new("kill")
            .arg(format!("-{}", signal))
            .arg(pid.to_string())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = InterruptOps::new(); } }
