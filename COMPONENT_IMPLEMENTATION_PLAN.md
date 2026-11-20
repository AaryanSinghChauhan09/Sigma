# SigmaOS Component Implementation Plan

## Overview
This document outlines the plan for implementing all SigmaOS components, applying GitHub Wiki principles, and ensuring proper repository structure.

## Component Structure
Each SigmaOS component follows a standardized structure:
- `Cargo.toml` - Package configuration
- `COMPONENT.toml` - Component manifest (name, tier, stability, dependencies)
- `src/lib.rs` - Library code with public API
- `src/main.rs` - Binary entry point
- `tests/` - Integration tests
- `wiki/` - GitHub Wiki documentation (moved from main branch)
- `.github/workflows/` - CI/CD pipelines

## Principles Applied

### 1. Single Responsibility Principle
- Each component has one well-defined purpose
- Internal structs have single responsibilities (e.g., `SystemAdjuster` only manages system parameters)
- Component handles only its specific domain

### 2. Fail-Fast System
- Invalid inputs rejected immediately
- `Tool::new()` fails on empty name
- `Component::init()` requires proper initialization
- All error conditions handled explicitly

### 3. Component Isolation
- Each component is self-contained
- Minimal dependencies between components
- `COMPONENT.toml` defines contracts (provides/depends_on)

### 4. Arrange-Act-ASSERT (AAA) Testing
All tests follow the AAA pattern:
```
// Arrange: Set up test fixtures
// Act: Execute code under test
// Assert: Verify expected outcomes
```

### 5. Demeter's Law / Law of Demeter
- Components don't expose internal structure
- Public APIs are minimal and focused
- Internal state is encapsulated

### 6. Clear Abstraction
- Well-defined public interfaces
- Documentation in GitHub Wikis
- Consistent naming conventions

### 7. Open/Closed Principle
- Components are open for extension (new features add new code)
- Components are closed for modification (existing code unchanged)
- New functionality added via new modules, not modifying existing ones

### 8. Composition over Inheritance
- Components composed of smaller modules
- Inheritance used sparingly, prefer composition
- Example: `PersonaManager` uses `SystemPersona` enum, not inheritance

### 9. Separation of Concerns
- Library code separated from binary entry point
- Adjustments management separate from persona management
- Tests separated from production code

### 10. Defensive Programming
- Input validation at all public boundaries
- Error types with descriptive messages
- State validation before operations

## Repository Structure

### Main Sigma Repo
- `Sigma/` - System core library (tier 0)
- `wiki/` - Sigma component wiki
- `.github/workflows/` - CI/CD

### Component Repos (one per component)
- `SigmaOS--Dante/` - CLI tool component (tier 2)
- `SigmaOS-0ad/` - 0.A.D. integration component (tier 2)
- `SigmaOS-3d-modeler/` - 3D modeling component (tier scaffold)
- `SigmaOS-ab/` - Basic ab utility component (tier scaffold)
- ... more components to be added

### Central GitHub Wiki
- `SigmaOS.wiki` - Consolidated wiki for all components
- Contains markdown files for each component
- Keeps main branches free from documentation

## Workflow

### 1. Component Creation
1. Create new directory with component name
2. Set up `Cargo.toml` and `COMPONENT.toml`
3. Implement library code in `src/lib.rs`
4. Add binary entry point in `src/main.rs`
5. Write AAA-style unit tests
6. Create GitHub Wiki documentation
7. Set up CI/CD workflow
8. Push to GitHub

### 2. Component Improvement
1. Review existing code for principle violations
2. Refactor to apply Single Responsibility
3. Ensure fail-fast on invalid input
4. Apply AAA testing pattern
5. Update wiki documentation
6. Push improvements to GitHub

### 3. Wiki Management
1. Create wiki markdown files in each component repo
2. Initialize separate wiki git repos
3. Consolidate wiki content to central `SigmaOS.wiki`
4. Keep main branches free from .md/.txt documentation files

## Next Steps
1. Create wikis for remaining components
2. Add missing component repositories
3. Improve code quality across all components
4. Update central SigmaOS.wiki with component summaries
5. Establish component registry and dependency mapping