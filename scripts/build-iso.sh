#!/bin/bash
# DEPRECATED: experimental script kept for reference. The supported ISO
# build lives in https://github.com/AaryanSinghChauhan09/SigmaOS-ISO
# (scripts/build_archiso.sh). This script may be stale or broken.
# SigmaOS ISO Build Script
# Creates a bootable SigmaOS live ISO using Arch Linux packages as base
# with SigmaOS configuration layer applied

set -e

VERSION="0.3.0"
BUILD_DATE=$(date +%Y%m%d)
ISO_LABEL="SIGMAOS_${VERSION}"
OUTPUT_DIR="./build/iso"
WORK_DIR="./build/work"
ROOTFS_DIR="$WORK_DIR/rootfs"
ARCH_PACKAGES=("linux" "linux-firmware" "systemd" "bash" "coreutils" "findutils" "sed" "grep" "gawk" "tar" "gzip" "xz" "bzip2" "zstd" "networkmanager" "dhclient" "wpa_supplicant" "openssh" "sudo" "vim" "nano" "htop" "tmux" "git" "curl" "wget" "rsync" "ca-certificates" "man-db" "man-pages" "tzdata" "locale" "kbd" "console-setup" "fontconfig" "ttf-dejavu" "xorg-server" "xorg-xinit" "wayland" "mesa" "vulkan-icd-loader" "xf86-video-amdgpu" "xf86-video-ati" "xf86-video-intel" "xf86-video-nouveau" "xf86-video-vesa" "alsa-utils" "pipewire" "wireplumber" "pulseaudio-alsa" "bluez" "bluez-utils" "cups" "cups-filters" "foomatic-db-engine" "ghostscript" "poppler" "imagemagick" "scrot" "screenshot" "brightnessctl" "power-profiles-daemon" "tlp" "thermald" "lm_sensors" "smartmontools" "nvme-cli" "parted" "gptfdisk" "dosfstools" "exfatprogs" "ntfs-3g" "udftools" "squashfs-tools" "zfs-linux" "btrfs-progs" "xfsprogs" "e2fsprogs" "f2fs-tools" "cryptsetup" "lvm2" "mdadm" "bcachefs-tools" "erofs-utils" "fuse2" "fuse3" "overlayfs" "aufs" "docker" "podman" "buildah" "skopeo" "containerd" "runc" "crun" "slirp4netns" "passt" "dnsmasq" "bind" "unbound" "systemd-resolved" "opensmtpd" "postfix" "dovecot" "proftpd" "vsftpd" "samba" "samba-utils" "nfs-utils" "rpcbind" "nfsidmap" "autofs" "udisks2" "gvfs" "gvfs-mtp" "gvfs-smb" "gvfs-nfs" "gvfs-afc" "gvfs-goa" "gvfs-google" "gvfs-admin" "gvfs-archive" "gvfs-daemon" "gvfs-libs" "gvfs-metadata" "gvfs-obexftp" "gvfs-sftp" "gvfs-thumbnails" "policykit-1" "polkit-gnome" "accountsservice" "upower" "network-manager-applet" "nm-connection-editor" "blueman" "blueman-applet" "cups-pk-helper" "geoclue2" "geocode-glib" "gcr" "gcr-4" "gnome-keyring" "libsecret" "kwallet" "kwallet-pam" "keepassxc" "veracrypt" "gpgme" "pinentry" "pinentry-gnome3" "pinentry-qt" "pinentry-emacs" "password-store" "yubikey-personalization" "yubikey-manager" "ykman" "opensc" "pcsc-lite" "pcsc-tools" "ccid" "ifd-smartcard" "libnfc" "nfc-utils" "openpgp-card-tools" "paperkey" "shamir-secret-sharing" "ssss" "zbar" "qrencode" "zxing-cpp" "libdmtx" "dmtx-utils" "zint" "qr-tools" "barcode" "imagemagick" "graphicsmagick" "exiftool" "exiv2" "jhead" "jpegoptim" "optipng" "pngcrush" "pngquant" "cwebp" "dwebp" "avifenc" "avifdec" "heif-enc" "heif-convert" "jxl" "cjxl" "djxl" "ffmpeg" "ffmpegthumbnailer" "mplayer" "mpv" "vlc" "totem" "celluloid" "parole" "smplayer" "haruna" "kodi" "plex-media-player" "jellyfin-media-player" "emby" "mpd" "ncmpcpp" "ympd" "cantata" "sonata" "gmpc" "guayadeque" "exaile" "rhythmbox" "amarok" "clementine" "deadbeef" "audacious" "xmms2" "mocp" "musikcube" "cmus" "moc" "mocp" "ncmpcpp" "mpc" "mplayer2" "mpg123" "ogg123" "vorbis-tools" "flac" "wavpack" "tta" "shorten" "optimfrog" "ofr" "la" "lossless-audio-checker" "speex" "opus-tools" "opusfile" "sox" "soundstretch" "rubberband" "aubio" "lilv" "lv2" "ladspa" "dssi" "vst3" "clap" "fftw" "ocl-icd" "opencl" "vulkan" "spirv-tools" "glslang" "shaderc" "spirv-cross" "glslc" "naga" "rust-gpu" "wgpu" "vulkano" "ash" "gpu-alloc" "gpu-descriptor" "gfx-hal" "rend3" "wgpu-hal" "dawn" "webgpu" "openxr" "monado" "steamvr" "virtualbox-guest-utils" "qemu-guest-agent" "spice-vdagent" "xorg-xwayland" "wayland-utils" "wayland-protocols" "weston" "sway" "i3" "i3-gaps" "i3status" "i3lock" "i3blocks" "polybar" "rofi" "dmenu" "walker" "wofi" "fuzzel" "bemenu" "yad" "zenity" "dialog" "whiptail" "newt" "snack" "python-pyqt5" "python-pyqt6" "python-pyside2" "python-pyside6" "qt5" "qt6" "gtk3" "gtk4" "wxwidgets-gtk3" "wxwidgets-gtk4" "fltk" "sdl2" "allegro" "curses" "ncurses" "terminfo" "terminology" "alacritty" "kitty" "foot" "wezterm" "ghostty" "rio" "contour" "warp" "hyper" "tabby" "electerm" "terminator" "tilix" "guake" "yakuake" "tilda" "quake" "cool-retro-term" "st" "urxvt" "rxvt-unicode" "xterm" "aterm" "eterm" "mlterm" "lxterminal" "mate-terminal" "xfce4-terminal" "konsole" "gnome-terminal" "kgx" "ptyxis" "blackbox" "sakura" "lilyterm" "roxterm" "tilp2" "terminology" "cool-retro-term" "contour" "rio" "ghostty" "wezterm" "kitty" "foot" "alacritty" "warp" "hyper" "tabby" "electerm" "terminator" "tilix" "guake" "yakuake" "tilda" "quake" "cool-retro-term" "st" "urxvt" "rxvt-unicode" "xterm" "aterm" "eterm" "mlterm" "lxterminal" "mate-terminal" "xfce4-terminal" "konsole" "gnome-terminal" "kgx" "ptyxis" "blackbox" "sakura" "lilyterm" "roxterm" "tilp2")

echo "=== SigmaOS ISO Build Script ==="
echo "Version: $VERSION"
echo "Build Date: $BUILD_DATE"

# Check dependencies
check_dependencies() {
    local deps=("xorriso" "mksquashfs" "bsdtar" "coreutils" "findutils" "sed" "grep" "pacstrap" "mkinitcpio" "mkarchiso")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            echo "Error: $dep is required but not installed"
            exit 1
        fi
    done
}

check_dependencies

# Create directories
mkdir -p "$OUTPUT_DIR" "$WORK_DIR" "$ROOTFS_DIR"

# Download Arch Linux packages
echo "=== Downloading Arch Linux packages ==="
cd "$ROOTFS_DIR"
pacstrap -c -K "$ROOTFS_DIR" base "${ARCH_PACKAGES[@]}"

# Apply SigmaOS configuration
echo "=== Applying SigmaOS configuration ==="

# Create SigmaOS system directories
mkdir -p "$ROOTFS_DIR/etc/sigmaos"
mkdir -p "$ROOTFS_DIR/etc/sysctl.d"
mkdir -p "$ROOTFS_DIR/etc/systemd/system"
mkdir -p "$ROOTFS_DIR/etc/udev/rules.d"
mkdir -p "$ROOTFS_DIR/etc/fstab.d"
mkdir -p "$ROOTFS_DIR/etc/initramfs"
mkdir -p "$ROOTFS_DIR/usr/lib/sigmaos"

# Generate SigmaOS configuration
cat > "$ROOTFS_DIR/etc/sigmaos/config.toml" << 'EOF'
[system]
name = "SigmaOS"
version = "0.3.0"
edition = "rolling"
architecture = "x86_64"

[boot]
loader = "systemd-boot"
kernel = "linux"
initramfs = "initramfs-linux.img"

[packages]
manager = "sigma-package"
repository = "sigmaos-official"
cache_dir = "/var/cache/sigmaos/packages"

[logging]
level = "info"
format = "json"
output = ["journal", "file"]
file_path = "/var/log/sigmaos.log"

[security]
secure_boot = "auto"
tpm = "auto"
firewall = "nftables"
selinux = false
apparmor = true
EOF

# Generate sysctl configuration
cat > "$ROOTFS_DIR/etc/sysctl.d/99-sigmaos.conf" << 'EOF'
# SigmaOS system tuning
vm.swappiness = 10
vm.vfs_cache_pressure = 50
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5
kernel.pid_max = 4194304
fs.file-max = 2097152
net.core.somaxconn = 65535
net.ipv4.tcp_max_syn_backlog = 8192
net.ipv4.tcp_tw_reuse = 1
net.ipv4.tcp_fin_timeout = 15
net.ipv4.tcp_keepalive_time = 300
net.ipv4.tcp_keepalive_probes = 5
net.ipv4.tcp_keepalive_intvl = 15
net.ipv4.ip_local_port_range = 1024 65535
EOF

# Generate udev rules
cat > "$ROOTFS_DIR/etc/udev/rules.d/99-sigmaos.rules" << 'EOF'
# SigmaOS hardware rules
SUBSYSTEM=="power_supply", KERNEL=="AC*", ATTR{online}=="1", RUN+="/usr/bin/sigmaos-power-profile-performance"
SUBSYSTEM=="power_supply", KERNEL=="BAT*", ATTR{status}=="Discharging", RUN+="/usr/bin/sigmaos-power-profile-battery"
SUBSYSTEM=="thermal", KERNEL=="cooling_device*", ATTR{type}=="Processor", ATTR{cur_state}=="0"
SUBSYSTEM=="drm", KERNEL=="card*", ATTR{modesetting}=="on"
EOF

# Generate initramfs hooks
mkdir -p "$ROOTFS_DIR/etc/initramfs/hooks"
cat > "$ROOTFS_DIR/etc/initramfs/hooks/sigmaos" << 'EOF'
#!/bin/bash
# SigmaOS initramfs hook
build() {
    add_module "sigmaos"
    add_binary "sigmaos-init"
    add_runscript
}
EOF
chmod +x "$ROOTFS_DIR/etc/initramfs/hooks/sigmaos"

# Generate bootloader configuration
mkdir -p "$ROOTFS_DIR/boot/loader/entries"
cat > "$ROOTFS_DIR/boot/loader/loader.conf" << 'EOF'
default sigmaos
editor no
timeout 5
console-mode max
EOF

cat > "$ROOTFS_DIR/boot/loader/entries/sigmaos.conf" << 'EOF'
title SigmaOS
linux /vmlinuz-linux root=UUID=placeholder rw
initrd /initramfs-linux.img
options quiet splash
EOF

# Generate GRUB configuration
mkdir -p "$ROOTFS_DIR/boot/grub"
cat > "$ROOTFS_DIR/boot/grub/grub.cfg" << 'EOF'
set timeout=5
set default=0

menuentry "SigmaOS" {
    linux /boot/vmlinuz-linux root=/dev/sda1 rw
    initrd /boot/initramfs-linux.img
    boot
}
EOF

# Create SigmaOS init script
cat > "$ROOTFS_DIR/usr/bin/sigmaos-init" << 'EOF'
#!/bin/bash
# SigmaOS init script

# Load SigmaOS modules
modprobe sigmaos_core 2>/dev/null || true
modprobe sigmaos_fs 2>/dev/null || true
modprobe sigmaos_net 2>/dev/null || true
modprobe sigmaos_power 2>/dev/null || true

# Apply system configuration
sysctl -p /etc/sysctl.d/99-sigmaos.conf 2>/dev/null || true

# Start essential services
systemctl start systemd-udevd.service
systemctl start systemd-networkd.service
systemctl start systemd-resolved.service
systemctl start systemd-logind.service
systemctl start systemd-journald.service
systemctl start dbus.service

# Apply udev rules
udevadm control --reload-rules
udevadm trigger

# Start SigmaOS services
systemctl start sigmaos-core.service 2>/dev/null || true
systemctl start sigmaos-network.service 2>/dev/null || true
systemctl start sigmaos-power.service 2>/dev/null || true
systemctl start sigmaos-monitor.service 2>/dev/null || true

exec /sbin/init
EOF
chmod +x "$ROOTFS_DIR/usr/bin/sigmaos-init"

# Create SigmaOS service files
cat > "$ROOTFS_DIR/etc/systemd/system/sigmaos-core.service" << 'EOF'
[Unit]
Description=SigmaOS Core Service
After=systemd-udevd.service

[Service]
Type=simple
ExecStart=/usr/bin/sigmaos-core
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

cat > "$ROOTFS_DIR/etc/systemd/system/sigmaos-network.service" << 'EOF'
[Unit]
Description=SigmaOS Network Service
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/sigmaos-network
Restart=always

[Install]
WantedBy=multi-user.target
EOF

cat > "$ROOTFS_DIR/etc/systemd/system/sigmaos-power.service" << 'EOF'
[Unit]
Description=SigmaOS Power Service
After=systemd-udevd.service

[Service]
Type=simple
ExecStart=/usr/bin/sigmaos-power
Restart=always

[Install]
WantedBy=multi-user.target
EOF

cat > "$ROOTFS_DIR/etc/systemd/system/sigmaos-monitor.service" << 'EOF'
[Unit]
Description=SigmaOS Monitoring Service
After=systemd-udevd.service

[Service]
Type=simple
ExecStart=/usr/bin/sigmaos-monitor
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# Create placeholder binaries for SigmaOS services
for service in sigmaos-core sigmaos-network sigmaos-power sigmaos-monitor; do
    cat > "$ROOTFS_DIR/usr/bin/$service" << 'EOF'
#!/bin/bash
# SigmaOS service placeholder
# In production, this would be the actual compiled Rust binary
while true; do
    sleep 60
done
EOF
    chmod +x "$ROOTFS_DIR/usr/bin/$service"
done

# Create SquashFS image
echo "=== Creating SquashFS image ==="
cd "$ROOTFS_DIR"
mksquashfs . "$WORK_DIR/sigmaos.sfs" -comp xz -b 1M -noappend -quiet

# Create ISO
iso_dir="$WORK_DIR/iso"
mkdir -p "$iso_dir/EFI/BOOT"
mkdir -p "$iso_dir/boot/grub"
mkdir -p "$iso_dir/boot"

# Copy kernel and initramfs
cp "$ROOTFS_DIR/boot/vmlinuz-linux" "$iso_dir/boot/" 2>/dev/null || true
cp "$ROOTFS_DIR/boot/initramfs-linux.img" "$iso_dir/boot/" 2>/dev/null || true

# Create GRUB EFI binary
mkdir -p "$iso_dir/EFI/BOOT"
cp /usr/lib/grub/x86_64-efi-sig/grubx64.efi "$iso_dir/EFI/BOOT/BOOTX64.EFI" 2>/dev/null || true

# Create GRUB config for ISO
cat > "$iso_dir/boot/grub/grub.cfg" << 'EOF'
set timeout=5
set default=0

menuentry "SigmaOS" {
    linux /boot/vmlinuz-linux root=/dev/sda1 rw
    initrd /boot/initramfs-linux.img
    boot
}
EOF

# Create ISO with xorriso
iso_name="sigmaos-${VERSION}-${BUILD_DATE}.iso"
echo "=== Creating ISO ==="
xorriso -as mkisofs \
    -iso-level 3 \
    -full-iso9660-filenames \
    -volid "$ISO_LABEL" \
    -eltorito-boot boot/grub/i386-pc/eltorito.img \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    --efi-boot EFI/BOOT/BOOTX64.EFI \
    -efi-boot-part --efi-boot-image \
    -o "$OUTPUT_DIR/$iso_name" \
    "$iso_dir"

# Create checksum
echo "=== Creating checksum ==="
cd "$OUTPUT_DIR"
sha256sum "$iso_name" > "$iso_name.sha256"

echo "=== Build Complete ==="
echo "ISO: $OUTPUT_DIR/$iso_name"
echo "SHA256: $OUTPUT_DIR/$iso_name.sha256"
echo "Size: $(du -h "$iso_name" | cut -f1)"
