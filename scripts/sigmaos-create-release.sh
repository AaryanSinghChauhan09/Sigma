#!/bin/bash
# SigmaOS release creation script
# Creates a release snapshot with changelog

set -e

VERSION="0.5.0"
BUILD_DATE=$(date +%Y%m%d)
RELEASE_NAME="sigmaos-$VERSION-$BUILD_DATE"

echo "=== SigmaOS Release Creation ==="
echo "Version: $VERSION"
echo "Build Date: $BUILD_DATE"
echo ""

# Color output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
section() { echo -e "${BLUE}[${1}]${NC}"; }

# Create release directory
section "Creating Release Directory"
RELEASE_DIR="./releases/$RELEASE_NAME"
mkdir -p "$RELEASE_DIR"
info "Release directory: $RELEASE_DIR"

# Copy library
section "Copying Library"
mkdir -p "$RELEASE_DIR/library"
cp -r src Cargo.toml Cargo.lock README.md LICENSE "$RELEASE_DIR/library/"
info "Library copied"

# Copy CLI
section "Copying CLI"
mkdir -p "$RELEASE_DIR/cli"
cp -r cli "$RELEASE_DIR/"
info "CLI copied"

# Copy scripts
section "Copying Scripts"
mkdir -p "$RELEASE_DIR/scripts"
cp scripts/*.sh "$RELEASE_DIR/scripts/"
chmod +x "$RELEASE_DIR/scripts"/*.sh
info "Scripts copied"

# Copy configuration
section "Copying Configuration"
mkdir -p "$RELEASE_DIR/config"
cp config/* "$RELEASE_DIR/config/"
info "Configuration copied"

# Copy documentation
section "Copying Documentation"
mkdir -p "$RELEASE_DIR/docs"
cp *.md "$RELEASE_DIR/docs/"
info "Documentation copied"

# Copy packages
section "Copying Packages"
mkdir -p "$RELEASE_DIR/packages"
cp -r packages "$RELEASE_DIR/"
info "Packages copied"

# Create changelog
section "Creating Changelog"
cat > "$RELEASE_DIR/CHANGELOG.md" << EOF
# SigmaOS $VERSION Release

**Build Date:** $BUILD_DATE

## What's New

### System Library
- 72 modules covering all major Linux subsystems
- 511 unit tests, 82 doctests
- Enterprise-grade operations (RAID, LVM, GPU, compliance, metrics, profiling)

### Distribution Tools
- 14 system management tools
- Enhanced ISO build script with comprehensive package list
- Improved installer with colored output
- Container-based build option
- Post-installation configuration
- System update and cleanup scripts
- Hardware detection
- Driver installation
- Desktop environment installation
- System health and maintenance services

### Configuration
- Comprehensive boot-level configurations
- GRUB, systemd-boot, kernel parameters
- Network, DNS, system tuning
- Live environment configuration

### Documentation
- Updated README with new statistics
- Enhanced BOOTABLE_PLAN.md
- INSTALL.md, DESKTOP.md, TOOLS.md

## Installation

See [INSTALL.md](docs/INSTALL.md) for installation instructions.

## Requirements

- Arch Linux or Arch-based system
- 4GB RAM minimum
- 20GB disk space minimum

## Download

For the latest release, visit:
https://github.com/AaryanSinghChauhan09/Sigma/releases

## Support

- GitHub: https://github.com/AaryanSinghChauhan09/Sigma
- Wiki: https://github.com/AaryanSinghChauhan09/Sigma/wiki
- Issues: https://github.com/AaryanSinghChauhan09/Sigma/issues

## License

GPL-3.0-or-later
EOF
info "Changelog created"

# Create release archive
section "Creating Release Archive"
cd releases
tar -czf "$RELEASE_NAME.tar.gz" "$RELEASE_NAME"
cd - > /dev/null
info "Archive created: releases/$RELEASE_NAME.tar.gz"

# Generate checksum
section "Generating Checksum"
cd releases
sha256sum "$RELEASE_NAME.tar.gz" > "$RELEASE_NAME.tar.gz.sha256"
cd - > /dev/null
info "Checksum generated"

echo ""
echo "=========================================="
info "=== Release Created ==="
echo "=========================================="
echo ""
echo "Release: $RELEASE_NAME"
echo "Archive: releases/$RELEASE_NAME.tar.gz"
echo "Checksum: releases/$RELEASE_NAME.tar.gz.sha256"
echo ""
echo "To upload to GitHub:"
echo "  gh release create v$VERSION --title 'SigmaOS $VERSION' --notes 'See CHANGELOG.md' releases/$RELEASE_NAME.tar.gz releases/$RELEASE_NAME.tar.gz.sha256"
echo "=========================================="
