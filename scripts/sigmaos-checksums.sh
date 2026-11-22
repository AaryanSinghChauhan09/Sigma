#!/bin/bash
# SigmaOS ISO checksum + manifest generator.
# Run after a successful ISO build. Produces SHA-256 sums and a build
# manifest for reproducibility verification (plan Phase 1).

set -euo pipefail

ISO_DIR="${1:-build/iso}"
OUT_DIR="${ISO_DIR}/checksums"
mkdir -p "$OUT_DIR"

shopt -s nullglob
isos=("$ISO_DIR"/*.iso)
if [ ${#isos[@]} -eq 0 ]; then
    echo "No ISO found in $ISO_DIR" >&2
    exit 1
fi

for iso in "${isos[@]}"; do
    name=$(basename "$iso")
    sha256sum "$iso" > "$OUT_DIR/${name}.sha256"
    sha512sum "$iso" > "$OUT_DIR/${name}.sha512"
    # b2sum if available for BitTorrent-style verification
    command -v b2sum >/dev/null && b2sum "$iso" > "$OUT_DIR/${name}.b2" || true
done

# Build manifest: record tool versions used
{
    echo "# SigmaOS ISO build manifest"
    echo "generated: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "git_commit: $(git rev-parse HEAD 2>/dev/null || echo unknown)"
    echo "git_dirty: $(git status --porcelain 2>/dev/null | wc -l) changed files"
    echo
    echo "## tools"
    for t in xorriso mksquashfs bsdtar pacstrap mkinitcpio mkarchiso qemu-system-x86_64; do
        if command -v "$t" >/dev/null; then
            echo "- $t: $($t --version 2>/dev/null | head -1 || echo present)"
        else
            echo "- $t: MISSING"
        fi
    done
    echo
    echo "## artifacts"
    ls -la "$ISO_DIR"/*.iso 2>/dev/null || true
} > "$OUT_DIR/MANIFEST.md"

echo "Checksums and manifest written to $OUT_DIR"
ls -la "$OUT_DIR"
