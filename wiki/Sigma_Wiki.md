# Sigma Component Wiki

## Overview
Sigma is the sovereign baseline substrate for the SigmaOS ecosystem. It provides the core system utilities and foundations for the operating system. Inspired by Linux kernel mintsystem, Arch Linux modularity, and Omarchy Linux sovereignty principles.

## Principles
- **Single Responsibility**: Each module has one well-defined purpose
- **High Cohesion**: Related functionality grouped together within modules
- **Low Coupling**: Minimal dependencies between modules
- **Defensive Programming**: Fail-fast validation at all public boundaries
- **Fail-Fast System**: Invalid inputs rejected immediately with descriptive errors
- **Component Isolation**: Each component is self-contained
- **Stable API**: Public interfaces are minimal, focused, and version-stable

## Architecture
Sigma follows a modular architecture with clearly defined responsibilities:

### System Adjustments (Linux kernel & mintsystem inspired)
- **CPU Governor Modes**: performance, powersave, schedutil, interactive, conservative, ondemand
- **Sysctl Tuning**: vm.swappiness, vm.vfs_cache_pressure parameters
- **Fail-fast Validation**: Range validation (0-100 for swappiness, 0-200 for vfs_cache_pressure)
- **Kernel Params Generation**: Inspired by Linux kernel command-line parameters

### Persona Management (Omarchy Linux & Arch Linux inspired)
- **Sovereign Personas**: Multiple desktop environment personas
- **Persona Validation**: Fail-fast on construction and switches
- **Desktop Environment Mapping**: Each persona maps to a desktop environment
- **Labeling System**: Human-readable persona labels

#### Available Personas
- **SovereignOmarchy** → "Omarchy Linux Sovereign" → sovereign overlay
- **LinuxMintClassic** → "Linux Mint Cinnamon Classic" → Cinnamon
- **ArchMinimal** → "Arch Linux Minimalist" → i3wm + base
- **NixOS** → "NixOS Configurable" → Hyprland
- **Gentoo** → "Gentoo Optimizing" → KDE Plasma
- **DragonflyBSD** → "DragonflyBSD Resilient" → Xfce

## API
The component provides the following public APIs:

### SystemAdjuster
- `new(swappiness, vfs_cache_pressure, governor)` → Result<Self, String>
  - Fail-fast: validates ranges immediately
- `swappiness(&self) -> u32`
- `vfs_cache_pressure(&self) -> u32`
- `governor(&self) -> GovernorMode`
- `generate_sysctl_conf(&self) -> String`
  - Linux-style sysctl configuration
- `generate_kernel_params(&self) -> String`
  - Inspired by Linux kernel cmdline format

### SystemPersona
- `label(&self) -> &'static str`
- `desktop_environment(&self) -> &'static str`
- `validate(_persona) -> SystemPersona` (fail-fast)

### PersonaManager
- `new(initial) -> Result<Self, String>`
  - Fail-fast: validates initial persona
- `current(&self) -> SystemPersona` (encapsulated access)
- `switch_persona(&mut self, next) -> SystemPersona`
  - Fail-fast: validates switch before applying
- `current_desktop_environment(&self) -> &'static str`
- `current_label(&self) -> &'static str`

## Development
- All code follows defensive programming principles
- Unit tests follow AAA (Arrange-Act-Assert) pattern
- Fail-fast on invalid input at all public boundaries
- No unsafe code allowed (`#![forbid(unsafe_code)]`)
- 100% test coverage for public API functions

## Tests
All tests follow the AAA pattern:

### System Adjustment Tests
- `test_system_adjuster_defaults` - Verify default values (swappiness=10, vfs_cache_pressure=50, governor=schedutil)
- `test_system_adjuster_valid_construction` - Test valid parameter construction
- `test_system_adjuster_fail_fast_swappiness` - Reject swappiness > 100
- `test_system_adjuster_fail_fast_vfs_cache_pressure` - Reject vfs_cache_pressure > 200
- `test_generate_sysctl_conf` - Generate proper sysctl config string
- `test_generate_kernel_params` - Generate kernel parameter string

### Persona Management Tests
- `test_persona_switch` - Switch between personas with AAA pattern
- `test_persona_new_valid` - Fail-fast validation on construction
- `test_persona_desktop_environment` - Test desktop environment mapping
- `test_persona_validate_success` - Test persona validation function

## CI/CD
- GitHub Actions workflows for Linux/Mac cross-platform testing
- Cargo test suite on every push
- Documentation generation and wiki sync
- Fail-fast principle enforced in CI (tests must pass for merge)

## TODO
- Add integration tests for end-to-end scenarios
- Expand governor mode documentation with Linux kernel references
- Create component-specific wiki pages for Dante, 0ad, 3d-modeler, etc.
- Set up AUR-like package metadata for Arch Linux distribution
- Add more personas inspired by other distros (CachyOS, ArcoLinux, etc.)