//! Logging module inspired by Linux syslog and audit logging.
//! Provides structured logging for all SigmaOS components.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Warning => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogError {
    InvalidComponent(String),
    InvalidMessage(String),
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidComponent(msg) => write!(f, "invalid component: {}", msg),
            Self::InvalidMessage(msg) => write!(f, "invalid message: {}", msg),
        }
    }
}

impl std::error::Error for LogError {}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub component: String,
    pub message: String,
    pub context: BTreeMap<String, String>,
}

impl LogEntry {
    pub fn new(level: LogLevel, component: impl Into<String>, message: impl Into<String>) -> Result<Self, LogError> {
        let component = component.into();
        if component.trim().is_empty() {
            return Err(LogError::InvalidComponent("component cannot be empty".to_string()));
        }
        let message = message.into();
        if message.trim().is_empty() {
            return Err(LogError::InvalidMessage("message cannot be empty".to_string()));
        }
        Ok(Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            level,
            component,
            message,
            context: BTreeMap::new(),
        })
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    pub fn format(&self) -> String {
        let mut formatted = format!(
            "[{}] {} {}: {}",
            self.timestamp,
            self.level.as_str(),
            self.component,
            self.message
        );
        
        if !self.context.is_empty() {
            formatted.push_str(" {");
            let mut first = true;
            for (key, value) in &self.context {
                if !first {
                    formatted.push_str(", ");
                }
                formatted.push_str(&format!("{}={}", key, value));
                first = false;
            }
            formatted.push('}');
        }
        
        formatted
    }
}

#[derive(Debug, Clone)]
pub struct Logger {
    pub component: String,
    pub min_level: LogLevel,
    pub entries: Vec<LogEntry>,
    pub max_entries: usize,
}

impl Logger {
    pub fn new(component: impl Into<String>) -> Result<Self, LogError> {
        let component = component.into();
        if component.trim().is_empty() {
            return Err(LogError::InvalidComponent("component cannot be empty".to_string()));
        }
        Ok(Self {
            component,
            min_level: LogLevel::Info,
            entries: vec![],
            max_entries: 1000,
        })
    }

    pub fn with_min_level(mut self, level: LogLevel) -> Self {
        self.min_level = level;
        self
    }

    pub fn with_max_entries(mut self, max: usize) -> Self {
        self.max_entries = max;
        self
    }

    pub fn log(&mut self, level: LogLevel, message: impl Into<String>) -> Result<(), LogError> {
        if !self.should_log(level) {
            return Ok(());
        }
        
        let entry = LogEntry::new(level, &self.component, message)?;
        self.entries.push(entry);
        
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
        
        Ok(())
    }

    pub fn error(&mut self, message: impl Into<String>) -> Result<(), LogError> {
        self.log(LogLevel::Error, message)
    }

    pub fn warn(&mut self, message: impl Into<String>) -> Result<(), LogError> {
        self.log(LogLevel::Warning, message)
    }

    pub fn info(&mut self, message: impl Into<String>) -> Result<(), LogError> {
        self.log(LogLevel::Info, message)
    }

    pub fn debug(&mut self, message: impl Into<String>) -> Result<(), LogError> {
        self.log(LogLevel::Debug, message)
    }

    pub fn trace(&mut self, message: impl Into<String>) -> Result<(), LogError> {
        self.log(LogLevel::Trace, message)
    }

    fn should_log(&self, level: LogLevel) -> bool {
        let level_order = [LogLevel::Trace, LogLevel::Debug, LogLevel::Info, LogLevel::Warning, LogLevel::Error];
        let level_idx = level_order.iter().position(|&l| l == level).unwrap_or(0);
        let min_idx = level_order.iter().position(|&l| l == self.min_level).unwrap_or(0);
        level_idx >= min_idx
    }

    pub fn get_entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn get_entries_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.entries.iter().filter(|e| e.level == level).collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn generate_syslog_config(&self) -> String {
        format!(
            "# SigmaOS Syslog Configuration\n\
             # Component: {}\n\
             # Min Level: {}\n\
             \n\
             [syslog]\n\
             ident={}\n\
             facility=local0\n\
             priority={}\n",
            self.component,
            self.min_level.as_str(),
            self.component.replace('-', "_"),
            self.min_level.as_str().to_lowercase()
        )
    }

    pub fn generate_audit_config(&self) -> String {
        format!(
            "# SigmaOS Audit Configuration\n\
             # Component: {}\n\
             \n\
             [audit]\n\
             enabled=true\n\
             log_all=true\n\
             component={}\n\
             max_entries={}\n",
            self.component,
            self.component,
            self.max_entries
        )
    }

    pub fn export_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n  \"component\": \"");
        json.push_str(&self.component);
        json.push_str("\",\n  \"entries\": [\n");
        
        for (i, entry) in self.entries.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"timestamp\": {},\n", entry.timestamp));
            json.push_str(&format!("      \"level\": \"{}\",\n", entry.level.as_str()));
            json.push_str(&format!("      \"component\": \"{}\",\n", entry.component));
            json.push_str(&format!("      \"message\": \"{}\"\n", entry.message));
            json.push_str("    }");
            if i < self.entries.len() - 1 {
                json.push_str(",");
            }
            json.push_str("\n");
        }
        
        json.push_str("  ]\n}\n");
        json
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
    }

    #[test]
    fn test_log_entry_new() {
        let entry = LogEntry::new(LogLevel::Info, "test", "test message");
        assert!(entry.is_ok());
        assert_eq!(entry.unwrap().component, "test");
    }

    #[test]
    fn test_log_entry_new_invalid_component() {
        let entry = LogEntry::new(LogLevel::Info, "", "test message");
        assert!(entry.is_err());
    }

    #[test]
    fn test_log_entry_new_invalid_message() {
        let entry = LogEntry::new(LogLevel::Info, "test", "");
        assert!(entry.is_err());
    }

    #[test]
    fn test_log_entry_with_context() {
        let entry = LogEntry::new(LogLevel::Info, "test", "test message")
            .unwrap()
            .with_context("key1", "value1");
        assert_eq!(entry.context.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_log_entry_format() {
        let entry = LogEntry::new(LogLevel::Info, "test", "test message")
            .unwrap()
            .with_context("key", "value");
        let formatted = entry.format();
        assert!(formatted.contains("INFO"));
        assert!(formatted.contains("test"));
        assert!(formatted.contains("key=value"));
    }

    #[test]
    fn test_logger_new() {
        let logger = Logger::new("test");
        assert!(logger.is_ok());
        assert_eq!(logger.unwrap().component, "test");
    }

    #[test]
    fn test_logger_new_invalid() {
        let logger = Logger::new("");
        assert!(logger.is_err());
    }

    #[test]
    fn test_logger_log() {
        let mut logger = Logger::new("test").unwrap();
        logger.info("test message").unwrap();
        assert_eq!(logger.entry_count(), 1);
    }

    #[test]
    fn test_logger_log_filtered() {
        let mut logger = Logger::new("test")
            .unwrap()
            .with_min_level(LogLevel::Warning);
        logger.info("test message").unwrap();
        assert_eq!(logger.entry_count(), 0);
        logger.error("error message").unwrap();
        assert_eq!(logger.entry_count(), 1);
    }

    #[test]
    fn test_logger_get_entries_by_level() {
        let mut logger = Logger::new("test").unwrap();
        logger.error("error1").unwrap();
        logger.info("info1").unwrap();
        logger.error("error2").unwrap();
        
        let errors = logger.get_entries_by_level(LogLevel::Error);
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_logger_max_entries() {
        let mut logger = Logger::new("test")
            .unwrap()
            .with_max_entries(5);
        for i in 0..10 {
            logger.info(format!("message {}", i)).unwrap();
        }
        assert_eq!(logger.entry_count(), 5);
    }

    #[test]
    fn test_generate_syslog_config() {
        let logger = Logger::new("test-component").unwrap();
        let config = logger.generate_syslog_config();
        assert!(config.contains("test-component"));
        assert!(config.contains("local0"));
    }

    #[test]
    fn test_generate_audit_config() {
        let logger = Logger::new("test").unwrap();
        let config = logger.generate_audit_config();
        assert!(config.contains("[audit]"));
        assert!(config.contains("enabled=true"));
    }

    #[test]
    fn test_export_json() {
        let mut logger = Logger::new("test").unwrap();
        logger.info("test message").unwrap();
        let json = logger.export_json();
        assert!(json.contains("\"component\": \"test\""));
        assert!(json.contains("\"entries\""));
    }
}
