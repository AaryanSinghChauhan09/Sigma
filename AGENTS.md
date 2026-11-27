# AGENTS.md — Master AI Agent Guidelines for SigmaOS

This is the master instruction file for all AI agents operating across SigmaOS repositories. Every repository-level `AGENTS.md` inherits and specializes these rules. Violation of any rule below is grounds for rejecting a change.

---

## 1. Identity & Branding
- **Always use SigmaOS branding** in code, comments, documentation, crate metadata, and UI strings.
- **Never reference other OS or upstream projects** (no mentions of external distributions, upstream forks, or third-party project names that reveal heritage or dilute SigmaOS identity).
- Normalize all component names, paths, and identifiers to the `SigmaOS` / `sigma-*` namespace.

## 2. Language, Safety & Licensing
- **Rust is preferred** for all systems, CLI, daemon, and tooling components.
- Enforce `#![forbid(unsafe_code)]` at the crate root for all standard crates. Any low-level hardware/kernel exception must be explicitly isolated, documented, and justified.
- All code must be licensed under **GPL-3.0-or-later**. Preserve license headers and `LICENSE` files.
- Always run and pass before finishing any code change:
  ```sh
  cargo fmt --check
  cargo clippy -- -D warnings
  cargo test
  ```

## 3. Testing Requirements
- **Every new function, module, or behavior change must have unit/integration tests.**
- Follow the **Arrange-Act-Assert (AAA)** pattern and **FIRST** principles (Fast, Isolated, Repeatable, Self-validating, Timely).
- Tests must be **deterministic**—no sleep-based synchronization, unseeded randomness, or external network dependencies.
- Never delete or weaken assertions to make a failing test pass.

## 4. Wiki & Documentation Requirements
- **Every component or behavior change requires a corresponding Wiki update** (in `sigmaos-wiki-staging/<component>/` or the repo's GitHub Wiki).
- Consolidate documentation into the Wiki; do **not** add miscellaneous `.md` or `.txt` files to repository roots.
- Keep `COMPONENT.toml` accurate and up to date.

## 5. Security & Secrets
- **Never hardcode secrets** (API keys, passwords, tokens, private keys, database credentials) in source code, tests, or configuration files.
- Always read sensitive values from **environment variables** or a dedicated secret manager.
- Enforce **Least Privilege** and **Secure Defaults** (debug modes disabled by default, fail-fast on invalid auth/config).
- Run a security scan on any diff before committing.

## 6. Version Control & Rolling Release
- **Single branch (`main`) per repository.** Never commit directly to `main` for feature work; use `feature/<name>` branches and merge into `main` once tests pass.
- **Never create release tags** (`git tag` / GitHub Releases). SigmaOS uses a continuous **Rolling Release Model**.
- Use Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).

## 7. Architecture & Modularity
- **Single responsibility per repository**: each repo owns one clear domain/component and exposes a clean interface (`COMPONENT.toml` + `tools-sigmaos.sh` interconnectivity).
- Follow **SOLID**, **DRY**, **KISS**, **YAGNI**, **Separation of Concerns**, **Loose Coupling**, and **High Cohesion**.
- Provide both **GUI & CLI** interfaces for user-facing tools where applicable, with shared core logic in common libraries.

## 8. Code Quality & Completeness
- **No demos, no stubs, no placeholder implementations.**
- **No `TODO` or `FIXME` comments** without an explicit tracking issue reference.
- Handle errors explicitly (`Result<T, E>` with `thiserror`); fail fast on invalid state.
- Leave the codebase cleaner than you found it (Boy Scout Rule).
