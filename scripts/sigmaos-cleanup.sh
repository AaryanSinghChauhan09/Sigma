#!/bin/bash
# SigmaOS system cleanup script
# Cleans up old packages, cache, and temporary files

set -e

echo "=== SigmaOS System Cleanup ==="
echo ""

# Check for root
if [ "$EUID" -ne 0 ]; then
    echo "Error: This script must be run as root"
    exit 1
fi

# Color output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
section() { echo -e "${BLUE}[${1}]${NC}"; }

# Clean package cache
section "Package Cache"
CACHE_SIZE=$(du -sh /var/cache/pacman/pkg 2>/dev/null | cut -f1)
echo "Current cache size: $CACHE_SIZE"
read -p "Clean package cache? (yes/no): " clean_cache
if [ "$clean_cache" = "yes" ]; then
    pacman -Sc --noconfirm
    info "Package cache cleaned"
fi

# Remove orphan packages
section "Orphan Packages"
ORPHANS=$(pacman -Qtdq)
if [ -n "$ORPHANS" ]; then
    echo "Found orphan packages:"
    echo "$ORPHANS"
    read -p "Remove orphan packages? (yes/no): " remove_orphans
    if [ "$remove_orphans" = "yes" ]; then
        pacman -Rns --noconfirm $ORPHANS
        info "Orphan packages removed"
    fi
else
    info "No orphan packages found"
fi

# Clean journal logs
section "Journal Logs"
JOURNAL_SIZE=$(journalctl --disk-usage | grep -oP 'Journals take \K[^ ]+')
echo "Current journal size: $JOURNAL_SIZE"
read -p "Clean journal logs (keep last 7 days)? (yes/no): " clean_journal
if [ "$clean_journal" = "yes" ]; then
    journalctl --vacuum-time=7d
    info "Journal logs cleaned"
fi

# Clean temporary files
section "Temporary Files"
TMP_SIZE=$(du -sh /tmp 2>/dev/null | cut -f1)
echo "Current /tmp size: $TMP_SIZE"
read -p "Clean /tmp directory? (yes/no): " clean_tmp
if [ "$clean_tmp" = "yes" ]; then
    rm -rf /tmp/*
    info "Temporary files cleaned"
fi

# Clean thumbnails
section "Thumbnails"
if [ -d /home ]; then
    for user in /home/*; do
        if [ -d "$user/.cache/thumbnails" ]; then
            THUMB_SIZE=$(du -sh "$user/.cache/thumbnails" 2>/dev/null | cut -f1)
            echo "Thumbnails for $(basename $user): $THUMB_SIZE"
        fi
    done
    read -p "Clean thumbnails for all users? (yes/no): " clean_thumbs
    if [ "$clean_thumbs" = "yes" ]; then
        for user in /home/*; do
            rm -rf "$user/.cache/thumbnails"/* 2>/dev/null || true
        done
        info "Thumbnails cleaned"
    fi
fi

# Clean old kernels
section "Old Kernels"
KERNELS=$(ls /boot/vmlinuz-* 2>/dev/null | wc -l)
echo "Current kernels: $KERNELS"
if [ "$KERNELS" -gt 2 ]; then
    warn "Multiple kernels detected"
    read -p "Clean old kernels (keep current and fallback)? (yes/no): " clean_kernels
    if [ "$clean_kernels" = "yes" ]; then
        CURRENT_KERNEL=$(uname -r)
        # Keep current and fallback, remove others
        ls /boot/vmlinuz-* 2>/dev/null | while read kernel; do
            KERNEL_VER=$(basename "$kernel" | sed 's/vmlinuz-//')
            if [ "$KERNEL_VER" != "$CURRENT_KERNEL" ] && [[ ! "$KERNEL_VER" =~ -fallback$ ]]; then
                warn "Manual kernel cleanup recommended"
                break
            fi
        done
        info "Manual kernel cleanup recommended (use pacman -Rns linux-<version>)"
    fi
else
    info "No old kernels to clean"
fi

# Clean pacman log
section "Pacman Log"
read -p "Clean pacman log? (yes/no): " clean_pacman_log
if [ "$clean_pacman_log" = "yes" ]; then
    > /var/log/pacman.log
    info "Pacman log cleaned"
fi

echo ""
echo "=========================================="
info "=== Cleanup Complete ==="
echo "=========================================="
echo ""
echo "System cleanup completed"
