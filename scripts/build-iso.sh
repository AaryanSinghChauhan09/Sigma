#!/bin/bash
# SigmaOS ISO Build Script
# This script builds the SigmaOS live ISO for testing and distribution

set -e

echo "=== SigmaOS ISO Build Script ==="

# Check dependencies
command -v qemu >/dev/null 2>&1 || { echo >&2 "QEMU is required but not installed. Aborting."; exit 1; }
command -v mktemp >/dev/null 2>&1 || { echo >&2 "mktemp is required but not installed. Aborting."; exit 1; }

# Create build directory
BUILD_DIR=$(mktemp -d /tmp/sigmaos-iso.XXXXXX)
trap "rm -rf $BUILD_DIR" EXIT

echo "Build directory: $BUILD_DIR"

# Create ISO directory structure
ISODIR="$BUILD_DIR/iso"
mkdir -p "$ISODIR/boot"

# Copy kernel and initramfs (placeholder - would be built from SigmaOS source)
# cp arch/boot/x86_64/vmlinuz "$ISODIR/boot/"
# cp arch/boot/x86_64/initramfs.img "$ISODIR/boot/"

# Create isolinux configuration (placeholder)
mkdir -p "$ISODIR/boot/extlinux"
echo "label sigmaos" > "$ISODIR/boot/extlinux/extlinux.conf"
echo "  menu label ^SigmaOS" >> "$ISODIR/boot/extlinux/extlinux.conf"
echo "  kernel /boot/vmlinuz" >> "$ISODIR/boot/extlinux/extlinux.conf"
echo "  append initrd=/boot/initramfs.img quiet splash" >> "$ISODIR/boot/extlinux/extlinux.conf"

# Create ISO image (placeholder)
echo "Creating ISO image..."
echo "This is a placeholder ISO build script."
echo "In a full implementation, this would use mkisofs or genisoimage."

echo "=== ISO Build Complete ==="
echo "ISO image would be located at: $BUILD_DIR/sigmaos.iso"
echo "Remember to clean up: rm -rf $BUILD_DIR"
