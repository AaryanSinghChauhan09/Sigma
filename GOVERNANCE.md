# SigmaOS Governance

SigmaOS is governed by the principles of **Governance and Do-cracy**, **Meritocracy and Inclusivity**, **Community-Oriented Development**, and **Respect & Ethical Participation**, under the **GPL-3.0-or-later** license.

## 1. Core Philosophy: Do-cracy

SigmaOS operates as a **do-cracy**: the individuals who do the work—writing code, fixing bugs, maintaining infrastructure, writing documentation, and reviewing pull requests—make the day-to-day technical and operational decisions for the components they actively maintain.

- **Ownership through Contribution**: Authority is earned by sustained, high-quality work, not by title or tenure alone.
- **Lazy Consensus**: Proposals and pull requests that align with SigmaOS architectural principles and pass all automated checks proceed unless a maintainer raises a concrete, reasoned technical objection.
- **No Unfunded Mandates**: Requesting a feature without contributing code, tests, or specification does not obligate maintainers to implement it.

## 2. How Decisions Are Made

1. **Component-Level Decisions**:
   - Made by the active maintainers of that repository via pull requests and issue discussions.
   - Must comply with `AGENTS.md`, `CONTRIBUTING.md`, and the SigmaOS `Development-Guidelines.md`.
2. **Cross-Repository & Architectural Decisions**:
   - Decisions affecting multiple repositories, shared libraries (`sigma-system`), `COMPONENT.toml` schemas, or `tools-sigmaos.sh` interconnectivity require an RFC or tracking issue in the master `Sigma` repository.
   - Consensus is sought among affected component maintainers. When consensus stalls, the project lead / core maintainers resolve the deadlock based on technical merit, simplicity (KISS), and SigmaOS goal alignment.
3. **Rolling Release & Integration Decisions**:
   - SigmaOS follows a continuous **Rolling Release Model**. No release tags are cut; `main` is always kept releasable and green.

## 3. Who Can Contribute

Everyone is welcome to contribute to SigmaOS regardless of background, location, or experience level, provided they:

- Agree to license their contributions under **GPL-3.0-or-later** (or the repository's compatible open-source license).
- Uphold SigmaOS identity and intellectual property rules (no unauthorized third-party code, no leaked credentials, no external project branding).
- Follow the Code of Conduct and engineering standards.

## 4. Meritocracy Model

SigmaOS recognizes contributors through a transparent meritocracy:

| Role | Criteria | Responsibilities & Privileges |
| :--- | :--- | :--- |
| **Contributor** | Any individual opening issues, PRs, or wiki improvements | Submit PRs from `feature/<name>` branches; participate in reviews and discussions |
| **Component Maintainer** | Consistent, high-quality PRs, tests, and wiki updates in a specific repo | Review and merge PRs to `main` after CI passes; maintain `COMPONENT.toml` and wiki |
| **Core Maintainer** | Sustained cross-repo architectural contributions and stewardship | Approve cross-repo RFCs, manage repo consolidation, enforce security & identity standards |
| **Project Lead** | Founding stewardship of SigmaOS | Final arbiter on governance disputes, licensing, and project-wide direction |

Advancement is based solely on demonstrated technical competence, reliability, and constructive collaboration.

## 5. How Contributions Are Accepted

A contribution is accepted into `main` only when all of the following are satisfied:

1. **Branching**: Created from an up-to-date `main` using `feature/<name>` (never committed directly to `main`).
2. **Code Quality**: Passes `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo build` with `#![forbid(unsafe_code)]` enforced where required.
3. **Testing**: All existing tests pass and new functionality includes deterministic tests following the Arrange-Act-Assert (AAA) pattern.
4. **No Demos or Stubs**: Code must be production-ready, with no placeholder stubs or unlinked `TODO`s.
5. **Documentation**: Corresponding GitHub Wiki / `sigmaos-wiki-staging` documentation is updated for every component change.
6. **Review & Merge**: Reviewed by at least one maintainer (or automated + peer verification) and merged cleanly into `main`.

## 6. Code of Conduct (Respect & Ethical Participation)

All human contributors and AI agents operating in SigmaOS spaces must uphold:

- **Respect**: Critique code and architecture, never people. Assume good faith while maintaining high technical standards.
- **Ethical Participation**: Never introduce backdoors, telemetry without consent, proprietary lock-in, or plagiarized code.
- **Transparency**: Disclose AI-assisted contributions clearly; never fabricate test results or security audits.
- **Inclusivity**: Keep discussions accessible, welcoming, and focused on solving engineering problems.
- **Enforcement**: Repeated or severe violations (harassment, malicious code, deliberate IP or security violations) result in revocation of commit/review privileges and removal from the community.
