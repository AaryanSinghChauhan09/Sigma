#!/bin/bash
# DEPRECATED: experimental script kept for reference. The supported ISO
# build lives in https://github.com/AaryanSinghChauhan09/SigmaOS-ISO
# (scripts/build_archiso.sh). This script may be stale or broken.
# SigmaOS ISO Build Script (Improved)
# Creates a bootable SigmaOS live ISO using Arch Linux packages as base
# This version uses mkarchiso for proper ISO creation when available

set -e

VERSION="0.5.0"
BUILD_DATE=$(date +%Y%m%d)
ISO_LABEL="SIGMAOS_${VERSION}"
OUTPUT_DIR="./build/iso"
WORK_DIR="./build/work"
PROFILE_DIR="./build/profile"

echo "=== SigmaOS ISO Build Script (Improved) ==="
echo "Version: $VERSION"
echo "Build Date: $BUILD_DATE"

# Check if running on Arch-based system
if [ -f /etc/arch-release ]; then
    echo "Detected Arch-based system"
    IS_ARCH=true
else
    echo "Warning: Not on Arch-based system. Some features may not work."
    IS_ARCH=false
fi

# Check dependencies
check_dependencies() {
    local deps=("xorriso" "mksquashfs" "bsdtar" "coreutils" "findutils" "sed" "grep")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            echo "Error: $dep is required but not installed"
            exit 1
        fi
    done

    if [ "$IS_ARCH" = true ]; then
        if ! command -v mkarchiso &> /dev/null; then
            echo "Warning: mkarchiso not found. Install with: sudo pacman -S archiso"
            echo "This script will attempt to continue without mkarchiso."
        else
            echo "mkarchiso found - will use for ISO creation"
            HAS_MKARCHISO=true
        fi

        if ! command -v pacstrap &> /dev/null; then
            echo "Error: pacstrap not found. Install with: sudo pacman -S arch-install-scripts"
            exit 1
        fi
    fi
}

check_dependencies

# Create directories
mkdir -p "$OUTPUT_DIR" "$WORK_DIR" "$PROFILE_DIR"

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

# Use mkarchiso if available (preferred method)
if [ "$HAS_MKARCHISO" = true ]; then
    echo "=== Using mkarchiso for ISO creation ==="

    # Create profile directory structure
    mkdir -p "$PROFILE_DIR/airootfs/etc"

    # Create profile configuration
    cat > "$PROFILE_DIR/profiledef.sh" << EOF
profile_name='sigmaos'
description='SigmaOS Live System'
version='$VERSION'
arch='x86_64'
EOF

    # Create packages list
    printf "%s\n" "${ARCH_PACKAGES[@]}" > "$PROFILE_DIR/packages.x86_64"

    # Apply SigmaOS configuration
    echo "sigmaos" > "$PROFILE_DIR/airootfs/etc/hostname"
    echo "en_US.UTF-8 UTF-8" > "$PROFILE_DIR/airootfs/etc/locale.gen"
    echo "LANG=en_US.UTF-8" > "$PROFILE_DIR/airootfs/etc/locale.conf"
    ln -sf /usr/share/zoneinfo/UTC "$PROFILE_DIR/airootfs/etc/localtime"

    # Create sysctl configuration
    mkdir -p "$PROFILE_DIR/airootfs/etc/sysctl.d"
    cat > "$PROFILE_DIR/airootfs/etc/sysctl.d/99-sigmaos.conf" << 'EOF'
# SigmaOS kernel parameters
net.ipv4.ip_forward = 1
net.ipv4.tcp_fastopen = 3
net.ipv4.tcp_congestion_control = bbr
vm.swappiness = 10
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5
EOF

    # Create SigmaOS version file
    mkdir -p "$PROFILE_DIR/airootfs/etc/sigmaos"
    cat > "$PROFILE_DIR/airootfs/etc/sigmaos/version" << EOF
SigmaOS $VERSION
Build Date: $BUILD_DATE
Repository: https://github.com/AaryanSinghChauhan09/Sigma
EOF

    # Run mkarchiso
    echo "=== Building ISO with mkarchiso ==="
    cd "$WORK_DIR"
    mkarchiso -v -w "$WORK_DIR" -o "$OUTPUT_DIR" "$PROFILE_DIR"

    echo "=== Build complete ==="
    echo "ISO location: $OUTPUT_DIR/sigmaos-$VERSION-x86_64.iso"
else
    echo "=== Using manual ISO creation ==="

    ROOTFS_DIR="$WORK_DIR/rootfs"

    # Create rootfs directory
    mkdir -p "$ROOTFS_DIR"

    # Download Arch Linux packages
    echo "=== Downloading Arch Linux packages ==="
    if command -v pacstrap &> /dev/null; then
        pacstrap -c -K "$ROOTFS_DIR" base "${ARCH_PACKAGES[@]}"
    else
        echo "Error: pacstrap not available. Cannot continue."
        exit 1
    fi

    # Apply SigmaOS configuration
    echo "=== Applying SigmaOS configuration ==="

    mkdir -p "$ROOTFS_DIR/etc/sigmaos"
    mkdir -p "$ROOTFS_DIR/etc/sysctl.d"

    echo "sigmaos" > "$ROOTFS_DIR/etc/hostname"
    echo "en_US.UTF-8 UTF-8" > "$ROOTFS_DIR/etc/locale.gen"
    chroot "$ROOTFS_DIR" locale-gen
    echo "LANG=en_US.UTF-8" > "$ROOTFS_DIR/etc/locale.conf"
    ln -sf /usr/share/zoneinfo/UTC "$ROOTFS_DIR/etc/localtime"

    echo "root:sigmaos" | chroot "$ROOTFS_DIR" chpasswd
    chroot "$ROOTFS_DIR" useradd -m -G wheel -s /bin/bash sigmaos
    echo "sigmaos:sigmaos" | chroot "$ROOTFS_DIR" chpasswd
    echo "%wheel ALL=(ALL) NOPASSWD: ALL" > "$ROOTFS_DIR/etc/sudoers.d/wheel"

    chroot "$ROOTFS_DIR" systemctl enable NetworkManager
    chroot "$ROOTFS_DIR" systemctl enable sshd
    chroot "$ROOTFS_DIR" systemctl enable systemd-timesyncd

    cat > "$ROOTFS_DIR/etc/sysctl.d/99-sigmaos.conf" << 'EOF'
# SigmaOS kernel parameters
net.ipv4.ip_forward = 1
net.ipv4.tcp_fastopen = 3
net.ipv4.tcp_congestion_control = bbr
vm.swappiness = 10
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5
EOF

    cat > "$ROOTFS_DIR/etc/sigmaos/version" << EOF
SigmaOS $VERSION
Build Date: $BUILD_DATE
Repository: https://github.com/AaryanSinghChauhan09/Sigma
EOF

    echo "=== Generating initramfs ==="
    chroot "$ROOTFS_DIR" mkinitcpio -P

    echo "=== Compressing rootfs ==="
    cd "$WORK_DIR"
    mksquashfs "$ROOTFS_DIR" "$OUTPUT_DIR/rootfs.squashfs" -comp xz -b 1M

    echo "=== Creating ISO ==="
    xorriso -as mkisofs \
        -r -V "$ISO_LABEL" \
        -o "$OUTPUT_DIR/sigmaos-$VERSION-$BUILD_DATE.iso" \
        -b boot/grub/grub.cfg \
        -no-emul-boot \
        -boot-load-size 4 \
        -boot-info-table \
        "$ROOTFS_DIR"

    echo "=== Build complete ==="
    echo "ISO: $OUTPUT_DIR/sigmaos-$VERSION-$BUILD_DATE.iso"
fi

# Generate checksum
echo "=== Generating checksum ==="
cd "$OUTPUT_DIR"
if [ -f "sigmaos-$VERSION-x86_64.iso" ]; then
    sha256sum "sigmaos-$VERSION-x86_64.iso" > "sigmaos-$VERSION-x86_64.iso.sha256"
    ISO_FILE="sigmaos-$VERSION-x86_64.iso"
elif [ -f "sigmaos-$VERSION-$BUILD_DATE.iso" ]; then
    sha256sum "sigmaos-$VERSION-$BUILD_DATE.iso" > "sigmaos-$VERSION-$BUILD_DATE.iso.sha256"
    ISO_FILE="sigmaos-$VERSION-$BUILD_DATE.iso"
fi

echo ""
echo "=== Summary ==="
echo "ISO: $OUTPUT_DIR/$ISO_FILE"
echo "SHA256: $OUTPUT_DIR/${ISO_FILE}.sha256"
echo ""
echo "To test in QEMU:"
echo "  qemu-system-x86_64 -m 2048 -cdrom $OUTPUT_DIR/$ISO_FILE"
echo ""
echo "To write to USB:"
echo "  sudo dd if=$OUTPUT_DIR/$ISO_FILE of=/dev/sdX bs=4M status=progress"
