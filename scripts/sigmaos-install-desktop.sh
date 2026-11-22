#!/bin/bash
# SigmaOS desktop environment installation script
# Installs KDE Plasma, GNOME, or XFCE desktop environments

set -e

echo "=== SigmaOS Desktop Installation ==="
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

# Select desktop environment
section "Desktop Environment Selection"
echo "Select desktop environment:"
echo "  1) KDE Plasma (Recommended)"
echo "  2) GNOME"
echo "  3) XFCE (Lightweight)"
echo "  4) i3 (Tiling window manager)"
echo "  5) No desktop (Headless server)"
read -p "Select option [1-5]: " de_choice

case $de_choice in
    1)
        DE="plasma"
        PACKAGES="plasma-desktop plasma-nm kde-gtk-config breeze-gtk-theme dolphin konsole okular gwenview kate ark kcalc spectacle"
        DISPLAY_MANAGER="sddm"
        ;;
    2)
        DE="gnome"
        PACKAGES="gnome gnome-tweaks gnome-extra gdm"
        DISPLAY_MANAGER="gdm"
        ;;
    3)
        DE="xfce"
        PACKAGES="xfce4 xfce4-goodies lightdm lightdm-gtk-greeter"
        DISPLAY_MANAGER="lightdm"
        ;;
    4)
        DE="i3"
        PACKAGES="i3-wm i3status i3lock dmenu rofi polybar picom feh xterm"
        DISPLAY_MANAGER="lightdm"
        ;;
    5)
        DE="none"
        PACKAGES=""
        DISPLAY_MANAGER=""
        ;;
    *)
        warn "Invalid choice, exiting"
        exit 1
        ;;
esac

if [ "$DE" = "none" ]; then
    info "Skipping desktop installation"
    exit 0
fi

section "Installing $DE"
info "This may take several minutes..."
pacman -S --noconfirm $PACKAGES

# Install display manager
if [ -n "$DISPLAY_MANAGER" ]; then
    section "Installing Display Manager"
    pacman -S --noconfirm $DISPLAY_MANAGER
    systemctl enable $DISPLAY_MANAGER
    info "$DISPLAY_MANAGER enabled"
fi

# Install common desktop applications
section "Installing Common Applications"
pacman -S --noconfirm \
    firefox \
    thunderbird \
    libreoffice-fresh \
    vlc \
    gimp \
    inkscape \
    okular \
    file-roller \
    p7zip \
    unrar \
    unzip \
    zip

# Install fonts
section "Installing Fonts"
pacman -S --noconfirm \
    ttf-dejavu \
    ttf-liberation \
    ttf-roboto \
    noto-fonts \
    noto-fonts-cjk \
    noto-fonts-emoji \
    adobe-source-code-pro-fonts

# Install audio (PipeWire)
section "Installing Audio (PipeWire)"
pacman -S --noconfirm \
    pipewire \
    pipewire-pulse \
    pipewire-alsa \
    pipewire-jack \
    wireplumber
systemctl --user enable pipewire pipewire-pulse wireplumber
info "Audio installed"

# Install printer support
section "Installing Printer Support"
pacman -S --noconfirm cups cups-pdf system-config-printer
systemctl enable cups
info "Printer support installed"

# Configure display manager
section "Configuring Display Manager"
if [ "$DISPLAY_MANAGER" = "sddm" ]; then
    systemctl enable sddm
elif [ "$DISPLAY_MANAGER" = "gdm" ]; then
    systemctl enable gdm
elif [ "$DISPLAY_MANAGER" = "lightdm" ]; then
    systemctl enable lightdm
fi

# Enable NetworkManager in desktop
section "Configuring Network"
systemctl enable NetworkManager

# Update initramfs
section "Updating Initramfs"
mkinitcpio -P

# Update GRUB
section "Updating GRUB"
if [ -f /etc/default/grub ]; then
    sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="quiet"/GRUB_CMDLINE_LINUX_DEFAULT="quiet splash"/' /etc/default/grub
    grub-mkconfig -o /boot/grub/grub.cfg
fi

echo ""
echo "=========================================="
info "=== Desktop Installation Complete ==="
echo "=========================================="
echo ""
echo "Desktop environment: $DE"
echo "Display manager: $DISPLAY_MANAGER"
echo ""
echo "You can now reboot into your new desktop environment"
echo "To reboot: reboot"
echo "=========================================="
