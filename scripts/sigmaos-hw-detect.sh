#!/bin/bash
# SigmaOS hardware detection script
# Detects and reports system hardware information

set -e

echo "=== SigmaOS Hardware Detection ==="
echo ""

# Color output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
section() { echo -e "${BLUE}[${1}]${NC}"; }

echo "Detecting system hardware..."
echo ""

# CPU
section "CPU"
echo "Model: $(grep 'model name' /proc/cpuinfo | head -n1 | cut -d: -f2 | xargs)"
echo "Cores: $(nproc)"
echo "Architecture: $(uname -m)"
echo ""

# Memory
section "Memory"
TOTAL_MEM=$(free -h | awk '/^Mem:/ {print $2}')
AVAILABLE_MEM=$(free -h | awk '/^Mem:/ {print $7}')
echo "Total: $TOTAL_MEM"
echo "Available: $AVAILABLE_MEM"
echo ""

# Disk
section "Disk Storage"
lsblk -d -o NAME,SIZE,MODEL | grep -v "^NAME"
echo ""

# GPU
section "Graphics"
if command -v lspci &> /dev/null; then
    lspci | grep -i vga
else
    info "lspci not available"
fi
echo ""

# Network
section "Network"
ip link show | grep -E "^[0-9]+:" | awk '{print $2}' | tr -d ':'
echo ""

# Audio
section "Audio"
if command -v lspci &> /dev/null; then
    lspci | grep -i audio
else
    info "lspci not available"
fi
echo ""

# Bluetooth
section "Bluetooth"
if command -v lsusb &> /dev/null; then
    lsusb | grep -i bluetooth || info "No Bluetooth detected"
else
    info "lsusb not available"
fi
echo ""

# Sensors
section "Sensors"
if command -v sensors &> /dev/null; then
    sensors
else
    info "lm_sensors not available (install with: pacman -S lm_sensors)"
fi
echo ""

# Battery (laptops)
section "Battery"
if [ -d /sys/class/power_supply/BAT0 ]; then
    CAPACITY=$(cat /sys/class/power_supply/BAT0/capacity 2>/dev/null || echo "N/A")
    STATUS=$(cat /sys/class/power_supply/BAT0/status 2>/dev/null || echo "N/A")
    echo "Capacity: $CAPACITY%"
    echo "Status: $STATUS"
else
    info "No battery detected (desktop system)"
fi
echo ""

# UEFI/BIOS
section "Firmware"
if [ -d /sys/firmware/efi ]; then
    echo "Boot mode: UEFI"
else
    echo "Boot mode: BIOS/Legacy"
fi
echo ""

# Kernel
section "Kernel"
echo "Version: $(uname -r)"
echo "Compiler: $(cat /proc/version | grep -oP 'gcc version \K[0-9.]+')"
echo ""

# Summary
echo "=========================================="
info "Hardware detection complete"
echo "=========================================="
