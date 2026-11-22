# SigmaOS Competitive Strategy vs Omarchy/Omacom

Strategy: compete on a cohesive, reliable Linux desktop — not repo count.
Omarchy's advantage is a unified experience (manual, themes, hotkeys, system
tools, documented extension model). SigmaOS must make ONE excellent,
supportable desktop release, then grow deliberately.

## Identity

**The sovereign, auditable Arch workstation**: safety-first install/rollback,
data protection, explicit hardware support, developer + AI-agent tooling,
Rust-oriented core, modular inspectable config, predictable updates.
Visible in the first minute — not just the README.

## Priorities

1. **One dependable release** — reproducible ISO build, automated
   live-boot/installer/installed-boot/desktop-start tests in disposable VMs,
   checksums + release notes + known issues, evidence-based hardware claims.
2. **Polished everyday desktop** — one default session (Hyprland + Sigma
   shell), discoverable workflows, keybinding reference on first boot.
3. **Trustworthy maintenance** — pacman as baseline; sigpkg experimental
   until upgrade/conflict/downgrade/rollback tested; one update command,
   tested recovery path.
4. **Clear repo architecture** — group repos: core release / maintained
   modules / research. Status badges: core, supported, experimental,
   community, retired. Archive empty shells.
5. **Ecosystem after base is solid** — one theme format, small plugin
   contract, reviewable registry, few integrated apps. No marketplace
   before stable release.
6. **Trust & community** — task-oriented manual, contribution/support
   guides, issue templates, workflow demos, hardware-test reports,
   claims synced with CI evidence.

## Product claims language (factual only)

- "Supports these Intel, AMD, NVIDIA configurations" (with evidence)
- "ISO built in CI and boot-tested in QEMU"
- "Dry-run installer; destructive mode requires disk confirmation"
- "Btrfs snapshot/rollback tested for supported profiles"

Never: "production-ready", "secure by default", "complete OS" without tests
+ stated threat model.

## 90-day sequence

- **Days 1–30**: audit claims, CI ISO build, QEMU smoke tests, consolidate
  theme/desktop repos, publish support levels, remove unsupported promises
- **Days 31–60**: stable installer + live desktop, keybinding help, settings
  UI, one update + one support command, workflow screenshots/demos
- **Days 61–90**: tested candidate ISO, checksums + notes, hardware testers,
  theme/plugin submission process, docs from real feedback

## Scorecard (monthly)

Clean build one-command repeatable · live boot automated pass · install
tested in VM matrix · first-boot core workflow · update+rollback tested ·
docs cover install→recovery · evidence-based hardware list · issue
severity/repro/response · core repos maintained · real users/contributors
