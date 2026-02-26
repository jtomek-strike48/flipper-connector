//! Hello World greeting tool

use async_trait::async_trait;
use hello_core::error::Result;
use hello_core::tools::{
    execute_timed, ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema,
};
use serde_json::{json, Value};

/// Hello World greeting tool
pub struct HelloWorldTool;

#[async_trait]
impl PentestTool for HelloWorldTool {
    fn name(&self) -> &str {
        "hello_world"
    }

    fn description(&self) -> &str {
        "Returns a friendly greeting, optionally personalized with a name"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema::new(self.name(), self.description())
            .param(ToolParam::optional(
                "name",
                ParamType::String,
                "Name to greet (defaults to \"World\")",
                json!("World"),
            ))
    }

    fn supported_platforms(&self) -> Vec<Platform> {
        vec![
            Platform::Desktop,
            Platform::Web,
            Platform::Android,
            Platform::Ios,
            Platform::Tui,
        ]
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> Result<ToolResult> {
        execute_timed(|| async {
            let name = params
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("World");

            let greeting = format!("Hello, {}!", name);

            Ok(json!({
                "greeting": greeting,
                "name": name,
            }))
        })
        .await
    }
}
