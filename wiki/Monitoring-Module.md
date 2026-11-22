# SigmaOS Monitoring Module

## Overview
The monitoring module provides system metrics, performance monitoring, and health checks. It is inspired by Linux kernel metrics and Arch Linux monitoring tools like Prometheus and Grafana.

## Features
- **System Metrics**: CPU, Memory, Disk, Network, Temperature, Load Average
- **Health Evaluation**: Threshold-based health status (Healthy/Warning/Critical)
- **Metric Retention**: Configurable metric retention with automatic cleanup
- **Configuration Generation**: Generate Prometheus and Grafana configurations
- **Health Reports**: Generate comprehensive system health reports

## Usage

```rust
use sigma_system::{MonitoringManager, SystemMetric, MetricType, Threshold};

// Create a monitoring manager
let mut mgr = MonitoringManager::new()
    .with_retention(3600); // 1 hour retention

// Set custom thresholds
let cpu_threshold = Threshold::new(70.0, 90.0).unwrap();
mgr.set_threshold(MetricType::CpuUsage, cpu_threshold).unwrap();

// Add metrics
let cpu_metric = SystemMetric::new(MetricType::CpuUsage, 45.5, "cpu0");
mgr.add_metric(cpu_metric);

let mem_metric = SystemMetric::new(MetricType::MemoryUsage, 60.2, "system");
mgr.add_metric(mem_metric);

// Evaluate health
let cpu_health = mgr.evaluate_health(MetricType::CpuUsage, 85.0);
let system_health = mgr.get_system_health();

// Generate configurations
let prometheus_config = mgr.generate_prometheus_config();
let grafana_dashboard = mgr.generate_grafana_dashboard();
let health_report = mgr.generate_health_report();
```

## Configuration

### MetricType
- `CpuUsage`: CPU usage percentage
- `MemoryUsage`: Memory usage percentage
- `DiskUsage`: Disk usage percentage
- `NetworkRx`: Network receive rate (bytes/s)
- `NetworkTx`: Network transmit rate (bytes/s)
- `Temperature`: Temperature in Celsius
- `LoadAverage`: System load average

### HealthStatus
- `Healthy`: All metrics within normal range
- `Warning`: Some metrics elevated
- `Critical`: Some metrics critical
- `Unknown`: Insufficient data

## API Reference

### Methods
- `add_metric()`: Add a metric sample
- `get_latest_metric()`: Get latest metric for a type/source
- `evaluate_health()`: Evaluate health for a value
- `get_system_health()`: Get overall system health
- `generate_prometheus_config()`: Generate Prometheus config
- `generate_grafana_dashboard()`: Generate Grafana dashboard JSON

## Integration
Integrates with Memory, Network, Power, and Filesystem modules for metric collection.
