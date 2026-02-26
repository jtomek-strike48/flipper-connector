//! Strike48 Connector SDK integration

use crate::audit::{AuditConfig, AuditContext, AuditLogger, JsonAuditLogger, NoOpAuditLogger};
use crate::tools::{ToolContext, ToolRegistry, ToolSchema};
use serde_json::Value;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;
use strike48_connector::{
    BaseConnector, ConnectorBehavior, ConnectorError, PayloadEncoding,
    Result as SdkResult, TaskTypeSchema,
};
use tokio::sync::RwLock;

/// Build `TaskTypeSchema` entries from the tool registry.
fn build_task_types(tools: &ToolRegistry) -> Vec<TaskTypeSchema> {
    tools
        .schemas()
        .iter()
        .map(|s| {
            let json_schema = s.to_json_schema();
            let input_schema = json_schema
                .get("parameters")
                .cloned()
                .unwrap_or(serde_json::json!({"type": "object", "properties": {}}));
            TaskTypeSchema {
                task_type_id: s.name.clone(),
                name: s.name.clone(),
                description: s.description.clone(),
                category: "flipper-zero".to_string(),
                icon: String::new(),
                input_schema_json: serde_json::to_string(&input_schema).unwrap_or_default(),
                output_schema_json: String::new(),
            }
        })
        .collect()
}

/// Build the connector metadata map.
fn build_metadata(tools: &ToolRegistry) -> HashMap<String, String> {
    let schemas: Vec<ToolSchema> = tools.schemas();
    let tool_names: Vec<String> = tools.names().iter().map(|s| s.to_string()).collect();
    let json_schemas: Vec<Value> = schemas.iter().map(|s| s.to_json_schema()).collect();

    let mut metadata = HashMap::new();
    metadata.insert(
        "tool_schemas".to_string(),
        serde_json::to_string(&json_schemas).unwrap_or_default(),
    );
    metadata.insert("tool_names".to_string(), tool_names.join(","));
    metadata.insert("tool_count".to_string(), tools.tools().len().to_string());
    metadata
}

/// Hello World connector implementation for the Strike48 Connector SDK.
pub struct FlipperConnector {
    tools: Arc<RwLock<ToolRegistry>>,
    metadata: HashMap<String, String>,
    task_types: Vec<TaskTypeSchema>,
    audit_logger: Arc<dyn AuditLogger>,
}

impl FlipperConnector {
    /// Create a new hello-world connector
    pub fn new(tools: ToolRegistry) -> Self {
        Self::with_audit_config(tools, None)
    }

    /// Create a new connector with audit configuration
    pub fn with_audit_config(tools: ToolRegistry, audit_config: Option<AuditConfig>) -> Self {
        let task_types = build_task_types(&tools);
        let metadata = build_metadata(&tools);

        let audit_logger: Arc<dyn AuditLogger> = if let Some(config) = audit_config {
            match JsonAuditLogger::new(config) {
                Ok(logger) => Arc::new(logger),
                Err(e) => {
                    tracing::error!("Failed to create audit logger: {}", e);
                    Arc::new(NoOpAuditLogger)
                }
            }
        } else {
            Arc::new(NoOpAuditLogger)
        };

        Self {
            tools: Arc::new(RwLock::new(tools)),
            metadata,
            task_types,
            audit_logger,
        }
    }

    /// Get the tool registry
    pub fn tools(&self) -> Arc<RwLock<ToolRegistry>> {
        self.tools.clone()
    }

    /// Get the audit logger
    pub fn audit_logger(&self) -> Arc<dyn AuditLogger> {
        self.audit_logger.clone()
    }
}

impl BaseConnector for FlipperConnector {
    fn connector_type(&self) -> &str {
        "flipper-zero"
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    fn execute(
        &self,
        request: Value,
        _capability_id: Option<&str>,
    ) -> Pin<Box<dyn std::future::Future<Output = SdkResult<Value>> + Send>> {
        let tools = self.tools.clone();
        let audit_logger = self.audit_logger.clone();

        Box::pin(async move {
            tracing::debug!("Raw execute request: {}", request);

            let start = Instant::now();

            // Parse the request
            let tool_name = request
                .get("tool")
                .and_then(|v| v.as_str())
                .ok_or_else(|| ConnectorError::InvalidConfig("Missing tool name".to_string()))?;

            // Backend sends params under "parameters" key
            let params = request
                .get("parameters")
                .cloned()
                .unwrap_or_else(|| request.clone());

            let name = tool_name.to_string();
            tracing::debug!(tool = %name, "Dispatching tool request");

            let ctx = ToolContext::default();
            let registry = tools.read().await;

            // Execute the tool
            let result = registry.execute(tool_name, params.clone(), &ctx).await;
            let duration_ms = start.elapsed().as_millis() as u64;

            // Create audit context
            let audit_context = AuditContext {
                user_id: None,
                session_id: None,
                source_ip: None,
                device_serial: None,
                connector_version: env!("CARGO_PKG_VERSION").to_string(),
            };

            // Log the execution
            match &result {
                Ok(tool_result) => {
                    let result_value = serde_json::to_value(tool_result).ok();

                    // Log successful execution
                    if let Err(e) = audit_logger.log_tool_execution(
                        tool_name,
                        Some(params),
                        tool_result.success,
                        duration_ms,
                        result_value.clone(),
                        tool_result.error.clone(),
                        Some(audit_context),
                    ) {
                        tracing::error!("Failed to log audit event: {}", e);
                    }

                    let result_value = serde_json::to_value(tool_result).unwrap_or(Value::Null);
                    Ok(result_value)
                }
                Err(e) => {
                    // Log failed execution
                    if let Err(log_err) = audit_logger.log_tool_execution(
                        tool_name,
                        Some(params),
                        false,
                        duration_ms,
                        None,
                        Some(e.to_string()),
                        Some(audit_context),
                    ) {
                        tracing::error!("Failed to log audit event: {}", log_err);
                    }

                    Ok(serde_json::json!({
                        "success": false,
                        "error": e.to_string()
                    }))
                }
            }
        })
    }

    fn behavior(&self) -> ConnectorBehavior {
        ConnectorBehavior::Tool
    }

    fn supported_encodings(&self) -> Vec<PayloadEncoding> {
        vec![PayloadEncoding::Json]
    }

    fn metadata(&self) -> HashMap<String, String> {
        self.metadata.clone()
    }

    fn capabilities(&self) -> Vec<TaskTypeSchema> {
        self.task_types.clone()
    }

    fn timeout_ms(&self) -> u64 {
        300_000 // 5 minutes
    }
}
