#!/bin/bash
# SigmaOS post-installation configuration script
# Run after installation to configure the installed system

set -e

VERSION="0.5.0"

echo "=== SigmaOS Post-Installation Configuration ==="
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
BLUE='\033[0;34m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
step() { echo -e "${BLUE}[STEP]${NC} $1"; }

# Configuration variables
HOSTNAME="sigmaos"
USERNAME="sigmaos"
TIMEZONE="UTC"
LOCALE="en_US.UTF-8"

# Functions
configure_hostname() {
    step "Configuring hostname..."
    read -p "Enter hostname [default: sigmaos]: " input_hostname
    HOSTNAME=${input_hostname:-$HOSTNAME}
    echo "$HOSTNAME" > /etc/hostname
    info "Hostname set to: $HOSTNAME"
}

configure_locale() {
    step "Configuring locale..."
    echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
    locale-gen
    echo "LANG=en_US.UTF-8" > /etc/locale.conf
    export LANG=en_US.UTF-8
    info "Locale configured"
}

configure_timezone() {
    step "Configuring timezone..."
    read -p "Enter timezone [default: UTC]: " input_timezone
    TIMEZONE=${input_timezone:-$TIMEZONE}
    ln -sf "/usr/share/zoneinfo/$TIMEZONE" /etc/localtime
    info "Timezone set to: $TIMEZONE"
}

configure_clock() {
    step "Configuring hardware clock..."
    hwclock --systohc
    info "Hardware clock synchronized"
}

configure_user() {
    step "Configuring user..."
    if id "$USERNAME" &>/dev/null; then
        info "User $USERNAME already exists"
        read -p "Change password for $USERNAME? (yes/no): " change_pw
        if [ "$change_pw" = "yes" ]; then
            passwd "$USERNAME"
        fi
    else
        info "Creating user $USERNAME"
        useradd -m -G wheel -s /bin/bash "$USERNAME"
        read -p "Set password for $USERNAME: " -s password
        echo "$USERNAME:$password" | chpasswd
        echo "%wheel ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/wheel
        chmod 440 /etc/sudoers.d/wheel
        info "User created and configured"
    fi
}

configure_network() {
    step "Configuring network..."
    systemctl enable NetworkManager
    systemctl start NetworkManager
    info "NetworkManager enabled and started"
}

configure_ssh() {
    step "Configuring SSH..."
    systemctl enable sshd
    systemctl start sshd
    info "SSH enabled and started"
}

configure_systemd() {
    step "Configuring systemd services..."
    systemctl enable systemd-timesyncd
    systemctl start systemd-timesyncd
    info "Systemd services configured"
}

configure_sysctl() {
    step "Configuring sysctl parameters..."
    cat > /etc/sysctl.d/99-sigmaos.conf << 'EOF'
# SigmaOS kernel parameters
net.ipv4.ip_forward = 1
net.ipv4.tcp_fastopen = 3
net.ipv4.tcp_congestion_control = bbr
vm.swappiness = 10
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5
net.core.default_qdisc = fq
net.ipv4.tcp_slow_start_after_idle = 0
EOF
    sysctl --system
    info "Sysctl parameters applied"
}

configure_mkinitcpio() {
    step "Configuring mkinitcpio..."
    if [ -f /etc/sigmaos/mkinitcpio.conf ]; then
        cp /etc/sigmaos/mkinitcpio.conf /etc/mkinitcpio.conf
        mkinitcpio -P
        info "Initramfs regenerated"
    else
        warn "mkinitcpio.conf not found, skipping"
    fi
}

configure_grub() {
    step "Configuring GRUB..."
    if [ -f /etc/sigmaos/grub.conf ]; then
        cp /etc/sigmaos/grub.conf /etc/default/grub
    fi
    grub-mkconfig -o /boot/grub/grub.cfg
    info "GRUB configured"
}

install_microcode() {
    step "Installing CPU microcode..."
    if grep -q "Intel" /proc/cpuinfo; then
        pacman -S --noconfirm intel-ucode
        info "Intel microcode installed"
    elif grep -q "AMD" /proc/cpuinfo; then
        pacman -S --noconfirm amd-ucode
        info "AMD microcode installed"
    else
        warn "Unable to detect CPU vendor"
    fi
}

install_additional_packages() {
    step "Installing additional packages..."
    pacman -S --noconfirm \
        man-db man-pages \
        htop btop neofetch \
        tmux screen \
        git curl wget \
        vim nano \
        rsync \
        base-devel
    info "Additional packages installed"
}

create_user_directories() {
    step "Creating user directories..."
    if id "$USERNAME" &>/dev/null; then
        mkdir -p "/home/$USERNAME/Documents"
        mkdir -p "/home/$USERNAME/Downloads"
        mkdir -p "/home/$USERNAME/Pictures"
        mkdir -p "/home/$USERNAME/Music"
        mkdir -p "/home/$USERNAME/Videos"
        chown -R "$USERNAME:$USERNAME" "/home/$USERNAME"
        info "User directories created"
    fi
}

set_default_editor() {
    step "Setting default editor..."
    export EDITOR=vim
    echo "export EDITOR=vim" >> /etc/profile
    info "Default editor set to vim"
}

# Main configuration flow
main() {
    echo "This script will configure your SigmaOS installation."
    echo ""
    read -p "Continue? (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        info "Configuration cancelled"
        exit 0
    fi

    configure_hostname
    configure_locale
    configure_timezone
    configure_clock
    configure_user
    configure_network
    configure_ssh
    configure_systemd
    configure_sysctl
    configure_mkinitcpio
    configure_grub
    install_microcode
    install_additional_packages
    create_user_directories
    set_default_editor

    echo ""
    echo "=========================================="
    info "=== Configuration Complete ==="
    echo "=========================================="
    echo ""
    echo "System configured successfully!"
    echo ""
    echo "Hostname: $HOSTNAME"
    echo "User: $USERNAME"
    echo "Timezone: $TIMEZONE"
    echo ""
    echo "You can now reboot into your configured system."
    echo "To reboot: reboot"
    echo "=========================================="
}

main
