//! Audit Logging for Flipper Zero Connector
//!
//! Provides comprehensive audit logging for compliance, debugging, and security monitoring.
//! All tool executions, connection events, and errors are logged in structured JSON format.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use uuid::Uuid;

/// Audit log event representing a tool execution or system event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique event identifier
    pub event_id: String,

    /// Event timestamp in RFC3339 format
    pub timestamp: String,

    /// Event type (tool_execution, connection, error, etc.)
    pub event_type: AuditEventType,

    /// Tool name that was executed (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,

    /// Tool parameters (sanitized - sensitive data removed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,

    /// Execution result status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,

    /// Execution duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,

    /// Error message if execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Result data (sanitized)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,

    /// User or session context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<AuditContext>,

    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Audit event type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    /// Tool execution event
    ToolExecution,

    /// Connection established
    Connection,

    /// Connection closed
    Disconnection,

    /// Error occurred
    Error,

    /// Connector started
    ConnectorStart,

    /// Connector stopped
    ConnectorStop,
}

/// Context information for audit events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditContext {
    /// User identifier (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Session identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Source IP address (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,

    /// Flipper device serial number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_serial: Option<String>,

    /// Connector version
    pub connector_version: String,
}

/// Audit logger configuration
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,

    /// Output file path (None = stdout)
    pub output_path: Option<PathBuf>,

    /// Log successful operations
    pub log_success: bool,

    /// Log failed operations
    pub log_failures: bool,

    /// Sanitize sensitive data
    pub sanitize_data: bool,

    /// Include request parameters
    pub include_parameters: bool,

    /// Include result data
    pub include_results: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            output_path: None,
            log_success: true,
            log_failures: true,
            sanitize_data: true,
            include_parameters: true,
            include_results: true,
        }
    }
}

/// Audit logger trait
pub trait AuditLogger: Send + Sync {
    /// Log an audit event
    fn log_event(&self, event: AuditEvent) -> Result<(), String>;

    /// Log a tool execution
    fn log_tool_execution(
        &self,
        tool_name: &str,
        parameters: Option<Value>,
        success: bool,
        duration_ms: u64,
        result: Option<Value>,
        error: Option<String>,
        context: Option<AuditContext>,
    ) -> Result<(), String> {
        let event = AuditEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            event_type: AuditEventType::ToolExecution,
            tool_name: Some(tool_name.to_string()),
            parameters,
            success: Some(success),
            duration_ms: Some(duration_ms),
            error,
            result,
            context,
            metadata: None,
        };

        self.log_event(event)
    }

    /// Log a connection event
    fn log_connection(&self, connected: bool, context: Option<AuditContext>) -> Result<(), String> {
        let event = AuditEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            event_type: if connected {
                AuditEventType::Connection
            } else {
                AuditEventType::Disconnection
            },
            tool_name: None,
            parameters: None,
            success: Some(connected),
            duration_ms: None,
            error: None,
            result: None,
            context,
            metadata: None,
        };

        self.log_event(event)
    }

    /// Log an error event
    fn log_error(&self, error: &str, context: Option<AuditContext>) -> Result<(), String> {
        let event = AuditEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            event_type: AuditEventType::Error,
            tool_name: None,
            parameters: None,
            success: Some(false),
            duration_ms: None,
            error: Some(error.to_string()),
            result: None,
            context,
            metadata: None,
        };

        self.log_event(event)
    }
}

/// JSON audit logger implementation
pub struct JsonAuditLogger {
    config: AuditConfig,
    file_handle: Arc<Mutex<Option<std::fs::File>>>,
}

impl JsonAuditLogger {
    /// Create a new JSON audit logger
    pub fn new(config: AuditConfig) -> Result<Self, String> {
        let file_handle = if let Some(ref path) = config.output_path {
            // Create parent directories if they don't exist
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create audit log directory: {}", e))?;
            }

            // Open or create the audit log file
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| format!("Failed to open audit log file: {}", e))?;

            Some(file)
        } else {
            None
        };

        Ok(Self {
            config,
            file_handle: Arc::new(Mutex::new(file_handle)),
        })
    }

    /// Sanitize sensitive data from value
    fn sanitize_value(&self, value: &Value) -> Value {
        if !self.config.sanitize_data {
            return value.clone();
        }

        match value {
            Value::Object(map) => {
                let mut sanitized = serde_json::Map::new();
                for (key, val) in map {
                    let key_lower = key.to_lowercase();

                    // Redact sensitive fields
                    if key_lower.contains("password")
                        || key_lower.contains("secret")
                        || key_lower.contains("key")
                        || key_lower.contains("token")
                        || key_lower.contains("auth")
                    {
                        sanitized.insert(key.clone(), Value::String("[REDACTED]".to_string()));
                    } else {
                        sanitized.insert(key.clone(), self.sanitize_value(val));
                    }
                }
                Value::Object(sanitized)
            }
            Value::Array(arr) => {
                let sanitized: Vec<Value> = arr.iter().map(|v| self.sanitize_value(v)).collect();
                Value::Array(sanitized)
            }
            _ => value.clone(),
        }
    }

    /// Write event to output
    fn write_event(&self, event: &AuditEvent) -> Result<(), String> {
        // Sanitize event
        let mut sanitized_event = event.clone();
        if self.config.sanitize_data {
            if let Some(params) = &sanitized_event.parameters {
                sanitized_event.parameters = Some(self.sanitize_value(params));
            }
            if let Some(result) = &sanitized_event.result {
                sanitized_event.result = Some(self.sanitize_value(result));
            }
        }

        // Filter based on config
        if !self.config.include_parameters {
            sanitized_event.parameters = None;
        }
        if !self.config.include_results {
            sanitized_event.result = None;
        }

        // Serialize to JSON
        let json = serde_json::to_string(&sanitized_event)
            .map_err(|e| format!("Failed to serialize audit event: {}", e))?;

        // Write to output
        let mut handle = self.file_handle.lock()
            .map_err(|e| format!("Failed to lock file handle: {}", e))?;

        if let Some(ref mut file) = *handle {
            // Write to file
            writeln!(file, "{}", json)
                .map_err(|e| format!("Failed to write to audit log: {}", e))?;
            file.flush()
                .map_err(|e| format!("Failed to flush audit log: {}", e))?;
        } else {
            // Write to stdout
            println!("{}", json);
        }

        Ok(())
    }
}

impl AuditLogger for JsonAuditLogger {
    fn log_event(&self, event: AuditEvent) -> Result<(), String> {
        if !self.config.enabled {
            return Ok(());
        }

        // Filter based on success/failure
        if let Some(success) = event.success {
            if success && !self.config.log_success {
                return Ok(());
            }
            if !success && !self.config.log_failures {
                return Ok(());
            }
        }

        self.write_event(&event)
    }
}

/// No-op audit logger for testing
pub struct NoOpAuditLogger;

impl AuditLogger for NoOpAuditLogger {
    fn log_event(&self, _event: AuditEvent) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            event_type: AuditEventType::ToolExecution,
            tool_name: Some("test_tool".to_string()),
            parameters: Some(serde_json::json!({"param": "value"})),
            success: Some(true),
            duration_ms: Some(100),
            error: None,
            result: Some(serde_json::json!({"result": "success"})),
            context: None,
            metadata: None,
        };

        assert_eq!(event.tool_name, Some("test_tool".to_string()));
        assert_eq!(event.success, Some(true));
    }

    #[test]
    fn test_sanitize_sensitive_data() {
        let config = AuditConfig {
            sanitize_data: true,
            ..Default::default()
        };

        let logger = JsonAuditLogger::new(config).unwrap();

        let sensitive_data = serde_json::json!({
            "username": "test",
            "password": "secret123",
            "api_key": "key123"
        });

        let sanitized = logger.sanitize_value(&sensitive_data);

        assert_eq!(sanitized["username"], "test");
        assert_eq!(sanitized["password"], "[REDACTED]");
        assert_eq!(sanitized["api_key"], "[REDACTED]");
    }

    #[test]
    fn test_noop_logger() {
        let logger = NoOpAuditLogger;
        let result = logger.log_tool_execution(
            "test_tool",
            None,
            true,
            100,
            None,
            None,
            None,
        );

        assert!(result.is_ok());
    }
}
