//! SigmaOS observer pattern module.
//!
//! Implements Observer design pattern for event notification.
//! Follows principles: Design Pattern, Loose Coupling, Composition Over Inheritance

pub trait ObserverTrait {
    fn update(&self, event: &str);
}

pub struct Observer;

impl Observer {
    pub fn new() -> Self { Self }
}

impl ObserverTrait for Observer {
    fn update(&self, event: &str) {
        println!("Observer received: {}", event);
    }
}

pub struct Subject {
    observers: Vec<Box<dyn ObserverTrait>>,
}

impl Subject {
    pub fn new() -> Self { Self { observers: Vec::new() } }
    
    pub fn attach(&mut self, observer: Box<dyn ObserverTrait>) {
        self.observers.push(observer);
    }
    
    pub fn notify(&self, event: &str) {
        for observer in &self.observers {
            observer.update(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Subject::new();
    }
}
