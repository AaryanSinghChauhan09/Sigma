#!/bin/bash
# SigmaOS ISO Build in Docker/Container
# Builds the archiso profile inside an Arch Linux container — works on
# non-Arch hosts (e.g. Mint/Ubuntu) where pacstrap/mkarchiso are absent.
#
# Requirements: docker or podman; ~25 GB free disk; network access.

set -euo pipefail

VERSION="${VERSION:-0.5.0}"
IMAGE="${IMAGE:-archlinux:latest}"
BUILD_DIR="$(pwd)/build/container-out"
mkdir -p "$BUILD_DIR"

# Pick container runtime
if command -v docker &>/dev/null; then
    RUN="docker"
elif command -v podman &>/dev/null; then
    RUN="podman"
else
    echo "Error: neither docker nor podman found" >&2
    exit 1
fi

echo "=== SigmaOS Container ISO Build ==="
echo "Runtime: $RUN | Image: $IMAGE | Out: $BUILD_DIR"

$RUN pull "$IMAGE"

# `yes |` feeds stdin so pacman/mkarchiso prompts auto-confirm; -i keeps
# stdin attached. Repo mounted read-only at /src; copied to writable /work.
# mkarchiso needs privileged mode for loop devices and mounts.
exec yes | $RUN run --rm -i \
    --privileged \
    --name "sigmaos-build-$$" \
    -v "$(pwd):/src:ro" \
    -v "$BUILD_DIR:/out" \
    "$IMAGE" \
    bash -c '
        set -e
        pacman -Sy --noconfirm --needed archiso git base-devel xorriso squashfs-tools
        # copy source excluding heavy build artifacts
        mkdir -p /work
        tar -C /src --exclude=./target --exclude=./build --exclude=./.git \
            -cf - . | tar -C /work -xf -
        cd /work
        ./scripts/build-iso-archiso.sh
        mkdir -p /out
        cp -a build/iso/*.iso /out/ 2>/dev/null || true
        cp -a build/iso/checksums /out/ 2>/dev/null || true
        echo "=== container build finished ==="
        ls -lh /out/
    '
