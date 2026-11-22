# SigmaOS Operating System Wiki

## Overview
SigmaOS is a bootable operating system inspired by Arch Linux and Omarchy Linux principles. It provides a complete desktop experience with a focus on modularity, sovereignty, and user freedom.

## Core Principles

### Arch Linux Inspired:
- Rolling Release Model, Cross Compilation, Cross Platform Compatibility
- Minimal Kernel Philosophy, Modular Kernel Design, Modularity
- Semantic Versioning, Race-Free Design, Secure Defaults
- Error Handling, Audit Logging, Reproducible Builds
- Interface Segregation, Least Astonishment, Least Privilege
- Talk is Cheap, CI/CD Testing, Worse is Better
- X THEIRS, YAGNI, Zen of Python, Zero-One-Infinity

### Omarchy Linux Inspired:
- Sovereign Desktop Personas, Community-Oriented Development
- Minimal Kernel Philosophy, Modular Kernel Design, Modularity
- Gall's Law, Computer-Oriented Development, Meritocracy and Inclusivity
- Free and Open Source Software, Offensive Programming
- Talk is Cheap, Rule of Three, Zen of Python
- You Aren't Gonna Need It, Zero-One-Infinity, THEIRS

### SOLID Principles:
- Single Responsibility, Open/Closed, Liskov Substitution
- Interface Segregation, Dependency Inversion

### SigmaOS-Specific Principles:
- Fail-Fast System, Defensive Programming, AAA Testing pattern
- Demeter's Law, Clear Abstraction, Cohesion Maximisation
- Coupling Minimisation, Separation of Concerns
- Composition over Inheritance, Open/Closed Principle
- DRY principle, Deterministic Testing, Code Reusability
- Component Isolation, Rolling Release Model
- Modular Kernel Design, Minimal Kernel Philosophy
- Semantic Versioning, Race-Free Design
- Error Handling, Secure Defaults, Audit Logging
- Interface Segregation, Least Astonishment, Least Privilege
- Working code priority, CI/CD Testing, Worse is Better
- X THEIRS, YAGNI principles, Zen of Python influences
- Zero-One-Infinity scalable design patterns

## Repository Structure

### SigmaOS Operating System Structure:
- sigmaos-core: Core system library (tier 0)
- sigmaos-iso: Live ISO and installation
- sigma-packages: Package definitions and metadata
- sigma-desktop: Desktop environment (Hyprland-based)
- sigma-installer: Installation workflow
- sigma-dotfiles: Official configuration and defaults
- sigma-hardware: Hardware support and profiles
- sigma-maintainer-tools: CI scripts, ISO builders, package validation

### Component Repositories:
- sigmaos-dante: CLI tool component (tier 2)
- sigmaos-0ad: 0.A.D. integration component (tier 2)
- sigmaos-3d-modeler: 3D modeling component (tier scaffold)
- sigmaos-ab: Basic utility component (tier scaffold)

### Core OS Components:
- sigmaos-core: System library and substrate (tier 0, beta)
- sigmaos-adjustments: System tuning and governor management (tier 0, beta)
- sigmaos-persona: Sovereign desktop personas (tier 0, beta)

## OS Architecture

### System Design:
- Read-Only Base System, Atomic Updates
- Declarative Services, Reproducible ISO Builds
- First-Class Recovery Mode, Hardware Compatibility Database
- Shell-First Workflow, Telemetry-Free Diagnostics

### Desktop Environment:
- Hyprland-Based: Inspired by Omarchy's focused experience
- Essential Shortcuts: Documented and accessible
- Welcome/Setup Flow: Guided onboarding experience
- Profiles: sigma-minimal, sigma-desktop, sigma-developer, sigma-creative, sigma-gaming, sigma-server

### Package Management:
- Pacman Integration: Standard pacman commands
- Package Sources: Official repositories and AUR
- Release Notes: Published with each release
- Checksums: SHA256 checksums for ISO verification
- Snapshot/Rollback Support: Where feasible

### Security:
- Secure Defaults: Secure by design
- No Telemetry: Privacy-focused by default
- Hardware Compatibility: Database of tested configurations
- Recovery Mode: Repair packages and reset configuration
- Consent-Based Data Collection: Only with explicit permission

### Release Cycle:
- Rolling Release: Continuous improvement without release tags
- Release Cadence: Regular releases with clear announcements
- Support Window: Defined support window for each release
- Compatibility Promises: Supported architectures and versions
- First-Class Recovery: Repair packages, reset configuration, or restore snapshots

## Getting Started

### For New Users:
1. Build the ISO: `./scripts/build-iso.sh` (or download pre-built ISO)
2. Boot the ISO: In QEMU or on physical hardware
3. Live Desktop: Connect to network, try the desktop environment
4. Run Installer: Graphical installer workflow
5. Reboot: Into installed system
6. Configure: Use profiles and settings to customize

### Desktop Experience:
- Essential Shortcuts: Documented and accessible
- Welcome/Setup Flow: Guided onboarding experience
- Profiles: sigma-minimal, sigma-desktop, sigma-developer, sigma-creative, sigma-gaming, sigma-server
- Theme: GTK, terminal, desktop, icon, and accent colors

### Package Management:
- Pacman Integration: Standard pacman commands
- Package Sources: Official repositories and AUR
- Release Notes: Published with each release
- Checksums: SHA256 checksums for ISO verification
- Snapshot/Rollback Support: Where feasible

### Security:
- Secure Defaults: Secure by design
- No Telemetry: Privacy-focused by default
- Hardware Compatibility: Database of tested configurations
- Recovery Mode: Repair packages and reset configuration
- Consent-Based Data Collection: Only with explicit permission

### Release Cycle:
- Rolling Release: Continuous improvement without release tags
- Release Cadence: Regular releases with clear announcements
- Support Window: Defined support window for each release
- Compatibility Promises: Supported architectures and versions
- First-Class Recovery: Repair packages, reset configuration, or restore snapshots

## Component Details

### sigmaos-dante:
CLI tool component for SigmaOS
- tier: 2
- status: scaffold

### sigmaos-0ad:
0.A.D. integration component
- tier: 2
- status: scaffold

### sigmaos-3d-modeler:
3D modeling component
- tier: 2
- status: scaffold

### sigmaos-ab:
Basic utility component
- tier: 2
- status: scaffold

## Versioning

- SigmaOS Version: 0.1.0 (initial alpha)
- Versioning Scheme: Semantic versioning (MAJOR.MINOR.PATCH)
- Release Tags: Used for released versions only
- Development Version: Development versions on main branch

## Community

- Contributing: Guidelines for contributions
- Issue Tracker: Bug reports and feature requests
- Discussions: Community discussions and Q&A
- Mailing List: Community mailing list (if applicable)
- Chat/IRC: Real-time chat (if applicable)
- Social Media: Official social media channels (if applicable)

## Licensing

- License: GPL-3.0-or-later
- Copyright: Copyright (c) 2026 SigmaOS Project
- License File: LICENSE file in root directory

## Contact

- Maintainer: SigmaOS Project
- Email: maintainer@sigmaos.example (placeholder)
- GitHub: https://github.com/AaryanSinghChauhan09/Sigma
- Discussions: https://github.com/AaryanSinghChauhan09/Sigma/discussions

---

*This wiki documents the SigmaOS operating system inspired by Arch Linux and Omarchy Linux principles.*
*Last updated: [DATE]*
*SigmaOS Version: 0.1.0 (initial alpha)*
*Based on Arch Linux and Omarchy Linux principles*