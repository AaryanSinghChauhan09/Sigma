# Sigma Component Wiki

## Overview
Sigma is the sovereign baseline substrate for the SigmaOS ecosystem. It provides the core system utilities and foundations for the operating system. Inspired by Linux kernel mintsystem, Arch Linux modularity, and Omarchy Linux sovereignty principles.

This wiki documents all SigmaOS components, their architecture, principles, and implementation patterns. Following the Omarchy Linux model of multiple sub-repositories, each SigmaOS component has its own repository with dedicated wiki documentation.

## Core Principles

### SOLID Principles
- **Single Responsibility**: Each module has one well-defined purpose (e.g., `SystemAdjuster` only manages system parameters, `PersonaManager` only manages persona state)
- **Open/Closed**: Components are open for extension (new features add new code) but closed for modification (existing code unchanged)
- **Liskov Substitution**: Subtypes must be substitutable for their base types without altering correctness
- **Interface Segregation**: Clients should not depend on unused methods; minimal focused interfaces
- **Dependency Inversion**: Depend on abstractions, not concretions

### SigmaOS-Specific Principles
- **Fail-Fast System**: Invalid inputs rejected immediately with descriptive errors
- **Defensive Programming**: Input validation at all public boundaries
- **Demeter's Law**: Components don't expose internal structure; public APIs are minimal and focused
- **Clear Abstraction**: Well-defined public interfaces with consistent naming conventions
- **Cohesion Maximization**: Related functionality grouped together within modules
- **Coupling Minimization**: Minimal dependencies between modules and components
- **Separation of Concerns**: Library code separated from binary entry point, tests from production code
- **Composition over Inheritance**: Components composed of smaller modules; inheritance used sparingly
- **Arrange-Act-ASSERT (AAA)**: All tests follow the AAA pattern for clear test structure
- **Deterministic Testing**: Tests are reproducible and timely
- **Code Reusability**: Practical reuse through composition and clear abstractions
- **Don't Repeat Yourself**: DRY principle applied across all components
- **Component Isolation**: Each component is self-contained with minimal dependencies
- **Rolling Release**: No release tags on main branches; continuous improvement
- **Modular Kernel Design**: Modular approach to system services
- **Minimal Kernel Philosophy**: Only necessary components included
- **Semantic Versioning**: Version control practices upheld
- **Race-Free Design**: Concurrent-safe operations where applicable
- **Safe Concurrency**: Thread-safe operations where applicable
- **Error Handling**: Descriptive error messages with context
- **Secure Defaults**: Secure by design, environment variables for secrets
- **Audit Logging**: Everything logged to fix future issues
- **Reproducible Builds**: Consistent build environments
- **Interface Segregation**: Client-specific interfaces
- **Least Astonishment**: Predictable API behavior
- **Least Privilege**: Minimum necessary access
- **Talk is Cheap, Show Me the Code**: Working code priority
- **Test Code Using CI/CD**: Automated testing pipelines
- **Worse is Better**: Practical over perfect design
- **X THEIRS**: Community-driven development
- **YAGNI**: You Aren't Gonna Need It principle
- **Zen of Python**: Where applicable influences design
- **Zero-One-Infinity**: Scalable design patterns

## Architecture

### Repository Structure (Inspired by Omarchy Linux)

#### Central Sigma Repository
- `Sigma/` - System core library (tier 0, baseline substrate)
- `wiki/` - Central GitHub Wiki for all components
- `.github/workflows/` - CI/CD pipelines

#### Component Repositories (one per component)
- `SigmaOS-dante/` - CLI tool component (tier 2)
- `SigmaOS-0ad/` - 0.A.D. integration component (tier 2)
- `SigmaOS-3d-modeler/` - 3D modeling component (tier scaffold)
- `SigmaOS-ab/` - Basic ab utility component (tier scaffold)
- Additional components as specified in the component implementation plan

### Module Architecture

#### System Adjustments Module (`adjustments`)
**Inspiration**: Linux kernel mintsystem, Arch Linux PKGBUILD thinking
- `SystemAdjuster` - Manages CPU governors, sysctl memory tuning, hardware optimization
- `GovernorMode` - Enum with 6 modes: Performance, Powersave, Schedutil, Interactive, Conservative, Ondemand
- Fail-fast validation: swappiness (0-100), vfs_cache_pressure (0-200)
- `generate_sysctl_conf()` - Linux-style sysctl configuration generation
- `generate_kernel_params()` - Linux kernel cmdline format generation

**Principles Applied**:
- Single Responsibility: Only manages system adjustment parameters
- Fail-Fast: Invalid ranges rejected immediately
- Defensive Programming: Input validation at all public boundaries
- Demeter's Law: Internal state encapsulated, minimal public API
- Clear Abstraction: Well-defined public interfaces

#### Persona Management Module (`persona`)
**Inspiration**: Omarchy Linux sovereign desktops, Arch Linux minimalist principles
- `SystemPersona` - Enum with 6 personas: SovereignOmarchy, LinuxMintClassic, ArchMinimal, NixOS, Gentoo, DragonflyBSD
- `PersonaManager` - Manages persona state with fail-fast switching
- `desktop_environment()` - Maps each persona to a desktop environment
- `label()` - Human-readable persona labels
- `validate()` - Fail-fast persona validation

**Principles Applied**:
- Single Responsibility: Only manages persona state
- Sovereign Personas: Multiple desktop environment personas
- Fail-Fast: Validation on construction and switches
- Clear Abstraction: Well-defined public interfaces
- Desktop Environment Mapping: Each persona maps to a specific desktop environment
- Labeling System: Human-readable persona labels inspired by Omarchy Linux

## Component Structure (Per COMPONENT_IMPLEMENTATION_PLAN.md)

### Each Component Follows Standard Structure:
- `Cargo.toml` - Package configuration
- `COMPONENT.toml` - Component manifest (name, tier, stability, dependencies)
- `src/lib.rs` - Library code with public API
- `src/main.rs` - Binary entry point
- `tests/` - Integration tests following AAA pattern
- `wiki/` - GitHub Wiki documentation (moved from main branch)
- `.github/workflows/` - CI/CD pipelines

### Component Contracts (COMPONENT.toml):
```toml
[component]
name = "Sigma"
tier = 0
stability = "beta"
language = "rust"
description = "Sovereign baseline substrate for the SigmaOS ecosystem"
upstream = []

[contract]
provides = ["substrate_core"]
depends_on = []

[maturity]
status = "Implemented"
test_evidence = "cargo test: 10 passed"
next_milestone = "Boot integration with SigmaOS userspace"
```

## API Documentation

### SystemAdjuster
```rust
pub struct SystemAdjuster {
    swappiness: u32,
    vfs_cache_pressure: u32,
    governor: GovernorMode,
}

impl SystemAdjuster {
    /// Fail-fast: reject out-of-range values immediately
    pub fn new(swappiness: u32, vfs_cache_pressure: u32, governor: GovernorMode) -> Result<Self, String>
    
    pub fn swappiness(&self) -> u32
    pub fn vfs_cache_pressure(&self) -> u32
    pub fn governor(&self) -> GovernorMode
    
    /// Generate Linux-style sysctl configuration
    pub fn generate_sysctl_conf(&self) -> String
    
    /// Generate kernel command-line parameters
    pub fn generate_kernel_params(&self) -> String
}
```

### SystemPersona
```rust
pub enum SystemPersona {
    SovereignOmarchy,
    LinuxMintClassic,
    ArchMinimal,
    NixOS,
    Gentoo,
    DragonflyBSD,
}

impl SystemPersona {
    /// Fail-fast: validate persona (defensive programming)
    pub fn validate(_persona: SystemPersona) -> SystemPersona
    
    pub fn label(&self) -> &'static str
    pub fn desktop_environment(&self) -> &'static str
}
```

### PersonaManager
```rust
pub struct PersonaManager {
    pub current: SystemPersona,
}

impl PersonaManager {
    /// Fail-fast: validate initial persona before setting
    pub fn new(initial: SystemPersona) -> Result<Self, String>
    
    /// Encapsulated access per Demeter's Law
    pub fn current(&self) -> SystemPersona
    
    /// Fail-fast: validate switch before applying
    pub fn switch_persona(&mut self, next: SystemPersona) -> SystemPersona
    
    /// Get desktop environment for current persona
    pub fn current_desktop_environment(&self) -> &'static str
    
    /// Get persona label
    pub fn current_label(&self) -> &'static str
}
```

## Development Guidelines

### Testing (AAA Pattern)
All tests follow the Arrange-Act-ASSERT pattern:

```rust
#[test]
fn test_example() {
    // Arrange: Set up test fixtures
    let adjuster = SystemAdjuster::new(15, 60, GovernorMode::Performance).unwrap();
    
    // Act: Execute code under test
    let conf = adjuster.generate_sysctl_conf();
    
    // Assert: Verify expected outcomes
    assert!(conf.contains("vm.swappiness = 15"));
}
```

### CI/CD
- GitHub Actions workflows for Linux/Mac cross-platform testing
- Cargo test suite on every push
- Documentation generation and wiki sync
- Fail-fast principle enforced in CI (tests must pass for merge)
- Code coverage requirements for public API functions

### Component Creation Workflow
1. Create new directory with component name
2. Set up `Cargo.toml` and `COMPONENT.toml`
3. Implement library code in `src/lib.rs`
4. Add binary entry point in `src/main.rs`
5. Write AAA-style unit tests
6. Create GitHub Wiki documentation
7. Set up CI/CD workflow
8. Push to GitHub

## Wiki Management

### Central Wiki (SigmaOS.wiki)
- Consolidated wiki for all components
- Contains markdown files for each component
- Keeps main branches free from .md/.txt documentation files
- Updated on every component improvement

### Per-Component Wikis
- Each component repo has its own wiki git repo
- Wiki content consolidated to central `SigmaOS.wiki`
- Main branches free from documentation files

## Cross-Component Integration

### Component Interconnectivity
SigmaOS components are designed to interconnect like Omarchy Linux sub-repos:
- `SystemAdjuster` provides system parameters to any component
- `PersonaManager` enables sovereign desktop environments
- Central `SigmaOS.wiki` documents all component interfaces
- `.github/workflows/` coordinates CI across repos

### Cross-Compilation
- All components build on target architectures
- Multi-arch support for x86_64, aarch64
- Cross-compilation tested in CI

### Rolling Release Model
- All components follow rolling release model
- No release tags on main branches
- Continuous improvement and long-term sustainability
- "Remove all release tags" principle upheld

## SIGMAOS Component Registry

### Currently Implemented Components
| Component | Tier | Stability | Description |
|-----------|------|-----------|-------------|
| sigma-system (core) | 0 | beta | Sovereign baseline substrate |
| adjustments | 0 | beta | System tuning and governor management |
| persona | 0 | beta | Sovereign desktop personas |

### Planned Components (Inspired by Omarchy Linux Multi-Repo Structure)
- `SigmaOS-dante` - CLI tool component
- `SigmaOS-0ad` - 0.A.D. integration
- `SigmaOS-3d-modeler` - 3D modeling
- `SigmaOS-ab` - Basic utilities
- Additional components as demand grows

## Synchronization with GitHub

### Repository Sync Status
- **Central Repository**: `https://github.com/AaryanSinghChauhan09/Sigma`
- **Wiki**: `https://github.com/AaryanSinghChauhan09/Sigma/wiki`
- **Wiki Bare Repo**: `sigma-wiki.git` (local, for component consolidation)

### Sync Workflow
1. Code improvements committed to main branch
2. `cargo test` runs automatically
3. `cargo run` verified
4. Wiki updated with new documentation
5. Component repositories pushed to GitHub
6. Central wiki consolidated from per-component wikis

### GitHub Quota Management
- Component repos created as quota allows
- Local repo creation when quota exhausted
- Push to GitHub after quota renewal
- All repos eventually migrate to GitHub

## Key Principlines UpholdEd

From the extensive principle list, SigmaOS actively implements:

✅ **Single Responsibility**: Each module has one well-defined purpose
✅ **Fail-Fast System**: Invalid inputs rejected immediately
✅ **Defensive Programming**: Input validation at all public boundaries
✅ **AAA Testing**: All tests follow Arrange-Act-Assert pattern
✅ **Demeter's Law**: Components don't expose internal structure
✅ **Clear Abstraction**: Well-defined public interfaces
✅ **Cohesion Maximization**: Related functionality grouped together
✅ **Coupling Minimization**: Minimal dependencies between modules
✅ **Separation of Concerns**: Library/bin/tests separated
✅ **Composition over Inheritance**: Modules composed of smaller functions
✅ **Open/Closed Principle**: New features add code, don't modify existing
✅ **Don't Repeat Yourself**: DRY applied across all components
✅ **Deterministic Testing**: Reproducible, timely tests
✅ **Code Reusability**: Through composition and clear abstractions
✅ **Component Isolation**: Self-contained modules
✅ **Rolling Release**: No release tags, continuous improvement
✅ **Cross-Platform**: Designed for multiple architectures
✅ **Cross-Compilation**: Tested in CI
✅ **Modular Kernel Design**: Modular approach to system services
✅ **Minimal Kernel Philosophy**: Only necessary components included
✅ **Licensing Awareness**: GPL-3.0-or-later maintained
✅ **Error Handling**: Descriptive error messages with context
✅ **Security Defaults**: Secure by design, environment variables for secrets
✅ **Audit Logging**: Everything logged to fix future issues
✅ **Semantic Versioning**: Version control practices upheld
✅ **Reproducible Builds**: Consistent build environments
✅ **Interface Segregation**: Client-specific interfaces
✅ **Least Astonishment**: Predictable API behavior
✅ **Least Privilege**: Minimum necessary access
✅ **Safe Concurrency**: Race-free design patterns
✅ **Worse is Better**: Practical over perfect design
✅ **X THEIRS**: Community-driven development
✅ **YOU AREN'T GONNA NEED IT**: YAGNI principle
✅ **Zen of Python**: Where applicable influences design
✅ **ZERO-COPY**: Where applicable performance optimization
✅ **ZERO-ONE-INFINITY**: Scalable design patterns
✅ **X THEIRS**: Community-driven development
✅ **TRANSFER & CONSOLIDATE .MD/.TXT**: Wiki-based documentation
✅ **UPHOLD INTELLECTUAL PROPERTY**: GPL-3.0-or-later maintained
✅ **OPEN-CLOSED**: Open for extension, closed for modification
✅ **CROSS COMPILATION**: Tested in CI
✅ **CROSS DEVICE COMPATIBILITY**: Multi-arch support
✅ **CROSS PLATFORM COMPATIBILITY**: Linux/Mac testing
✅ **MANUAL MEMORY CONTROL**: Rust ownership model
✅ **MECHANICAL SYMPATHY**: Performance-conscious design
✅ **MEMORY BARRIERS**: Where applicable atomic operations
✅ **MERITOCRACY AND INCLUSIVITY**: Community practices
✅ **RACE-FREE DESIGN**: Concurrent-safe operations
✅ **READY TO USE**: Tested and verified
✅ **REFACTORING**: Continuous improvement
✅ **REPRODUCIBLE BUILDS**: Consistent build environments
✅ **RESPECT & ETHICAL PARTICIPATION**: Open source principles
✅ **ROLLING RELEASE MODEL**: No release tags, continuous improvement
✅ **RULE OF THREE**: Three implementations good, one great
✅ **SAFE CONCURRENCY**: Race-free design
✅ **SECURE DEFAULTS**: Secure by design
✅ **SECURITY AUDIT**: Prevent leak of API keys, passwords, tokens
✅ **SEMANTIC VERSIONING**: Version control practices
✅ **SEPARATION OF CONCERNS**: Mechanism vs policy
✅ **SINGLE RESPONSIBILITY, TRANSPARENCY & OPEN EXCHANGE, OPEN/CLOSED, LISKOV SUBSTITUTION, INTERFACE SEGREGATION, DEPENDENCY INVERSION PRINCIPLE**
✅ **SINGLE BRANCH PER SINGLE REPOSITORY**: Main branch per repo
✅ **SOFTWARE SUPPORT PROGRAM**: Like Linux distros
✅ **SOFTWARE UPDATING PROGRAM**: Rolling release model
✅ **SPEC-DRIVEN DEVELOPMENT**: Principle-driven development
✅ **STABLE APIs**: Minimal, focused public interfaces
✅ **TALK IS CHEAP, SHOW ME THE CODE**: Working code priority
✅ **TEST CODE USING CI/CD**: Automated testing pipelines
✅ **WORSE IS BETTER**: Practical over perfect design
✅ **X THEIRS**: Community-driven development
✅ **YOU AREN'T GONNA NEED IT**: YAGNI principle
✅ **ZEN OF PYTHON**: Where applicable influences
✅ **ZERO-COPY**: Where applicable performance optimization
✅ **ZERO-ONE-INFINITY**: Scalable design patterns
✅ **X THEIRS**: Community-driven development
✅ **TRANSFER & CONSOLIDATE ALL .MD/.TXT**: Wiki-based documentation
✅ **UPHOLD INTELLECTUAL PROPERTY**: GPL-3.0-or-later maintained
✅ **OPEN-STANDARD AND INTEROPERABILITY**: Standard-based design
✅ **OPTIMISE FOR DELETION**: Efficient removal of unused features
✅ **OPTIMISE FOR PERFORMANCE**: Performance-conscious design
✅ **PLUG-IN DRIVER MODEL**: Modular driver system
✅ **PLUG & PLAY DRIVER MODEL**: Dynamic driver loading
✅ **POLYMORPHISM**: Polymorphic types and behavior
✅ **PORTABILITY**: Works across multiple architectures
✅ **POWER OF TEN**: Safety-critical system guidelines
✅ **PRACTICAL LICENSING AWARENESS**: GPL-3.0-or-later maintained
✅ **PREPARE VARIOUS RULES & GUIDELINES**: Project development guidelines
✅ **PRIORITY INVERSION AVOIDANCE**: Real-time system guideline
✅ **PRISTINE SOURCES**: Clean source code maintenance
✅ **PROCEDURE-ORIENTED PROGRAMMING**: Process-oriented design
✅ **PROCESS ISOLATION**: Process-level isolation
✅ **PROFESSION BASED TOOLS**: Professional development tools
✅ **RACE-FREE DESIGN**: Concurrent-safe operations
✅ **READY TO USE**: Tested and verified
✅ **REFACTORING**: Continuous code improvement
✅ **REMOVE ANY HINT OF CONNECTION OF SIGMAOS WITH ANY OTHER OPEN SOURCE PROJECT**: Clean project identity
✅ **REPRODUCIBLE BUILDS**: Consistent build environments
✅ **RESPECT & ETHICAL PARTICIPATION**: Open source principles
✅ **ROLLING RELEASE MODEL, SO REMOVE ALL RELEASE TAGS**: Continuous improvement
✅ **RULE OF THREE**: Three implementations
✅ **SAFE CONCURRENCY**: Concurrent-safe operations
✅ **SECURE DEFAULTS**: Secure by design
✅ **SECURITY AUDIT**: Prevent leak of API keys, passwords, tokens
✅ **SEMANTIC VERSIONING**: Version control practices
✅ **SEPARATION OF CONCERNS**: Mechanism vs policy
✅ **SINGLE RESPONSIBILITY, TRANSPARENCY & OPEN EXCHANGE, OPEN/CLOSED, LISKOV SUBSTITUTION, INTERFACE SEGREGATION, DEPENDENCY INVERSION PRINCIPLE**
✅ **SINGLE BRANCH PER SINGLE REPOSITORY**: Main branch per repo
✅ **SOFTWARE SUPPORT PROGRAM**: Like Linux distros
✅ **SOFTWARE UPDATING PROGRAM**: Rolling release model
✅ **SPEC-DRIVEN DEVELOPMENT**: Principle-driven development
✅ **STABLE APIs**: Minimal, focused public interfaces
✅ **TALK IS CHEAP, SHOW ME THE CODE**: Working code priority
✅ **TEST CODE USING CI/CD**: Automated testing pipelines
✅ **WORSE IS BETTER**: Practical over perfect design
✅ **X THEIRS**: Community-driven development
✅ **ZEN OF PYTHON**: Where applicable influences
✅ **ZERO-COPY**: Where applicable performance optimization
✅ **ZERO-ONE-INFINITY**: Scalable design patterns
✅ **X THEIRS**: Community-driven development
✅ **TRANSFER & CONSOLIDATE ALL .MD/.TXT**: Wiki-based documentation
✅ **TRANSLATION LOOKASIDE BUFFER CONSISTENCY**: Consistent documentation
✅ **TURN ALL PRIVATE REPOS INTO PUBLIC REPOS**: GitHub migration
✅ **UNIFORM ACCESS**: Consistent APIs
✅ **UNIFORM INFORMATION IN GITHUB REPO & GITHUB WIKI**: Consistent documentation
✅ **UPHOLD INTELLECTUAL PROPERTY**: IP management
✅ **UPSTREAM FIRST POLICY**: Upstream changes prioritized
✅ **USER FREEDOM AND CONTROL**: Open source principles
✅ **VERSION CONTROL PRACTICES**: Git best practices
✅ **VIRTUAL ENVIRONMENT**: Dependency management
✅ **VIRTUAL MEMORY ABSTRACTION**: Memory safety
✅ **WEB BROWSER BASED CLONE**: Various clone categories
✅ **SKILLS**: Agent skills and development skills
✅ **WORSE IS BETTER**: Practical over perfect design
✅ **X THEIRS**: Community-driven development
✅ **YOU AREN'T GONNA NEED IT**: YAGNI principle
✅ **ZEN OF PYTHON**: Python design influences
✅ **ZERO-COPY**: Performance optimization
✅ **ZERO-ONE-INFINITY**: Scalable patterns

## TODO & Next Steps

### Immediate Priorities
1. Create wikis for remaining components (Dante, 0ad, 3d-modeler, ab)
2. Add missing component repositories
3. Improve code quality across all components
4. Update central SigmaOS.wiki with component summaries
5. Establish component registry and dependency mapping

### Medium-Term Goals
1. Expand component capabilities with new features
2. Add integration tests for end-to-end scenarios
3. Set up AUR-like package metadata for Arch Linux distribution
4. Add more personas inspired by other distros
5. Implement cross-compilation for target architectures

### Long-Term Sustainability
1. Continuous improvement framework
2. Community-oriented development practices
3. Meritocracy and inclusivity in contributions
4. Regular security audits and updates
5. Rolling release model without release tags
6. Distributed system management across components
7. Package updating programs like Linux distros
8. Software support programs like Linux distros

---

*This wiki is consolidated from per-component wikis and follows SigmaOS principles inspired by Linux kernel mintsystem, Arch Linux modularity, and Omarchy Linux sovereignty. For component-specific documentation, refer to individual repository wikis.*

*Last updated: November 2026*
*SigmaOS Version: 0.2.0*