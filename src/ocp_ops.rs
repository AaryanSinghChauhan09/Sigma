//! Open-Closed Principle for extensible systems
//! Follows principles: Open-Closed, Polymorphism, Extensibility

pub trait Renderer { fn render(&self, data: &str); }

pub struct TextRenderer;
impl Renderer for TextRenderer { fn render(&self, data: &str) { println!("Text: {}", data); } }

pub struct HtmlRenderer;
impl Renderer for HtmlRenderer { fn render(&self, data: &str) { println!("<html>{}</html>", data); } }

#[cfg(test)] mod tests { use super::*; #[test] fn test_renderers() { let _ = TextRenderer; let _ = HtmlRenderer; } }
