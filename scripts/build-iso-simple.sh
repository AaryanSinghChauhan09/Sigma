#!/bin/bash
# DEPRECATED: experimental script kept for reference. The supported ISO
# build lives in https://github.com/AaryanSinghChauhan09/SigmaOS-ISO
# (scripts/build_archiso.sh). This script may be stale or broken.
# SigmaOS ISO Build Script (Simplified)
# Creates a bootable SigmaOS live ISO using Arch Linux packages as base
# with SigmaOS configuration layer applied

set -e

VERSION="0.4.0"
BUILD_DATE=$(date +%Y%m%d)
ISO_LABEL="SIGMAOS_${VERSION}"
OUTPUT_DIR="./build/iso"
WORK_DIR="./build/work"
ROOTFS_DIR="$WORK_DIR/rootfs"

# Essential packages for a minimal but functional system
ARCH_PACKAGES=(
    # Core system
    "linux" "linux-firmware" "systemd" "bash" "coreutils" "findutils" "sed" "grep" "gawk"
    "tar" "gzip" "xz" "bzip2" "zstd" "file" "which" "procps-ng" "psmisc" "util-linux"
    
    # Boot and init
    "grub" "efibootmgr" "os-prober" "mkinitcpio" "sudo" "polkit"
    
    # Networking
    "networkmanager" "wpa_supplicant" "openssh" "iproute2" "iptables" "nftables" "iputils" "bind" "inetutils"
    
    # Text editors and tools
    "vim" "nano" "less" "man-db" "man-pages" "tzdata" "locales" "kbd" "console-setup"
    
    # Storage and filesystems
    "parted" "gptfdisk" "dosfstools" "e2fsprogs" "btrfs-progs" "xfsprogs" "cryptsetup" "lvm2"
    
    # Hardware monitoring
    "lm_sensors" "smartmontools" "nvme-cli" "usbutils" "pciutils"
    
    # Development tools
    "git" "curl" "wget" "rsync" "ca-certificates"
    
    # System monitoring
    "htop" "iotop" "strace" "lsof" "tmux" "screen"
)

echo "=== SigmaOS ISO Build Script (Simplified) ==="
echo "Version: $VERSION"
echo "Build Date: $BUILD_DATE"

# Check dependencies
check_dependencies() {
    local deps=("xorriso" "mksquashfs" "bsdtar" "coreutils" "findutils" "sed" "grep")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            echo "Error: $dep is required but not installed"
            exit 1
        fi
    done
    
    # Check for arch-install-scripts (pacstrap, arch-chroot)
    if ! command -v pacstrap &> /dev/null; then
        echo "Warning: pacstrap not found. You need arch-install-scripts on Arch-based systems."
        echo "Install with: sudo pacman -S arch-install-scripts"
        echo "This script will try to continue without pacstrap, but will likely fail."
    fi
}

check_dependencies

# Create directories
mkdir -p "$OUTPUT_DIR" "$WORK_DIR" "$ROOTFS_DIR"

# Download Arch Linux packages
echo "=== Downloading Arch Linux packages ==="
if command -v pacstrap &> /dev/null; then
    cd "$ROOTFS_DIR"
    pacstrap -c -K "$ROOTFS_DIR" base "${ARCH_PACKAGES[@]}"
else
    echo "Error: pacstrap not available. Cannot continue."
    echo "Install arch-install-scripts on Arch-based systems:"
    echo "  sudo pacman -S arch-install-scripts"
    exit 1
fi

# Apply SigmaOS configuration
echo "=== Applying SigmaOS configuration ==="

# Create SigmaOS system directories
mkdir -p "$ROOTFS_DIR/etc/sigmaos"
mkdir -p "$ROOTFS_DIR/etc/sysctl.d"
mkdir -p "$ROOTFS_DIR/etc/systemd/system"
mkdir -p "$ROOTFS_DIR/etc/udev/rules.d"
mkdir -p "$ROOTFS_DIR/etc/fstab.d"

# Set hostname
echo "sigmaos" > "$ROOTFS_DIR/etc/hostname"

# Set locale
echo "en_US.UTF-8 UTF-8" > "$ROOTFS_DIR/etc/locale.gen"
chroot "$ROOTFS_DIR" locale-gen
echo "LANG=en_US.UTF-8" > "$ROOTFS_DIR/etc/locale.conf"

# Set timezone
ln -sf /usr/share/zoneinfo/UTC "$ROOTFS_DIR/etc/localtime"

# Set root password (default: sigmaos)
echo "root:sigmaos" | chroot "$ROOTFS_DIR" chpasswd

# Create sigmaos user
chroot "$ROOTFS_DIR" useradd -m -G wheel -s /bin/bash sigmaos
echo "sigmaos:sigmaos" | chroot "$ROOTFS_DIR" chpasswd
echo "%wheel ALL=(ALL) NOPASSWD: ALL" > "$ROOTFS_DIR/etc/sudoers.d/wheel"

# Enable services
chroot "$ROOTFS_DIR" systemctl enable NetworkManager
chroot "$ROOTFS_DIR" systemctl enable sshd
chroot "$ROOTFS_DIR" systemctl enable systemd-timesyncd

# Create sysctl configuration
cat > "$ROOTFS_DIR/etc/sysctl.d/99-sigmaos.conf" << 'EOF'
# SigmaOS kernel parameters
net.ipv4.ip_forward = 1
net.ipv4.tcp_fastopen = 3
net.ipv4.tcp_congestion_control = bbr
vm.swappiness = 10
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5
EOF

# Create udev rules
cat > "$ROOTFS_DIR/etc/udev/rules.d/99-sigmaos.rules" << 'EOF'
# SigmaOS udev rules
EOF

# Generate initramfs
echo "=== Generating initramfs ==="
chroot "$ROOTFS_DIR" mkinitcpio -P

# Configure bootloader
echo "=== Configuring bootloader ==="
# Note: Actual bootloader configuration requires installation to a disk
# For ISO, we'll create a minimal configuration
mkdir -p "$ROOTFS_DIR/boot/grub"
cat > "$ROOTFS_DIR/boot/grub/grub.cfg" << 'EOF'
set timeout=5
set default=0

menuentry "SigmaOS" {
    linux /boot/vmlinuz-linux root=/dev/sr0 ro quiet
    initrd /boot/initramfs-linux.img
}
EOF

# Create fstab
echo "=== Creating fstab ==="
cat > "$ROOTFS_DIR/etc/fstab" << 'EOF'
# Static file system information
proc /proc proc defaults 0 0
EOF

# Create SigmaOS version file
cat > "$ROOTFS_DIR/etc/sigmaos/version" << EOF
SigmaOS $VERSION
Build Date: $BUILD_DATE
Repository: https://github.com/AaryanSinghChauhan09/Sigma
EOF

# Compress rootfs
echo "=== Compressing rootfs ==="
cd "$WORK_DIR"
mksquashfs "$ROOTFS_DIR" "$OUTPUT_DIR/rootfs.squashfs" -comp xz -b 1M

# Create ISO
echo "=== Creating ISO ==="
xorriso -as mkisofs \
    -r -V "$ISO_LABEL" \
    -o "$OUTPUT_DIR/sigmaos-$VERSION-$BUILD_DATE.iso" \
    -b boot/grub/grub.cfg \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    "$ROOTFS_DIR"

# Generate checksum
echo "=== Generating checksum ==="
cd "$OUTPUT_DIR"
sha256sum "sigmaos-$VERSION-$BUILD_DATE.iso" > "sigmaos-$VERSION-$BUILD_DATE.iso.sha256"

echo "=== Build complete ==="
echo "ISO: $OUTPUT_DIR/sigmaos-$VERSION-$BUILD_DATE.iso"
echo "SHA256: $OUTPUT_DIR/sigmaos-$VERSION-$BUILD_DATE.iso.sha256"
echo ""
echo "To test in QEMU:"
echo "  qemu-system-x86_64 -m 2048 -cdrom $OUTPUT_DIR/sigmaos-$VERSION-$BUILD_DATE.iso"
echo ""
echo "To write to USB:"
echo "  sudo dd if=$OUTPUT_DIR/sigmaos-$VERSION-$BUILD_DATE.iso of=/dev/sdX bs=4M status=progress"
