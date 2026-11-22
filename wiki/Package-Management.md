# SigmaOS Package Management Module

## Overview
Dependency resolver for SigmaOS component ecosystem inspired by Arch Linux pacman.

## Features
- Semantic versioning with constraints
- Dependency resolution with topological sort
- Circular dependency detection
- Version conflict detection
- Package install/uninstall tracking
- Makepkg config generation

## Usage

```rust
use sigma_system::{PackageManager, Package, PackageVersion};

let mut mgr = PackageManager::new();
mgr.add_package(Package::new("app", PackageVersion::new(1,0,0))
    .with_dependency("base", "1.0"));
let order = mgr.install("app").unwrap();
```
