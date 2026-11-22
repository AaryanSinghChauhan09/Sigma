#!/bin/bash
# SigmaOS driver installation script
# Installs proprietary drivers for NVIDIA, AMD, Intel

set -e

echo "=== SigmaOS Driver Installation ==="
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

# Detect GPU
section "GPU Detection"
if command -v lspci &> /dev/null; then
    GPU_INFO=$(lspci | grep -i vga)
    echo "$GPU_INFO"
    echo ""
else
    warn "lspci not available"
    exit 1
fi

# Install NVIDIA drivers
if echo "$GPU_INFO" | grep -iq nvidia; then
    section "NVIDIA Driver Installation"
    echo "Select NVIDIA driver type:"
    echo "  1) Open source (nouveau)"
    echo "  2) Proprietary (nvidia)"
    echo "  3) Proprietary with OpenGL support (nvidia)"
    echo "  4) Proprietary for 32-bit applications (nvidia-libgl)"
    read -p "Select option [1-4]: " nvidia_choice

    case $nvidia_choice in
        1)
            info "Using open source nouveau driver (included in kernel)"
            ;;
        2)
            info "Installing proprietary NVIDIA driver..."
            pacman -S --noconfirm nvidia
            info "NVIDIA driver installed"
            ;;
        3)
            info "Installing proprietary NVIDIA driver with OpenGL..."
            pacman -S --noconfirm nvidia libgl
            info "NVIDIA driver with OpenGL installed"
            ;;
        4)
            info "Installing proprietary NVIDIA driver with 32-bit support..."
            pacman -S --noconfirm nvidia lib32-nvidia-utils libgl lib32-libgl
            info "NVIDIA driver with 32-bit support installed"
            ;;
        *)
            warn "Invalid choice, skipping NVIDIA driver installation"
            ;;
    esac
fi

# Install AMD drivers
if echo "$GPU_INFO" | grep -iq amd; then
    section "AMD Driver Installation"
    info "AMD GPUs use open source amdgpu driver (included in kernel)"
    info "Installing additional AMD tools..."
    pacman -S --noconfirm xf86-video-amdgpu vulkan-radeon libva-mesa-driver mesa-vdpau
    info "AMD driver tools installed"
fi

# Install Intel drivers
if echo "$GPU_INFO" | grep -iq intel; then
    section "Intel Driver Installation"
    info "Intel GPUs use open source i915 driver (included in kernel)"
    info "Installing additional Intel tools..."
    pacman -S --noconfirm xf86-video-intel vulkan-intel libva-intel-driver intel-media-driver
    info "Intel driver tools installed"
fi

# Install audio drivers
section "Audio Driver Installation"
pacman -S --noconfirm pipewire pipewire-pulse pipewire-alsa pipewire-jack
systemctl --user enable pipewire pipewire-pulse wireplumber
info "Audio drivers (PipeWire) installed"

# Install Bluetooth drivers
section "Bluetooth Driver Installation"
pacman -S --noconfirm bluez bluez-utils
systemctl enable bluetooth
info "Bluetooth drivers installed"

# Install WiFi drivers
section "WiFi Driver Installation"
pacman -S --noconfirm wpa_supplicant wireless_tools iw
info "WiFi drivers installed"

# Update initramfs
section "Updating Initramfs"
mkinitcpio -P
info "Initramfs updated"

# Update GRUB
section "Updating GRUB"
grub-mkconfig -o /boot/grub/grub.cfg
info "GRUB updated"

echo ""
echo "=========================================="
info "=== Driver Installation Complete ==="
echo "=========================================="
echo ""
echo "You may need to reboot for drivers to take effect"
echo "To reboot: reboot"
