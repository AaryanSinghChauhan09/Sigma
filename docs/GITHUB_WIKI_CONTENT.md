# GitHub Wiki Content for Manual Migration

This document contains the content for all GitHub Wiki pages that need to be created manually for the SigmaOS repository.

## Home.md

```markdown
# SigmaOS

SigmaOS is a sovereign Linux distribution with a comprehensive Rust-based system management library (100 modules) and suite of command-line tools. Built with modular architecture and community-oriented development, SigmaOS provides a complete operating system ecosystem with programmatic system management.

## Quick Start

### Using the Library

```rust
use sigma_system::cpu_ops::*;

let cpu = CpuOps::new();
let usage = cpu.get_cpu_usage()?;
```

### Installation

See [Installation](Installation.md) for detailed installation instructions.

## Features

- **100 modules** covering all major Linux subsystems
- **580 unit tests, 105 doctests**
- Real Linux interaction via system commands
- SOLID principles implementation
- Security audit module
- Comprehensive documentation

## Documentation

- [Installation Guide](Installation.md)
- [API Reference](API.md)
- [Development Guide](Development.md)
- [Contributing Guidelines](Contributing.md)
- [Testing Guide](Testing.md)
- [Security Guidelines](Security.md)
- [FAQ](FAQ.md)
- [Troubleshooting](Troubleshooting.md)
- [Architecture](Core-Library-Architecture.md)
- [Repository Restructuring](Repository-Restructuring.md)

## Project Status

- **Library**: 100 modules, 580 unit tests, 105 doctests
- **Tools**: 14 system tools
- **Packages**: 3 metapackages (base, kde, gnome)
- **Config**: pacman.conf, sysctl, GRUB, KDE configs
- **Scripts**: ISO build (improved), installer (improved), testing, pipeline

## Important: SigmaOS is NOT Yet Bootable

SigmaOS has complete infrastructure but requires a **Linux system** to build and test the actual ISO. See [Installation](Installation.md) for detailed steps.

## License

GPL-3.0-or-later

## Contact

- **Maintainer**: Aaryan Singh Chauhan
- **Email**: aaryan.singh.chauhan.09@gmail.com
- **GitHub**: [AaryanSinghChauhan09/Sigma](https://github.com/AaryanSinghChauhan09/Sigma)
```

## Installation.md

(Content from docs/INSTALLATION.md)

## API.md

(Content from docs/API.md)

## Development.md

(Content from docs/DEVELOPMENT.md)

## Contributing.md

(Content from docs/CONTRIBUTING.md)

## Testing.md

(Content from docs/TESTING.md)

## Security.md

(Content from docs/SECURITY.md)

## FAQ.md

(Content from docs/FAQ.md)

## Troubleshooting.md

(Content from docs/TROUBLESHOOTING.md)

## Core-Library-Architecture.md

(Content from docs/ARCHITECTURE.md)

## Repository-Restructuring.md

(Content from REPOSITORY_RESTRUCTURING_PLAN.md)

## Changelog.md

```markdown
# Changelog

## [Unreleased]

### Added
- 100 modules for system management
- Security audit module
- Core OS infrastructure modules (kernel, bootloader, initramfs, package manager, repository manager, system config)
- System management modules (service, network, update, storage, user, log, backup, audit, release)
- Comprehensive documentation
- Repository restructuring plan
- Wiki migration plan

### Changed
- Normalized SigmaOS identity (removed references to other projects)
- Updated README with 100 modules
- All modules follow SOLID principles

### Fixed
- Fixed Default implementations across all modules
- Fixed error handling consistency

## [0.5.0] - 2024

### Added
- Initial 86 modules
- ISO build scripts
- Installer script
- System tools (14 tools)
- Configuration files
```

## Migration Instructions

To migrate this content to GitHub Wiki:

1. Go to https://github.com/AaryanSinghChauhan09/Sigma/wiki
2. Click "Create Page"
3. For each page above:
   - Copy the content from this document
   - Paste into the wiki page
   - Save the page
4. Update the wiki sidebar with links to all pages

## Wiki Sidebar Suggested Order

- Home
- Installation
- API Reference
- Development
- Contributing
- Testing
- Security
- FAQ
- Troubleshooting
- Core Library Architecture
- Repository Restructuring
- Changelog

---

## Component Repositories Wiki Index

Every `SigmaOS-*` repository has a corresponding wiki. Pages follow the
standard template: Overview, Architecture, Principles, API, Usage, Testing,
Security, Configuration, Troubleshooting, Contributing, Changelog.

### Pushed component repos

- SigmaOS-app-env — Windows app compatibility (Bottles/CrossOver/Lutris/Proton/Wine inspired)
- SigmaOS-betteroffice — Diagrams, Documents, Spreadsheets, Slides
- SigmaOS-bi-analytics — Superset, Metabase, Redash, Grafana, Lightdash, PostHog
- SigmaOS-bigdata — Flink, Spark, DuckDB, Trino
- SigmaOS-browser — Brave, Chromium, Firefox, LibreWolf, Waterfox, Zen
- SigmaOS-craft — 3D, CAD, design, PDF, photo, sound, vector creative tools
- SigmaOS-data-wrangling — Airflow, NiFi, dbt, Pandas, Polars, Talend
- SigmaOS-dos — Free/MS DOS, BASIC, ReactOS compatibility
- SigmaOS-eda — Altair, Bokeh, ggplot2, Matplotlib, Seaborn, Plotly
- SigmaOS-email — Thunderbird, Mailspring
- SigmaOS-genoffice — Docs, HTML, Markdown, PDF, Sheet, Slides
- SigmaOS-iloveapi — iLoveAPI/iLoveIMG/iLovePDF/iLoveSign
- SigmaOS-ml-dl — PyTorch, scikit-learn, Statsmodels, TensorFlow
- SigmaOS-notebooks — Jupyter, Colab, KNIME
- SigmaOS-platform — GUI frameworks (GTK, Qt, Tauri, Electron, etc.)
- SigmaOS-rdbms — PostgreSQL, MySQL, MariaDB, SQLite, Oracle, DB2, SQL Server
- SigmaOS-skills — AI agent skills collection
- SigmaOS-plugin-marketplace, SigmaOS-plugin-registry — omacom marketplace pattern
- SigmaOS-theme-registry, SigmaOS-theme-marketplace — omacom theme pattern
- SigmaOS-configurator — first-boot wizard (omarchy-configurator inspired)
- SigmaOS-notifications — D-Bus notification center (omacom plugin inspired)
- SigmaOS-pkg-core — package format/repos (omarchy-pkgs inspired)
- SigmaOS-installer — guided + unattended installer (archinstall inspired)
- SigmaOS-rebuild — declarative config engine (NixOS inspired)
- SigmaOS-rollback — atomic generations/rollback (NixOS inspired)
- SigmaOS-zfs-integration — ZFS datasets/snapshots (FreeBSD inspired)
- SigmaOS-omarchy, SigmaOS-omarchy-pkgs, SigmaOS-omarchy-iso, SigmaOS-omarchy-ui — core Omarchy-pattern components

### Staged locally (pending repo-creation quota)

SigmaOS-sec-audit, SigmaOS-dkms, SigmaOS-live-media, SigmaOS-ports,
SigmaOS-wiki-hub, SigmaOS-local-ai, SigmaOS-speedtest, SigmaOS-site,
SigmaOS-mac-support, SigmaOS-shell-config, SigmaOS-omarchy-mac-pkgs

See `docs/OMARCHY_INSPIRATION.md` for the full omacom→SigmaOS mapping and
`docs/FUTURE_PLAN_DISTROS.md` for the Linux/BSD distro roadmap.
