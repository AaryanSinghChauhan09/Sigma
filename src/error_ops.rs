//! Comprehensive error handling operations
//! Follows principles: Error Handling, Fail-Fast System, Defensive Programming

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SigmaError {
    IoError(String),
    NetworkError(String),
    SecurityError(String),
    ConfigError(String),
}

impl fmt::Display for SigmaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SigmaError::IoError(e) => write!(f, "IO Error: {}", e),
            SigmaError::NetworkError(e) => write!(f, "Network Error: {}", e),
            SigmaError::SecurityError(e) => write!(f, "Security Error: {}", e),
            SigmaError::ConfigError(e) => write!(f, "Config Error: {}", e),
        }
    }
}

impl std::error::Error for SigmaError {}

pub struct ErrorOps;

impl ErrorOps {
    pub fn new() -> Self { Self }
    pub fn handle_error(&self, error: SigmaError) {
        eprintln!("Error: {}", error);
    }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = ErrorOps::new(); } }
