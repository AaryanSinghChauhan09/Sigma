#!/bin/bash
# SigmaOS system check script
# Runs system health checks on boot

set -e

echo "=== SigmaOS System Check ==="

# Check disk space
echo "Checking disk space..."
DISK_USAGE=$(df -h / | awk 'NR==2 {print $5}' | sed 's/%//')
if [ "$DISK_USAGE" -gt 90 ]; then
    echo "WARNING: Root disk usage is ${DISK_USAGE}%"
fi

# Check memory
echo "Checking memory..."
MEM_AVAILABLE=$(free -m | awk '/^Mem:/ {print $7}')
if [ "$MEM_AVAILABLE" -lt 512 ]; then
    echo "WARNING: Low available memory: ${MEM_AVAILABLE}MB"
fi

# Check for failed services
echo "Checking services..."
FAILED=$(systemctl list-units --state=failed --no-legend | wc -l)
if [ "$FAILED" -gt 0 ]; then
    echo "WARNING: $FAILED failed service(s)"
    systemctl list-units --state=failed --no-legend
fi

# Check for pending updates
echo "Checking for updates..."
UPDATES=$(pacman -Qu 2>/dev/null | wc -l)
if [ "$UPDATES" -gt 0 ]; then
    echo "INFO: $updates package update(s) available"
fi

echo "System check complete"
