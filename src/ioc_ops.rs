//! Inversion of Control container
//! Follows principles: Inversion of Control, Dependency Inversion, Composition Over Inheritance

pub struct IocContainer;

impl IocContainer {
    pub fn new() -> Self { Self }
    pub fn resolve<T: Default>() -> T { T::default() }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = IocContainer::new(); } }
