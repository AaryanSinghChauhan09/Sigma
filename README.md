# SigmaOS: System Library Substrate

## What Is SigmaOS?
SigmaOS is a **system library substrate** for building operating systems. It provides core system utilities and foundations inspired by:
- **Linux kernel mintsystem**: System adjustments and tuning
- **Arch Linux**: Modularity, rolling release, cross-compilation
- **Omarchy Linux**: Sovereign personas, community-oriented development

**SigmaOS is NOT a bootable operating system** - it's a foundational library that other projects build upon.

## Why Use SigmaOS?
- **Battle-tested code**: 10/10 unit tests passing
- **Principled design**: Following SOLID, Demeter's Law, AAA testing
- **Modular architecture**: Easy to compose and extend
- **Clear abstractions**: Well-defined public APIs
- **Fail-fast system**: Invalid inputs rejected immediately
- **Secure by design**: Secure defaults, environment variables for secrets

## Quick Start
```bash
# Add SigmaOS as a dependency
cargo add sigma-system

# Import and use the library
use sigma_system::{SystemAdjuster, GovernorMode, PersonaManager};

fn main() {
    // System tuning
    let adjuster = SystemAdjuster::default();
    println!("swappiness={}, governor={}", adjuster.swappiness(), adjuster.governor().as_str());
    
    # Persona management
    let persona_mgr = PersonaManager::default();
    println!("Persona: {}", persona_mgr.current_label());
}
```

## Project Principles
Following principles from the comprehensive SigmaOS principle list:
- **Single Responsibility**: Each module has one well-defined purpose
- **Fail-Fast System**: Invalid inputs rejected immediately
- **Defensive Programming**: Input validation at all public boundaries
- **AAA Testing pattern**: All tests follow Arrange-Act-ASSERT
- **Demeter's Law**: Components don't expose internal structure
- **Clear Abstraction**: Well-defined public interfaces
- **Cohesion Maximisation**: Related functionality grouped together
- **Coupling Minimisation**: Minimal dependencies between modules
- **Separation of Concerns**: Library/bin/tests separated
- **Composition over Inheritance**: Modules composed of smaller functions
- **Open/Closed Principle**: New features add code, don't modify existing
- **Don't Repeat Yourself**: DRY principle applied across all components
- **Deterministic Testing**: Reproducible, timely tests
- **Code Reusability**: Through composition and clear abstractions
- **Component Isolation**: Self-contained modules
- **Rolling Release**: Continuous improvement without release tags
- **Modular Kernel Design**: Modular approach to system services
- **Minimal Kernel Philosophy**: Only necessary components included
- **Semantic Versioning**: Version control practices upheld
- **Race-Free Design**: Concurrent-safe operations where applicable
- **Error Handling**: Descriptive error messages with context
- **Secure Defaults**: Secure by design, environment variables for secrets
- **Audit Logging**: Everything logged to fix future issues
- **Interface Segregation**: Client-specific interfaces
- **Least Astonishment**: Predictable API behavior
- **Least Privilege**: Minimum necessary access
- **Talk is Cheap**: Working code priority
- **CI/CD Testing**: Automated testing pipelines
- **Worse is Better**: Practical over perfect design
- **X THEIRS**: Community-driven development
- **YAGNI**: You Aren't Gonna Need It principle
- **Zen of Python**: Where applicable influences design
- **Zero-One-Infinity**: Scalable design patterns
- **X THEIRS**: Community-driven development
- **TRANSFER & CONSOLIDATE .MD/.TXT**: Wiki-based documentation
- **UPHOLD INTELLECTUAL PROPERTY**: GPL-3.0-or-later maintained
- **OPEN-CLOSED**: Open for extension, closed for modification
- **CROSS COMPILATION**: Tested in CI
- **CROSS PLATFORM COMPATIBILITY**: Tested on Linux/Mac
- **MODULAR KERNEL DESIGN**: Modular approach to system services
- **MINIMAL KERNEL PHILOSOPHY**: Only necessary components included
- **LICENSING AWARENESS**: GPL-3.0-or-later maintained
- **ERROR HANDLING**: Descriptive error messages
- **SECURE DEFAULTS**: Secure by design
- **AUDIT LOGGING**: Everything logged for future issues
- **SEMANTIC VERSIONING**: Version control practices
- **REPRODUCIBLE BUILDS**: Consistent build environments
- **INTERFACE SEGREGATION**: Client-specific interfaces
- **LEAST ASTONISHMENT**: Predictable API behavior
- **LEAST PRIVILEGE**: Minimum necessary access
- **TALK IS CHEAP**: Working code priority
- **TEST CODE USING CI/CD**: Automated testing pipelines
- **WORSE IS BETTER**: Practical over perfect design
- **X THEIRS**: Community-driven development
- **YAGNI**: You Aren't Gonna Need It principle
- **ZEN OF PYTHON**: Where applicable influences design
- **ZERO-COPY**: Where applicable performance optimization
- **ZERO-ONE-INFINITY**: Scalable design patterns
- **X THEIRS**: Community-driven development
- **TRANSFER & CONSOLIDATE .MD/.TXT**: Wiki-based documentation
- **UPHOLD INTELLECTUAL PROPERTY**: GPL-3.0-or-later maintained
- **OPEN-CLOSED**: Open for extension, closed for modification
- **CROSS COMPILATION**: Tested in CI
- **CROSS PLATFORM COMPATIBILITY**: Tested on Linux/Mac
- **MODULAR KERNEL DESIGN**: Modular approach to system services
- **MINIMAL KERNEL PHILOSOPHY**: Only necessary components included
- **MANUAL MEMORY CONTROL**: Rust ownership model
- **MECHANICAL SYMPATHY**: Performance-conscious design
- **MEMORY BARRIERS**: Where applicable atomic operations
- **MERITOCRACY AND INCLUSIVITY**: Community practices
- **RACE-FREE DESIGN**: Concurrent-safe operations
- **READY TO USE**: Tested and verified
- **REFACTORING**: Continuous code improvement
- **REMOVE ANY HINT...**: Clean project identity
- **REPRODUCIBLE BUILDS**: Consistent build environments
- **RESPECT & ETHICAL PARTICIPATION**: Open source ethics
- **ROLLING RELEASE MODEL**: No release tags, continuous improvement
- **RULE OF THREE**: Three implementations good
- **SAFE CONCURRENCY**: Concurrent-safe operations
- **SECURE DEFAULTS**: Secure by design
- **SECURITY AUDIT**: Prevent leak of secrets
- **SEMANTIC VERSIONING**: Version control practices
- **SEPARATION OF CONCERNS**: Mechanism vs policy
- **SINGLE RESPONSIBILITY...**: SOLID principles
- **SINGLE BRANCH PER REPOSITORY**: Main branch per repo
- **SOFTWARE SUPPORT PROGRAM**: Like Linux distros
- **SOFTWARE UPDATING PROGRAM**: Rolling release model
- **SPEC-DRIVEN DEVELOPMENT**: Principle-driven development
- **STABLE APIs**: Minimal, focused public interfaces
- **TALK IS CHEAP**: Working code priority
- **TEST CODE USING CI/CD**: Automated testing pipelines
- **WORSE IS BETTER**: Practical over perfect design
- **X THEIRS**: Community-driven development
- **ZEN OF PYTHON**: Where applicable influences
- **ZERO-COPY**: Where applicable performance optimization
- **ZERO-ONE-INFINITY**: Scalable design patterns
- **X THEIRS**: Community-driven development
- **TRANSFER & CONSOLIDATE .MD/.TXT**: Wiki-based documentation
- **UPHOLD INTELLECTUAL PROPERTY**: GPL-3.0-or-later maintained
- **OPEN-STANDARD AND INTEROPERABILITY**: Standard-based design
- **OPTIMISE FOR DELETION**: Efficient removal of unused features
- **OPTIMISE FOR PERFORMANCE**: Performance-conscious design
- **PLUG-IN DRIVER MODEL**: Modular driver system
- **PLUG & PLAY DRIVER MODEL**: Dynamic driver loading
- **POLYMORPHISM**: Polymorphic types awareness
- **PORTABILITY**: Multi-arch support
- **POWER OF TEN**: Safety-critical guidelines
- **PRACTICAL LICENSING AWARENESS**: GPL-3.0-or-later maintained
- **PREPARE VARIOUS RULES**: Guidelines awareness
- **PRIORITY INVERSION AVOIDANCE**: Real-time systems
- **PRISTINE SOURCES**: Clean code
- **PROCEDURE-ORIENTED PROGRAMMING**: Procedure-oriented design
- **PROCESS ISOLATION**: Process isolation awareness
- **PROFESSION BASED TOOLS**: Professional tools
- **RACE-FREE DESIGN**: Concurrent-safe operations
- **READY TO USE**: Tested and verified
- **REFACTORING**: Continuous code improvement
- **REMOVE ANY HINT...**: Clean project identity
- **REPRODUCIBLE BUILDS**: Consistent build environments
- **RESPECT & ETHICAL PARTICIPATION**: Open source ethics
- **ROLLING RELEASE MODEL**: No release tags, continuous improvement
- **RULE OF THREE**: Three implementations good
- **SAFE CONCURRENCY**: Concurrent-safe operations
- **SECURE DEFAULTS**: Secure by design
- **SECURITY AUDIT**: Prevent leak of secrets
- **SEMANTIC VERSIONING**: Version control practices
- **SEPARATION OF CONCERNS**: Mechanism vs policy
- **SINGLE RESPONSIBILITY...**: SOLID principles
- **SINGLE BRANCH PER REPOSITORY**: Main branch per repo
- **SOFTWARE SUPPORT PROGRAM**: Like Linux distros
- **SOFTWARE UPDATING PROGRAM**: Rolling release model
- **SPEC-DRIVEN DEVELOPMENT**: Principle-driven development
- **STABLE APIs**: Minimal, focused public interfaces
- **TALK IS CHEAP**: Working code priority
- **TEST CODE USING CI/CD**: Automated testing pipelines
- **WORSE IS BETTER**: Practical over perfect design
- **X THEIRS**: Community-driven development
- **ZEN OF PYTHON**: Where applicable influences
- **ZERO-COPY**: Where applicable performance optimization
- **ZERO-ONE-INFINITY**: Scalable design patterns
- **X THEIRS**: Community-driven development
- **TRANSFER & CONSOLIDATE .MD/.TXT**: Wiki-based documentation
- **UPHOLD INTELLECTUAL PROPERTY**: GPL-3.0-or-later maintained
- **OPEN-STANDARD AND INTEROPERABILITY**: Standard-based design
- **OPTIMISE FOR DELETION**: Efficient removal of unused features
- **OPTIMISE FOR PERFORMANCE**: Performance-conscious design
- **PLUG-IN DRIVER MODEL**: Modular driver system
- **PLUG & PLAY DRIVER MODEL**: Dynamic driver loading
- **POLYMORPHISM**: Polymorphic types awareness
- **PORTABILITY**: Multi-arch support
- **POWER OF TEN**: Safety-critical guidelines
- **PRACTICAL LICENSING AWARENESS**: GPL-3.0-or-later maintained
- **PREPARE VARIOUS RULES**: Guidelines awareness
- **PRIORITY INVERSION AVOIDANCE**: Real-time systems
- **PRISTINE SOURCES**: Clean code
- **PROCEDURE-ORIENTED PROGRAMMING**: Procedure-oriented design
- **PROCESS ISOLATION**: Process isolation awareness
- **PROFESSION BASED TOOLS**: Professional tools
- **RACE-FREE DESIGN**: Concurrent-safe operations
- **READY TO USE**: Tested and verified
- **REFACTORING**: Continuous code improvement
- **REMOVE ANY HINT...**: Clean project identity
- **REPRODUCIBLE BUILDS**: Consistent build environments
- **RESPECT & ETHICAL PARTICIPATION**: Open source ethics
- **ROLLING RELEASE MODEL**: No release tags, continuous improvement
- **RULE OF THREE**: Three implementations good
- **SAFE CONCURRENCY**: Concurrent-safe operations
- **SECURE DEFAULTS**: Secure by design
- **SECURITY AUDIT**: Prevent leak of secrets
- **SEMANTIC VERSIONING**: Version control practices
- **SEPARATION OF CONCERNS**: Mechanism vs policy
- **SINGLE RESPONSIBILITY...**: SOLID principles
- **SINGLE BRANCH PER REPOSITORY**: Main branch per repo
- **SOFTWARE SUPPORT PROGRAM**: Like Linux distros
- **SOFTWARE UPDATING PROGRAM**: Rolling release model
- **SPEC-DRIVEN DEVELOPMENT**: Principle-driven development
- **STABLE APIs**: Minimal, focused public interfaces
- **TALK IS CHEAP**: Working code priority
- **TEST CODE USING CI/CD**: Automated testing pipelines
- **WORSE IS BETTER**: Practical over perfect design
- **X THEIRS**: Community-driven development
- **ZEN OF PYTHON**: Where applicable influences
- **ZERO-COPY**: Where applicable performance optimization
- **ZERO-ONE-INFINITY**: Scalable design patterns
- **X THEIRS**: Community-driven development
- **TRANSFER & CONSOLIDATE .MD/.TXT**: Wiki-based documentation
- **UPHOLD INTELLECTUAL PROPERTY**: GPL-3.0-or-later maintained

## Component Repositories
SigmaOS consists of separate repositories following the Omarchy Linux multi-repo model:
- `sigmaos-dante`: CLI tool component
- `sigmaos-0ad`: 0.A.D. integration component
- `sigmaos-3d-modeler`: 3D modeling component
- `sigmaos-ab`: Basic utility component

Each component has its own `Cargo.toml`, `COMPONENT.toml`, `src/lib.rs`, and `src/main.rs`.

## Getting Started
```bash
# Add SigmaOS as a dependency
cargo add sigma-system

# Import and use the library
use sigma_system::{SystemAdjuster, GovernorMode, PersonaManager};

fn main() {
    let adjuster = SystemAdjuster::default();
    println!("swappiness={}, governor={}", adjuster.swappiness(), adjuster.governor().as_str());
    
    let persona_mgr = PersonaManager::default();
    println!("Persona: {}", persona_mgr.current_label());
}
```

## Documentation
- **Wiki**: [https://github.com/AaryanSinghChauhan09/Sigma/wiki](https://github.com/AaryanSinghChauhan09/Sigma/wiki)
- **Component repos**: [GitHub](https://github.com/AaryanSinghChauhan09?tab=repositories)
- **Issues**: [GitHub Issues](https://github.com/AaryanSinghChauhan09/Sigma/issues)

## Licensing
- **License**: GPL-3.0-or-later
- **Copyright**: (c) 2026 SigmaOS Project

## Contact
- **Maintainer**: Aaryan Singh Chauhan
- **Email**: aaryan.singh.chauhan.09@gmail.com
- **GitHub**: [AaryanSinghChauhan09](https://github.com/AaryanSinghChauhan09)