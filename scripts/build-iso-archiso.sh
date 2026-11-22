#!/bin/bash
# SigmaOS archiso Profile Build Script
# Creates a proper bootable ISO using Arch's archiso tools

set -e

VERSION="0.5.0"
BUILD_DATE=$(date +%Y%m%d)
PROFILE_DIR="./archiso-profiles/sigmaos"
WORK_DIR="./build/archiso"
OUTPUT_DIR="./build/iso"

echo "=== SigmaOS archiso Build ==="
echo "Version: $VERSION"
echo "Build Date: $BUILD_DATE"

# Check for archiso
if ! command -v mkarchiso &> /dev/null; then
    echo "Error: mkarchiso not found. Install with: sudo pacman -S archiso"
    exit 1
fi

# Create profile directory structure
mkdir -p "$PROFILE_DIR/airootfs/etc/sigmaos"
mkdir -p "$PROFILE_DIR/airootfs/etc/sysctl.d"
mkdir -p "$PROFILE_DIR/airootfs/etc/sudoers.d"

# Create profiledef.sh
cat > "$PROFILE_DIR/profiledef.sh" << EOF
profile_name='sigmaos'
description='SigmaOS Live System'
version='$VERSION'
arch='x86_64'
EOF

# Create packages list
cat > "$PROFILE_DIR/packages.x86_64" << 'PACKAGES'
# Core system
linux
linux-firmware
systemd
bash
coreutils
findutils
sed
grep
gawk
tar
gzip
xz
bzip2
zstd
file
which
procps-ng
psmisc
util-linux

# Boot and init
grub
efibootmgr
os-prober
mkinitcpio
sudo
polkit

# Networking
networkmanager
wpa_supplicant
openssh
iproute2
iptables
nftables
iputils
bind
inetutils
wireless_tools
wpa_supplicant

# Text editors and tools
vim
nano
less
man-db
man-pages
tzdata
locales
kbd
console-setup

# Storage and filesystems
parted
gptfdisk
dosfstools
e2fsprogs
btrfs-progs
xfsprogs
cryptsetup
lvm2

# Hardware monitoring
lm_sensors
smartmontools
nvme-cli
usbutils
pciutils

# Development tools
git
curl
wget
rsync
ca-certificates

# System monitoring
htop
iotop
strace
lsof
tmux
screen

# Installation tools
arch-install-scripts
PACKAGES

# Create pacman.conf
cat > "$PROFILE_DIR/pacman.conf" << 'EOF'
[options]
HoldPkg     = pacman glibc
Architecture = auto
CheckSpace
SigLevel    = Required
LocalFileSigLevel = Optional

[core]
Include = /etc/pacman.d/mirrorlist

[extra]
Include = /etc/pacman.d/mirrorlist

[community]
Include = /etc/pacman.d/mirrorlist
EOF

# Apply SigmaOS configuration
echo "sigmaos" > "$PROFILE_DIR/airootfs/etc/hostname"
echo "en_US.UTF-8 UTF-8" > "$PROFILE_DIR/airootfs/etc/locale.gen"
echo "LANG=en_US.UTF-8" > "$PROFILE_DIR/airootfs/etc/locale.conf"
ln -sf /usr/share/zoneinfo/UTC "$PROFILE_DIR/airootfs/etc/localtime"

# Set root password
echo "root:sigmaos" > "$PROFILE_DIR/airootfs/root/.autologin"

# Create sigmaos user
cat > "$PROFILE_DIR/airootfs/etc/passwd" << 'EOF'
root:x:0:0:root:/root:/bin/bash
sigmaos:x:1000:1000:SigmaOS User:/home/sigmaos:/bin/bash
EOF

# Create sysctl configuration
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
cat > "$PROFILE_DIR/airootfs/etc/sigmaos/version" << EOF
SigmaOS $VERSION
Build Date: $BUILD_DATE
Repository: https://github.com/AaryanSinghChauhan09/Sigma
EOF

# Enable services
mkdir -p "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants"
ln -sf /usr/lib/systemd/system/NetworkManager.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/NetworkManager.service"
ln -sf /usr/lib/systemd/system/sshd.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/sshd.service"
ln -sf /usr/lib/systemd/system/systemd-timesyncd.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/systemd-timesyncd.service"

# Create live boot configuration
mkdir -p "$PROFILE_DIR/airootfs/etc/systemd/system/getty@tty1.service.d"
cat > "$PROFILE_DIR/airootfs/etc/systemd/system/getty@tty1.service.d/autologin.conf" << 'EOF'
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin root --noclear %I $TERM
EOF

# Copy installer script
mkdir -p "$PROFILE_DIR/airootfs/root"
cp scripts/install-sigmaos.sh "$PROFILE_DIR/airootfs/root/install-sigmaos.sh"
chmod +x "$PROFILE_DIR/airootfs/root/install-sigmaos.sh"

# Create .script to auto-start installer
cat > "$PROFILE_DIR/airootfs/root/.bash_profile" << 'EOF'
if [ -z "$DISPLAY" ] && [ $(tty) = "/dev/tty1" ]; then
    echo "Welcome to SigmaOS Live System"
    echo "To install SigmaOS, run: /root/install-sigmaos.sh"
    exec bash
fi
EOF

# Build the ISO
echo "=== Building ISO with mkarchiso ==="
cd "$WORK_DIR"
mkarchiso -v -w "$WORK_DIR" -o "$OUTPUT_DIR" "$PROFILE_DIR"

echo "=== Build complete ==="
echo "ISO: $OUTPUT_DIR/sigmaos-$VERSION-x86_64.iso"
cd - > /dev/null

# Generate checksum
cd "$OUTPUT_DIR"
sha256sum "sigmaos-$VERSION-x86_64.iso" > "sigmaos-$VERSION-x86_64.iso.sha256"

echo ""
echo "=== Summary ==="
echo "ISO: $OUTPUT_DIR/sigmaos-$VERSION-x86_64.iso"
echo "SHA256: $OUTPUT_DIR/sigmaos-$VERSION-x86_64.iso.sha256"
echo ""
echo "To test in QEMU:"
echo "  qemu-system-x86_64 -m 2048 -cdrom $OUTPUT_DIR/sigmaos-$VERSION-x86_64.iso"
echo ""
echo "To write to USB:"
echo "  sudo dd if=$OUTPUT_DIR/sigmaos-$VERSION-x86_64.iso of=/dev/sdX bs=4M status=progress"
