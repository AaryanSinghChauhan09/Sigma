//! Interface segregation for fine-grained interfaces
//! Follows principles: Interface Segregation, Abstraction, Composition Over Inheritance

pub trait Reader { fn read(&self) -> String; }
pub trait Writer { fn write(&self, data: &str); }
pub trait Serializer { fn serialize(&self, data: &str) -> String; }

pub struct TextReader;
impl Reader for TextReader { fn read(&self) -> String { "text".to_string() } }

pub struct TextWriter;
impl Writer for TextWriter { fn write(&self, data: &str) { println!("Writing: {}", data); } }

#[cfg(test)] mod tests { use super::*; #[test] fn test_interfaces() { let _ = TextReader; let _ = TextWriter; } }
