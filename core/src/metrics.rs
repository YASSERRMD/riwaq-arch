//! Metrics and Monitoring Module
//!
//! Provides:
//! - Performance metrics collection
//! - Health check endpoints
//! - Usage analytics
//! - Timing utilities

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: HealthState,
    pub version: String,
    pub uptime_seconds: u64,
    pub checks: Vec<HealthCheck>,
    pub timestamp: String,
}

/// Health state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthState {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Individual health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: HealthState,
    pub message: Option<String>,
    pub latency_ms: Option<u64>,
}

/// Metrics collection
#[derive(Debug, Default)]
pub struct MetricsCollector {
    /// Request counters
    request_counts: RwLock<HashMap<String, AtomicU64>>,
    /// Error counters
    error_counts: RwLock<HashMap<String, AtomicU64>>,
    /// Timing histograms
    timings: RwLock<HashMap<String, Vec<Duration>>>,
    /// Start time for uptime calculation
    start_time: RwLock<Option<Instant>>,
    /// Active operations
    active_operations: AtomicU64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let collector = Self::default();
        *collector.start_time.write().unwrap() = Some(Instant::now());
        collector
    }

    /// Increment a counter
    pub fn increment(&self, metric: &str) {
        if let Ok(mut counts) = self.request_counts.write() {
            counts
                .entry(metric.to_string())
                .or_insert_with(|| AtomicU64::new(0))
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Increment error counter
    pub fn increment_error(&self, error_type: &str) {
        if let Ok(mut counts) = self.error_counts.write() {
            counts
                .entry(error_type.to_string())
                .or_insert_with(|| AtomicU64::new(0))
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Record a timing
    pub fn record_timing(&self, operation: &str, duration: Duration) {
        if let Ok(mut timings) = self.timings.write() {
            let bucket = timings.entry(operation.to_string()).or_default();
            bucket.push(duration);
            
            // Keep only last 1000 samples
            if bucket.len() > 1000 {
                bucket.drain(0..500);
            }
        }
    }

    /// Start tracking an operation
    pub fn start_operation(&self) -> OperationTracker {
        self.active_operations.fetch_add(1, Ordering::Relaxed);
        OperationTracker {
            start: Instant::now(),
            collector: self,
            operation: None,
        }
    }

    /// Get current metrics snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        let request_counts = self.request_counts.read()
            .map(|c| c.iter().map(|(k, v)| (k.clone(), v.load(Ordering::Relaxed))).collect())
            .unwrap_or_default();

        let error_counts = self.error_counts.read()
            .map(|c| c.iter().map(|(k, v)| (k.clone(), v.load(Ordering::Relaxed))).collect())
            .unwrap_or_default();

        let timing_stats = self.timings.read()
            .map(|t| t.iter().map(|(k, v)| (k.clone(), TimingStats::from_durations(v))).collect())
            .unwrap_or_default();

        let uptime = self.start_time.read()
            .ok()
            .and_then(|t| *t)
            .map(|t| t.elapsed())
            .unwrap_or_default();

        MetricsSnapshot {
            request_counts,
            error_counts,
            timing_stats,
            uptime_seconds: uptime.as_secs(),
            active_operations: self.active_operations.load(Ordering::Relaxed),
        }
    }

    /// Get uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.read()
            .ok()
            .and_then(|t| *t)
            .map(|t| t.elapsed())
            .unwrap_or_default()
    }

    /// Reset all metrics
    pub fn reset(&self) {
        if let Ok(mut counts) = self.request_counts.write() {
            counts.clear();
        }
        if let Ok(mut counts) = self.error_counts.write() {
            counts.clear();
        }
        if let Ok(mut timings) = self.timings.write() {
            timings.clear();
        }
    }
}

/// Operation tracker for timing
pub struct OperationTracker<'a> {
    start: Instant,
    collector: &'a MetricsCollector,
    operation: Option<String>,
}

impl<'a> OperationTracker<'a> {
    /// Set the operation name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.operation = Some(name.into());
        self
    }

    /// Complete the operation and record timing
    pub fn complete(self) {
        let duration = self.start.elapsed();
        if let Some(op) = &self.operation {
            self.collector.record_timing(op, duration);
        }
        self.collector.active_operations.fetch_sub(1, Ordering::Relaxed);
    }
}

impl<'a> Drop for OperationTracker<'a> {
    fn drop(&mut self) {
        self.collector.active_operations.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub request_counts: HashMap<String, u64>,
    pub error_counts: HashMap<String, u64>,
    pub timing_stats: HashMap<String, TimingStats>,
    pub uptime_seconds: u64,
    pub active_operations: u64,
}

/// Timing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingStats {
    pub count: usize,
    pub min_ms: f64,
    pub max_ms: f64,
    pub avg_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
}

impl TimingStats {
    pub fn from_durations(durations: &[Duration]) -> Self {
        if durations.is_empty() {
            return Self::default();
        }

        let mut ms: Vec<f64> = durations.iter().map(|d| d.as_secs_f64() * 1000.0).collect();
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let count = ms.len();
        let min = *ms.first().unwrap_or(&0.0);
        let max = *ms.last().unwrap_or(&0.0);
        let avg = ms.iter().sum::<f64>() / count as f64;
        let p50 = percentile(&ms, 50);
        let p95 = percentile(&ms, 95);
        let p99 = percentile(&ms, 99);

        Self {
            count,
            min_ms: min,
            max_ms: max,
            avg_ms: avg,
            p50_ms: p50,
            p95_ms: p95,
            p99_ms: p99,
        }
    }
}

impl Default for TimingStats {
    fn default() -> Self {
        Self {
            count: 0,
            min_ms: 0.0,
            max_ms: 0.0,
            avg_ms: 0.0,
            p50_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
        }
    }
}

fn percentile(sorted: &[f64], p: usize) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = (p * sorted.len() / 100).min(sorted.len() - 1);
    sorted[idx]
}

/// Health checker
pub struct HealthChecker {
    checks: Vec<Box<dyn Fn() -> HealthCheck + Send + Sync>>,
    start_time: Instant,
    version: String,
}

impl HealthChecker {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            checks: Vec::new(),
            start_time: Instant::now(),
            version: version.into(),
        }
    }

    /// Add a health check
    pub fn add_check<F>(&mut self, check: F)
    where
        F: Fn() -> HealthCheck + Send + Sync + 'static,
    {
        self.checks.push(Box::new(check));
    }

    /// Run all health checks
    pub fn check(&self) -> HealthStatus {
        let checks: Vec<HealthCheck> = self.checks.iter().map(|c| c()).collect();
        
        let status = if checks.iter().any(|c| c.status == HealthState::Unhealthy) {
            HealthState::Unhealthy
        } else if checks.iter().any(|c| c.status == HealthState::Degraded) {
            HealthState::Degraded
        } else {
            HealthState::Healthy
        };

        HealthStatus {
            status,
            version: self.version.clone(),
            uptime_seconds: self.start_time.elapsed().as_secs(),
            checks,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Quick liveness check
    pub fn is_alive(&self) -> bool {
        true // Basic liveness
    }

    /// Readiness check
    pub fn is_ready(&self) -> bool {
        self.check().status != HealthState::Unhealthy
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new(env!("CARGO_PKG_VERSION"))
    }
}

/// Create a basic memory health check
pub fn memory_check() -> HealthCheck {
    // Would check memory usage in real implementation
    HealthCheck {
        name: "memory".to_string(),
        status: HealthState::Healthy,
        message: None,
        latency_ms: Some(1),
    }
}

/// Create a disk health check for a path
pub fn disk_check(path: &std::path::Path) -> HealthCheck {
    let start = Instant::now();
    
    match std::fs::metadata(path) {
        Ok(_) => HealthCheck {
            name: "disk".to_string(),
            status: HealthState::Healthy,
            message: None,
            latency_ms: Some(start.elapsed().as_millis() as u64),
        },
        Err(e) => HealthCheck {
            name: "disk".to_string(),
            status: HealthState::Unhealthy,
            message: Some(e.to_string()),
            latency_ms: Some(start.elapsed().as_millis() as u64),
        },
    }
}

/// Timer guard for measuring operation duration
pub struct Timer {
    start: Instant,
    operation: String,
}

impl Timer {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            operation: operation.into(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        tracing::debug!(
            operation = %self.operation,
            duration_ms = %self.start.elapsed().as_millis(),
            "operation completed"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let collector = MetricsCollector::new();
        
        collector.increment("requests");
        collector.increment("requests");
        collector.increment_error("timeout");
        
        let snapshot = collector.snapshot();
        assert_eq!(*snapshot.request_counts.get("requests").unwrap_or(&0), 2);
        assert_eq!(*snapshot.error_counts.get("timeout").unwrap_or(&0), 1);
    }

    #[test]
    fn test_timing_stats() {
        let durations = vec![
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(30),
            Duration::from_millis(40),
            Duration::from_millis(50),
        ];
        
        let stats = TimingStats::from_durations(&durations);
        assert_eq!(stats.count, 5);
        assert_eq!(stats.min_ms, 10.0);
        assert_eq!(stats.max_ms, 50.0);
        assert_eq!(stats.avg_ms, 30.0);
    }

    #[test]
    fn test_health_checker() {
        let mut checker = HealthChecker::new("1.0.0");
        
        checker.add_check(|| HealthCheck {
            name: "test".to_string(),
            status: HealthState::Healthy,
            message: None,
            latency_ms: Some(1),
        });
        
        let status = checker.check();
        assert_eq!(status.status, HealthState::Healthy);
        assert_eq!(status.version, "1.0.0");
    }

    #[test]
    fn test_timer() {
        let timer = Timer::new("test_op");
        std::thread::sleep(Duration::from_millis(10));
        assert!(timer.elapsed_ms() >= 10);
    }
}
