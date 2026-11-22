# SigmaOS Contributing Guidelines

## How to Contribute

We welcome contributions to SigmaOS! Please follow these guidelines.

## Code of Conduct

- Be respectful and inclusive
- Focus on technical excellence
- Collaborate constructively
- Follow meritocracy and do-cracy principles

## Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure tests pass
5. Submit a pull request

## Code Standards

### Rust Code

- Follow Rust style guidelines
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write unit tests for new code
- Document public APIs

### Shell Scripts

- Use `set -e` for error handling
- Use shellcheck for linting
- Add comments for complex logic
- Test scripts before committing

## Testing

### Unit Tests

All modules must have unit tests:
```rust
#[test]
fn test_new() {
    let module = ModuleOps::new();
    let _ = module;
}
```

### Integration Tests

Add integration tests for complex workflows.

## Documentation

### Module Documentation

Each module must have:
- Module-level doc comment
- Example usage
- Error documentation

### API Documentation

Document all public functions and types.

## Pull Request Process

1. Describe your changes
2. Link to related issues
3. Ensure CI passes
4. Request review from maintainers

## Security

- Report security issues privately
- Don't commit secrets
- Use security audit module
- Follow secure defaults

## License

By contributing, you agree to license your contributions under GPL-3.0-or-later.
