#!/bin/bash
# SigmaOS chroot helper script
# Used during installation to chroot into installed system

set -e

VERSION="0.5.0"

echo "=== SigmaOS Chroot Helper ==="
echo "Version: $VERSION"
echo ""

# Check if /mnt is mounted
if ! mountpoint -q /mnt; then
    echo "Error: /mnt is not mounted"
    echo "Run installer first or mount partitions manually"
    exit 1
fi

# Check for root
if [ "$EUID" -ne 0 ]; then
    echo "Error: This script must be run as root"
    exit 1
fi

# Color output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }

# Copy configuration files to chroot
info "Copying SigmaOS configuration files..."

# Copy sysctl configuration
mkdir -p /mnt/etc/sysctl.d
cp config/99-sigmaos-sysctl.conf /mnt/etc/sysctl.d/99-sigmaos.conf 2>/dev/null || true

# Copy GRUB configuration
mkdir -p /mnt/etc/default
cp config/grub.conf /mnt/etc/default/grub 2>/dev/null || true

# Copy NetworkManager configuration
mkdir -p /mnt/etc/NetworkManager
cp config/NetworkManager.conf /mnt/etc/NetworkManager/NetworkManager.conf 2>/dev/null || true

# Copy resolved configuration
cp config/resolved.conf /mnt/etc/systemd/resolved.conf 2>/dev/null || true

# Copy mkinitcpio configuration
cp config/mkinitcpio.conf /mnt/etc/mkinitcpio.conf 2>/dev/null || true

# Copy kernel parameters
cp config/kernel-params.conf /mnt/etc/kernel-params.conf 2>/dev/null || true

info "Configuration files copied"

# Enter chroot
info "Entering chroot environment..."
echo ""
echo "Available commands in chroot:"
echo "  - pacman: Package management"
echo "  - systemctl: Service management"
echo "  - mkinitcpio: Initramfs generation"
echo "  - grub-install: Bootloader installation"
echo "  - grub-mkconfig: GRUB configuration"
echo ""
echo "Type 'exit' to leave chroot"
echo ""

# Mount necessary filesystems
mount --bind /dev /mnt/dev
mount --bind /dev/pts /mnt/dev/pts
mount --bind /proc /mnt/proc
mount --bind /sys /mnt/sys
mount --bind /run /mnt/run

# Enter chroot
chroot /mnt /bin/bash

# Cleanup on exit
info "Cleaning up mounts..."
umount -R /mnt/dev /mnt/proc /mnt/sys /mnt/run 2>/dev/null || true

info "Chroot session ended"
