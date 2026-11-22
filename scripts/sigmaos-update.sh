#!/bin/bash
# SigmaOS system update script
# Handles system updates similar to Arch's pacman -Syu

set -e

VERSION="0.5.0"

echo "=== SigmaOS System Update ==="
echo "Version: $VERSION"
echo ""

# Check for root
if [ "$EUID" -ne 0 ]; then
    echo "Error: This script must be run as root"
    exit 1
fi

# Color output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check if running in chroot
if [ "$(stat -c %d:%i /)" != "$(stat -c %d:%i /proc/1/root/.)" ]; then
    warn "Running in chroot environment"
fi

# Update system
info "Updating package databases..."
pacman -Sy

info "Checking for updates..."
UPDATES=$(pacman -Qu | wc -l)

if [ "$UPDATES" -eq 0 ]; then
    info "System is up to date"
    exit 0
fi

echo ""
info "Found $UPDATES package(s) to update"
echo ""

# Show updates
pacman -Qu

echo ""
read -p "Continue with update? (yes/no): " confirm
if [ "$confirm" != "yes" ]; then
    info "Update cancelled"
    exit 0
fi

# Perform update
info "Updating system..."
pacman -Syu --noconfirm

# Update initramfs if kernel was updated
if pacman -Q linux &>/dev/null; then
    info "Updating initramfs..."
    mkinitcpio -P
fi

# Update GRUB if kernel was updated
if pacman -Q linux &>/dev/null; then
    info "Updating GRUB configuration..."
    grub-mkconfig -o /boot/grub/grub.cfg
fi

# Clean package cache
info "Cleaning package cache..."
pacman -Sc --noconfirm

# Remove orphans
ORPHANS=$(pacman -Qtdq)
if [ -n "$ORPHANS" ]; then
    warn "Found orphan packages:"
    echo "$ORPHANS"
    read -p "Remove orphan packages? (yes/no): " remove_orphans
    if [ "$remove_orphans" = "yes" ]; then
        pacman -Rns --noconfirm $ORPHANS
    fi
fi

echo ""
info "=== Update Complete ==="
echo ""
echo "System updated successfully"
echo ""
echo "You may want to reboot if the kernel was updated"
