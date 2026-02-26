//! File Operation Tools

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use flipper_protocol::flipper_rpc::rpc::res::ReadDirItem;
use serde_json::{json, Value};

// === File List Tool ===

pub struct FileListTool;

#[async_trait]
impl PentestTool for FileListTool {
    fn name(&self) -> &str {
        "flipper_file_list"
    }

    fn description(&self) -> &str {
        "List files and directories on the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Directory path to list (e.g., /ext, /ext/nfc)".to_string(),
                    required: true,
                    default: Some(json!("/ext")),
                },
                ToolParam {
                    name: "include_md5".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Include MD5 checksums for files".to_string(),
                    required: false,
                    default: Some(json!(false)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let path = params["path"].as_str().unwrap_or("/ext");
        let include_md5 = params["include_md5"].as_bool().unwrap_or(false);

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let items = client.list_directory(path, include_md5).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let items_json: Vec<Value> = items.iter().map(|item| {
            match item {
                ReadDirItem::Dir(name) => json!({
                    "name": name,
                    "type": "directory"
                }),
                ReadDirItem::File(name, size, md5) => json!({
                    "name": name,
                    "type": "file",
                    "size": size,
                    "md5": md5
                }),
            }
        }).collect();

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "items": items_json,
                "count": items.len()
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === File Read Tool ===

pub struct FileReadTool;

#[async_trait]
impl PentestTool for FileReadTool {
    fn name(&self) -> &str {
        "flipper_file_read"
    }

    fn description(&self) -> &str {
        "Read a file from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "File path to read (e.g., /ext/test.txt)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing path parameter".to_string()))?;

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let data = client.read_file(path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Try to convert to UTF-8 string, otherwise return base64
        let content = if let Ok(text) = String::from_utf8(data.clone()) {
            json!({
                "type": "text",
                "content": text
            })
        } else {
            json!({
                "type": "binary",
                "content": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data),
                "size": data.len()
            })
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "data": content
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === File Write Tool ===

pub struct FileWriteTool;

#[async_trait]
impl PentestTool for FileWriteTool {
    fn name(&self) -> &str {
        "flipper_file_write"
    }

    fn description(&self) -> &str {
        "Write a file to the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "File path to write (e.g., /ext/test.txt)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "content".to_string(),
                    param_type: ParamType::String,
                    description: "Content to write (text or base64)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "encoding".to_string(),
                    param_type: ParamType::String,
                    description: "Content encoding: 'text' or 'base64'".to_string(),
                    required: false,
                    default: Some(json!("text")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing path parameter".to_string()))?;

        let content_str = params["content"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing content parameter".to_string()))?;

        let encoding = params["encoding"].as_str().unwrap_or("text");

        // Convert content based on encoding
        let data = match encoding {
            "base64" => base64::Engine::decode(&base64::engine::general_purpose::STANDARD, content_str)
                .map_err(|e| flipper_core::error::Error::InvalidParams(format!("Invalid base64: {}", e)))?,
            _ => content_str.as_bytes().to_vec(),
        };

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        client.write_file(path, data.clone()).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "bytes_written": data.len(),
                "message": "File written successfully"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === File Delete Tool ===

pub struct FileDeleteTool;

#[async_trait]
impl PentestTool for FileDeleteTool {
    fn name(&self) -> &str {
        "flipper_file_delete"
    }

    fn description(&self) -> &str {
        "Delete a file or directory from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "File or directory path to delete".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "recursive".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Delete directories recursively".to_string(),
                    required: false,
                    default: Some(json!(false)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing path parameter".to_string()))?;

        let recursive = params["recursive"].as_bool().unwrap_or(false);

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        client.delete_path(path, recursive).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "message": "Deleted successfully"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
