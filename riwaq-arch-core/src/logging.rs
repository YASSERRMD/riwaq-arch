//! Logging configuration and utilities for Riwaq Arch.
//!
//! This module provides structured logging using the `tracing` crate,
//! with support for different output formats and verbosity levels.

use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// Initialize the logging system.
///
/// # Arguments
///
/// * `verbose` - If true, enables debug-level logging
///
/// # Example
///
/// ```rust
/// use riwaq_arch_core::logging::init_logging;
///
/// init_logging(true).expect("Failed to initialize logging");
/// ```
pub fn init_logging(verbose: bool) -> anyhow::Result<()> {
    let level = if verbose { Level::DEBUG } else { Level::INFO };

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level.to_string()));

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(verbose)
        .with_line_number(verbose)
        .with_span_events(if verbose {
            FmtSpan::ENTER | FmtSpan::EXIT
        } else {
            FmtSpan::NONE
        });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))?;

    Ok(())
}

/// Initialize logging with JSON output format.
///
/// This is useful for production environments where logs are
/// processed by log aggregation systems.
pub fn init_json_logging(verbose: bool) -> anyhow::Result<()> {
    let level = if verbose { Level::DEBUG } else { Level::INFO };

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level.to_string()));

    let json_layer = fmt::layer()
        .json()
        .with_target(true)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(json_layer)
        .try_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize JSON logger: {}", e))?;

    Ok(())
}

/// A guard type for timing operations.
///
/// When dropped, logs the duration of the operation.
pub struct TimingGuard {
    operation: String,
    start: std::time::Instant,
}

impl TimingGuard {
    /// Create a new timing guard for the given operation.
    pub fn new(operation: impl Into<String>) -> Self {
        let operation = operation.into();
        tracing::debug!(operation = %operation, "Starting operation");
        Self {
            operation,
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for TimingGuard {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        tracing::info!(
            operation = %self.operation,
            duration_ms = duration.as_millis(),
            "Operation completed"
        );
    }
}

/// Macro for creating a timing guard in the current scope.
#[macro_export]
macro_rules! timed {
    ($operation:expr) => {
        let _guard = $crate::logging::TimingGuard::new($operation);
    };
}

/// Progress reporter for long-running operations.
pub struct ProgressReporter {
    operation: String,
    total: usize,
    current: usize,
    report_interval: usize,
}

impl ProgressReporter {
    /// Create a new progress reporter.
    ///
    /// # Arguments
    ///
    /// * `operation` - Name of the operation
    /// * `total` - Total number of items to process
    pub fn new(operation: impl Into<String>, total: usize) -> Self {
        let operation = operation.into();
        tracing::info!(operation = %operation, total, "Starting");
        Self {
            operation,
            total,
            current: 0,
            report_interval: (total / 10).max(1),
        }
    }

    /// Increment progress by one.
    pub fn tick(&mut self) {
        self.current += 1;
        if self.current % self.report_interval == 0 || self.current == self.total {
            let percentage = (self.current as f64 / self.total as f64 * 100.0) as u32;
            tracing::info!(
                operation = %self.operation,
                current = self.current,
                total = self.total,
                percentage,
                "Progress"
            );
        }
    }

    /// Report completion.
    pub fn finish(self) {
        tracing::info!(
            operation = %self.operation,
            total = self.total,
            "Completed"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timing_guard_creation() {
        let guard = TimingGuard::new("test operation");
        assert_eq!(guard.operation, "test operation");
    }

    #[test]
    fn test_progress_reporter() {
        let mut reporter = ProgressReporter::new("test", 100);
        for _ in 0..50 {
            reporter.tick();
        }
        assert_eq!(reporter.current, 50);
    }
}
