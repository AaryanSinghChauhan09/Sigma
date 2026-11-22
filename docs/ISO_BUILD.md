# SigmaOS ISO Build Guide (Reproducible)

Phase-1 requirement from `docs/SIGMAOS_VS_OMARCHY_PLAN.md`: a contributor must
be able to build and boot the ISO using documented steps.

## Prerequisites

- Arch Linux (or Arch-based) build host — pacstrap/mkinitcpio/mkarchiso are
  Arch tools
- ~20 GB free disk for package downloads + rootfs + squashfs
- Tools: `xorriso mksquashfs bsdtar pacman` (see `scripts/check-build-env.sh`)

## Build

```sh
git clone https://github.com/AaryanSinghChauhan09/Sigma
cd Sigma
./scripts/check-build-env.sh          # validate tools
./scripts/build-iso-archiso.sh        # preferred: archiso-based build
# alternatives: build-iso.sh, build-iso-improved.sh, build-iso-simple.sh
```

### Non-Arch hosts (Docker/Podman)

```sh
./scripts/build-iso-container.sh      # builds inside archlinux container;
                                      # works on Mint/Ubuntu/etc.
```

Output lands in `build/container-out/`. The container runs privileged
(mkarchiso needs loop devices) and auto-confirms prompts.

Output: `build/iso/*.iso`

## Checksums & manifest (required before publishing)

```sh
./scripts/sigmaos-checksums.sh build/iso
```

Produces `build/iso/checksums/*.sha256|sha512|b2` and `MANIFEST.md` recording
git commit, dirty state, and tool versions. Commit checksums alongside
release notes.

## Smoke test (QEMU)

```sh
./scripts/sigmaos-test-iso           # boots ISO under QEMU (UEFI+BIOS),
                                     # verifies structure, writes report
```

Prereqs: `qemu-system-x86_64`, OVMF firmware (`/usr/share/ovmf/x64/OVMF.fd`).

## What "verified" means

An ISO is **verified** only when:

1. Built from a clean checkout on a documented environment
2. Checksums published and reproducible (same commit → same artifact modulo
   timestamps; document any nondeterminism)
3. `sigmaos-test-iso` passes: live boot, desktop launch signal in serial log
4. Installed boot tested via automated install path

Until all four hold, status stays **planned** in
`docs/FEATURE_INVENTORY.md`.
