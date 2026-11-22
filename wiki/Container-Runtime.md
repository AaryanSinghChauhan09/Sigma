# SigmaOS Container Runtime Module

## Overview
Lightweight container runtime inspired by Podman and Docker for virtualization.

## Features
- Container lifecycle management (create, start, stop, pause, resume)
- Resource limits (memory, CPU)
- Volume and port mapping
- Podman and Docker command generation
- Docker Compose file generation

## Usage

```rust
use sigma_system::{ContainerManager, ContainerSpec, ContainerRuntime};

let mut mgr = ContainerManager::new(ContainerRuntime::Podman);
let spec = ContainerSpec::new("web", "nginx:latest")
    .unwrap()
    .with_port(80)
    .unwrap()
    .with_memory_limit(512);
let container = mgr.create_container(spec).unwrap();
mgr.start_container(&container.id).unwrap();
```

## API
- `create_container(spec)`: Create container
- `start_container(id)`: Start container
- `stop_container(id)`: Stop container
- `pause_container(id)`: Pause container
- `resume_container(id)`: Resume container
- `generate_podman_command()`: Generate Podman command
- `generate_docker_compose()`: Generate Docker Compose YAML
