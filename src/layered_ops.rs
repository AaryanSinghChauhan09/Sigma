//! Layered design for system architecture
//! Follows principles: Layered Design, Separation of Concerns, Modularity

pub mod presentation { pub fn present(data: &str) { println!("Presenting: {}", data); } }
pub mod business { pub fn process(data: &str) -> String { format!("Processed: {}", data) } }
pub mod data { pub fn persist(data: &str) { println!("Persisting: {}", data); } }

#[cfg(test)] mod tests { use super::*; #[test] fn test_layers() { presentation::present("test"); business::process("test"); data::persist("test"); } }
