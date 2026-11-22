#!/bin/bash
# SigmaOS Live System Configuration
# Applied during ISO build to configure the live environment

set -e

echo "=== SigmaOS Live Configuration ==="

# Set hostname
echo "sigmaos-live" > /etc/hostname

# Set locale
echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf

# Set timezone
ln -sf /usr/share/zoneinfo/UTC /etc/localtime

# Set keymap
echo "KEYMAP=us" > /etc/vconsole.conf
echo "FONT=lat9w-16" >> /etc/vconsole.conf

# Configure sudo
echo "%wheel ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/wheel
chmod 440 /etc/sudoers.d/wheel

# Configure NetworkManager
systemctl enable NetworkManager

# Configure SSH
systemctl enable sshd
sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# Configure systemd-timesyncd
systemctl enable systemd-timesyncd

# Configure sysctl
cat > /etc/sysctl.d/99-sigmaos.conf << 'EOF'
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

# Configure NetworkManager
mkdir -p /etc/NetworkManager
cat > /etc/NetworkManager/NetworkManager.conf << 'EOF'
[main]
rc-manager=resolvconf
dhcp=internal
dns=none
no-auto-default=*

[connection]
ipv6.ip6-privacy=1

[device]
wifi.scan-rand-mac-address=yes
wifi.backend=iwd
EOF

# Configure resolved
cat > /etc/systemd/resolved.conf << 'EOF'
[Resolve]
DNS=1.1.1.1
DNS=1.0.0.1
DNS=8.8.8.8
FallbackDNS=9.9.9.9
DNSOverTLS=yes
DNSSEC=yes
EDNS0=yes
CacheSize=4096
StubListener=yes
LLMNR=yes
MulticastDNS=yes
ReadEtcHosts=yes
EOF

# Create SigmaOS version file
mkdir -p /etc/sigmaos
cat > /etc/sigmaos/version << 'EOF'
SigmaOS 0.5.0 (Live)
Build Date: AUTO
Repository: https://github.com/AaryanSinghChauhan09/Sigma
Based on: Arch Linux
EOF

# Create MOTD
cat > /etc/motd << 'EOF'
==========================================
       Welcome to SigmaOS Live System
==========================================

SigmaOS is a Linux distribution based on Arch Linux.
Repository: https://github.com/AaryanSinghChauhan09/Sigma

Live Environment:
  - Install: /root/install-sigmaos.sh
  - Help: https://github.com/AaryanSinghChauhan09/Sigma/wiki

Default user: root
Default password: sigmaos

==========================================
EOF

# Configure auto-login for tty1
mkdir -p /etc/systemd/system/getty@tty1.service.d
cat > /etc/systemd/system/getty@tty1.service.d/autologin.conf << 'EOF'
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin root --noclear %I $TERM
EOF

# Disable sleep/suspend in live environment
systemctl mask sleep.target suspend.target hibernate.target hybrid-sleep.target

echo "Live configuration complete"
