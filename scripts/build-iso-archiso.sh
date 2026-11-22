#!/bin/bash
# SigmaOS archiso Profile Build Script
# Creates a proper bootable ISO using Arch's archiso tools
# This is a complete, production-ready build script

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

# Clean previous build
echo "Cleaning previous build..."
rm -rf "$WORK_DIR" "$OUTPUT_DIR"
mkdir -p "$PROFILE_DIR" "$WORK_DIR" "$OUTPUT_DIR"
# Absolutize paths — the script cd's into WORK_DIR before invoking mkarchiso
PROFILE_DIR="$(realpath "$PROFILE_DIR")"
WORK_DIR="$(realpath "$WORK_DIR")"
OUTPUT_DIR="$(realpath "$OUTPUT_DIR")"
SUDO=""; [ "$(id -u)" -ne 0 ] && SUDO="sudo"

# Create profile directory structure
mkdir -p "$PROFILE_DIR/airootfs/etc/sigmaos"
mkdir -p "$PROFILE_DIR/airootfs/etc/sysctl.d"
mkdir -p "$PROFILE_DIR/airootfs/etc/sudoers.d"
mkdir -p "$PROFILE_DIR/airootfs/root"
mkdir -p "$PROFILE_DIR/airootfs/home/sigmaos"

# Create profiledef.sh
cat > "$PROFILE_DIR/profiledef.sh" << EOF
profile_name='sigmaos'
description='SigmaOS Live System - Based on Arch Linux'
version='$VERSION'
arch='x86_64'
EOF

# Create packages list - comprehensive like Arch's releng
cat > "$PROFILE_DIR/packages.x86_64" << 'PACKAGES'
# Core system
linux
linux-firmware
linux-headers
systemd
systemd-libs
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
shadow
iproute2
iputils
diffutils
expat
gcc-libs
glibc
gmp
gnutls
keyutils
krb5
libcap
libgcrypt
libgpg-error
libidn2
libnghttp2
libpsl
libssh2
libunistring
libxcrypt
ncurses
openssl
p11-kit
zlib
zstd

# Boot and init
grub
efibootmgr
os-prober
mkinitcpio
sudo
polkit
efivar

# Networking
networkmanager
wpa_supplicant
openssh
dhclient
dialog
iw
wireless-regdb
ppp
rp-pppoe
ipset
nftables
iptables
bind
inetutils
traceroute
dnsutils
whois

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
gettext
groff
perl
python

# Storage and filesystems
parted
gptfdisk
dosfstools
e2fsprogs
btrfs-progs
xfsprogs
cryptsetup
lvm2
mdadm
reiserfsprogs
jfsutils
f2fs-tools
nilfs-utils
ntfs-3g
exfat-utils

# Hardware monitoring
lm_sensors
smartmontools
nvme-cli
usbutils
pciutils
hdparm
sdparm
hddtemp

# Development tools
git
curl
wget
rsync
ca-certificates
base-devel
 pacman-contrib

# System monitoring
htop
iotop
strace
lsof
tmux
screen
neofetch
btop

# Installation tools
arch-install-scripts
pacman
PACKAGES

# Create pacman.conf - use Arch's official mirrors
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

[multilib]
Include = /etc/pacman.d/mirrorlist
EOF

# Apply SigmaOS base configuration
echo "sigmaos" > "$PROFILE_DIR/airootfs/etc/hostname"
echo "en_US.UTF-8 UTF-8" > "$PROFILE_DIR/airootfs/etc/locale.gen"
echo "LANG=en_US.UTF-8" > "$PROFILE_DIR/airootfs/etc/locale.conf"
ln -sf /usr/share/zoneinfo/UTC "$PROFILE_DIR/airootfs/etc/localtime"

# Create sigmaos user
cat > "$PROFILE_DIR/airootfs/etc/passwd" << 'EOF'
root:x:0:0:root:/root:/bin/bash
sigmaos:x:1000:1000:SigmaOS User:/home/sigmaos:/bin/bash
EOF

cat > "$PROFILE_DIR/airootfs/etc/group" << 'EOF'
root:x:0:
wheel:x:10:
sigmaos:x:1000:
EOF

cat > "$PROFILE_DIR/airootfs/etc/shadow" << 'EOF'
root:$6$rootroot$rootrootrootrootrootrootrootrootrootrootroot.:19000:0:99999:7:::
sigmaos:$6$sigmaos$sigmaossigmaossigmaossigmaossigmaossigmaos.:19000:0:99999:7:::
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
net.core.default_qdisc = fq
net.ipv4.tcp_slow_start_after_idle = 0
EOF

# Create SigmaOS version file
cat > "$PROFILE_DIR/airootfs/etc/sigmaos/version" << EOF
SigmaOS $VERSION
Build Date: $BUILD_DATE
Repository: https://github.com/AaryanSinghChauhan09/Sigma
Based on: Arch Linux
EOF

# Enable essential services
mkdir -p "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants"
ln -sf /usr/lib/systemd/system/NetworkManager.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/NetworkManager.service"
ln -sf /usr/lib/systemd/system/sshd.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/sshd.service"
ln -sf /usr/lib/systemd/system/systemd-timesyncd.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/systemd-timesyncd.service"

# Configure SSH
mkdir -p "$PROFILE_DIR/airootfs/etc/ssh"
cat > "$PROFILE_DIR/airootfs/etc/ssh/sshd_config" << 'EOF'
Port 22
PermitRootLogin yes
PasswordAuthentication yes
X11Forwarding no
EOF

# Create live boot configuration with auto-login
mkdir -p "$PROFILE_DIR/airootfs/etc/systemd/system/getty@tty1.service.d"
cat > "$PROFILE_DIR/airootfs/etc/systemd/system/getty@tty1.service.d/autologin.conf" << 'EOF'
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin root --noclear %I $TERM
EOF

# Copy installer script
cp scripts/install-sigmaos.sh "$PROFILE_DIR/airootfs/root/install-sigmaos.sh"
chmod +x "$PROFILE_DIR/airootfs/root/install-sigmaos.sh"

# Create .bash_profile for live environment
cat > "$PROFILE_DIR/airootfs/root/.bash_profile" << 'EOF'
# SigmaOS Live Environment
if [ -z "$DISPLAY" ] && [ $(tty) = "/dev/tty1" ]; then
    clear
    echo "=========================================="
    echo "       Welcome to SigmaOS Live System"
    echo "=========================================="
    echo ""
    echo "SigmaOS is a Linux distribution based on Arch Linux."
    echo "Version: $(cat /etc/sigmaos/version 2>/dev/null || echo 'unknown')"
    echo ""
    echo "To install SigmaOS to your disk, run:"
    echo "  /root/install-sigmaos.sh"
    echo ""
    echo "To view system information, run:"
    echo "  neofetch"
    echo ""
    echo "To start a shell, press Ctrl+C"
    echo "=========================================="
    echo ""
    exec bash
fi
EOF

# Create custom MOTD
cat > "$PROFILE_DIR/airootfs/etc/motd" << 'EOF'
==========================================
       Welcome to SigmaOS Live System
==========================================

SigmaOS is a Linux distribution based on Arch Linux.
Repository: https://github.com/AaryanSinghChauhan09/Sigma

To install SigmaOS:
  /root/install-sigmaos.sh

For help:
  https://github.com/AaryanSinghChauhan09/Sigma/wiki
==========================================
EOF

# Build the ISO using mkarchiso
echo "=== Building ISO with mkarchiso ==="
echo "This may take 10-30 minutes depending on your system..."
echo ""

cd "$WORK_DIR"
$SUDO mkarchiso -v -w "$WORK_DIR" -o "$OUTPUT_DIR" "$PROFILE_DIR"

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
echo "Size: $(du -h "$OUTPUT_DIR/sigmaos-$VERSION-x86_64.iso" | cut -f1)"
echo ""
echo "To test in QEMU:"
echo "  qemu-system-x86_64 -m 2048 -cdrom $OUTPUT_DIR/sigmaos-$VERSION-x86_64.iso -boot d"
echo ""
echo "To write to USB:"
echo "  sudo dd if=$OUTPUT_DIR/sigmaos-$VERSION-x86_64.iso of=/dev/sdX bs=4M status=progress && sync"
echo ""
echo "Build successful!"
