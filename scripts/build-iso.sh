#!/bin/bash
# SigmaOS ISO Build Script
# This script builds the SigmaOS live ISO for testing and distribution
# Inspired by Arch Linux archiso and Gentoo livecd-tools

set -e

VERSION="0.3.0"
BUILD_DATE=$(date +%Y%m%d)
ISO_LABEL="SIGMAOS_${VERSION}"
OUTPUT_DIR="./build/iso"
WORK_DIR="./build/work"

echo "=== SigmaOS ISO Build Script ==="
echo "Version: $VERSION"
echo "Build Date: $BUILD_DATE"

# Check dependencies
check_dependencies() {
    local deps=("xorriso" "mksquashfs" "bsdtar" "coreutils" "findutils" "sed" "grep")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" >/dev/null 2>&1; then
            echo "Error: $dep is required but not installed."
            echo "Install with: sudo apt install $dep (Debian/Ubuntu)"
            exit 1
        fi
    done
    echo "All dependencies found."
}

# Create build directories
setup_directories() {
    echo "Setting up build directories..."
    mkdir -p "$OUTPUT_DIR"
    mkdir -p "$WORK_DIR"/{iso,rootfs,efiboot}
    mkdir -p "$WORK_DIR/iso/boot"
    mkdir -p "$WORK_DIR/iso/boot/grub"
    mkdir -p "$WORK_DIR/iso/EFI/BOOT"
}

# Generate kernel configuration
generate_kernel_config() {
    echo "Generating kernel configuration..."
    cat > "$WORK_DIR/kernel.config" << 'EOF'
# SigmaOS Kernel Configuration
# Minimal, secure, and optimized

CONFIG_LOCALVERSION="-sigmaos"
CONFIG_DEFAULT_HOSTNAME="sigmaos"

# Basic setup
CONFIG_BINFMT_ELF=y
CONFIG_BINFMT_SCRIPT=y

# Filesystems
CONFIG_EXT4_FS=y
CONFIG_EXT4_FS_POSIX_ACL=y
CONFIG_EXT4_FS_SECURITY=y
CONFIG_BTRFS_FS=y
CONFIG_BTRFS_FS_POSIX_ACL=y
CONFIG_ZFS_FS=m
CONFIG_XFS_FS=y
CONFIG_F2FS_FS=y

# Networking
CONFIG_INET=y
CONFIG_NET=y
CONFIG_UNIX=y
CONFIG_IPV6=y
CONFIG_NETFILTER=y
CONFIG_IP_NF_IPTABLES=y
CONFIG_IP6_NF_IPTABLES=y

# Security
CONFIG_SECURITY=y
CONFIG_SECURITY_NETWORK=y
CONFIG_IKCONFIG=y
CONFIG_IKCONFIG_PROC=y

# Cgroups
CONFIG_CGROUPS=y
CONFIG_MEMCG=y
CONFIG_CPUSETS=y
CONFIG_BLK_CGROUP=y

# Hardware support
CONFIG_ACPI=y
CONFIG_PCI=y
CONFIG_GENERIC_IRQ=y
CONFIG_GENERIC_CLOCKEVENTS=y

# Kernel compression
CONFIG_KERNEL_XZ=y
EOF
}

# Generate initramfs configuration
generate_initramfs_config() {
    echo "Generating initramfs configuration..."
    cat > "$WORK_DIR/initramfs.conf" << 'EOF'
# SigmaOS Initramfs Configuration
MODULES="sd_mod ext4 btrfs zfs xfs f2fs"
BINARIES="busybox sh bash ls cat cp mv rm mkdir mount umount modprobe insmod blkid"
FILES=""
HOOKS="base udev autodetect modconf block filesystems keyboard keymap consolefont shutdown"
COMPRESSION="xz"
EOF
}

# Generate GRUB configuration
generate_grub_config() {
    echo "Generating GRUB configuration..."
    cat > "$WORK_DIR/iso/boot/grub/grub.cfg" << EOF
# SigmaOS GRUB Configuration
set timeout=5
set default=0

menuentry "SigmaOS ${VERSION}" {
    linux /boot/vmlinuz-linux quiet splash
    initrd /boot/initramfs-linux.img
}

menuentry "SigmaOS ${VERSION} (Fallback)" {
    linux /boot/vmlinuz-linux
    initrd /boot/initramfs-linux-fallback.img
}

menuentry "SigmaOS ${VERSION} (Recovery)" {
    linux /boot/vmlinuz-linux single
    initrd /boot/initfs-linux.img
}
EOF

    # Copy GRUB EFI boot loader
    cp /usr/share/grub/x86_64-efi-signed/grubx64.efi.signed "$WORK_DIR/iso/EFI/BOOT/BOOTX64.EFI" 2>/dev/null || true
}

# Generate systemd-boot configuration
generate_systemd_boot_config() {
    echo "Generating systemd-boot configuration..."
    cat > "$WORK_DIR/iso/loader/loader.conf" << EOF
default sigmaos
timeout 5
editor 0
EOF

    cat > "$WORK_DIR/iso/loader/entries/sigmaos.conf" << EOF
title SigmaOS ${VERSION}
linux /vmlinuz-linux
initrd /initramfs-linux.img
options quiet splash
EOF

    cat > "$WORK_DIR/iso/loader/entries/sigmaos-fallback.conf" << EOF
title SigmaOS ${VERSION} (Fallback)
linux /vmlinuz-linux
initrd /initramfs-linux-fallback.img
options
EOF
}

# Generate root filesystem structure
generate_rootfs() {
    echo "Generating root filesystem structure..."
    
    # Create basic directory structure
    mkdir -p "$WORK_DIR/rootfs"/{bin,boot,dev,etc,home,lib,media,mnt,opt,proc,root,run,sbin,srv,sys,tmp,var}
    mkdir -p "$WORK_DIR/rootfs/etc"/{systemd,modprobe.d,sysctl.d,udev/rules.d}
    mkdir -p "$WORK_DIR/rootfs/usr"/{bin,lib,local,share}
    
    # Generate system configuration using SigmaOS library
    cat > "$WORK_DIR/rootfs/etc/sigmaos.toml" << EOF
# SigmaOS System Configuration
version = "${VERSION}"

[system]
hostname = "sigmaos"
timezone = "UTC"
locale = "en_US.UTF-8"

[persona]
default = "SovereignOmarchy"

[memory]
cgroup_enabled = true
swap_enabled = true

[network]
firewall_enabled = true
ipv6_enabled = true

[security]
secure_boot = true
tpm_enabled = true

[filesystem]
default_type = "btrfs"
snapshots_enabled = true

[power]
profile = "balanced"
auto_optimize = true
EOF

    # Generate sysctl configuration
    cat > "$WORK_DIR/rootfs/etc/sysctl.d/99-sigmaos.conf" << 'EOF'
# SigmaOS sysctl configuration
vm.swappiness = 10
vm.vfs_cache_pressure = 50
vm.min_free_kbytes = 8192
net.ipv4.ip_forward = 0
net.ipv6.conf.all.forwarding = 0
kernel.kptr_restrict = 2
EOF

    # Generate udev rules
    cat > "$WORK_DIR/rootfs/etc/udev/rules.d/99-sigmaos.rules" << 'EOF'
# SigmaOS udev rules
SUBSYSTEM=="net", ACTION=="add", DRIVERS=="?*", ATTR{address}=="*", NAME="net0"
EOF
}

# Create squashfs image
create_squashfs() {
    echo "Creating squashfs image..."
    mksquashfs "$WORK_DIR/rootfs" "$WORK_DIR/iso/airootfs.squashfs" -comp xz -b 1M -noappend
}

# Create ISO image
create_iso() {
    echo "Creating ISO image..."
    xorriso -as mkisofs \
        -o "$OUTPUT_DIR/sigmaos-${VERSION}-${BUILD_DATE}-x86_64.iso" \
        -V "$ISO_LABEL" \
        -J -joliet-long \
        -r \
        -b boot/grub/i386-pc/eltorito.img \
        -c boot/grub/boot.cat \
        -no-emul-boot \
        -boot-load-size 4 \
        -boot-info-table \
        -eltorito-alt-boot \
        -e EFI/BOOT/BOOTX64.EFI \
        -no-emul-boot \
        -isohybrid-gpt-basdat \
        -isohybrid-apm-hfsplus \
        "$WORK_DIR/iso"
    
    echo "ISO created: $OUTPUT_DIR/sigmaos-${VERSION}-${BUILD_DATE}-x86_64.iso"
}

# Generate verification checksums
generate_checksums() {
    echo "Generating checksums..."
    cd "$OUTPUT_DIR"
    sha256sum "sigmaos-${VERSION}-${BUILD_DATE}-x86_64.iso" > "sigmaos-${VERSION}-${BUILD_DATE}-x86_64.iso.sha256"
    cd - > /dev/null
}

# Clean up
cleanup() {
    echo "Cleaning up work directory..."
    rm -rf "$WORK_DIR"
}

# Main build process
main() {
    check_dependencies
    setup_directories
    generate_kernel_config
    generate_initramfs_config
    generate_grub_config
    generate_systemd_boot_config
    generate_rootfs
    
    # Note: In a full implementation, you would:
    # 1. Build the actual Linux kernel with the generated config
    # 2. Generate initramfs with dracut/mkinitcpio
    # 3. Copy kernel and initramfs to iso/boot/
    # 4. Install base system packages to rootfs/
    
    echo "Note: This is a configuration generator. Full kernel build requires:"
    echo "  1. Linux kernel source"
    echo "  2. Cross-compilation toolchain"
    echo "  3. Package manager (pacman/apt)"
    echo ""
    echo "To complete the build:"
    echo "  1. Build kernel: make -C /path/to/linux ARCH=x86_64 O=$WORK_DIR/kernel-build"
    echo "  2. Generate initramfs: mkinitcpio -c $WORK_DIR/initramfs.conf"
    echo "  3. Install packages to rootfs"
    echo "  4. Run: create_squashfs && create_iso && generate_checksums"
    
    # cleanup
    # Uncomment in production: cleanup
    
    echo "=== Configuration generation complete ==="
    echo "Work directory: $WORK_DIR"
    echo "Output directory: $OUTPUT_DIR"
}

main "$@"
