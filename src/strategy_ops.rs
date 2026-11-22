//! SigmaOS strategy pattern module.
//!
//! This module implements the Strategy design pattern for interchangeable algorithms.
//!
//! Follows principles: Design Pattern, Composition Over Inheritance, Open-Closed
//!
//! # Example
//!
//! ```
//! use sigma_system::strategy_ops::*;
//!
//! let strategy = CompressionStrategy::new();
//! let result = strategy.execute("data");
//! ```

/// Strategy trait for interchangeable algorithms.
pub trait Strategy {
    fn execute(&self, input: &str) -> String;
}

/// Compression strategy.
pub struct CompressionStrategy;

impl Strategy for CompressionStrategy {
    fn execute(&self, input: &str) -> String {
        format!("Compressed: {}", input)
    }
}

/// Encryption strategy.
pub struct EncryptionStrategy;

impl Strategy for EncryptionStrategy {
    fn execute(&self, input: &str) -> String {
        format!("Encrypted: {}", input)
    }
}

/// Strategy operations manager.
pub struct StrategyOps {
    strategy: Box<dyn Strategy>,
}

impl StrategyOps {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Self { strategy }
    }

    pub fn execute(&self, input: &str) -> String {
        self.strategy.execute(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_strategy() {
        let strategy = CompressionStrategy;
        let result = strategy.execute("test");
        assert!(result.contains("Compressed"));
    }
}
