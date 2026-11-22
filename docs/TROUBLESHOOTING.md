# SigmaOS Troubleshooting Guide

## Installation Issues

### Disk Partitioning Fails

**Problem:** `parted` command fails
**Solution:**
```bash
# Check if disk is in use
lsblk

# Unmount if mounted
umount /dev/sda*

# Use wipefs to clear disk
wipefs -a /dev/sda
```

### Bootloader Installation Fails

**Problem:** `grub-install` fails
**Solution:**
```bash
# Check EFI partition is mounted
mount | grep boot

# Reinstall GRUB
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=SigmaOS --recheck
```

### System Won't Boot

**Problem:** System hangs at boot
**Solution:**
1. Boot from live USB
2. Chroot into installed system
3. Check kernel parameters
4. Regenerate initramfs: `mkinitcpio -P`
5. Regenerate GRUB config: `grub-mkconfig -o /boot/grub/grub.cfg`

## Network Issues

### No Network Connection

**Problem:** Network not working after installation
**Solution:**
```bash
# Check NetworkManager status
sudo systemctl status NetworkManager

# Start NetworkManager
sudo systemctl start NetworkManager

# List connections
nmcli connection show

# Connect to network
nmcli connection up <connection-name>
```

### WiFi Not Working

**Problem:** WiFi connection fails
**Solution:**
```bash
# Scan for networks
nmcli device wifi list

# Connect to WiFi
nmcli device wifi connect <SSID> password <password>
```

## Service Issues

### Service Won't Start

**Problem:** Service fails to start
**Solution:**
```bash
# Check service status
sudo systemctl status <service>

# Check journal logs
sudo journalctl -xe -u <service>

# Try restarting
sudo systemctl restart <service>
```

### Service Fails at Boot

**Problem:** Service fails to start automatically
**Solution:**
```bash
# Enable service
sudo systemctl enable <service>

# Check if service is masked
sudo systemctl unmask <service>
```

## Package Issues

### Package Installation Fails

**Problem:** `pacman` cannot install package
**Solution:**
```bash
# Update package databases
sudo pacman -Sy

# Fix corrupted database
sudo pacman -Syu

# Clear package cache
sudo pacman -Sc
```

### Dependency Conflicts

**Problem:** Package conflicts during installation
**Solution:**
```bash
# Check for conflicts
sudo pacman -Syu

# Use --asdeps for automatic dependencies
sudo pacman -S --asdeps <package>
```

## Performance Issues

### System Slow

**Problem:** System performance degraded
**Solution:**
```bash
# Check CPU usage
htop

# Check memory usage
free -h

# Check disk I/O
iotop

# Check system load
uptime
```

### High Memory Usage

**Problem:** Memory usage is high
**Solution:**
```bash
# Check memory usage
free -h

# Check swap usage
swapon --show

# Clear page cache
sudo sync
sudo echo 3 > /proc/sys/vm/drop_caches
```

## Hardware Issues

### GPU Not Detected

**Problem:** GPU not recognized
**Solution:**
```bash
# Check for GPU
lspci | grep -i vga

# Check loaded modules
lsmod | grep -i nvidia

# Install appropriate drivers
sudo pacman -S nvidia  # for NVIDIA
sudo pacman -S xf86-video-amdgpu  # for AMD
```

### Audio Not Working

**Problem:** No sound output
**Solution:**
```bash
# Check audio devices
aplay -l

# Check volume
amixer set Master 100%

# Unmute channels
amixer set Master unmute

# Restart PipeWire
sudo systemctl restart pipewire
```

## Debugging

### Enable Debug Logging

```bash
# Enable verbose logging
sudo journalctl -f

# Check boot messages
sudo dmesg | tail
```

### Check System Logs

```bash
# View all logs
sudo journalctl

# View boot logs
sudo journalctl -b

# View service logs
sudo journalctl -u <service>
```

## Common Errors

### Permission Denied

**Problem:** Command fails with "Permission denied"
**Solution:**
```bash
# Use sudo
sudo <command>

# Or check file permissions
ls -l <file>
```

### Command Not Found

**Problem:** Command not found
**Solution:**
```bash
# Check if package is installed
pacman -Qs <package>

# Install package
sudo pacman -S <package>
```

### Library Not Found

**Problem:** Library not found error
**Solution:**
```bash
# Update library cache
sudo ldconfig

# Install missing library
sudo pacman -S <library>
```

## Getting Help

If you can't resolve your issue:
1. Check the [FAQ](FAQ.md)
2. Check the [API Reference](API.md)
3. Report issue on GitHub with:
   - SigmaOS version
   - Hardware specs
   - Error messages
   - Steps to reproduce
