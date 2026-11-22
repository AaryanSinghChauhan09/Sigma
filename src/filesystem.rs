//! Filesystem management inspired by Linux VFS and Arch Linux storage management.
//! Provides filesystem management, ZFS/Btrfs support, and snapshot operations.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSystemType {
    Ext4,
    Xfs,
    Btrfs,
    Zfs,
    F2fs,
    Swap,
}

impl FileSystemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ext4 => "ext4",
            Self::Xfs => "xfs",
            Self::Btrfs => "btrfs",
            Self::Zfs => "zfs",
            Self::F2fs => "f2fs",
            Self::Swap => "swap",
        }
    }

    pub fn supports_snapshots(&self) -> bool {
        matches!(self, Self::Btrfs | Self::Zfs)
    }

    pub fn supports_compression(&self) -> bool {
        matches!(self, Self::Btrfs | Self::Zfs | Self::F2fs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountOption {
    Noatime,
    Nodiratime,
    Relatime,
    Noexec,
    Nosuid,
    Nodev,
    Ro,
    Rw,
    Compress,
    Zstd,
    Lzo,
}

impl MountOption {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Noatime => "noatime",
            Self::Nodiratime => "nodiratime",
            Self::Relatime => "relatime",
            Self::Noexec => "noexec",
            Self::Nosuid => "nosuid",
            Self::Nodev => "nodev",
            Self::Ro => "ro",
            Self::Rw => "rw",
            Self::Compress => "compress",
            Self::Zstd => "compress=zstd",
            Self::Lzo => "compress=lzo",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileSystemError {
    InvalidPath(String),
    InvalidDevice(String),
    SnapshotNotSupported(String),
    InvalidOption(String),
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath(msg) => write!(f, "invalid path: {}", msg),
            Self::InvalidDevice(msg) => write!(f, "invalid device: {}", msg),
            Self::SnapshotNotSupported(msg) => write!(f, "snapshot not supported: {}", msg),
            Self::InvalidOption(msg) => write!(f, "invalid option: {}", msg),
        }
    }
}

impl std::error::Error for FileSystemError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    pub id: String,
    pub name: String,
    pub filesystem: String,
    pub timestamp: u64,
    pub readonly: bool,
}

impl Snapshot {
    pub fn new(id: impl Into<String>, name: impl Into<String>, filesystem: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            filesystem: filesystem.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            readonly: true,
        }
    }

    pub fn writable(mut self) -> Self {
        self.readonly = false;
        self
    }
}

#[derive(Debug, Clone)]
pub struct MountPoint {
    pub device: String,
    pub mount_point: String,
    pub fs_type: FileSystemType,
    pub options: Vec<MountOption>,
}

impl MountPoint {
    pub fn new(device: impl Into<String>, mount_point: impl Into<String>, fs_type: FileSystemType) -> Self {
        Self {
            device: device.into(),
            mount_point: mount_point.into(),
            fs_type,
            options: vec![MountOption::Rw, MountOption::Relatime],
        }
    }

    pub fn with_option(mut self, option: MountOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn with_options(mut self, options: Vec<MountOption>) -> Self {
        self.options.extend(options);
        self
    }

    pub fn readonly(mut self) -> Self {
        self.options.push(MountOption::Ro);
        self.options.retain(|o| *o != MountOption::Rw);
        self
    }

    pub fn generate_fstab_entry(&self) -> String {
        let opts: Vec<String> = self.options.iter().map(|o| o.as_str().to_string()).collect();
        format!(
            "{} {} {} {} 0 0",
            self.device,
            self.mount_point,
            self.fs_type.as_str(),
            opts.join(",")
        )
    }
}

#[derive(Debug, Clone)]
pub struct FileSystemManager {
    pub mount_points: BTreeMap<String, MountPoint>,
    pub snapshots: BTreeMap<String, Snapshot>,
    pub default_fs_type: FileSystemType,
}

impl Default for FileSystemManager {
    fn default() -> Self {
        Self {
            mount_points: BTreeMap::new(),
            snapshots: BTreeMap::new(),
            default_fs_type: FileSystemType::Ext4,
        }
    }
}

impl FileSystemManager {
    pub fn new(default_fs_type: FileSystemType) -> Self {
        Self {
            mount_points: BTreeMap::new(),
            snapshots: BTreeMap::new(),
            default_fs_type,
        }
    }

    pub fn add_mount_point(&mut self, mount: MountPoint) -> Result<(), FileSystemError> {
        let mp = mount.mount_point.clone();
        if mp.trim().is_empty() {
            return Err(FileSystemError::InvalidPath("mount point cannot be empty".to_string()));
        }
        if mount.device.trim().is_empty() {
            return Err(FileSystemError::InvalidDevice("device cannot be empty".to_string()));
        }
        self.mount_points.insert(mp, mount);
        Ok(())
    }

    pub fn remove_mount_point(&mut self, mount_point: &str) -> Option<MountPoint> {
        self.mount_points.remove(mount_point)
    }

    pub fn get_mount_point(&self, mount_point: &str) -> Option<&MountPoint> {
        self.mount_points.get(mount_point)
    }

    pub fn create_snapshot(&mut self, snapshot: Snapshot) -> Result<(), FileSystemError> {
        let fs_type = self.get_filesystem_type(&snapshot.filesystem);
        if !fs_type.supports_snapshots() {
            return Err(FileSystemError::SnapshotNotSupported(
                format!("filesystem type {} does not support snapshots", fs_type.as_str())
            ));
        }
        let id = snapshot.id.clone();
        if id.trim().is_empty() {
            return Err(FileSystemError::InvalidPath("snapshot ID cannot be empty".to_string()));
        }
        self.snapshots.insert(id, snapshot);
        Ok(())
    }

    pub fn remove_snapshot(&mut self, id: &str) -> Option<Snapshot> {
        self.snapshots.remove(id)
    }

    pub fn get_snapshot(&self, id: &str) -> Option<&Snapshot> {
        self.snapshots.get(id)
    }

    pub fn get_filesystem_type(&self, filesystem: &str) -> FileSystemType {
        self.mount_points
            .values()
            .find(|m| m.device == filesystem || m.mount_point == filesystem)
            .map(|m| m.fs_type)
            .unwrap_or(self.default_fs_type)
    }

    pub fn generate_fstab(&self) -> String {
        let mut fstab = String::new();
        
        fstab.push_str("# /etc/fstab: static file system information\n");
        fstab.push_str("# Generated by SigmaOS FileSystemManager\n");
        fstab.push_str("# <file system> <mount point>   <type>  <options>       <dump>  <pass>\n");
        fstab.push_str("\n");
        
        for mount in self.mount_points.values() {
            fstab.push_str(&mount.generate_fstab_entry());
            fstab.push_str("\n");
        }
        
        fstab
    }

    pub fn generate_crypttab(&self) -> String {
        let mut crypttab = String::new();
        
        crypttab.push_str("# /etc/crypttab: encryption mappings\n");
        crypttab.push_str("# Generated by SigmaOS FileSystemManager\n");
        crypttab.push_str("# <target name> <source device>         <key file>      <options>\n");
        crypttab.push_str("\n");
        
        for mount in self.mount_points.values() {
            if mount.device.starts_with("/dev/mapper/") {
                let name = mount.device.strip_prefix("/dev/mapper/").unwrap_or(&mount.device);
                crypttab.push_str(&format!("{} UUID=none none luks\n", name));
            }
        }
        
        crypttab
    }

    pub fn generate_btrfs_subvolume_commands(&self) -> String {
        let mut commands = String::new();
        
        commands.push_str("# SigmaOS Btrfs Subvolume Management\n");
        commands.push_str("# Generated by SigmaOS FileSystemManager\n\n");
        
        for mount in self.mount_points.values() {
            if mount.fs_type == FileSystemType::Btrfs {
                commands.push_str(&format!("# Subvolume: {}\n", mount.mount_point));
                commands.push_str(&format!("btrfs subvolume create {}\n", mount.mount_point));
                commands.push_str("\n");
            }
        }
        
        commands
    }

    pub fn generate_zfs_pool_commands(&self) -> String {
        let mut commands = String::new();
        
        commands.push_str("# SigmaOS ZFS Pool Management\n");
        commands.push_str("# Generated by SigmaOS FileSystemManager\n\n");
        
        for mount in self.mount_points.values() {
            if mount.fs_type == FileSystemType::Zfs {
                commands.push_str(&format!("# Pool: {}\n", mount.device));
                commands.push_str(&format!("zpool create {} {}\n", mount.device, mount.mount_point));
                commands.push_str("\n");
            }
        }
        
        commands
    }

    pub fn get_snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    pub fn get_mount_point_count(&self) -> usize {
        self.mount_points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_type_as_str() {
        assert_eq!(FileSystemType::Btrfs.as_str(), "btrfs");
        assert_eq!(FileSystemType::Zfs.as_str(), "zfs");
    }

    #[test]
    fn test_filesystem_type_supports_snapshots() {
        assert!(FileSystemType::Btrfs.supports_snapshots());
        assert!(FileSystemType::Zfs.supports_snapshots());
        assert!(!FileSystemType::Ext4.supports_snapshots());
    }

    #[test]
    fn test_mount_option_as_str() {
        assert_eq!(MountOption::Noatime.as_str(), "noatime");
        assert_eq!(MountOption::Zstd.as_str(), "compress=zstd");
    }

    #[test]
    fn test_snapshot_new() {
        let snap = Snapshot::new("snap1", "backup", "/home");
        assert_eq!(snap.id, "snap1");
        assert!(snap.readonly);
    }

    #[test]
    fn test_snapshot_writable() {
        let snap = Snapshot::new("snap1", "backup", "/home").writable();
        assert!(!snap.readonly);
    }

    #[test]
    fn test_mount_point_new() {
        let mount = MountPoint::new("/dev/sda1", "/mnt", FileSystemType::Ext4);
        assert_eq!(mount.device, "/dev/sda1");
        assert_eq!(mount.mount_point, "/mnt");
    }

    #[test]
    fn test_mount_point_with_option() {
        let mount = MountPoint::new("/dev/sda1", "/mnt", FileSystemType::Ext4)
            .with_option(MountOption::Noatime);
        assert!(mount.options.contains(&MountOption::Noatime));
    }

    #[test]
    fn test_mount_point_readonly() {
        let mount = MountPoint::new("/dev/sda1", "/mnt", FileSystemType::Ext4).readonly();
        assert!(mount.options.contains(&MountOption::Ro));
        assert!(!mount.options.contains(&MountOption::Rw));
    }

    #[test]
    fn test_generate_fstab_entry() {
        let mount = MountPoint::new("/dev/sda1", "/mnt", FileSystemType::Ext4);
        let entry = mount.generate_fstab_entry();
        assert!(entry.contains("/dev/sda1"));
        assert!(entry.contains("/mnt"));
        assert!(entry.contains("ext4"));
    }

    #[test]
    fn test_filesystem_manager_new() {
        let mgr = FileSystemManager::new(FileSystemType::Btrfs);
        assert_eq!(mgr.default_fs_type, FileSystemType::Btrfs);
    }

    #[test]
    fn test_filesystem_manager_add_mount_point() {
        let mut mgr = FileSystemManager::default();
        let mount = MountPoint::new("/dev/sda1", "/mnt", FileSystemType::Ext4);
        mgr.add_mount_point(mount).unwrap();
        assert_eq!(mgr.get_mount_point_count(), 1);
    }

    #[test]
    fn test_filesystem_manager_add_mount_point_invalid() {
        let mut mgr = FileSystemManager::default();
        let mount = MountPoint::new("", "/mnt", FileSystemType::Ext4);
        assert!(mgr.add_mount_point(mount).is_err());
    }

    #[test]
    fn test_filesystem_manager_create_snapshot_supported() {
        let mut mgr = FileSystemManager::new(FileSystemType::Btrfs);
        let snap = Snapshot::new("snap1", "backup", "/home");
        mgr.create_snapshot(snap).unwrap();
        assert_eq!(mgr.get_snapshot_count(), 1);
    }

    #[test]
    fn test_filesystem_manager_create_snapshot_unsupported() {
        let mut mgr = FileSystemManager::new(FileSystemType::Ext4);
        let snap = Snapshot::new("snap1", "backup", "/home");
        assert!(mgr.create_snapshot(snap).is_err());
    }

    #[test]
    fn test_generate_fstab() {
        let mut mgr = FileSystemManager::default();
        let mount = MountPoint::new("/dev/sda1", "/mnt", FileSystemType::Ext4);
        mgr.add_mount_point(mount).unwrap();
        let fstab = mgr.generate_fstab();
        assert!(fstab.contains("/etc/fstab"));
        assert!(fstab.contains("/dev/sda1"));
    }

    #[test]
    fn test_generate_btrfs_subvolume_commands() {
        let mut mgr = FileSystemManager::default();
        let mount = MountPoint::new("/dev/sda1", "/mnt", FileSystemType::Btrfs);
        mgr.add_mount_point(mount).unwrap();
        let commands = mgr.generate_btrfs_subvolume_commands();
        assert!(commands.contains("btrfs subvolume create"));
    }

    #[test]
    fn test_generate_zfs_pool_commands() {
        let mut mgr = FileSystemManager::default();
        let mount = MountPoint::new("tank", "/mnt", FileSystemType::Zfs);
        mgr.add_mount_point(mount).unwrap();
        let commands = mgr.generate_zfs_pool_commands();
        assert!(commands.contains("zpool create"));
    }
}
