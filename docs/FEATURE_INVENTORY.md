# SigmaOS Feature Inventory

Status legend: **verified** (tested end-to-end), **implemented** (code exists,
unit-tested, not yet verified on real hardware/ISO), **experimental** (exists
but not production-ready), **planned** (designed, not implemented),
**deferred** (explicitly postponed).

This inventory reconciles README claims with actual evidence, per the
SigmaOS-vs-Omarchy plan (docs/SIGMAOS_VS_OMARCHY_PLAN.md).

## Core system

| Feature | Status | Evidence / notes |
|---|---|---|
| Rust system-management library | implemented | 184 modules in `src/`, ~700 unit tests + doctests; compiles clean |
| CLI tools (`cli/`, `scripts/sigmaos-*`) | implemented | install, update, service, pkg, backup, audit, mirror, etc. |
| Package groups (`packages/sigmaos-*`) | implemented | PKGBUILD definitions for base + desktops |
| ALPM/pacman integration | implemented | `scripts/sigmaos-pkg`, pacman.conf |
| sigpkg experimental tooling | experimental | Must not be presented as production until upgrade/recovery tests pass |

## ISO & install

| Feature | Status | Evidence / notes |
|---|---|---|
| ISO build scripts (5 variants) | implemented | `scripts/build-iso*.sh`; archiso variant preferred |
| Bootable ISO artifact | planned | Not yet built end-to-end; requires Arch Linux build host |
| UEFI + BIOS boot | planned | QEMU smoke test script exists (`scripts/sigmaos-test-iso`) |
| Installer (`install-sigmaos.sh`) | implemented | Text-based; dry-run validation pending on real media |
| First-boot service/config | implemented | `config/sigmaos-*.service`, `live-config.sh` |

## Desktop

| Feature | Status | Evidence / notes |
|---|---|---|
| Hyprland/Wayland session | planned | Compositor decided: Hyprland ships; custom compositor is research-only (docs/COMPOSITOR_DECISION.md) |
| Quickshell/Waybar/notifications | planned | Component repos exist; not integrated into ISO yet |
| Theme switching | planned | `SigmaOS-theme-*` repos; single user-facing path not yet chosen |
| GNOME/KDE desktop packages | implemented | `packages/sigmaos-desktop-{gnome,kde}` PKGBUILDs |

## Recovery & updates

| Feature | Status | Evidence / notes |
|---|---|---|
| Update tooling (`sigmaos-update`) | implemented | Script + config; rollback path untested |
| Snapshot/rollback | implemented | `SigmaOS-rollback` repo (generation manager); not wired into installer |
| ZFS integration | implemented | `SigmaOS-zfs-integration` repo; dataset layout doc only |
| Backup (`sigmaos-backup`) | implemented | Script present |

## Component repos (GitHub)

| Area | Status |
|---|---|
| ~30 pushed component repos | implemented (scaffolds) — domain code deepening in progress |
| 12 staged locally | pending repo-creation quota |
| Per-repo wikis | pending manual wiki init |

## Known gaps vs plan Phase 1

- [ ] ISO built from clean environment with published checksums
- [ ] QEMU smoke tests run against a real ISO artifact
- [x] Compositor ambiguity resolved — Hyprland ships, custom compositor research-only (`docs/COMPOSITOR_DECISION.md`)
- [ ] Component/repo counts reconciled in README
