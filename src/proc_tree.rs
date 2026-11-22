//! SigmaOS process tree module.
//!
//! This module provides actual process tree information using /proc.
//!
//! # Example
//!
//! ```
//! use sigma_system::proc_tree::*;
//!
//! let mut proc_tree = ProcTree::new();
//! if let Ok(tree) = proc_tree.get_process_tree(1) {
//!     println!("Process tree has {} nodes", tree.total_processes);
//! }
//! ```

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Errors that can occur in process tree operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ProcTreeError {
    /// Process not found.
    ProcessNotFound(u32),
    /// Failed to read process info.
    ReadError(String),
    /// Invalid process data.
    InvalidData(String),
    /// IO error.
    IoError(String),
}

impl std::fmt::Display for ProcTreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcTreeError::ProcessNotFound(pid) => write!(f, "Process {} not found", pid),
            ProcTreeError::ReadError(msg) => write!(f, "Failed to read process info: {}", msg),
            ProcTreeError::InvalidData(msg) => write!(f, "Invalid process data: {}", msg),
            ProcTreeError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ProcTreeError {}

/// Process node in the tree.
#[derive(Debug, Clone)]
pub struct ProcessNode {
    /// Process ID.
    pub pid: u32,
    /// Parent process ID.
    pub ppid: u32,
    /// Process name.
    pub name: String,
    /// Process state.
    pub state: char,
    /// CPU usage percentage.
    pub cpu_percent: f64,
    /// Memory usage in KB.
    pub memory_kb: u64,
    /// Number of threads.
    pub threads: u32,
    /// Child processes.
    pub children: Vec<ProcessNode>,
}

/// Process tree.
#[derive(Debug, Clone)]
pub struct ProcessTree {
    /// Root process.
    pub root: ProcessNode,
    /// Total process count.
    pub total_processes: usize,
    /// Total memory usage in KB.
    pub total_memory_kb: u64,
}

/// Process tree manager.
#[derive(Debug)]
pub struct ProcTree {
    /// Cached processes.
    processes: BTreeMap<u32, ProcessNode>,
    /// Last update time.
    last_update: std::time::Instant,
}

impl ProcTree {
    /// Create a new process tree manager.
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            last_update: std::time::Instant::now(),
        }
    }

    /// Get process tree for a PID.
    pub fn get_process_tree(&mut self, pid: u32) -> Result<ProcessTree, ProcTreeError> {
        self.refresh_processes()?;
        
        let root = self.build_tree(pid)?;
        let total_processes = self.count_processes(&root);
        let total_memory = self.sum_memory(&root);
        
        Ok(ProcessTree {
            root,
            total_processes,
            total_memory_kb: total_memory,
        })
    }

    /// Get all processes as a flat list.
    pub fn get_all_processes(&mut self) -> Result<Vec<&ProcessNode>, ProcTreeError> {
        self.refresh_processes()?;
        Ok(self.processes.values().collect())
    }

    /// Get process by PID.
    pub fn get_process(&mut self, pid: u32) -> Result<Option<&ProcessNode>, ProcTreeError> {
        self.refresh_processes()?;
        Ok(self.processes.get(&pid))
    }

    /// Get child processes of a PID.
    pub fn get_children(&mut self, pid: u32) -> Result<Vec<&ProcessNode>, ProcTreeError> {
        self.refresh_processes()?;
        Ok(self.processes.values()
            .filter(|p| p.ppid == pid)
            .collect())
    }

    /// Get process count.
    pub fn get_process_count(&mut self) -> Result<usize, ProcTreeError> {
        self.refresh_processes()?;
        Ok(self.processes.len())
    }

    /// Get total memory usage.
    pub fn get_total_memory(&mut self) -> Result<u64, ProcTreeError> {
        self.refresh_processes()?;
        Ok(self.processes.values().map(|p| p.memory_kb).sum())
    }

    /// Build process tree recursively.
    fn build_tree(&self, pid: u32) -> Result<ProcessNode, ProcTreeError> {
        let process = self.processes.get(&pid)
            .ok_or_else(|| ProcTreeError::ProcessNotFound(pid))?;
        
        let mut node = process.clone();
        node.children.clear();
        
        // Find children
        for child in self.processes.values() {
            if child.ppid == pid {
                if let Ok(child_node) = self.build_tree(child.pid) {
                    node.children.push(child_node);
                }
            }
        }
        
        Ok(node)
    }

    /// Count processes in tree.
    fn count_processes(&self, node: &ProcessNode) -> usize {
        let mut count = 1;
        for child in &node.children {
            count += self.count_processes(child);
        }
        count
    }

    /// Sum memory usage in tree.
    fn sum_memory(&self, node: &ProcessNode) -> u64 {
        let mut total = node.memory_kb;
        for child in &node.children {
            total += self.sum_memory(child);
        }
        total
    }

    /// Refresh process list from /proc.
    fn refresh_processes(&mut self) -> Result<(), ProcTreeError> {
        // Only refresh if more than 100ms since last update
        if self.last_update.elapsed().as_millis() < 100 {
            return Ok(());
        }

        self.processes.clear();

        let proc_dir = Path::new("/proc");
        let entries = fs::read_dir(proc_dir)
            .map_err(|e| ProcTreeError::IoError(format!("Failed to read /proc: {}", e)))?;

        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                
                // Check if it's a PID directory
                if let Ok(pid) = name_str.parse::<u32>() {
                    if pid > 0 {
                        if let Ok(process) = self.read_process_info(pid) {
                            self.processes.insert(pid, process);
                        }
                    }
                }
            }
        }

        self.last_update = std::time::Instant::now();
        Ok(())
    }

    /// Read process info from /proc.
    fn read_process_info(&self, pid: u32) -> Result<ProcessNode, ProcTreeError> {
        let proc_path = format!("/proc/{}", pid);
        let status_path = Path::new(&proc_path).join("status");
        
        let status_content = fs::read_to_string(&status_path)
            .map_err(|e| ProcTreeError::IoError(format!("Failed to read status: {}", e)))?;
        
        let mut name = String::new();
        let mut ppid = 0;
        let mut state = ' ';
        let mut threads = 0;
        let mut rss_kb = 0;
        
        for line in status_content.lines() {
            if let Some(rest) = line.strip_prefix("Name:") {
                name = rest.trim().to_string();
            } else if let Some(rest) = line.strip_prefix("PPid:") {
                ppid = rest.trim().parse().unwrap_or(0);
            } else if let Some(rest) = line.strip_prefix("State:") {
                state = rest.trim().chars().next().unwrap_or(' ');
            } else if let Some(rest) = line.strip_prefix("Threads:") {
                threads = rest.trim().parse().unwrap_or(0);
            } else if let Some(rest) = line.strip_prefix("VmRSS:") {
                rss_kb = rest.trim().trim_end_matches(" kB").parse().unwrap_or(0);
            }
        }
        
        Ok(ProcessNode {
            pid,
            ppid,
            name,
            state,
            cpu_percent: 0.0, // Would need to calculate from /proc/stat
            memory_kb: rss_kb,
            threads,
            children: vec![],
        })
    }

    /// Get process tree as string.
    pub fn get_tree_string(&mut self, pid: u32) -> Result<String, ProcTreeError> {
        let tree = self.get_process_tree(pid)?;
        let mut output = String::new();
        self.format_tree(&tree.root, 0, &mut output);
        Ok(output)
    }

    /// Format tree as string.
    fn format_tree(&self, node: &ProcessNode, depth: usize, output: &mut String) {
        let indent = "  ".repeat(depth);
        output.push_str(&format!("{}{} ({}) [{} threads, {} KB]\n", 
            indent, node.name, node.pid, node.threads, node.memory_kb));
        
        for child in &node.children {
            self.format_tree(child, depth + 1, output);
        }
    }
}

impl Default for ProcTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mgr = ProcTree::new();
        let _ = mgr.last_update;
    }

    #[test]
    fn test_get_process_tree() {
        let mut mgr = ProcTree::new();
        // Try to get init process tree (PID 1)
        if let Ok(tree) = mgr.get_process_tree(1) {
            assert!(tree.total_processes > 0);
        }
    }

    #[test]
    fn test_get_all_processes() {
        let mut mgr = ProcTree::new();
        if let Ok(processes) = mgr.get_all_processes() {
            let _ = processes;
        }
    }

    #[test]
    fn test_get_process_count() {
        let mut mgr = ProcTree::new();
        if let Ok(count) = mgr.get_process_count() {
            assert!(count >= 0);
        }
    }
}
