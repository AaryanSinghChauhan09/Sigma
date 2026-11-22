#!/bin/bash
# SigmaOS Installation Script
# Simple text-based installer inspired by archinstall
# Handles partitioning, formatting, and system installation

set -e

VERSION="0.5.0"
SIGMAOS_VERSION="SigmaOS $VERSION"

echo "=== SigmaOS Installer ==="
echo "$SIGMAOS_VERSION"
echo "Based on Arch Linux"
echo ""

# Check for root
if [ "$EUID" -ne 0 ]; then
    echo "Error: This script must be run as root"
    exit 1
fi

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }
step() { echo -e "${BLUE}[STEP]${NC} $1"; }

# Installation variables
TARGET_DISK=""
ROOT_PART=""
BOOT_PART=""
SWAP_PART=""
ROOT_FS="ext4"
BOOT_FS="vfat"
HOSTNAME="sigmaos"
USERNAME="sigmaos"
TIMEZONE="UTC"
LOCALE="en_US.UTF-8"

# Functions
select_disk() {
    echo ""
    echo "Available disks:"
    lsblk -d -n -o NAME,SIZE,MODEL | nl
    echo ""
    read -p "Select disk number (e.g., 1): " disk_num
    disk_name=$(lsblk -d -n -o NAME | sed -n "${disk_num}p")
    
    if [ -z "$disk_name" ]; then
        error "Invalid disk selection"
        exit 1
    fi
    
    TARGET_DISK="/dev/$disk_name"
    info "Selected disk: $TARGET_DISK"
    
    # Confirm
    echo ""
    warn "WARNING: All data on $TARGET_DISK will be erased!"
    read -p "Continue? (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        info "Installation cancelled"
        exit 0
    fi
}

partition_disk() {
    step "Partitioning $TARGET_DISK..."
    
    # Clear partition table
    wipefs -a "$TARGET_DISK" 2>/dev/null || true
    sgdisk --zap-all "$TARGET_DISK" 2>/dev/null || true
    sleep 2
    
    # Create partitions: EFI (512M), Swap (2G), Root (rest)
    sgdisk -n 1:0:+512M -t 1:EF00 "$TARGET_DISK"  # EFI
    sgdisk -n 2:0:+2G -t 2:8200 "$TARGET_DISK"     # Swap
    sgdisk -n 3:0:0 -t 3:8300 "$TARGET_DISK"      # Root
    
    partprobe "$TARGET_DISK" 2>/dev/null || true
    sleep 2
    
    BOOT_PART="${TARGET_DISK}1"
    SWAP_PART="${TARGET_DISK}2"
    ROOT_PART="${TARGET_DISK}3"
    
    info "Created partitions:"
    info "  Boot: $BOOT_PART"
    info "  Swap: $SWAP_PART"
    info "  Root: $ROOT_PART"
}

format_partitions() {
    step "Formatting partitions..."
    
    mkfs.vfat -F32 "$BOOT_PART"
    mkfs.ext4 -F "$ROOT_PART"
    mkswap "$SWAP_PART"
    
    info "Partitions formatted"
}

mount_filesystems() {
    step "Mounting filesystems..."
    
    mount "$ROOT_PART" /mnt
    mkdir -p /mnt/boot
    mount "$BOOT_PART" /mnt/boot
    swapon "$SWAP_PART"
    
    info "Filesystems mounted"
}

install_base_system() {
    step "Installing base system..."
    info "This may take several minutes..."
    
    pacstrap -K /mnt base linux linux-firmware linux-headers systemd \
               networkmanager wpa_supplicant openssh sudo vim nano \
               bash coreutils findutils sed grep gawk tar gzip xz bzip2 \
               zstd file which procps-ng util-linux grub efibootmgr \
               mkinitcpio dhclient dialog iw wireless-regdb iproute2 \
               iptables nftables iputils curl wget git man-db man-pages \
               tzdata locales kbd console-setup parted gptfdisk dosfstools \
               e2fsprogs btrfs-progs xfsprogs cryptsetup lvm2 mdadm \
               lm_sensors smartmontools nvme-cli usbutils pciutils htop \
               iotop strace lsof tmux screen neofetch btop
    
    info "Base system installed"
}

configure_system() {
    step "Configuring system..."
    
    # Generate fstab
    genfstab -U /mnt >> /mnt/etc/fstab
    
    # Set hostname
    echo "$HOSTNAME" > /mnt/etc/hostname
    
    # Set locale
    echo "en_US.UTF-8 UTF-8" > /mnt/etc/locale.gen
    chroot /mnt locale-gen
    echo "LANG=en_US.UTF-8" > /mnt/etc/locale.conf
    
    # Set timezone
    ln -sf "/usr/share/zoneinfo/$TIMEZONE" /mnt/etc/localtime
    
    # Set root password
    echo "root:sigmaos" | chroot /mnt chpasswd
    
    # Create user
    chroot /mnt useradd -m -G wheel -s /bin/bash "$USERNAME"
    echo "$USERNAME:sigmaos" | chroot /mnt chpasswd
    echo "%wheel ALL=(ALL) NOPASSWD: ALL" > /mnt/etc/sudoers.d/wheel
    
    # Enable services
    chroot /mnt systemctl enable NetworkManager
    chroot /mnt systemctl enable sshd
    chroot /mnt systemctl enable systemd-timesyncd
    
    # Configure sysctl
    cat > /mnt/etc/sysctl.d/99-sigmaos.conf << 'EOF'
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
    
    # Create SigmaOS version file
    mkdir -p /mnt/etc/sigmaos
    cat > /mnt/etc/sigmaos/version << EOF
SigmaOS $VERSION
Repository: https://github.com/AaryanSinghChauhan09/Sigma
Based on: Arch Linux
EOF
    
    # Create MOTD
    cat > /mnt/etc/motd << 'EOF'
==========================================
       Welcome to SigmaOS
==========================================

SigmaOS is a Linux distribution based on Arch Linux.
Repository: https://github.com/AaryanSinghChauhan09/Sigma

Default user: sigmaos
Default password: sigmaos

Please change your password after first boot.
==========================================
EOF
    
    info "System configured"
}

install_bootloader() {
    step "Installing bootloader..."
    
    chroot /mnt mkinitcpio -P
    chroot /mnt grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=SigmaOS --recheck
    
    cat > /mnt/etc/default/grub << 'EOF'
GRUB_DEFAULT=0
GRUB_TIMEOUT=5
GRUB_DISTRIBUTOR="SigmaOS"
GRUB_CMDLINE_LINUX_DEFAULT="quiet splash"
GRUB_CMDLINE_LINUX=""
EOF
    
    chroot /mnt grub-mkconfig -o /boot/grub/grub.cfg
    
    info "Bootloader installed"
}

cleanup() {
    step "Cleaning up..."
    
    umount -R /mnt 2>/dev/null || true
    swapoff -a 2>/dev/null || true
    
    info "Installation complete!"
}

# Main installation flow
main() {
    echo "This will install SigmaOS to a disk."
    echo "All data on the selected disk will be erased."
    echo ""
    
    select_disk
    partition_disk
    format_partitions
    mount_filesystems
    install_base_system
    configure_system
    install_bootloader
    cleanup
    
    echo ""
    echo "=========================================="
    info "=== Installation Complete ==="
    echo "=========================================="
    echo ""
    echo "You can now reboot into your new SigmaOS system."
    echo ""
    echo "Default user: $USERNAME"
    echo "Default password: sigmaos"
    echo ""
    echo "Please change your password after first boot."
    echo ""
    echo "To reboot: reboot"
    echo "=========================================="
}

main
