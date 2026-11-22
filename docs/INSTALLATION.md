# SigmaOS Installation Guide

## Overview

This guide provides comprehensive instructions for installing SigmaOS on your system.

## Prerequisites

- x86_64 compatible system
- UEFI firmware (BIOS legacy support planned)
- At least 20GB disk space
- 2GB RAM minimum (4GB recommended)
- Internet connection for installation

## Installation Methods

### Method 1: Live ISO Installation

1. Download the SigmaOS ISO (when available)
2. Write to USB drive:
   ```bash
   dd if=sigmaos-0.5.0-x86_64.iso of=/dev/sdX bs=4M status=progress sync
   ```
3. Boot from USB
4. Run the installer:
   ```bash
   sudo ./install-sigmaos.sh
   ```

### Method 2: Manual Installation

1. Boot from an existing Linux live system
2. Partition your disk
3. Install base system
4. Configure the system
5. Install bootloader

## Manual Installation Steps

### 1. Partitioning

```bash
# Create GPT partition table
parted /dev/sda mklabel gpt

# Create EFI partition (512MB)
parted /dev/sda mkpart ESP fat32 1MiB 513MiB
parted /dev/sda set 1 esp on

# Create swap partition (2GB)
parted /dev/sda mkpart swap linux-swap 513MiB 2561MiB

# Create root partition (remaining space)
parted /dev/sda mkpart primary ext4 2561MiB 100%

# Format partitions
mkfs.vfat -F32 /dev/sda1
mkfs.ext4 /dev/sda3
mkswap /dev/sda2
```

### 2. Mount Filesystems

```bash
swapon /dev/sda2
mount /dev/sda3 /mnt
mkdir /mnt/boot
mount /dev/sda1 /mnt/boot
```

### 3. Install Base System

```bash
# Install base packages
pacstrap -i /mnt base linux linux-firmware systemd networkmanager sudo

# Generate fstab
genfstab -U /mnt >> /mnt/etc/fstab
```

### 4. Configure System

```bash
# Chroot into installed system
arch-chroot /mnt

# Set timezone
ln -sf /usr/share/zoneinfo/UTC /etc/localtime
hwclock --systohc

# Set locale
echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf

# Set hostname
echo "sigmaos" > /etc/hostname

# Create user
useradd -m -G wheel -s /bin/bash sigmaos
echo "sigmaos ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/sigmaos

# Set root password
passwd
```

### 5. Install Bootloader

```bash
# Install GRUB
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=SigmaOS --recheck

# Generate GRUB config
grub-mkconfig -o /boot/grub/grub.cfg
```

### 6. Finalize Installation

```bash
# Exit chroot
exit

# Unmount filesystems
umount -R /mnt

# Reboot
reboot
```

## Post-Installation

### 1. Update System

```bash
sudo pacman -Syu
```

### 2. Enable Services

```bash
sudo systemctl enable NetworkManager
sudo systemctl enable sshd
sudo systemctl enable systemd-timesyncd
```

### 3. Install Desktop Environment

See [Desktop Guide](DESKTOP.md) for desktop environment installation.

## Troubleshooting

### Boot Issues

If the system doesn't boot:
1. Check GRUB configuration
2. Verify kernel parameters
3. Check EFI partition is mounted correctly

### Network Issues

If network doesn't work:
1. Check NetworkManager is running: `sudo systemctl status NetworkManager`
2. List connections: `nmcli connection show`
3. Connect to network: `nmcli connection up <connection-name>`

### Installation Issues

If installation fails:
1. Check disk partitions: `lsblk`
2. Verify mount points: `mount`
3. Check logs: `journalctl -xe`

## Rolling Release Model

SigmaOS follows a rolling release model:
- No release tags
- Continuous updates
- Snapshot-based rollbacks
- Use `sigmaos-release snapshot` to create snapshots
- Use `sigmaos-release rollback <snapshot>` to rollback

## Next Steps

After installation:
1. Configure your desktop environment
2. Install additional software
3. Set up backups
4. Configure security settings

## Support

For issues and questions:
- Check [Troubleshooting Guide](TROUBLESHOOTING.md)
- Check [FAQ](FAQ.md)
- Report issues on GitHub
