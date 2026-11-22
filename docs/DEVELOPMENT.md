# SigmaOS Development Guide

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo
- Git
- Linux system (for testing)

### Clone Repository

```bash
git clone https://github.com/AaryanSinghChauhan09/Sigma
cd Sigma
```

### Build Library

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

## Project Structure

```
Sigma/
├── src/              # Source code (100 modules)
├── cli/              # CLI tool
├── scripts/          # Build and install scripts
├── config/           # Configuration files
├── docs/             # Documentation
├── archiso-profiles/ # ISO build profiles
└── Cargo.toml        # Workspace configuration
```

## Adding a New Module

### 1. Create Module File

```rust
//! SigmaOS module_name module.
//!
//! This module provides...
//!
//! # Example
//!
//! ```
//! use sigma_system::module_name::*;
//!
//! let module = ModuleOps::new();
//! ```

use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleError {
    CommandError(String),
    IoError(String),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::CommandError(msg) => write!(f, "Command error: {}", msg),
            ModuleError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ModuleError {}

#[derive(Debug)]
pub struct ModuleOps;

impl ModuleOps {
    pub fn new() -> Self { Self }
    
    pub fn do_something(&self) -> Result<String, ModuleError> {
        let output = Command::new("command")
            .output()
            .map_err(|e| ModuleError::CommandError(format!("Failed: {}", e)))?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

impl Default for ModuleOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = ModuleOps::new();
        let _ = module;
    }
}
```

### 2. Add to lib.rs

```rust
/// Module name module.
/// Provides...
pub mod module_name;

pub use module_name::{ModuleOps, ModuleError as ModuleManagerError};
```

### 3. Run Tests

```bash
cargo test
```

### 4. Commit and Push

```bash
git add -A
git commit -m "Add module_name module"
git push origin main
```

## SOLID Principles

### Single Responsibility
Each module handles one specific concern.

### Open/Closed
Modules are open for extension, closed for modification.

### Liskov Substitution
Subtypes are substitutable for base types.

### Interface Segregation
Minimal, focused public APIs.

### Dependency Inversion
Depend on abstractions, not concretions.

## Testing

### Unit Tests

Each module has unit tests:
```rust
#[test]
fn test_new() {
    let module = ModuleOps::new();
    let _ = module;
}
```

### Run All Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test test_new
```

## Documentation

### Module Documentation

Each module has:
- Module-level doc comment
- Example usage
- Error documentation

### API Documentation

Run:
```bash
cargo doc --open
```

## Code Style

### Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Release Process

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create git tag
4. Push to GitHub
5. Create GitHub Release

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## License

GPL-3.0-or-later
