# Contributing to SigmaOS

Thank you for contributing to SigmaOS! This guide outlines the practical workflow, coding standards, testing requirements, and review process for all SigmaOS repositories.

---

## 1. Setting Up Your Development Environment

### Prerequisites
- **Rust toolchain** (latest stable via `rustup`, with `rustfmt` and `clippy` components):
  ```sh
  rustup component add rustfmt clippy
  ```
- **Git** (2.30+)
- **System build essentials** (`gcc`, `pkg-config`, `libssl-dev` or equivalent for your host)

### Cloning & Building
```sh
git clone <sigmaos-repo-url>
cd Sigma
cargo build
cargo test
```

If working across multiple SigmaOS repositories, use `tools-sigmaos.sh` (in `SigmaOS-wiki-hub` or component root) to verify cross-repo interconnectivity and `COMPONENT.toml` metadata.

---

## 2. Branching & Version Control

SigmaOS enforces a **Single Branch Per Single Repository** model with a **Rolling Release** workflow:

- `main` is the single permanent, always-releasable branch. **Never commit directly to `main`.**
- **Never create release tags** (no `v1.0.0` tags). SigmaOS ships continuously from `main`.
- **Create a branch only for a new feature or fix**:
  ```sh
  git checkout main
  git pull origin main
  git checkout -b feature/<descriptive-name>
  ```
- Keep branches short-lived and focused on one task. Once tests and checks pass, merge into `main` via Pull Request and delete the feature branch.

---

## 3. Code Style & Quality Standards

All Rust code in SigmaOS must adhere to strict quality and safety standards:

- **Formatting**: Run `cargo fmt` before committing. CI enforces:
  ```sh
  cargo fmt --check
  ```
- **Linting**: Zero warnings allowed. CI enforces:
  ```sh
  cargo clippy -- -D warnings
  ```
- **Memory Safety**: Every Rust crate must include `#![forbid(unsafe_code)]` at the crate root unless explicitly exempted in low-level kernel/hardware modules with documented safety invariants.
- **Error Handling**: Use structured error types (e.g., `thiserror`) and fail-fast validation. Never silently swallow errors.
- **No Demos, No Stubs, No Unlinked TODOs**: Every committed function must be complete. Any deferred work must reference a valid tracking issue.
- **SigmaOS Identity**: All branding, crate names, comments, and docs must use SigmaOS identity. Never reference unrelated external projects.
- **License**: All source files must be compatible with `GPL-3.0-or-later`.

---

## 4. Test Requirements

Every new function, module, or bug fix **must** include automated tests.

- **All tests must pass** locally and in CI before merge:
  ```sh
  cargo test
  ```
- **Arrange-Act-Assert (AAA) Pattern**: Structure every unit and integration test clearly into:
  1. **Arrange**: Set up inputs, mocks, and preconditions.
  2. **Act**: Execute the function or behavior under test.
  3. **Assert**: Verify the exact expected output or state change.
- **FIRST Principles**: Tests must be **F**ast, **I**solated, **R**epeatable, **S**elf-validating, and **T**imely.
- **Deterministic**: No reliance on wall-clock timing races, external network calls, or random unseeded state.

Example:
```rust
#[test]
fn parses_valid_component_manifest() {
    // Arrange
    let raw = "name = \"sigma-example\"\nversion = \"0.1.0\"";

    // Act
    let parsed = parse_manifest(raw);

    // Assert
    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap().name, "sigma-example");
}
```

---

## 5. Wiki Update Requirements

SigmaOS consolidates documentation into repository-based GitHub Wikis (staged in `sigmaos-wiki-staging/` and `SigmaOS-wiki-hub`):

- **Every code or component change requires a corresponding Wiki update.**
- Do not clutter repository roots with ad-hoc `.md` or `.txt` files; keep root documentation minimal (`README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `GOVERNANCE.md`, `LICENSE`, `COMPONENT.toml`).
- Update the component's wiki page with new CLI flags, configuration options, API changes, and architecture notes.

---

## 6. Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/) for clear, searchable history:

```text
<type>(<scope>): <short imperative summary>

[optional body explaining why, not what]
```

Common types:
- `feat`: New feature or capability
- `fix`: Bug fix
- `docs`: Documentation or wiki updates
- `refactor`: Code restructuring without behavior change
- `test`: Adding or fixing tests
- `chore`: Build, CI, or maintenance tasks

Example:
```text
feat(cli): add declarative package validation command
```

---

## 7. Pull Request Process

1. Ensure your branch is rebased on the latest `main`.
2. Run the full verification suite locally:
   ```sh
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```
3. Verify your Wiki update is ready in `sigmaos-wiki-staging/` (or the component wiki).
4. Open a Pull Request targeting `main` with:
   - A clear description of what changed and why.
   - Reference to the related issue or spec.
   - Confirmation that tests and wiki updates are included.
5. Address review feedback promptly. Once approved and CI is green, merge into `main`.

---

## 8. Security Reporting Process

**Never open a public GitHub issue for a security vulnerability.**

If you discover a security issue (credential leak, privilege escalation, memory corruption, or authentication bypass):

1. Report it privately to the SigmaOS maintainers via the repository's private security advisory channel or direct maintainer contact (`aaryan.singh.chauhan.09@gmail.com`).
2. Include steps to reproduce, affected components, and potential impact.
3. Do not disclose the vulnerability publicly until a fix is merged to `main` and users have been notified.
4. Never commit API keys, passwords, tokens, or debug-mode bypasses—use environment variables or a dedicated secret manager.
