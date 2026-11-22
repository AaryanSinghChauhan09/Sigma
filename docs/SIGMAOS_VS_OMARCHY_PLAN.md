# SigmaOS vs Omarchy — Practical Plan

Recommendation: don't match Omarchy feature-for-feature. Make SigmaOS a
coherent, dependable, easy-to-install Arch desktop with a distinct strength —
approachable setup, hardware support, and safe recovery — proven across real
machines and contributors.

Omarchy's advantage: unified experience with substantial manual, themes,
hotkeys, system tools, documented extension model. SigmaOS's breadth must
become a small, polished, verifiable product.

## Where to focus

| Area | SigmaOS starting point | What to do |
|---|---|---|
| Install & first boot | Arch-based live ISO, defensive installer | One supported install path reliably boots, installs, updates, recovers in clean VMs |
| Desktop experience | Hyprland, Quickshell, Waybar, themes | One consistent default desktop: keybindings, bar, launcher, notifications, settings, wallpaper, theme switching |
| Package & update | ALPM/pacman + experimental sigpkg | Keep pacman as foundation; label experimental tooling until upgrade/recovery tests pass |
| Documentation | Many features described | Task-based guide: install, navigation, updates, themes, troubleshooting, recovery — aligned to actual release |
| Components & repos | Many SigmaOS-* repos | Main OS release = source of truth; integrate only maintained, tested, needed components |
| Trust & clarity | Broad "working" claims | Audit claims vs reproducible builds/tests/boot evidence; reconcile counts |

## Phase 1 — Trustworthy baseline (weeks 1–2)

- Define target: one architecture, one install method, tested VM configs
- Build ISO from clean env; publish checksums + build instructions
- Automated QEMU smoke tests: live boot, installer dry run, installed boot, desktop launch, update
- Inventory features: verified / experimental / planned / deferred
- Reconcile component & repo counts in README
- Exit: contributor can build+boot ISO via docs; tests catch boot/installer regressions

## Phase 2 — Polished day-one desktop (weeks 3–6)

- One default session; remove redundant profile options from initial path
- Discoverable workflows: launch apps, workspaces, Wi-Fi, audio/display, screenshots, themes, update
- Small curated theme set + one reliable theme-switch path
Conservative defaults; document undo
- Test matrix: Intel/AMD graphics + documented NVIDIA status; state limits plainly
- Exit: first-time user installs in VM, reaches desktop, finds actions, updates without troubleshooting

## Phase 3 — Maintenance & recovery as selling point (weeks 7–10)

- Document update cadence, risks, recovery from failed update
- Test snapshot/rollback on supported filesystems (not just listed)
- Validate installer safety in disposable VMs; dry-run default
- Release checks: package conflicts, missing assets, broken shortcuts, config drift
- Release notes, known issues, upgrade guidance per release
- Exit: repeatable build/test evidence + tested recovery procedure

## Phase 4 — Community (weeks 11–12+)

- Concise user manual, troubleshooting, contribution guide, support expectations
- Single roadmap/triage place; link component repos
- Label repos: core / maintained / experimental / archived
- Early-user hardware testing → reproducible issues
- Demos and release notes focused on workflows, not feature counts
- Exit: users can tell what's supported, get help, contribute

## Repository priorities

- **SigmaOS** — release engineering, ISO, installer, docs, integration
- **SigmaOS-compositor** — clarify production vs research; not a dependency until validated
- **SigmaOS-packages-tools / package-core** — single supported package workflow; no experimental-as-production
- **SigmaOS-theme-manager / theme-settings / theme repos** — consolidate into one user-facing theme story
- **SigmaOS-platform / rebuild / zfs-integration** — integrate only where they support tested install/build/recovery

Resolve compositor ambiguity: component README says Wayland compositor,
main README says Hyprland — pick one in docs and packaging.

## Progress metrics (verifiable)

- ISO builds and boots reproducibly
- Install/first-boot success in automated VM tests
- Supported hardware configs with test evidence
- Update/rollback success in clean environments
- New-user install+setup time
- Open high-severity issues, ack time, releases with known-issue notes
- Contributors and independent installs — not repo/feature counts
