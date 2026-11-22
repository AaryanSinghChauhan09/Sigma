# SigmaOS Logging System

## Overview
Structured logging for all SigmaOS components inspired by Linux syslog and audit logging.

## Features
- Log levels: Error, Warning, Info, Debug, Trace
- Structured log entries with context
- Syslog and audit config generation
- JSON export
- Configurable retention

## Usage

```rust
use sigma_system::{Logger, LogLevel};

let mut logger = Logger::new("sigmaos-core")
    .unwrap()
    .with_min_level(LogLevel::Info);

logger.info("System initialized").unwrap();
logger.error("Critical failure").unwrap();
```

## API
- `Logger::new(component)`: Create logger
- `log(level, message)`: Log message
- `error/warn/info/debug/trace()`: Convenience methods
- `get_entries_by_level()`: Filter by level
- `generate_syslog_config()`: Syslog config
- `export_json()`: JSON export
