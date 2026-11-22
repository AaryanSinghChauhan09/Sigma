# SigmaOS FAQ

## General Questions

### What is SigmaOS?

SigmaOS is a sovereign Linux distribution with a comprehensive Rust-based system management library (100 modules) and suite of command-line tools.

### Is SigmaOS based on Arch Linux?

SigmaOS uses Linux packages and follows a rolling release model, but has its own identity and comprehensive tooling.

### How do I install SigmaOS?

See the [Installation Guide](INSTALLATION.md) for detailed installation instructions.

### Is SigmaOS bootable?

The infrastructure for a bootable ISO exists, but an actual ISO has not been built and tested yet. This requires an Arch Linux system to build and test.

## Technical Questions

### How many modules does SigmaOS have?

SigmaOS has 100 modules covering all major Linux subsystems.

### What programming language is SigmaOS written in?

The core library is written in Rust. System scripts are written in Bash.

### Does SigmaOS follow SOLID principles?

Yes, all 100 modules follow SOLID principles (Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, Dependency Inversion).

### How do I use the SigmaOS library?

```rust
use sigma_system::cpu_ops::*;

let cpu = CpuOps::new();
let usage = cpu.get_cpu_usage()?;
```

## Development Questions

### How do I contribute?

See the [Contributing Guidelines](CONTRIBUTING.md).

### How do I run tests?

```bash
cargo test
```

### How do I build the library?

```bash
cargo build --release
```

## Security Questions

### Does SigmaOS have security auditing?

Yes, SigmaOS includes a security audit module to detect API key leaks, hardcoded secrets, and default passwords.

### How do I run a security audit?

```rust
use sigma_system::security_audit::*;

let auditor = SecurityAuditor::new();
let report = auditor.audit_codebase("/path/to/code")?;
```

## Distribution Questions

### Will SigmaOS have multiple repositories?

Yes, there is a comprehensive plan to split SigmaOS into 50+ modular repositories. See [Repository Restructuring Plan](REPOSITORY_RESTRUCTURING_PLAN.md).

### Will documentation be moved to GitHub Wiki?

Yes, there is a plan to migrate all documentation to GitHub Wikis. See [Wiki Migration Plan](WIKI_MIGRATION_PLAN.md).

## Support

### Where can I get help?

- Check the [Troubleshooting Guide](TROUBLESHOOTING.md)
- Check the [API Reference](API.md)
- Report issues on GitHub
