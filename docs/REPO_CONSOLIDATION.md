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
| SigmaOS-osaka-jade-theme | absorbed → theme-registry | Registry entry added (`themes/sigmaos-osaka-jade.json`), pending commit |
| SigmaOS-synthwave84-theme | absorbed → theme-registry | Registry entry added (`themes/sigmaos-synthwave84.json`), pending commit |
| SigmaOS-site-crt | distinct | CRT variant site — keep if maintained |

## Shell / desktop config

| Repo | Status | Notes |
|---|---|---|
| SigmaOS-shell-config | canonical | Shell defaults/UX layer |
| SigmaOS-dots | canonical | User dotfiles and shell configs (starship, tmux, git, fish, zsh) |
| SigmaOS-fish | absorbed → dots | Fish config lives in `SigmaOS-dots` (`config/fish`, `bin`); repo kept for package builds until packaging migrates |
| SigmaOS-zsh | absorbed → dots | Zsh config lives in `SigmaOS-dots` (`config/zsh`, `config/shell`, `bin`); repo kept for package builds until packaging migrates |
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
| SigmaOS-mac-support | archived | Unmaintained umbrella; archived 2026-11-25 — use the focused Mac repos below |
| SigmaOS-mac-pkgs | canonical | Apple Silicon add-on packages, release/qualification tooling, Mac manual (formerly `omarchy-mac-pkgs`) |
| SigmaOS-mac-installer | canonical | macOS app that installs SigmaOS on Apple Silicon next to macOS |
| SigmaOS-mac-vm | canonical | VM shell that runs stock SigmaOS on Apple Silicon Macs with macOS integration (formerly `try-omarchy`) |
| SigmaOS-maclab | distinct | Hardware test lab for SigmaOS on real Apple Silicon Macs |
| SigmaOS-omarchy-mac-pkgs | absorbed → mac-pkgs | Renamed to `SigmaOS-mac-pkgs` |
| SigmaOS-try-omarchy | absorbed → mac-vm | Renamed to `SigmaOS-mac-vm` |
| SigmaOS-try-omarchy-windows | distinct | Windows variant — separate |

## Marketplace / discovery

| Repo | Status | Notes |
|---|---|---|
| SigmaOS-plugin-registry | canonical | Plugin data/registry |
| SigmaOS-plugin-marketplace-tools | canonical | Consolidated marketplace fork: site/ + worker/ + root tooling merged 2026-11-24 |
| SigmaOS-plugin-marketplace-worker | archived → tools | Merged into plugin-marketplace-tools (site/worker reunified) |
| SigmaOS-plugin-marketplace-site | archived → tools | Merged into plugin-marketplace-tools (site/worker reunified) |
| SigmaOS-plugin-marketplace-assets | pending | Preview images — needs LFS decision; merged site references generated images not yet vendored |
| SigmaOS-plugin-marketplace | distinct | SigmaOS-native Rust marketplace component (cinnamon-spices-inspired), not the omarchy fork |
| SigmaOS-local-ai | distinct | Local AI features |

## App forks (distinct — keep)

hype, signed, plain, omawrite, omaview, omachee, omacon, omacut, omadots,
omafind, omapager, omarchs, omareel, omasnap, omaterm, omacalc, cards,
checkmate, herdr, aether.nvim, tte-go, chrom(ium), mirror, configurator,
notification-center, port-forward, audio-tuner, screensaver,
amiga-screensaver, ui, m1n1, mesa, oligarchy,
lumon.nvim, pixel.nvim, ttfx, openpgpkey, elsewhen, fantasi, flowtunes,
folio, gliff, monologue, owe, aether.

## Scaffold vs fork collisions (resolve on push)

- SigmaOS-browser (real impl) vs SigmaOS-chromium (fork) — keep both, distinct purposes
- SigmaOS-notifications (scaffold) vs SigmaOS-notification-center (fork) — fork wins, archive scaffold
- SigmaOS-configurator — fork replaced scaffold (done)
- SigmaOS-speedtest, SigmaOS-ports, SigmaOS-dkms, SigmaOS-sec-audit, SigmaOS-live-media — scaffolds pending real impl or fork equivalents

## Archived stubs / empty duplicates (2026-11-24)

Empty scaffolding accidents archived on GitHub (content preserved in canonical repos):

| Repo | Reason |
|---|---|
| SigmaOS-cards-checkmate-dup | Empty stub; canonical = SigmaOS-cards |
| SigmaOS-m1n1-done | Empty stub; canonical = SigmaOS-m1n1 |
| SigmaOS-omarchy-pkgs-sigmaos | Empty repo; canonical = SigmaOS-omarchy-pkgs (private) |
| SigmaOS-site-crt2 | Empty repo; canonical = SigmaOS-site-crt |
| SigmaOS-omarchy-plugin-marketplace-tools | Empty misnamed repo; canonical = SigmaOS-plugin-marketplace-tools |
| SigmaOS-omachee-plugin-marketplace-tools | Empty misnamed repo; canonical = SigmaOS-plugin-marketplace-tools |
| SigmaOS-omakub | Empty repo; upstream push blocked by secret scanning (user decision) |

## GitHub Wikis

GitHub only materializes `<repo>.wiki.git` after the Wiki tab is opened once in
a browser — there is no API/CLI path to create it. Staged Home.md content for
every SigmaOS-* component lives in `sigmaos-repos/sigmaos-wiki-staging/` on the
build host, and `sigmaos-repos/push-wikis.sh` pushes it to any wiki that has
been initialized. Until the one-time browser visit per repo, wikis remain
pending; the staging content is the source of truth in the meantime.

Wiki staging: 104 component Home.md files generated (2026-11-24), each with
repo name, short description, and links to SigmaOS/ISO/main.

## Component manifests (2026-11-24)

All 100 SigmaOS-* repos now have consistent `COMPONENT.toml` (not `COMPONENT.tomL`)
with proper name/id/metadata. Previously 32 repos had real manifests, 68 had
broken template stubs with `$(basename $d)` placeholders; these were normalized
and committed in bulk.

## Branch hygiene (2026-11-24)

All SigmaOS-* repos now use `main` as the canonical branch. SigmaOS-configurator
was migrated from `master` to `main` (master branch deleted on GitHub); all
others were already `main`.

## Action items

1. Archive "absorbed" repos after canonical repos document the absorbed content.
2. Replace remaining scaffolds with forks or real implementations.
3. Do NOT delete — mark archived on GitHub to preserve links/history.
4. Open the Wiki tab once per SigmaOS-* repo, then run `push-wikis.sh`.
