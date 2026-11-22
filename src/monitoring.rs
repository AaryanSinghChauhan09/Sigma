//! Monitoring module inspired by Linux kernel metrics and Arch Linux monitoring tools.
//! Provides system metrics, performance monitoring, and health checks.
//! Applies fail-fast validation and clear abstraction per SigmaOS principles.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum MetricType {
    CpuUsage,
    MemoryUsage,
    DiskUsage,
    NetworkRx,
    NetworkTx,
    Temperature,
    LoadAverage,
}

impl MetricType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CpuUsage => "cpu_usage",
            Self::MemoryUsage => "memory_usage",
            Self::DiskUsage => "disk_usage",
            Self::NetworkRx => "network_rx",
            Self::NetworkTx => "network_tx",
            Self::Temperature => "temperature",
            Self::LoadAverage => "load_average",
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            Self::CpuUsage => "%",
            Self::MemoryUsage => "%",
            Self::DiskUsage => "%",
            Self::NetworkRx => "bytes/s",
            Self::NetworkTx => "bytes/s",
            Self::Temperature => "°C",
            Self::LoadAverage => "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

impl HealthStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Warning => "warning",
            Self::Critical => "critical",
            Self::Unknown => "unknown",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            Self::Healthy => "green",
            Self::Warning => "yellow",
            Self::Critical => "red",
            Self::Unknown => "gray",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonitoringError {
    InvalidMetricValue(String),
    InvalidThreshold(String),
}

impl fmt::Display for MonitoringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMetricValue(msg) => write!(f, "invalid metric value: {}", msg),
            Self::InvalidThreshold(msg) => write!(f, "invalid threshold: {}", msg),
        }
    }
}

impl std::error::Error for MonitoringError {}

#[derive(Debug, Clone)]
pub struct SystemMetric {
    pub metric_type: MetricType,
    pub value: f64,
    pub timestamp: u64,
    pub source: String,
}

impl SystemMetric {
    pub fn new(metric_type: MetricType, value: f64, source: impl Into<String>) -> Self {
        Self {
            metric_type,
            value,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            source: source.into(),
        }
    }

    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Threshold {
    pub warning: f64,
    pub critical: f64,
}

impl Threshold {
    pub fn new(warning: f64, critical: f64) -> Result<Self, MonitoringError> {
        if warning < 0.0 || critical < 0.0 {
            return Err(MonitoringError::InvalidThreshold("thresholds must be non-negative".to_string()));
        }
        if warning > critical {
            return Err(MonitoringError::InvalidThreshold("warning threshold must be <= critical".to_string()));
        }
        Ok(Self { warning, critical })
    }

    pub fn evaluate(&self, value: f64) -> HealthStatus {
        if value >= self.critical {
            HealthStatus::Critical
        } else if value >= self.warning {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonitoringManager {
    pub metrics: BTreeMap<String, Vec<SystemMetric>>,
    pub thresholds: BTreeMap<MetricType, Threshold>,
    pub alert_enabled: bool,
    pub retention_seconds: u64,
}

impl Default for MonitoringManager {
    fn default() -> Self {
        let mut thresholds = BTreeMap::new();
        thresholds.insert(MetricType::CpuUsage, Threshold::new(70.0, 90.0).unwrap());
        thresholds.insert(MetricType::MemoryUsage, Threshold::new(80.0, 95.0).unwrap());
        thresholds.insert(MetricType::DiskUsage, Threshold::new(80.0, 95.0).unwrap());
        thresholds.insert(MetricType::Temperature, Threshold::new(75.0, 85.0).unwrap());
        thresholds.insert(MetricType::LoadAverage, Threshold::new(2.0, 4.0).unwrap());

        Self {
            metrics: BTreeMap::new(),
            thresholds,
            alert_enabled: true,
            retention_seconds: 3600,
        }
    }
}

impl MonitoringManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_retention(mut self, seconds: u64) -> Self {
        self.retention_seconds = seconds;
        self
    }

    pub fn set_threshold(&mut self, metric_type: MetricType, threshold: Threshold) -> Result<(), MonitoringError> {
        if threshold.warning < 0.0 || threshold.critical < 0.0 {
            return Err(MonitoringError::InvalidThreshold("thresholds must be non-negative".to_string()));
        }
        self.thresholds.insert(metric_type, threshold);
        Ok(())
    }

    pub fn add_metric(&mut self, metric: SystemMetric) {
        let key = format!("{}:{}", metric.metric_type.as_str(), metric.source);
        self.metrics.entry(key).or_insert_with(Vec::new).push(metric);
        self.cleanup_old_metrics();
    }

    pub fn get_latest_metric(&self, metric_type: MetricType, source: &str) -> Option<&SystemMetric> {
        let key = format!("{}:{}", metric_type.as_str(), source);
        self.metrics.get(&key)?.last()
    }

    pub fn get_metrics_history(&self, metric_type: MetricType, source: &str) -> Vec<&SystemMetric> {
        let key = format!("{}:{}", metric_type.as_str(), source);
        self.metrics.get(&key).map(|v| v.iter().collect()).unwrap_or_default()
    }

    pub fn evaluate_health(&self, metric_type: MetricType, value: f64) -> HealthStatus {
        self.thresholds
            .get(&metric_type)
            .map(|t| t.evaluate(value))
            .unwrap_or(HealthStatus::Unknown)
    }

    pub fn get_system_health(&self) -> HealthStatus {
        let mut has_critical = false;
        let mut has_warning = false;

        for (_metric_type, metrics) in &self.metrics {
            if let Some(latest) = metrics.last() {
                let health = self.evaluate_health(latest.metric_type, latest.value);
                if health == HealthStatus::Critical {
                    has_critical = true;
                } else if health == HealthStatus::Warning {
                    has_warning = true;
                }
            }
        }

        if has_critical {
            HealthStatus::Critical
        } else if has_warning {
            HealthStatus::Warning
        } else if !self.metrics.is_empty() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unknown
        }
    }

    pub fn cleanup_old_metrics(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        for metrics in self.metrics.values_mut() {
            metrics.retain(|m| now - m.timestamp <= self.retention_seconds);
        }
    }

    pub fn generate_prometheus_config(&self) -> String {
        let mut config = String::new();
        
        config.push_str("# SigmaOS Prometheus Exporter Configuration\n");
        config.push_str("# Generated by SigmaOS MonitoringManager\n\n");
        
        config.push_str("global:\n");
        config.push_str("  scrape_interval: 15s\n");
        config.push_str("  evaluation_interval: 15s\n\n");
        
        config.push_str("scrape_configs:\n");
        config.push_str("  - job_name: 'sigmaos'\n");
        config.push_str("    static_configs:\n");
        config.push_str("      - targets: ['localhost:9090']\n");
        
        config
    }

    pub fn generate_grafana_dashboard(&self) -> String {
        String::from(
            r#"{
  "dashboard": {
    "title": "SigmaOS System Monitoring",
    "panels": [
      {
        "title": "CPU Usage",
        "type": "graph",
        "targets": [{"expr": "sigmaos_cpu_usage"}]
      },
      {
        "title": "Memory Usage",
        "type": "graph",
        "targets": [{"expr": "sigmaos_memory_usage"}]
      },
      {
        "title": "Disk Usage",
        "type": "graph",
        "targets": [{"expr": "sigmaos_disk_usage"}]
      }
    ]
  }
}
"#
        )
    }

    pub fn generate_health_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# SigmaOS Health Report\n");
        report.push_str(&format!("# Overall Status: {}\n", self.get_system_health().as_str()));
        report.push_str(&format!("# Generated: {}\n", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        ));
        report.push_str("\n");
        
        for (_key, metrics) in &self.metrics {
            if let Some(latest) = metrics.last() {
                let health = self.evaluate_health(latest.metric_type, latest.value);
                report.push_str(&format!(
                    "{}: {:.2}{} ({})\n",
                    latest.metric_type.as_str(),
                    latest.value,
                    latest.metric_type.unit(),
                    health.as_str()
                ));
            }
        }
        
        report
    }

    pub fn metric_count(&self) -> usize {
        self.metrics.values().map(|v| v.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_type_as_str() {
        assert_eq!(MetricType::CpuUsage.as_str(), "cpu_usage");
        assert_eq!(MetricType::MemoryUsage.as_str(), "memory_usage");
    }

    #[test]
    fn test_metric_type_unit() {
        assert_eq!(MetricType::CpuUsage.unit(), "%");
        assert_eq!(MetricType::Temperature.unit(), "°C");
    }

    #[test]
    fn test_health_status_as_str() {
        assert_eq!(HealthStatus::Healthy.as_str(), "healthy");
        assert_eq!(HealthStatus::Critical.as_str(), "critical");
    }

    #[test]
    fn test_threshold_new_valid() {
        let threshold = Threshold::new(70.0, 90.0);
        assert!(threshold.is_ok());
    }

    #[test]
    fn test_threshold_new_invalid() {
        let threshold = Threshold::new(90.0, 70.0);
        assert!(threshold.is_err());
    }

    #[test]
    fn test_threshold_evaluate() {
        let threshold = Threshold::new(70.0, 90.0).unwrap();
        assert_eq!(threshold.evaluate(50.0), HealthStatus::Healthy);
        assert_eq!(threshold.evaluate(80.0), HealthStatus::Warning);
        assert_eq!(threshold.evaluate(95.0), HealthStatus::Critical);
    }

    #[test]
    fn test_system_metric_new() {
        let metric = SystemMetric::new(MetricType::CpuUsage, 50.0, "cpu0");
        assert_eq!(metric.metric_type, MetricType::CpuUsage);
        assert_eq!(metric.value, 50.0);
    }

    #[test]
    fn test_monitoring_manager_new() {
        let mgr = MonitoringManager::new();
        assert!(mgr.thresholds.contains_key(&MetricType::CpuUsage));
    }

    #[test]
    fn test_monitoring_manager_add_metric() {
        let mut mgr = MonitoringManager::new();
        let metric = SystemMetric::new(MetricType::CpuUsage, 50.0, "cpu0");
        mgr.add_metric(metric);
        assert_eq!(mgr.metric_count(), 1);
    }

    #[test]
    fn test_monitoring_manager_set_threshold() {
        let mut mgr = MonitoringManager::new();
        let threshold = Threshold::new(60.0, 80.0).unwrap();
        mgr.set_threshold(MetricType::CpuUsage, threshold).unwrap();
        assert_eq!(mgr.thresholds[&MetricType::CpuUsage].warning, 60.0);
    }

    #[test]
    fn test_monitoring_manager_set_threshold_invalid() {
        let threshold = Threshold::new(90.0, 70.0);
        assert!(threshold.is_err());
    }

    #[test]
    fn test_evaluate_health() {
        let mgr = MonitoringManager::new();
        assert_eq!(mgr.evaluate_health(MetricType::CpuUsage, 50.0), HealthStatus::Healthy);
        assert_eq!(mgr.evaluate_health(MetricType::CpuUsage, 85.0), HealthStatus::Warning);
        assert_eq!(mgr.evaluate_health(MetricType::CpuUsage, 95.0), HealthStatus::Critical);
    }

    #[test]
    fn test_get_system_health() {
        let mut mgr = MonitoringManager::new();
        let metric = SystemMetric::new(MetricType::CpuUsage, 50.0, "cpu0");
        mgr.add_metric(metric);
        assert_eq!(mgr.get_system_health(), HealthStatus::Healthy);
    }

    #[test]
    fn test_generate_prometheus_config() {
        let mgr = MonitoringManager::new();
        let config = mgr.generate_prometheus_config();
        assert!(config.contains("scrape_configs"));
    }

    #[test]
    fn test_generate_health_report() {
        let mut mgr = MonitoringManager::new();
        let metric = SystemMetric::new(MetricType::CpuUsage, 50.0, "cpu0");
        mgr.add_metric(metric);
        let report = mgr.generate_health_report();
        assert!(report.contains("Health Report"));
    }
}
