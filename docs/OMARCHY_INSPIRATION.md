# SigmaOS — Omarchy (omacom) Inspiration Map

Every repository in `github.com/orgs/omacom/repositories` (93 repos) is mapped
below to its SigmaOS counterpart and the concrete improvement to apply.
Pattern source: omacom repo structure (AGENTS.md, CI workflows, registry
manifests, plugin/theme marketplaces, archiso ISO pipeline, pkgs repo).

## Direct equivalents (exist on SigmaOS side)

| omacom repo | SigmaOS repo | Improvement to apply |
|---|---|---|
| omarchy | SigmaOS-omarchy | Mirror top-level layout: `bin/`, `config/`, `default/`, `install/`, `migrations/`, `shell/`, `agents/`; add `AGENTS.md` rules file |
| omarchy-pkgs | SigmaOS-omarchy-pkgs | Add `pkgbuilds/`, `build/`, `release/`, `ci/`, `helpers/`, `tests/` dirs; package-build CI |
| omarchy-iso | SigmaOS-omarchy-iso | Add `archiso/`, `builder/`, `configs/`, `manifests/`, `presets/` dirs; ISO build pipeline |
| omarchy-ui | SigmaOS-omarchy-ui | Adopt omacom/quattro-style component registry |
| mesa | (SigmaOS-mesa if present) | Wrap as component repo with PKGBUILD-style build |
| m1n1 | — | Low priority; firmware blob repo if needed |
| sddm | SigmaOS-sddm (if present) | Display-manager component packaging |
| quickshell | SigmaOS-quickshell (if present) | Shell-component packaging |

## Missing SigmaOS repos to create (staged locally)

| omacom repo | Purpose | SigmaOS repo to create |
|---|---|---|
| omarchy-plugin-marketplace | Community plugin registry (registry.json + site + worker) | SigmaOS-plugin-marketplace |
| omarchy-plugin-registry | Plugin index/metadata | SigmaOS-plugin-registry |
| omarchy-theme-registry | Theme index (pnpm workspace, themes/, overrides/) | SigmaOS-theme-registry |
| omarchy-theme-marketplace | Theme distribution | SigmaOS-theme-marketplace |
| omarchy-themes-loops / -intros | Theme assets | fold into SigmaOS-themes |
| omarchy-theme-sync | Theme sync tooling | SigmaOS-theme-sync |
| omarchy-local-ai | Local model catalog for agent use | SigmaOS-local-ai |
| omarchy-speedtest | Network speed test | SigmaOS-speedtest |
| try-omarchy / try-omarchy-windows | Try-before-install live images | SigmaOS-live-media (already staged) + SigmaOS-try |
| omarchy-site / -site-crt | Project website | SigmaOS-site |
| omarchy-mac / -mac-installer / -mac-pkgs / -maclab / homebrew-mac | Apple Silicon support | SigmaOS-mac-support (single consolidated repo) |
| omarchy-configurator | First-boot/system configurator | SigmaOS-configurator |
| omarchy-fish / -zsh / -lazyvim | Shell/dotfile components | SigmaOS-shell-config |
| omarchy-chromium | Browser component | SigmaOS-browser (exists) — add omarchy-chromium recipes |
| omarchy-notification-center-plugin | Notifications | SigmaOS-notifications |
| omarchy-port-forward-plugin | Port forwarding | SigmaOS-port-forward |
| omarchy-screensaver / -amiga-screensaver | Screensavers | SigmaOS-screensaver |
| omarchy-audio-tuner | Audio tuning | SigmaOS-audio-tuner |
| omarchy-hyprland-titlebars | WM polish | SigmaOS-window-manager (exists) |
| omarchy-mirror | Mirror/repo sync | SigmaOS-repomgr (exists in main crate) |
| omarchy-pkgs → packages/tap | Packaging taps | SigmaOS-pkg-core (staged) |

## omacom app/tool repos → SigmaOS app repos

| omacom repo | SigmaOS repo |
|---|---|
| hype (Markdown presentations) | SigmaOS-hype |
| signed (PDF sign/fill) | SigmaOS-signed |
| plain (config text editor) | SigmaOS-plain |
| folio | SigmaOS-folio |
| omacalc | SigmaOS-omacalc |
| omachee | SigmaOS-omachee |
| omacon / omacon-site | SigmaOS-omacon |
| omacut | SigmaOS-omacut |
| omadots | SigmaOS-omadots |
| omafind | SigmaOS-omafind |
| omakub | SigmaOS-omakub |
| omamac | SigmaOS-omamac |
| omapager | SigmaOS-omapager |
| omarchs | SigmaOS-omarchs |
| omareel | SigmaOS-omareel |
| omasnap | SigmaOS-omasnap |
| omaterm / -site | SigmaOS-omaterm |
| omaview | SigmaOS-omaview |
| omawrite | SigmaOS-omawrite |
| cards, checkmate, elsewhen, flowtunes, gliff, herdr, monologue, oligarchy, owe, pixel.nvim, aether, aether.nvim, lumon.nvim, ttfx, tte-go, wecanfixeverything, asdcontrol-git, state-of-omarchy, radio.omarchy.org, website, projects, people, repositories, openpgpkey, libva-v4l2_request, acestep-image, llamacpp-image, sdcpp-image, stable-diffusion.cpp, tensorfold-nemotron-bolt, fantasi, linux-aurora, macvdmtool | Evaluate per-repo; create SigmaOS-* only where a real component gap exists (avoid fragmentation — consolidate into category repos: SigmaOS-tools, SigmaOS-media, SigmaOS-ai, SigmaOS-themes) |

## Cross-cutting improvements adopted org-wide

1. **AGENTS.md** in every repo root — machine-readable rules for AI agents
   (already partially present; standardize content: build/test commands,
   principles, branch policy).
2. **CI workflows** — omacom uses `.github/workflows` per repo; every SigmaOS
   component repo gets `cargo test`, `cargo clippy -D warnings`, `cargo fmt
   --check`, `cargo audit`.
3. **Registry manifests** — plugin/theme/marketplace repos use a single
   `registry.json` (or `COMPONENT.toml` already adopted) as the index.
4. **Wiki-backed docs** — `.md` files live in each repo's GitHub Wiki; repos
   keep only `wiki/` staging content until wiki init.
5. **Shell-first tooling** — omacom is shell-heavy; SigmaOS keeps Rust core +
   shell helpers (`tools-sigmaos.sh` pattern already added).
6. **Single `main` branch**, merge after tests — matches existing policy.

## Rules

- No repo created without a real component behind it (no empty shells).
- Consolidate similar omacom repos into one SigmaOS repo where sensible
  (e.g. themes-loops + themes-intros → SigmaOS-themes).
- All repos GPL-3.0-or-later, SigmaOS identity only, no upstream references
  in shipped metadata.

---

## Fork status (updated)

`SigmaOS-omarchy` is now a **real fork** of `omacom/omarchy` — full upstream
history (8,206 commits) plus SigmaOS rebrand commit `846060e64` adding
`SIGMAOS_FORK.md` fork policy and SigmaOS-branded README. MIT license and
upstream copyright retained. The earlier Rust scaffold was replaced via
force-push (it was a placeholder, not the Omarchy codebase).

Fork policy: SigmaOS identity for user-facing branding; upstream names kept
only for compatibility (package names, paths, license). Divergence per
`docs/COMPOSITOR_DECISION.md`, `docs/THEME_DECISION.md`,
`docs/COMPETITIVE_STRATEGY.md`. Periodic upstream syncs preserve SigmaOS
decisions.

## Fork status (updated)

Core Omarchy forks now live on GitHub (full upstream history + SigmaOS
rebrand commit + `SIGMAOS_FORK.md` in each):

- `SigmaOS-omarchy` @ `846060e64` — distro/desktop (8,206 commits)
- `SigmaOS-omarchy-pkgs` @ `fdb7af21` — package recipes/tooling (63M)
- `SigmaOS-omarchy-iso` @ `c6257b0` — ISO builder/presets/manifests (59M)

All MIT-licensed; upstream copyright retained. Fork policy per
`SIGMAOS_FORK.md` in each repo: SigmaOS branding on repo docs, upstream
names kept for compatibility (pkgs.omarchy.org URLs, package names, paths).

## Fork status (final batch)

Additional Omarchy forks pushed this session:

- `SigmaOS-theme-marketplace` @ `ad52c77` — themes.omarchy.org site (Rails)
- `SigmaOS-plugin-registry` @ `d1a9877` — plugin index
- `SigmaOS-local-ai` @ `96f6430` — local-model engine/recipes (repo created after quota reset)

Previously-blocked component repos now on GitHub (quota reset):
`SigmaOS-speedtest`, `SigmaOS-site`, `SigmaOS-mac-support`,
`SigmaOS-shell-config`, `SigmaOS-omarchy-mac-pkgs`, `SigmaOS-dkms`,
`SigmaOS-sec-audit`, `SigmaOS-live-media`, `SigmaOS-ports`.

Only `SigmaOS-wiki-hub` remains uncreated (throttled again at last repo).

## Fork status (app batch)

Additional Omarchy forks live on GitHub: `SigmaOS-plain`, `SigmaOS-omaview`,
`SigmaOS-omadots`, `SigmaOS-omafind` (repos pre-existed; fork force-pushed).

15 more forks staged locally in `~/sigmaos-repos/` with upstream remotes and
`push-forks.sh` (unshallow → create → push, idempotent) — awaiting
repo-creation quota: hype, signed, omawrite, omachee, omacon, omacut,
omakub, omapager, omarchs, omareel, omasnap, omaterm, mac-installer,
wiki-hub, plugin-marketplace (needs partial-clone approach).

## Fork fleet status (full pass)

**Live on GitHub (13 Omarchy forks):** omarchy, omarchy-pkgs, omarchy-iso,
theme-registry, theme-marketplace, plugin-registry, local-ai, mac-support,
omarchy-mac-pkgs, plain, omaview, omadots, omafind.

**Staged locally (40 forks, awaiting repo-creation quota):**
`~/sigmaos-repos/` — each has full/partial upstream history, `origin` →
SigmaOS repo, `upstream` → omacom repo, `SIGMAOS_FORK.md`, rebranded README.
`push-forks.sh` unshallows → creates → pushes (idempotent).

Groups staged: apps (hype, signed, omawrite, omachee, omacon, omacut,
omakub, omapager, omarchs, omareel, omasnap, omaterm, omacalc, omamac?),
desktop/shell (ui, configurator, notification-center, port-forward,
hyprland-titlebars, fish, zsh, lazyvim, mirror, chromium, quickshell, sddm,
screensaver, amiga-screensaver, audio-tuner, themes: osaka-jade,
synthwave84, themes-loops, theme-sync, site-crt), mac (mac-installer,
maclab), try/state (try-omarchy, try-omarchy-windows, state-of-omarchy),
plugin-marketplace (needs partial-clone), wiki-hub.

**Remaining omacom repos not yet forked** are mostly niche/experimental
(library images, themes for specific apps, research tools) — candidates for
the research layer rather than forks.
