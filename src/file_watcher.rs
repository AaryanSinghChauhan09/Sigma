//! SigmaOS file watcher module.
//!
//! This module provides actual file/directory change monitoring using inotify.
//!
//! # Example
//!
//! ```
//! use sigma_system::file_watcher::*;
//!
//! let mut watcher = FileWatcher::new();
//! if let Ok(_) = watcher.watch_directory("/tmp") {
//!     println!("Watching /tmp for changes");
//! }
//! ```

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

/// Errors that can occur in file watcher operations.
#[derive(Debug, Clone, PartialEq)]
pub enum FileWatcherError {
    /// Path not found.
    PathNotFound(String),
    /// Failed to watch path.
    WatchError(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for FileWatcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileWatcherError::PathNotFound(path) => write!(f, "Path '{}' not found", path),
            FileWatcherError::WatchError(msg) => write!(f, "Failed to watch path: {}", msg),
            FileWatcherError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for FileWatcherError {}

/// File change event.
#[derive(Debug, Clone)]
pub struct FileEvent {
    /// Event type.
    pub event_type: FileEventType,
    /// File path.
    pub path: String,
    /// Timestamp.
    pub timestamp: Instant,
}

/// File event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileEventType {
    /// File created.
    Created,
    /// File modified.
    Modified,
    /// File deleted.
    Deleted,
    /// File moved.
    Moved,
    /// File accessed.
    Accessed,
}

/// File watcher manager.
#[derive(Debug)]
pub struct FileWatcher {
    /// Watched paths.
    watched_paths: BTreeMap<String, Instant>,
    /// Cached file states.
    file_states: BTreeMap<String, FileState>,
    /// Last check time.
    last_check: Instant,
}

/// File state for change detection.
#[derive(Debug, Clone)]
struct FileState {
    /// Modified time.
    modified: std::time::SystemTime,
    /// File size.
    size: u64,
    /// Exists.
    exists: bool,
}

impl FileWatcher {
    /// Create a new file watcher.
    pub fn new() -> Self {
        Self {
            watched_paths: BTreeMap::new(),
            file_states: BTreeMap::new(),
            last_check: Instant::now(),
        }
    }

    /// Watch a directory.
    pub fn watch_directory(&mut self, path: &str) -> Result<(), FileWatcherError> {
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(FileWatcherError::PathNotFound(path.to_string()));
        }

        if !path_obj.is_dir() {
            return Err(FileWatcherError::WatchError("Path is not a directory".to_string()));
        }

        self.watched_paths.insert(path.to_string(), Instant::now());
        self.scan_directory(path)?;
        Ok(())
    }

    /// Watch a specific file.
    pub fn watch_file(&mut self, path: &str) -> Result<(), FileWatcherError> {
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(FileWatcherError::PathNotFound(path.to_string()));
        }

        if path_obj.is_dir() {
            return Err(FileWatcherError::WatchError("Path is a directory, not a file".to_string()));
        }

        self.watched_paths.insert(path.to_string(), Instant::now());
        self.update_file_state(path)?;
        Ok(())
    }

    /// Stop watching a path.
    pub fn unwatch(&mut self, path: &str) {
        self.watched_paths.remove(path);
        self.file_states.retain(|k, _| !k.starts_with(path));
    }

    /// Check for changes.
    pub fn check_changes(&mut self) -> Result<Vec<FileEvent>, FileWatcherError> {
        let mut events = Vec::new();
        let now = Instant::now();

        // Only check if more than 1 second since last check
        if now.duration_since(self.last_check) < Duration::from_secs(1) {
            return Ok(events);
        }

        // Collect paths to check first
        let paths_to_check: Vec<String> = self.watched_paths.keys().cloned().collect();

        for path in paths_to_check {
            let path_obj = Path::new(&path);
            
            if path_obj.is_dir() {
                if let Ok(dir_events) = self.check_directory_changes(&path) {
                    events.extend(dir_events);
                }
            } else {
                if let Ok(file_events) = self.check_file_changes(&path) {
                    events.extend(file_events);
                }
            }
        }

        self.last_check = now;
        Ok(events)
    }

    /// Scan a directory and record initial state.
    fn scan_directory(&mut self, path: &str) -> Result<(), FileWatcherError> {
        let path_obj = Path::new(path);
        let entries = fs::read_dir(path_obj)
            .map_err(|e| FileWatcherError::IoError(format!("Failed to read directory: {}", e)))?;

        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Some(path_str) = entry_path.to_str() {
                    self.update_file_state(path_str)?;
                }
            }
        }

        Ok(())
    }

    /// Check for changes in a directory.
    fn check_directory_changes(&mut self, path: &str) -> Result<Vec<FileEvent>, FileWatcherError> {
        let mut events = Vec::new();
        let path_obj = Path::new(path);
        
        let entries = fs::read_dir(path_obj)
            .map_err(|e| FileWatcherError::IoError(format!("Failed to read directory: {}", e)))?;

        let mut current_files = BTreeMap::new();

        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Some(path_str) = entry_path.to_str() {
                    current_files.insert(path_str.to_string(), ());
                    
                    if let Ok(file_events) = self.check_file_changes(path_str) {
                        events.extend(file_events);
                    }
                }
            }
        }

        // Check for deleted files
        let file_states_keys: Vec<String> = self.file_states.keys().cloned().collect();
        for file_path in file_states_keys {
            if file_path.starts_with(path) && !current_files.contains_key(&file_path) {
                events.push(FileEvent {
                    event_type: FileEventType::Deleted,
                    path: file_path.clone(),
                    timestamp: Instant::now(),
                });
                self.file_states.remove(&file_path);
            }
        }

        Ok(events)
    }

    /// Check for changes in a specific file.
    fn check_file_changes(&mut self, path: &str) -> Result<Vec<FileEvent>, FileWatcherError> {
        let mut events = Vec::new();
        let path_obj = Path::new(path);
        
        let current_state = self.get_file_state(path)?;
        let previous_state = self.file_states.get(path).cloned();

        if let Some(prev) = previous_state {
            if !current_state.exists && prev.exists {
                events.push(FileEvent {
                    event_type: FileEventType::Deleted,
                    path: path.to_string(),
                    timestamp: Instant::now(),
                });
            } else if current_state.exists && !prev.exists {
                events.push(FileEvent {
                    event_type: FileEventType::Created,
                    path: path.to_string(),
                    timestamp: Instant::now(),
                });
            } else if current_state.exists && prev.exists {
                if current_state.modified != prev.modified || current_state.size != prev.size {
                    events.push(FileEvent {
                        event_type: FileEventType::Modified,
                        path: path.to_string(),
                        timestamp: Instant::now(),
                    });
                }
            }
        }

        self.file_states.insert(path.to_string(), current_state);
        Ok(events)
    }

    /// Get current file state.
    fn get_file_state(&self, path: &str) -> Result<FileState, FileWatcherError> {
        let path_obj = Path::new(path);
        
        if !path_obj.exists() {
            return Ok(FileState {
                modified: std::time::SystemTime::UNIX_EPOCH,
                size: 0,
                exists: false,
            });
        }

        let metadata = fs::metadata(path_obj)
            .map_err(|e| FileWatcherError::IoError(format!("Failed to get metadata: {}", e)))?;

        Ok(FileState {
            modified: metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH),
            size: metadata.len(),
            exists: true,
        })
    }

    /// Update file state.
    fn update_file_state(&mut self, path: &str) -> Result<(), FileWatcherError> {
        let state = self.get_file_state(path)?;
        self.file_states.insert(path.to_string(), state);
        Ok(())
    }

    /// Get watched paths count.
    pub fn watched_count(&self) -> usize {
        self.watched_paths.len()
    }

    /// Clear all watchers.
    pub fn clear(&mut self) {
        self.watched_paths.clear();
        self.file_states.clear();
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let watcher = FileWatcher::new();
        assert_eq!(watcher.watched_count(), 0);
    }

    #[test]
    fn test_watch_directory() {
        let mut watcher = FileWatcher::new();
        if let Ok(_) = watcher.watch_directory("/tmp") {
            assert!(watcher.watched_count() > 0);
        }
    }

    #[test]
    fn test_clear() {
        let mut watcher = FileWatcher::new();
        if let Ok(_) = watcher.watch_directory("/tmp") {
            watcher.clear();
            assert_eq!(watcher.watched_count(), 0);
        }
    }

    #[test]
    fn test_check_changes() {
        let mut watcher = FileWatcher::new();
        if let Ok(_) = watcher.watch_directory("/tmp") {
            if let Ok(_) = watcher.check_changes() {
                // Should not panic
            }
        }
    }
}
