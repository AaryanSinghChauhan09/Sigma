# SigmaOS Supported Target

Phase-1 requirement: define ONE architecture, ONE install method, and a short
list of tested VM configurations before expanding hardware promises.

## Architecture

- **x86_64 only.** No ARM/other-arch support claims until a second arch passes
  the same ISO build + QEMU smoke test.

## Install method

- **Live ISO → installer.** Single path:
  1. Boot `sigmaos-*.iso` (UEFI preferred; BIOS fallback)
  2. Run `install-sigmaos.sh` from the live session
  3. Reboot into installed system
- No netinstall, no image-flash, no dual-boot wizard in v1.

## Tested VM configurations (target matrix)

| Config | Status |
|---|---|
| QEMU + OVMF (UEFI), 2 vCPU, 2 GB RAM, 20 GB qcow2, virtio-gpu/bochs | target — must pass `sigmaos-test-iso full` |
| QEMU + SeaBIOS, same specs | target — must pass |
| VirtualBox/VMware | planned — test after QEMU passes |

## Hardware support levels (honest labeling)

- **Supported**: whatever passes install+boot+desktop in the VM matrix AND
  has at least one real-hardware report via the `install_test` issue
  template.
- **Experimental**: drivers/firmware present but untested on real HW
  (e.g. NVIDIA proprietary — status documented, not promised).
- **Unsupported**: anything else. README must not imply otherwise.

## Update policy (v1)

- Rolling `main`; ISO builds are dated snapshots.
- `sigmaos-update` for installed systems; rollback via SigmaOS-rollback
  generations once wired into the installer (Phase 3).

## Explicit non-goals for v1

- ARM, RISC-V, niche filesystems as defaults
- Custom compositor (research only — see COMPOSITOR_DECISION.md)
- Experimental sigpkg as the default package path (pacman/ALPM is the
  supported path)
