#!/bin/bash
# SigmaOS daily maintenance script
# Runs automated maintenance tasks

set -e

echo "=== SigmaOS Daily Maintenance ==="
echo "$(date)"

# Clean journal logs older than 30 days
echo "Cleaning old journal logs..."
journalctl --vacuum-time=30d

# Clean package cache
echo "Cleaning package cache..."
pacman -Sc --noconfirm

# Check for orphan packages
ORPHANS=$(pacman -Qtdq)
if [ -n "$ORPHANS" ]; then
    echo "Removing orphan packages..."
    pacman -Rns --noconfirm $ORPHANS
fi

# Clean temporary files
echo "Cleaning temporary files..."
find /tmp -type f -atime +7 -delete 2>/dev/null || true

# Update mlocate database
if command -v updatedb &> /dev/null; then
    echo "Updating file database..."
    updatedb
fi

# Update man database
echo "Updating man database..."
mandb --quiet

echo "Daily maintenance complete"
