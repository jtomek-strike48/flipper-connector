//! Storage and SD Card Management Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use flipper_protocol::flipper_rpc::rpc::res::ReadDirItem;
use serde_json::{json, Value};

// === Storage Info Tool ===

pub struct StorageInfoTool;

#[async_trait]
impl PentestTool for StorageInfoTool {
    fn name(&self) -> &str {
        "flipper_storage_info"
    }

    fn description(&self) -> &str {
        "Get Flipper Zero storage and SD card information"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, _params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Check if SD card is accessible
        let sd_accessible = client.list_directory("/ext", false).await.is_ok();
        let int_accessible = client.list_directory("/int", false).await.is_ok();

        Ok(ToolResult {
            success: true,
            data: json!({
                "internal_storage": {
                    "path": "/int",
                    "accessible": int_accessible,
                    "description": "Internal flash storage"
                },
                "external_storage": {
                    "path": "/ext",
                    "accessible": sd_accessible,
                    "description": "External SD card"
                },
                "message": if sd_accessible {
                    "SD card detected and accessible"
                } else {
                    "SD card not detected or not accessible"
                },
                "storage_paths": {
                    "nfc": "/ext/nfc",
                    "rfid": "/ext/lfrfid",
                    "subghz": "/ext/subghz",
                    "infrared": "/ext/infrared",
                    "ibutton": "/ext/ibutton",
                    "badusb": "/ext/badusb",
                    "apps": "/ext/apps"
                }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Storage Format Tool ===

pub struct StorageFormatTool;

#[async_trait]
impl PentestTool for StorageFormatTool {
    fn name(&self) -> &str {
        "flipper_storage_format"
    }

    fn description(&self) -> &str {
        "Format Flipper Zero SD card (destructive operation)"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "confirm".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Confirm format operation (required)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let confirm = params["confirm"]
            .as_bool()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("confirm parameter required".to_string()))?;

        if !confirm {
            return Err(flipper_core::error::Error::InvalidParams(
                "Format operation requires explicit confirmation".to_string()
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "message": "SD card format requires manual operation",
                "instructions": {
                    "step_1": "Navigate to Settings → Storage on Flipper Zero",
                    "step_2": "Select 'Format SD Card'",
                    "step_3": "Confirm the operation",
                    "step_4": "Wait for format to complete"
                },
                "warnings": [
                    "⚠️  Formatting will DELETE ALL DATA on SD card",
                    "⚠️  This operation cannot be undone",
                    "⚠️  Backup important files before formatting",
                    "⚠️  Ensure SD card is properly inserted"
                ],
                "note": "RPC protocol does not support format command. Use device menu."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Storage Benchmark Tool ===

pub struct StorageBenchmarkTool;

#[async_trait]
impl PentestTool for StorageBenchmarkTool {
    fn name(&self) -> &str {
        "flipper_storage_benchmark"
    }

    fn description(&self) -> &str {
        "Benchmark Flipper Zero SD card read/write speed"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "test_size_kb".to_string(),
                    param_type: ParamType::Integer,
                    description: "Size of test file in KB (default 100)".to_string(),
                    required: false,
                    default: Some(json!(100)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let test_size_kb = params["test_size_kb"]
            .as_u64()
            .unwrap_or(100);

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Test write speed
        let test_data = vec![0xAA; (test_size_kb * 1024) as usize];
        let test_path = "/ext/.benchmark_test.bin";

        let write_start = std::time::Instant::now();
        client.write_file(test_path, test_data.clone()).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(format!("Write test failed: {}", e)))?;
        let write_duration = write_start.elapsed();

        // Test read speed
        let read_start = std::time::Instant::now();
        let read_data = client.read_file(test_path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(format!("Read test failed: {}", e)))?;
        let read_duration = read_start.elapsed();

        // Cleanup
        let _ = client.delete_path(test_path, false).await;

        let write_speed_kbps = (test_size_kb as f64) / write_duration.as_secs_f64();
        let read_speed_kbps = (read_data.len() as f64 / 1024.0) / read_duration.as_secs_f64();

        Ok(ToolResult {
            success: true,
            data: json!({
                "test_size_kb": test_size_kb,
                "write_speed_kbps": format!("{:.2}", write_speed_kbps),
                "read_speed_kbps": format!("{:.2}", read_speed_kbps),
                "write_duration_ms": write_duration.as_millis(),
                "read_duration_ms": read_duration.as_millis(),
                "message": "SD card benchmark completed",
                "performance_rating": if write_speed_kbps > 500.0 && read_speed_kbps > 1000.0 {
                    "Excellent (Class 10 or better)"
                } else if write_speed_kbps > 200.0 && read_speed_kbps > 500.0 {
                    "Good (Class 6-10)"
                } else if write_speed_kbps > 50.0 {
                    "Fair (Class 4 or lower)"
                } else {
                    "Poor (consider replacing SD card)"
                }
            }),
            error: None,
            duration_ms: (write_duration + read_duration).as_millis() as u64,
        })
    }
}

// === Backup Create Tool ===

pub struct BackupCreateTool;

#[async_trait]
impl PentestTool for BackupCreateTool {
    fn name(&self) -> &str {
        "flipper_backup_create"
    }

    fn description(&self) -> &str {
        "Create backup of Flipper Zero user data"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "backup_path".to_string(),
                    param_type: ParamType::String,
                    description: "Local path to save backup archive".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "include_apps".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Include installed apps in backup".to_string(),
                    required: false,
                    default: Some(json!(false)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let backup_path = params["backup_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("backup_path required".to_string()))?;

        let include_apps = params["include_apps"]
            .as_bool()
            .unwrap_or(false);

        Ok(ToolResult {
            success: true,
            data: json!({
                "backup_path": backup_path,
                "include_apps": include_apps,
                "message": "Backup creation prepared",
                "backup_includes": [
                    "NFC files (/ext/nfc)",
                    "RFID files (/ext/lfrfid)",
                    "Sub-GHz files (/ext/subghz)",
                    "Infrared files (/ext/infrared)",
                    "iButton files (/ext/ibutton)",
                    "BadUSB scripts (/ext/badusb)",
                    if include_apps { "Installed apps (/ext/apps)" } else { "" }
                ],
                "instructions": {
                    "recommended_tool": "qFlipper has built-in backup/restore",
                    "manual_method": "Copy /ext directory to local storage",
                    "command_line": "Use flipper-cli or custom script to download files"
                },
                "note": "Full backup implementation requires recursive directory download"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Archive Tool ===

pub struct ArchiveTool;

#[async_trait]
impl PentestTool for ArchiveTool {
    fn name(&self) -> &str {
        "flipper_archive_browse"
    }

    fn description(&self) -> &str {
        "Browse Flipper Zero archive (all saved files)"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "file_type".to_string(),
                    param_type: ParamType::String,
                    description: "Filter by file type: nfc, rfid, subghz, infrared, ibutton, badusb, all".to_string(),
                    required: false,
                    default: Some(json!("all")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let file_type = params["file_type"]
            .as_str()
            .unwrap_or("all");

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let paths = match file_type {
            "nfc" => vec!["/ext/nfc"],
            "rfid" => vec!["/ext/lfrfid"],
            "subghz" => vec!["/ext/subghz"],
            "infrared" => vec!["/ext/infrared"],
            "ibutton" => vec!["/ext/ibutton"],
            "badusb" => vec!["/ext/badusb"],
            "all" => vec!["/ext/nfc", "/ext/lfrfid", "/ext/subghz", "/ext/infrared", "/ext/ibutton", "/ext/badusb"],
            _ => return Err(flipper_core::error::Error::InvalidParams(
                "Invalid file_type. Must be: nfc, rfid, subghz, infrared, ibutton, badusb, all".to_string()
            )),
        };

        let mut archive_contents = serde_json::Map::new();

        for path in paths {
            match client.list_directory(path, false).await {
                Ok(files) => {
                    let file_list: Vec<Value> = files.iter()
                        .map(|item| {
                            match item {
                                ReadDirItem::Dir(name) => json!({
                                    "name": name,
                                    "type": "directory"
                                }),
                                ReadDirItem::File(name, size, _) => json!({
                                    "name": name,
                                    "type": "file",
                                    "size": size
                                }),
                            }
                        })
                        .collect();
                    archive_contents.insert(path.to_string(), json!({
                        "count": files.len(),
                        "items": file_list
                    }));
                },
                Err(_) => {
                    archive_contents.insert(path.to_string(), json!({"error": "Not accessible"}));
                }
            }
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "file_type": file_type,
                "archive_contents": archive_contents,
                "message": format!("Archive browsed for type: {}", file_type),
                "available_types": ["nfc", "rfid", "subghz", "infrared", "ibutton", "badusb", "all"]
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
