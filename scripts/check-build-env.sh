#!/bin/bash
# SigmaOS Build Environment Checker
# Validates that the system has required tools for ISO building

set -e

echo "=== SigmaOS Build Environment Check ==="
echo ""

REQUIRED_TOOLS=(
    "bash"
    "git"
    "sed"
    "grep"
    "awk"
)

ARCH_REQUIRED_TOOLS=(
    "pacman"
    "mkarchiso"
    "pacstrap"
    "genfstab"
)

CONTAINER_OPTIONAL=(
    "docker"
    "podman"
)

MISSING=0
ARCH_MISSING=0

echo "Checking basic tools..."
for tool in "${REQUIRED_TOOLS[@]}"; do
    if command -v $tool &> /dev/null; then
        echo "  ✓ $tool"
    else
        echo "  ✗ $tool (missing)"
        MISSING=1
    fi
done

echo ""
echo "Checking Arch-specific tools..."
for tool in "${ARCH_REQUIRED_TOOLS[@]}"; do
    if command -v $tool &> /dev/null; then
        echo "  ✓ $tool"
    else
        echo "  ✗ $tool (missing)"
        ARCH_MISSING=1
    fi
done

echo ""
echo "Checking container runtimes (optional)..."
for tool in "${CONTAINER_OPTIONAL[@]}"; do
    if command -v $tool &> /dev/null; then
        echo "  ✓ $tool"
    else
        echo "  - $tool (not found, optional)"
    fi
done

echo ""
echo "=== Summary ==="

if [ $ARCH_MISSING -eq 0 ]; then
    echo "✓ You can build ISO directly on this Arch system"
    echo "  Run: ./scripts/build-iso-archiso.sh"
elif [ $MISSING -eq 0 ]; then
    echo "✓ You can build ISO using container"
    echo "  Run: ./scripts/build-iso-container.sh"
else
    echo "✗ Missing required tools"
    echo "  Install missing tools and try again"
    exit 1
fi

echo ""
echo "=== Build Options ==="
echo ""
echo "Option 1: Direct build (requires Arch Linux)"
echo "  ./scripts/build-iso-archiso.sh"
echo ""
echo "Option 2: Container build (requires Docker or Podman)"
echo "  ./scripts/build-iso-container.sh"
echo ""
echo "Option 3: Manual build (see BOOTABLE_PLAN.md)"
echo "  Clone on Arch system and build manually"
