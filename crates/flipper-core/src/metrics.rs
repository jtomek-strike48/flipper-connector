//! Prometheus Metrics for Flipper Connector

use lazy_static::lazy_static;
use prometheus::{
    register_counter_vec, register_gauge, register_histogram_vec, CounterVec, Gauge, HistogramVec,
};

lazy_static! {
    /// Tool execution counter (success/failure)
    pub static ref TOOL_EXECUTIONS: CounterVec = register_counter_vec!(
        "flipper_tool_executions_total",
        "Total number of tool executions",
        &["tool_name", "status"]
    )
    .unwrap();

    /// Tool execution duration histogram
    pub static ref TOOL_DURATION: HistogramVec = register_histogram_vec!(
        "flipper_tool_duration_seconds",
        "Tool execution duration in seconds",
        &["tool_name"],
        vec![0.001, 0.01, 0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0]
    )
    .unwrap();

    /// Active tool executions gauge
    pub static ref ACTIVE_EXECUTIONS: Gauge = register_gauge!(
        "flipper_active_executions",
        "Number of currently executing tools"
    )
    .unwrap();

    /// Retry counter
    pub static ref RETRY_ATTEMPTS: CounterVec = register_counter_vec!(
        "flipper_retry_attempts_total",
        "Total number of retry attempts",
        &["tool_name", "attempt"]
    )
    .unwrap();

    /// Timeout counter
    pub static ref TIMEOUTS: CounterVec = register_counter_vec!(
        "flipper_timeouts_total",
        "Total number of tool execution timeouts",
        &["tool_name"]
    )
    .unwrap();

    /// Connection counter
    pub static ref CONNECTIONS: CounterVec = register_counter_vec!(
        "flipper_connections_total",
        "Total number of Flipper Zero connections",
        &["status"]
    )
    .unwrap();
}

/// Get all metrics as a Prometheus-formatted string
pub fn get_metrics() -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

/// Record a successful tool execution
pub fn record_tool_success(tool_name: &str, duration_ms: u64) {
    TOOL_EXECUTIONS
        .with_label_values(&[tool_name, "success"])
        .inc();
    TOOL_DURATION
        .with_label_values(&[tool_name])
        .observe(duration_ms as f64 / 1000.0);
}

/// Record a failed tool execution
pub fn record_tool_failure(tool_name: &str, duration_ms: u64) {
    TOOL_EXECUTIONS
        .with_label_values(&[tool_name, "failure"])
        .inc();
    TOOL_DURATION
        .with_label_values(&[tool_name])
        .observe(duration_ms as f64 / 1000.0);
}

/// Record a timeout
pub fn record_timeout(tool_name: &str) {
    TIMEOUTS.with_label_values(&[tool_name]).inc();
}

/// Record a retry attempt
pub fn record_retry(tool_name: &str, attempt: u32) {
    RETRY_ATTEMPTS
        .with_label_values(&[tool_name, &attempt.to_string()])
        .inc();
}

/// Increment active executions
pub fn execution_started() {
    ACTIVE_EXECUTIONS.inc();
}

/// Decrement active executions
pub fn execution_finished() {
    ACTIVE_EXECUTIONS.dec();
}

/// Record a connection attempt
pub fn record_connection(success: bool) {
    let status = if success { "success" } else { "failure" };
    CONNECTIONS.with_label_values(&[status]).inc();
}
