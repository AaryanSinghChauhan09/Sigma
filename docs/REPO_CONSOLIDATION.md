# SigmaOS Repository Consolidation Map

Status labels: **canonical** (keep), **absorbed** (fold into canonical, archive),
**distinct** (keep separate), **scaffold** (replace with real impl or archive).

## Theme ecosystem

| Repo | Status | Notes |
|---|---|---|
| SigmaOS-theme-registry | canonical | Theme data/validation/catalog — upstream omarchy-theme-registry fork |
| SigmaOS-theme-marketplace | canonical | Theme marketplace site — upstream fork |
| SigmaOS-themes-intros | absorbed → theme-registry | Intro/theme packs belong in registry |
| SigmaOS-themes-loops | absorbed → theme-registry | Loop themes belong in registry |
| SigmaOS-theme-sync | absorbed → theme-marketplace | Sync tooling serves marketplace |
| SigmaOS-osaka-jade-theme | absorbed → theme-registry | Single theme → registry entry |
| SigmaOS-synthwave84-theme | absorbed → theme-registry | Single theme → registry entry |
| SigmaOS-site-crt | distinct | CRT variant site — keep if maintained |

## Shell / desktop config

| Repo | Status | Notes |
|---|---|---|
| SigmaOS-shell-config | canonical | Shell defaults/UX |
| SigmaOS-fish | absorbed → shell-config | Fish config is shell config |
| SigmaOS-zsh | absorbed → shell-config | Zsh config is shell config |
| SigmaOS-lazyvim | distinct | Editor config — separate concern |
| SigmaOS-quickshell | canonical | Shell UI framework fork |
| SigmaOS-hyprland-titlebars | absorbed → packages/sigmaos-desktop-hyprland | Desktop package concern |

## Sites / docs

| Repo | Status | Notes |
|---|---|---|
| SigmaOS-site | canonical | Main website |
| SigmaOS-wiki-hub | canonical | Wiki content hub |
| SigmaOS-state-of-omarchy | absorbed → docs | State/report → main docs |
| SigmaOS-omarchy | canonical | Distro fork |

## Mac support

| Repo | Status | Notes |
|---|---|---|
| SigmaOS-mac-support | canonical | Mac support umbrella |
| SigmaOS-omarchy-mac-pkgs | absorbed → mac-support | Mac packages belong under support |
| SigmaOS-mac-installer | absorbed → mac-support | Installer is part of support flow |
| SigmaOS-maclab | distinct | Mac lab/testing — keep if active |
| SigmaOS-try-omarchy | absorbed → mac-support | Try-on-Mac flow |
| SigmaOS-try-omarchy-windows | distinct | Windows variant — separate |

## Marketplace / discovery

| Repo | Status | Notes |
|---|---|---|
| SigmaOS-plugin-registry | canonical | Plugin data/registry |
| SigmaOS-plugin-marketplace-tools | canonical | Submission/validation tooling |
| SigmaOS-plugin-marketplace-worker | canonical | Marketplace backend |
| SigmaOS-plugin-marketplace-site | canonical | Marketplace frontend |
| SigmaOS-plugin-marketplace-assets | pending | Preview images — needs LFS decision |
| SigmaOS-local-ai | distinct | Local AI features |

## App forks (distinct — keep)

hype, signed, plain, omawrite, omaview, omachee, omacon, omacut, omadots,
omafind, omapager, omarchs, omareel, omasnap, omaterm, omacalc, cards,
checkmate, herdr, aether.nvim, tte-go, chrom(ium), mirror, configurator,
notification-center, port-forward, audio-tuner, screensaver,
amiga-screensaver, fish/zsh (see shell), ui, m1n1, mesa, oligarchy,
lumon.nvim, pixel.nvim, ttfx, openpgpkey, elsewhen, fantasi, flowtunes,
folio, gliff, monologue, owe, aether, omarchy-mac-pkgs.

## Scaffold vs fork collisions (resolve on push)

- SigmaOS-browser (real impl) vs SigmaOS-chromium (fork) — keep both, distinct purposes
- SigmaOS-notifications (scaffold) vs SigmaOS-notification-center (fork) — fork wins, archive scaffold
- SigmaOS-configurator — fork replaced scaffold (done)
- SigmaOS-speedtest, SigmaOS-ports, SigmaOS-dkms, SigmaOS-sec-audit, SigmaOS-live-media — scaffolds pending real impl or fork equivalents

## Action items

1. Archive "absorbed" repos after canonical repos document the absorbed content.
2. Replace remaining scaffolds with forks or real implementations.
3. Do NOT delete — mark archived on GitHub to preserve links/history.
