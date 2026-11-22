//! Liskov Substitution Principle for substitutable types
//! Follows principles: Liskov Substitution, Polymorphism, Interface Segregation

pub trait Shape { fn area(&self) -> f64; }

pub struct Rectangle { width: f64, height: f64 }
impl Rectangle { pub fn new(w: f64, h: f64) -> Self { Self { width: w, height: h } } }
impl Shape for Rectangle { fn area(&self) -> f64 { self.width * self.height } }

pub struct Circle { radius: f64 }
impl Circle { pub fn new(r: f64) -> Self { Self { radius: r } } }
impl Shape for Circle { fn area(&self) -> f64 { 3.14159 * self.radius * self.radius } }

#[cfg(test)] mod tests { use super::*; #[test] fn test_shapes() { let _ = Rectangle::new(1.0, 2.0); let _ = Circle::new(1.0); } }
