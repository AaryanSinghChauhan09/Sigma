# SigmaOS Memory Management Module

## Overview
The memory management module provides cgroups v2 control, memory pressure monitoring, and OOM prevention. It is inspired by Linux kernel cgroups and Arch Linux memory optimization.

## Features
- **Cgroups v2 Control**: Configure memory limits, swap limits, and OOM settings
- **Memory Pressure Monitoring**: Track memory pressure levels (Low/Medium/High/Critical)
- **OOM Prevention**: Get recommendations before system becomes unstable
- **Sysctl Generation**: Generate optimized sysctl memory tuning configurations

## Usage

```rust
use sigma_system::{MemoryManager, CgroupConfig, MemoryPressureLevel};

// Create a memory manager with cgroups
let config = CgroupConfig::new("sigmaos-app")
    .with_memory_limit(4 * 1024 * 1024 * 1024) // 4GB
    .unwrap();
let mut mgr = MemoryManager::new("/sys/fs/cgroup/sigmaos", config).unwrap();

// Update memory pressure based on current usage
mgr.total_memory_mb = 8192;
mgr.available_memory_mb = 2000;
mgr.update_pressure();

// Generate cgroups configuration
let cgroup_config = mgr.generate_cgroup_memory_config();
println!("{}", cgroup_config);

// Generate sysctl tuning
let sysctl = mgr.generate_sysctl_memory_tuning();
println!("{}", sysctl);
```

## Configuration

### CgroupConfig
- `memory_limit_bytes`: Maximum memory limit (0 = unlimited)
- `memory_swap_limit_bytes`: Maximum swap limit
- `oom_kill_disable`: Disable OOM killer
- `memory_pressure_notification`: Enable pressure notifications

### MemoryManager
- `cgroup_path`: Path to cgroup directory
- `config`: Cgroup configuration
- `current_pressure`: Current memory pressure level
- `total_memory_mb`: Total system memory in MB
- `available_memory_mb`: Available memory in MB

## API Reference

### MemoryPressureLevel
- `Low`: Usage < 50%
- `Medium`: Usage 50-70%
- `High`: Usage 70-90%
- `Critical`: Usage > 90%

### Methods
- `memory_usage_percentage()`: Calculate memory usage percentage
- `update_pressure()`: Update pressure level based on usage
- `should_trigger_oom_prevention()`: Check if OOM prevention is needed
- `get_recommendation()`: Get system recommendations

## Examples

### Setting Memory Limits
```rust
let config = CgroupConfig::new("database")
    .with_memory_limit(8 * 1024 * 1024 * 1024) // 8GB
    .with_swap_limit(2 * 1024 * 1024 * 1024) // 2GB swap
    .disable_oom_kill()
    .unwrap();
```

### Monitoring Memory Pressure
```rust
mgr.total_memory_mb = 16384;
mgr.available_memory_mb = 4000;
mgr.update_pressure();
println!("Pressure: {}", mgr.current_pressure.as_str());
println!("Recommendation: {}", mgr.get_recommendation());
```

## Integration with Other Modules
The memory module integrates with:
- **Power module**: Auto-optimizes based on battery level
- **Monitoring module**: Reports memory metrics
- **Init module**: Configures cgroups for services

## Principles Applied
- Single Responsibility: Only manages memory subsystem
- Fail-Fast: Validates limits before applying
- Clear Abstraction: Well-defined public API
- AAA Testing: All tests follow Arrange-Act-Assert pattern
