# SigmaOS — Future Plan: Lessons from Linux & BSD Distributions

This document defines SigmaOS's roadmap by studying how established Linux and
BSD distributions solve the problems SigmaOS must solve: packaging, releases,
installation, configuration, hardware support, security, and community
governance.

## Reference Distributions

### Linux

| Distro | Key Strengths to Adopt |
|--------|------------------------|
| **Arch Linux** | Rolling release, pacman binary packages, PKGBUILD packaging format, Arch User Repository (AUR) model, minimal base + user choice, excellent documentation culture |
| **Linux Mint** | Beginner-friendly installer, update manager with safety levels, software manager, desktop polish, LTS-style stability option |
| **NixOS** | Fully declarative configuration, reproducible builds, atomic upgrades and rollbacks, nix store isolation |
| **Omarchy** | Opinionated modern desktop defaults, fast setup, curated component repos, Hyprland-based polish, strong "just works" UX |

### BSD

| Distro | Key Strengths to Adopt |
|--------|------------------------|
| **FreeBSD** | Ports tree, pkgng binary packages, ZFS-first storage, jails for isolation, base system stability |
| **OpenBSD** | Security-first posture, secure defaults by default, pledge/unveil sandboxing, minimal attack surface, proactive auditing |
| **NetBSD** | Portability across many architectures, pkgsrc packaging, clean design |
| **DragonFly BSD** | HAMMER2 filesystem ideas, modern SMP architecture, packaging via dports |

## Feature Matrix → SigmaOS Actions

### 1. Package Management

- **From Arch**: binary package format + source-build fallback; a SigmaOS
  `sigma-pkg` tool wrapping pacman-compatible metadata where licensing allows.
- **From NixOS**: declarative package sets; SigmaOS packages declared in a
  NixOS-like config (`sigma.nix` style module system already planned).
- **From FreeBSD/pkg**: signed repository metadata, atomic package installs.
- **From OpenBSD**: `syspatch`-style minimal security patch stream.

### 2. Release Model

- Rolling release (already chosen): continuous `main`, no version tags.
- **From Arch**: `-testing` repo tier for new packages before stable.
- **From Mint**: optional "stable track" snapshot for users wanting fewer
  changes — implemented as a pinned package-set channel, not a fork.
- **From OpenBSD**: errata/security advisories published per component repo.

### 3. Configuration System

- **From NixOS**: declarative, reproducible system configuration; every
  component repo exposes a config schema; `sigmaos-rebuild` regenerates the
  system from config (dry-run + `--rollback` support).
- **From Omarchy**: sensible opinionated defaults so users need zero config.

### 4. Installation & Upgrades

- **From Arch**: `archinstall`-style guided installer + fully unattended
  preseeded installs for CI/VM images.
- **From NixOS**: atomic upgrades — new generation boot entry, instant
  rollback to previous generation on failure.
- **From FreeBSD**: bsdinstall-style slice/partitioning model for disks.

### 5. Security

- **From OpenBSD**: secure-by-default services, privilege separation,
  `secupgrade`-style audit tooling across all component repos.
- **From FreeBSD**: jails → SigmaOS containers/namespaces for untrusted apps.
- **From all**: reproducible builds so users can verify artifacts.

### 6. Filesystems & Storage

- **From FreeBSD/DragonFly**: ZFS support as first-class option (dataset
  layout, snapshots, send/recv backup integration with SigmaOS backupmgr).
- **From NetBSD**: filesystem-agnostic mount layer with pluggable backends.

### 7. Hardware & Drivers

- **From Arch**: linux-firmware split, DKMS-style out-of-tree module build
  system for components like SigmaOS-NVIDIA.
- **From BSD ports**: curated driver metadata per component repo.

### 8. Documentation & Community

- **From Arch Wiki**: every component repo wiki documents install, config,
  troubleshooting — uniform page template (already standardized: AAA pattern).
- **From Debian/BSD**: maintainer handbook per repo; do-ocracy governance
  already adopted in GOVERNANCE.md docs.

## New Repositories to Create (Next Quota Window)

Priority-ordered component repos still missing vs. the distro survey:

1. `SigmaOS-pkg-core` — package format, repo metadata, signing
2. `SigmaOS-installer` — guided + unattended installer
3. `SigmaOS-rebuild` — declarative config engine (NixOS-like)
4. `SigmaOS-rollback` — atomic upgrade/rollback manager
5. `SigmaOS-zfs-integration` — ZFS datasets, snapshots, replication
6. `SigmaOS-sec-audit` — cross-repo security auditing (OpenBSD-style)
7. `SigmaOS-dkms` — out-of-tree kernel module builder
8. `SigmaOS-live-media` — live ISO build pipeline (archiso-style)
9. `SigmaOS-ports` — source-build ports tree (BSD-style)
10. `SigmaOS-wiki-hub` — aggregated wiki index across all component repos

## Rules for This Plan

- Every new repo: single responsibility, `main`-only branching until tests
  pass, GPL-3.0-or-later, no hardcoded secrets, wiki-backed docs.
- Prefer consolidating into existing repos over creating near-duplicates.
- All repos interoperate via shell tooling (`sigmaos-cli`) and shared
  `COMPONENT.toml` metadata.
