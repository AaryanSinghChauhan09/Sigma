#!/bin/bash
# SigmaOS Live System Configuration
# Configures the live ISO environment

set -e

# Set hostname
echo "sigmaos-live" > /etc/hostname

# Set locale
echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf

# Set timezone
ln -sf /usr/share/zoneinfo/UTC /etc/localtime

# Set root password
echo "root:sigmaos" | chpasswd

# Create sigmaos user
useradd -m -G wheel -s /bin/bash sigmaos
echo "sigmaos:sigmaos" | chpasswd
echo "%wheel ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/wheel

# Enable essential services
systemctl enable NetworkManager
systemctl enable sshd
systemctl enable systemd-timesyncd

# Configure sysctl
cat > /etc/sysctl.d/99-sigmaos.conf << 'EOF'
net.ipv4.ip_forward = 1
net.ipv4.tcp_fastopen = 3
net.ipv4.tcp_congestion_control = bbr
vm.swappiness = 10
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5
EOF

# Create SigmaOS version file
mkdir -p /etc/sigmaos
cat > /etc/sigmaos/version << 'EOF'
SigmaOS 0.5.0 Live
Build Date: $(date)
Repository: https://github.com/AaryanSinghChauhan09/Sigma
EOF

# Copy installer script
if [ -f /root/install-sigmaos.sh ]; then
    chmod +x /root/install-sigmaos.sh
fi

echo "SigmaOS live system configured"
