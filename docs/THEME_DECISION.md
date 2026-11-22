# Theme System Decision

Status: **decided** — Phase-2 requirement (one reliable theme-switching path).

## Decision

**One user-facing theme path: `sigmaos-theme` CLI + `SigmaOS-theme-registry`
as the index.**

- `SigmaOS-theme-registry` — canonical index of available themes
  (`registry.json` + `themes/` + `overrides/`, already structured per
  omacom/omarchy-theme-registry).
- `SigmaOS-theme-marketplace` — distribution/submission channel only; not a
  second switching path.
- `SigmaOS-theme-manager` / `SigmaOS-theme-settings` (where they exist) —
  consolidated into the CLI + registry; overlapping repos get archived or
  merged per the plan's repo-consolidation rule.

## User-facing contract

```sh
sigmaos-theme list          # themes from registry
sigmaos-theme apply <name>  # switch theme atomically
sigmaos-theme current       # show active theme
sigmaos-theme reset         # restore default
```

Implementation notes:

- Hyprland session (per COMPOSITOR_DECISION.md): themes are Hyprland/Waybar/
  wofi color+layout configs under `~/.config/hypr/themes/<name>/`.
- Apply = copy theme dir → `~/.config/hypr/current` symlink swap (atomic,
  same pattern as SigmaOS-rollback generations).
- `sigmaos-theme reset` restores the bundled default theme.

## Repo dispositions

| Repo | Disposition |
|---|---|
| SigmaOS-theme-registry | **core** — canonical index |
| SigmaOS-theme-marketplace | maintained — submission/distribution |
| SigmaOS-theme-manager | merge into sigmaos-theme CLI or archive |
| SigmaOS-theme-settings | merge into configurator or archive |
| SigmaOS-theme-sync | keep only if it syncs registry→system; else archive |

Curated default set: ship 3 themes initially (default dark, light, high-
contrast) — small, consistent, per plan Phase 2.
