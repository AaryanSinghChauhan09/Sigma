# Compositor Decision Record

Status: **decided** — resolves the ambiguity flagged in
`docs/SIGMAOS_VS_OMARCHY_PLAN.md`.

## Decision

**Hyprland is the production Wayland compositor for SigmaOS.**

The `SigmaOS-compositor` component repository (Wayland compositor research)
is classified **research / not a dependency**. It must not be wired into the
ISO, installer, or desktop session until it passes the same bar as any other
component: builds reproducibly, passes tests, boots a session in QEMU, and has
a maintainer.

## Rationale

- Hyprland is proven, packaged in Arch repos, actively maintained, and is the
  same class of compositor Omarchy ships — directly comparable UX.
- SigmaOS's Phase-1/2 goals (polished day-one desktop) favor a mature
  compositor over an unproven custom one.
- The Rust library's `compositor` persona flag refers to *session config*,
  not a custom compositor implementation.

## Consequences

- ISO/desktop packages depend on `hyprland` from Arch repos, not on
  `SigmaOS-compositor`.
- `DESKTOP.md` persona `compositor: true` means "Way predecessor session with
  compositor enabled" (Hyprland), not "use SigmaOS-compositor".
- If `SigmaOS-compositor` matures, revisit via a new decision record; until
  then it stays out of the critical path.

## Docs updated

- `docs/FEATURE_INVENTORY.md` — ambiguity item resolved
- `docs/SIGMAOS_VS_OMARCHY_PLAN.md` — repo-priority note stands
