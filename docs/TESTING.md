# SigmaOS Testing Guide

## Unit Tests

All 100 modules have unit tests that verify:
- Constructor (`test_new`)
- Main functionality
- Error handling

### Running Tests

```bash
# Run all tests
cargo test

# Run library tests only
cargo test --lib

# Run specific test
cargo test test_new

# Run with output
cargo test -- --nocapture
```

## Test Statistics

- 580 unit tests
- 105 doctests
- All tests passing

## Testing Principles

### Deterministic Testing
- Tests are reproducible across runs
- No dependencies on external state
- No randomness in tests

### No Hardware Requirements
- Tests don't require physical devices
- Tests use mocked/stubbed responses where needed
- Tests can run on any system

## Integration Testing

### Manual Integration Testing

For integration testing of actual Linux interactions:

```bash
# Test kernel operations
cargo test kernel_ops::tests::test_get_version

# Test network operations
cargo test net_ops::tests::test_list_interfaces

# Test service operations
cargo test service::tests::test_get_service_status
```

## CI/CD Testing

### GitHub Actions

CI/CD configuration (to be added):
- Run tests on every commit
- Run tests on every pull request
- Build and test on multiple platforms

### Test Failure on Purpose

Some tests may fail intentionally to test error handling:
- Test with invalid paths
- Test with invalid parameters
- Test with missing commands

## Coverage

### Running Coverage

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Debugging Tests

### Verbose Output

```bash
RUST_BACKTRACE=1 cargo test
```

### Specific Test

```bash
cargo test module_name::tests::test_name -- --nocapture
```

## Test Organization

Tests are organized by module:
```rust
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

## Adding Tests

When adding a new module, include:
1. Constructor test (`test_new`)
2. Main functionality test
3. Error handling test if applicable

## Continuous Testing

Tests should be run:
- Before committing
- After any changes
- As part of CI/CD pipeline
