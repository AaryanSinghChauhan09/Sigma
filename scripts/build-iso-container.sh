#!/bin/bash
# SigmaOS ISO Build in Docker/Container
# This script builds SigmaOS ISO using Arch Linux container
# Useful for building on non-Arch systems

set -e

VERSION="0.5.0"
CONTAINER_NAME="sigmaos-build"
IMAGE="archlinux:latest"

echo "=== SigmaOS Container Build ==="
echo "Version: $VERSION"
echo ""

# Check for Docker or Podman
if command -v docker &> /dev/null; then
    CONTAINER_CMD="docker"
elif command -v podman &> /dev/null; then
    CONTAINER_CMD="podman"
else
    echo "Error: Neither Docker nor Podman found"
    echo "Install Docker: https://docs.docker.com/get-docker/"
    echo "Or Podman: https://podman.io/getting-started/installation"
    exit 1
fi

echo "Using container runtime: $CONTAINER_CMD"

# Pull Arch Linux image
echo "Pulling Arch Linux image..."
$CONTAINER_CMD pull $IMAGE

# Create build directory
BUILD_DIR="$(pwd)/build"
mkdir -p "$BUILD_DIR"

# Run build in container
echo "Building ISO in container..."
$CONTAINER_CMD run --rm -it \
    --name $CONTAINER_NAME \
    -v "$(pwd):/sigmaos:ro" \
    -v "$BUILD_DIR:/build" \
    -w /sigmaos \
    $IMAGE \
    bash -c "
        set -e
        echo 'Installing build dependencies...'
        pacman -Sy --noconfirm archiso git base-devel
        
        echo 'Running ISO build...'
        ./scripts/build-iso-archiso.sh
        
        echo 'Copying ISO to build directory...'
        cp -r build/iso/* /build/
        
        echo 'Build complete!'
    "

echo ""
echo "=== Build Complete ==="
echo "ISO location: $BUILD_DIR/"
echo ""
echo "ISO files:"
ls -lh "$BUILD_DIR"/*.iso 2>/dev/null || echo "No ISO found in build directory"
echo ""
echo "To test in QEMU:"
echo "  qemu-system-x86_64 -m 2048 -cdrom $BUILD_DIR/sigmaos-$VERSION-x86_64.iso -boot d"
