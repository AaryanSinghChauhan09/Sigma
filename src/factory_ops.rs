//! Factory pattern module
pub trait Factory { fn create(&self) -> Box<dyn Product>; }
pub trait Product { fn use(&self); }
pub struct ConcreteFactory;
impl Factory for ConcreteFactory { fn create(&self) -> Box<dyn Product> { Box::new(ConcreteProduct) } }
pub struct ConcreteProduct;
impl Product for ConcreteProduct { fn use(&self) { println!("Using product"); } }
#[cfg(test)] mod tests { use super::*; #[test] fn test_factory() { let f = ConcreteFactory; let p = f.create(); p.use(); } }
