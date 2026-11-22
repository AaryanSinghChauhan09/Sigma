//! SigmaOS printing module.
//!
//! This module provides abstractions for printer management,
//! print queue control, and printing configuration.
//!
//! # Example
//!
//! ```
//! use sigma_system::printing::*;
//!
//! let mut print_mgr = PrintManager::new();
//! print_mgr.add_printer("hp-laserjet", PrinterType::Network, "ipp://192.168.1.100", "hp-laserjet").unwrap();
//! print_mgr.print_file("hp-laserjet", "/path/to/file.pdf").unwrap();
//! ```

use std::collections::BTreeMap;

/// Errors that can occur in printing operations.
#[derive(Debug, Clone, PartialEq)]
pub enum PrintError {
    /// Printer not found.
    PrinterNotFound(String),
    /// Printer already exists.
    PrinterExists(String),
    /// Print job not found.
    JobNotFound(String),
    /// Invalid printer configuration.
    InvalidConfig(String),
    /// Printer offline.
    PrinterOffline(String),
    /// Queue full.
    QueueFull(String),
}

impl std::fmt::Display for PrintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintError::PrinterNotFound(name) => write!(f, "Printer '{}' not found", name),
            PrintError::PrinterExists(name) => write!(f, "Printer '{}' already exists", name),
            PrintError::JobNotFound(id) => write!(f, "Print job '{}' not found", id),
            PrintError::InvalidConfig(msg) => write!(f, "Invalid config: {}", msg),
            PrintError::PrinterOffline(name) => write!(f, "Printer '{}' is offline", name),
            PrintError::QueueFull(name) => write!(f, "Print queue for '{}' is full", name),
        }
    }
}

impl std::error::Error for PrintError {}

/// Printer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrinterType {
    /// Network printer.
    Network,
    /// USB printer.
    Usb,
    /// Bluetooth printer.
    Bluetooth,
    /// Serial printer.
    Serial,
    /// Virtual printer.
    Virtual,
}

impl PrinterType {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            PrinterType::Network => "network",
            PrinterType::Usb => "usb",
            PrinterType::Bluetooth => "bluetooth",
            PrinterType::Serial => "serial",
            PrinterType::Virtual => "virtual",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Result<Self, PrintError> {
        match s.to_lowercase().as_str() {
            "network" | "net" | "ip" => Ok(PrinterType::Network),
            "usb" => Ok(PrinterType::Usb),
            "bluetooth" | "bt" => Ok(PrinterType::Bluetooth),
            "serial" | "com" => Ok(PrinterType::Serial),
            "virtual" | "file" => Ok(PrinterType::Virtual),
            _ => Err(PrintError::InvalidConfig(format!("Unknown printer type: {}", s))),
        }
    }
}

/// Printer status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterStatus {
    /// Printer is idle.
    Idle,
    /// Printer is printing.
    Printing,
    /// Printer is offline.
    Offline,
    /// Printer is error.
    Error,
    /// Printer is paused.
    Paused,
}

impl PrinterStatus {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            PrinterStatus::Idle => "idle",
            PrinterStatus::Printing => "printing",
            PrinterStatus::Offline => "offline",
            PrinterStatus::Error => "error",
            PrinterStatus::Paused => "paused",
        }
    }
}

/// Printer information.
#[derive(Debug, Clone)]
pub struct Printer {
    /// Printer name.
    pub name: String,
    /// Printer type.
    pub printer_type: PrinterType,
    /// Connection URI.
    pub uri: String,
    /// Driver name.
    pub driver: String,
    /// Printer status.
    pub status: PrinterStatus,
    /// Whether printer is default.
    pub is_default: bool,
    /// Maximum queue size.
    pub max_queue_size: usize,
    /// Current queue size.
    pub queue_size: usize,
}

impl Printer {
    /// Create a new printer.
    pub fn new(name: &str, printer_type: PrinterType, uri: &str, driver: &str) -> Result<Self, PrintError> {
        if name.is_empty() {
            return Err(PrintError::InvalidConfig("Printer name cannot be empty".to_string()));
        }
        if uri.is_empty() {
            return Err(PrintError::InvalidConfig("Printer URI cannot be empty".to_string()));
        }
        if driver.is_empty() {
            return Err(PrintError::InvalidConfig("Printer driver cannot be empty".to_string()));
        }
        
        Ok(Self {
            name: name.to_string(),
            printer_type,
            uri: uri.to_string(),
            driver: driver.to_string(),
            status: PrinterStatus::Idle,
            is_default: false,
            max_queue_size: 100,
            queue_size: 0,
        })
    }

    /// Set printer status.
    pub fn set_status(&mut self, status: PrinterStatus) {
        self.status = status;
    }

    /// Add to queue.
    pub fn add_to_queue(&mut self) -> Result<(), PrintError> {
        if self.queue_size >= self.max_queue_size {
            return Err(PrintError::QueueFull(self.name.clone()));
        }
        self.queue_size += 1;
        Ok(())
    }

    /// Remove from queue.
    pub fn remove_from_queue(&mut self) -> Result<(), PrintError> {
        if self.queue_size == 0 {
            return Err(PrintError::InvalidConfig("Queue is empty".to_string()));
        }
        self.queue_size -= 1;
        Ok(())
    }
}

/// Print job.
#[derive(Debug, Clone)]
pub struct PrintJob {
    /// Job ID.
    pub id: String,
    /// Printer name.
    pub printer_name: String,
    /// File path.
    pub file_path: String,
    /// Job status.
    pub status: JobStatus,
    /// Priority.
    pub priority: u8,
    /// Pages.
    pub pages: u32,
    /// Created timestamp.
    pub created_at: i64,
}

/// Job status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    /// Job is pending.
    Pending,
    /// Job is printing.
    Printing,
    /// Job is completed.
    Completed,
    /// Job is cancelled.
    Cancelled,
    /// Job failed.
    Failed,
}

impl JobStatus {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Pending => "pending",
            JobStatus::Printing => "printing",
            JobStatus::Completed => "completed",
            JobStatus::Cancelled => "cancelled",
            JobStatus::Failed => "failed",
        }
    }
}

/// Print manager.
#[derive(Debug)]
pub struct PrintManager {
    /// Printers by name.
    printers: BTreeMap<String, Printer>,
    /// Print jobs by ID.
    jobs: BTreeMap<String, PrintJob>,
    /// Default printer.
    default_printer: Option<String>,
    /// Print server enabled.
    server_enabled: bool,
}

impl PrintManager {
    /// Create a new print manager.
    pub fn new() -> Self {
        Self {
            printers: BTreeMap::new(),
            jobs: BTreeMap::new(),
            default_printer: None,
            server_enabled: true,
        }
    }

    /// Add a printer.
    pub fn add_printer(&mut self, name: &str, printer_type: PrinterType, uri: &str, driver: &str) -> Result<Printer, PrintError> {
        if self.printers.contains_key(name) {
            return Err(PrintError::PrinterExists(name.to_string()));
        }
        let printer = Printer::new(name, printer_type, uri, driver)?;
        // Set as default if first printer
        if self.printers.is_empty() {
            self.default_printer = Some(name.to_string());
            let mut printer = printer;
            printer.is_default = true;
            self.printers.insert(name.to_string(), printer.clone());
            Ok(printer)
        } else {
            self.printers.insert(name.to_string(), printer.clone());
            Ok(printer)
        }
    }

    /// Remove a printer.
    pub fn remove_printer(&mut self, name: &str) -> Result<(), PrintError> {
        if !self.printers.contains_key(name) {
            return Err(PrintError::PrinterNotFound(name.to_string()));
        }
        self.printers.remove(name);
        // Update default printer if needed
        if self.default_printer.as_deref() == Some(name) {
            self.default_printer = self.printers.keys().next().cloned();
        }
        Ok(())
    }

    /// Get a printer.
    pub fn get_printer(&self, name: &str) -> Option<&Printer> {
        self.printers.get(name)
    }

    /// Get all printers.
    pub fn get_printers(&self) -> Vec<&Printer> {
        self.printers.values().collect()
    }

    /// Set default printer.
    pub fn set_default_printer(&mut self, name: &str) -> Result<(), PrintError> {
        if !self.printers.contains_key(name) {
            return Err(PrintError::PrinterNotFound(name.to_string()));
        }
        // Update old default
        if let Some(old_default) = &self.default_printer {
            if let Some(printer) = self.printers.get_mut(old_default) {
                printer.is_default = false;
            }
        }
        // Set new default
        if let Some(printer) = self.printers.get_mut(name) {
            printer.is_default = true;
        }
        self.default_printer = Some(name.to_string());
        Ok(())
    }

    /// Get default printer.
    pub fn get_default_printer(&self) -> Option<&Printer> {
        self.default_printer.as_ref().and_then(|name| self.printers.get(name))
    }

    /// Print a file.
    pub fn print_file(&mut self, printer_name: &str, file_path: &str) -> Result<PrintJob, PrintError> {
        if !self.printers.contains_key(printer_name) {
            return Err(PrintError::PrinterNotFound(printer_name.to_string()));
        }
        
        let printer = self.printers.get_mut(printer_name).unwrap();
        if printer.status == PrinterStatus::Offline {
            return Err(PrintError::PrinterOffline(printer_name.to_string()));
        }
        
        printer.add_to_queue()?;
        
        let job_id = format!("job-{}-{}", printer_name, self.jobs.len());
        let job = PrintJob {
            id: job_id.clone(),
            printer_name: printer_name.to_string(),
            file_path: file_path.to_string(),
            status: JobStatus::Pending,
            priority: 5,
            pages: 1,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };
        
        self.jobs.insert(job_id, job.clone());
        Ok(job)
    }

    /// Cancel a print job.
    pub fn cancel_job(&mut self, job_id: &str) -> Result<(), PrintError> {
        if let Some(job) = self.jobs.get_mut(job_id) {
            job.status = JobStatus::Cancelled;
            // Remove from printer queue
            if let Some(printer) = self.printers.get_mut(&job.printer_name) {
                let _ = printer.remove_from_queue();
            }
            Ok(())
        } else {
            Err(PrintError::JobNotFound(job_id.to_string()))
        }
    }

    /// Get a print job.
    pub fn get_job(&self, job_id: &str) -> Option<&PrintJob> {
        self.jobs.get(job_id)
    }

    /// Get all print jobs.
    pub fn get_jobs(&self) -> Vec<&PrintJob> {
        self.jobs.values().collect()
    }

    /// Get jobs for a printer.
    pub fn get_printer_jobs(&self, printer_name: &str) -> Vec<&PrintJob> {
        self.jobs.values()
            .filter(|j| j.printer_name == printer_name)
            .collect()
    }

    /// Enable/disable print server.
    pub fn set_server_enabled(&mut self, enabled: bool) {
        self.server_enabled = enabled;
    }

    /// Check if print server is enabled.
    pub fn is_server_enabled(&self) -> bool {
        self.server_enabled
    }

    /// Generate CUPS configuration.
    pub fn generate_cups_config(&self) -> String {
        let mut config = String::new();
        config.push_str("# SigmaOS CUPS Configuration\n\n");
        
        for printer in self.printers.values() {
            config.push_str(&format!("<Printer {}>\n", printer.name));
            config.push_str(&format!("  DeviceURI {}\n", printer.uri));
            config.push_str(&format!("  Driver {}\n", printer.driver));
            config.push_str(&format!("  Type {}\n", printer.printer_type.as_str()));
            config.push_str(&format!("  State {}\n", printer.status.as_str()));
            config.push_str(&format!("  Default {}\n", if printer.is_default { "yes" } else { "no" }));
            config.push_str("</Printer>\n\n");
        }
        
        config
    }

    /// Generate printer configuration.
    pub fn generate_printer_config(&self) -> String {
        let mut config = String::new();
        config.push_str("# SigmaOS Printer Configuration\n\n");
        
        for printer in self.printers.values() {
            config.push_str(&format!("[printer:{}]\n", printer.name));
            config.push_str(&format!("name = {}\n", printer.name));
            config.push_str(&format!("type = {}\n", printer.printer_type.as_str()));
            config.push_str(&format!("uri = {}\n", printer.uri));
            config.push_str(&format!("driver = {}\n", printer.driver));
            config.push_str(&format!("status = {}\n", printer.status.as_str()));
            config.push_str(&format!("default = {}\n", if printer.is_default { "true" } else { "false" }));
            config.push_str("\n");
        }
        
        config
    }

    /// Add default printers.
    pub fn add_default_printers(&mut self) {
        // Add common printers
        let _ = self.add_printer("pdf", PrinterType::Virtual, "cups-pdf:/", "cups-pdf");
        let _ = self.add_printer("file", PrinterType::Virtual, "file:/tmp/print", "raw");
    }
}

impl Default for PrintManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_printer() {
        let mut mgr = PrintManager::new();
        let printer = mgr.add_printer("hp-laserjet", PrinterType::Network, "ipp://192.168.1.100", "hp-laserjet").unwrap();
        assert_eq!(printer.name, "hp-laserjet");
        assert!(printer.is_default);
    }

    #[test]
    fn test_add_duplicate_printer() {
        let mut mgr = PrintManager::new();
        mgr.add_printer("hp-laserjet", PrinterType::Network, "ipp://192.168.1.100", "hp-laserjet").unwrap();
        let result = mgr.add_printer("hp-laserjet", PrinterType::Network, "ipp://192.168.1.100", "hp-laserjet");
        assert!(result.is_err());
    }

    #[test]
    fn test_set_default_printer() {
        let mut mgr = PrintManager::new();
        mgr.add_printer("printer1", PrinterType::Network, "ipp://192.168.1.100", "driver1").unwrap();
        mgr.add_printer("printer2", PrinterType::Network, "ipp://192.168.1.101", "driver2").unwrap();
        mgr.set_default_printer("printer2").unwrap();
        assert_eq!(mgr.get_default_printer().unwrap().name, "printer2");
    }

    #[test]
    fn test_print_file() {
        let mut mgr = PrintManager::new();
        mgr.add_printer("hp-laserjet", PrinterType::Network, "ipp://192.168.1.100", "hp-laserjet").unwrap();
        let job = mgr.print_file("hp-laserjet", "/path/to/file.pdf").unwrap();
        assert_eq!(job.printer_name, "hp-laserjet");
        assert_eq!(job.status, JobStatus::Pending);
    }

    #[test]
    fn test_cancel_job() {
        let mut mgr = PrintManager::new();
        mgr.add_printer("hp-laserjet", PrinterType::Network, "ipp://192.168.1.100", "hp-laserjet").unwrap();
        let job = mgr.print_file("hp-laserjet", "/path/to/file.pdf").unwrap();
        mgr.cancel_job(&job.id).unwrap();
        assert_eq!(mgr.get_job(&job.id).unwrap().status, JobStatus::Cancelled);
    }

    #[test]
    fn test_printer_offline() {
        let mut mgr = PrintManager::new();
        let mut printer = mgr.add_printer("hp-laserjet", PrinterType::Network, "ipp://192.168.1.100", "hp-laserjet").unwrap();
        printer.set_status(PrinterStatus::Offline);
        mgr.printers.insert("hp-laserjet".to_string(), printer);
        let result = mgr.print_file("hp-laserjet", "/path/to/file.pdf");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_cups_config() {
        let mut mgr = PrintManager::new();
        mgr.add_printer("hp-laserjet", PrinterType::Network, "ipp://192.168.1.100", "hp-laserjet").unwrap();
        let config = mgr.generate_cups_config();
        assert!(config.contains("hp-laserjet"));
        assert!(config.contains("ipp://192.168.1.100"));
    }

    #[test]
    fn test_printer_type_parse() {
        assert_eq!(PrinterType::parse("network").unwrap(), PrinterType::Network);
        assert_eq!(PrinterType::parse("usb").unwrap(), PrinterType::Usb);
        assert!(PrinterType::parse("invalid").is_err());
    }
}
