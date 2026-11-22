# SigmaOS Filesystem Management Module

## Overview
The filesystem management module provides filesystem management, ZFS/Btrfs support, and snapshot operations. It is inspired by Linux VFS and Arch Linux storage management.

## Features
- **Multiple Filesystems**: Support for Ext4, Xfs, Btrfs, Zfs, F2fs, Swap
- **Snapshot Management**: Create and manage Btrfs/ZFS snapshots
- **Mount Configuration**: Generate fstab and crypttab entries
- **Btrfs Subvolumes**: Generate Btrfs subvolume commands
- **ZFS Pools**: Generate ZFS pool creation commands

## Usage

```rust
use sigma_system::{FileSystemManager, FileSystemType, MountPoint, MountOption, Snapshot};

// Create a filesystem manager
let mut mgr = FileSystemManager::new(FileSystemType::Btrfs);

// Add mount points
let root = MountPoint::new("/dev/sda1", "/", FileSystemType::Btrfs)
    .with_option(MountOption::Noatime)
    .with_option(MountOption::Compress);
mgr.add_mount_point(root).unwrap();

let home = MountPoint::new("/dev/sda2", "/home", FileSystemType::Btrfs)
    .with_option(MountOption::Noatime);
mgr.add_mount_point(home).unwrap();

// Create snapshots
let snapshot = Snapshot::new("snap-2024-01-01", "backup", "/home");
mgr.create_snapshot(snapshot).unwrap();

// Generate configurations
let fstab = mgr.generate_fstab();
let crypttab = mgr.generate_crypttab();
let btrfs_commands = mgr.generate_btrfs_subvolume_commands();
```

## Configuration

### FileSystemType
- `Ext4`: Standard Linux filesystem
- `Xfs`: High-performance filesystem
- `Btrfs`: Advanced filesystem with snapshots
- `Zfs`: Enterprise-grade filesystem
- `F2fs`: Flash-friendly filesystem
- `Swap`: Swap space

### MountOption
- `Noatime`: Disable access time updates
- `Nodiratime`: Disable directory access time updates
- `Relatime`: Relative access time updates
- `Noexec`: Disable execution
- `Nosuid`: Disable SUID/SGID
- `Nodev`: Disable device files
- `Ro`: Read-only
- `Rw`: Read-write
- `Compress`: Enable compression
- `Zstd`: Use Zstd compression
- `Lzo`: Use LZO compression

### Snapshot
- `id`: Snapshot identifier
- `name`: Snapshot name
- `filesystem`: Target filesystem
- `timestamp`: Creation timestamp
- `readonly`: Read-only or writable

## API Reference

### Methods
- `add_mount_point()`: Add a mount point configuration
- `create_snapshot()`: Create a filesystem snapshot
- `remove_snapshot()`: Remove a snapshot
- `generate_fstab()`: Generate /etc/fstab entries
- `generate_crypttab()`: Generate /etc/crypttab entries
- `generate_btrfs_subvolume_commands()`: Generate Btrfs commands
- `generate_zfs_pool_commands()`: Generate ZFS commands

## Examples

### Configuring Btrfs with Compression
```rust
let root = MountPoint::new("/dev/sda1", "/", FileSystemType::Btrfs)
    .with_option(MountOption::Noatime)
    .with_option(MountOption::Zstd)
    .with_option(MountOption::Discard);
mgr.add_mount_point(root).unwrap();
```

### Creating Snapshots
```rust
// Read-only snapshot
let backup = Snapshot::new("backup-$(date +%Y%m%d)", "system-backup", "/");

// Writable snapshot for testing
let test = Snapshot::new("test-snapshot", "test", "/").writable();
```

### ZFS Pool Configuration
```rust
let zfs_mgr = FileSystemManager::new(FileSystemType::Zfs);
let pool = MountPoint::new("tank", "/mnt", FileSystemType::Zfs);
zfs_mgr.add_mount_point(pool).unwrap();
let commands = zfs_mgr.generate_zfs_pool_commands();
```

## Integration with Other Modules
The filesystem module integrates with:
- **Security module**: LUKS encryption for disk security
- **Boot module**: Root filesystem for boot
- **Memory module**: Swap partition configuration
- **Init module**: Filesystem-dependent service ordering

## Principles Applied
- Single Responsibility: Only manages filesystem subsystem
- Fail-Fast: Validates paths before adding mount points
- Clear Abstraction: Well-defined public API
- AAA Testing: All tests follow Arrange-Act-Assert pattern
