use anyhow::Result;
use flipper_tools::{SubGhzReadTool, SubGhzWriteTool, create_tool_registry};
use flipper_core::tools::{PentestTool, ToolContext};
use flipper_protocol::FlipperClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing Sub-GHz Tools\n");

    let ctx = ToolContext::default();

    // Test 1: Write Princeton protocol file
    println!("========================================");
    println!("Test 1: Write Princeton Remote");
    println!("========================================");

    let write_tool = SubGhzWriteTool;

    let params = json!({
        "path": "/ext/subghz/test_garage.sub",
        "frequency": 433920000,
        "protocol": "Princeton",
        "key": "00 00 00 00 00 12 34 56",
        "bit": 24,
        "te": 400
    });

    match write_tool.execute(params, &ctx).await {
        Ok(result) => {
            if result.success {
                println!("✅ Success!");
                println!("{}", serde_json::to_string_pretty(&result.data)?);

                // Read back
                println!("\n--- Reading back file ---");
                let mut client = FlipperClient::new()?;
                let content = client.read_file("/ext/subghz/test_garage.sub").await?;
                let text = String::from_utf8(content)?;
                println!("{}", text);

                // Parse with read tool
                println!("--- Parsing with read tool ---");
                let read_tool = SubGhzReadTool;
                let params = json!({"path": "/ext/subghz/test_garage.sub"});
                if let Ok(result) = read_tool.execute(params, &ctx).await {
                    println!("{}", serde_json::to_string_pretty(&result.data)?);
                }
            } else {
                println!("❌ Failed: {:?}", result.error);
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Test 2: Write GateTX protocol file
    println!("\n========================================");
    println!("Test 2: Write GateTX Remote");
    println!("========================================");

    let params = json!({
        "path": "/ext/subghz/test_gate.sub",
        "frequency": 315000000,
        "protocol": "GateTX",
        "key": "00 00 00 00 00 AB CD EF",
        "bit": 24
    });

    match write_tool.execute(params, &ctx).await {
        Ok(result) => {
            if result.success {
                println!("✅ Success!");
                println!("{}", serde_json::to_string_pretty(&result.data)?);
            } else {
                println!("❌ Failed: {:?}", result.error);
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Test 3: Verify tool registration
    println!("\n========================================");
    println!("Registry Check");
    println!("========================================");

    let registry = create_tool_registry();
    let names = registry.names();
    println!("Total tools: {}", names.len());

    if names.contains(&"flipper_subghz_read") {
        println!("✅ flipper_subghz_read registered");
    } else {
        println!("❌ flipper_subghz_read NOT registered");
    }

    if names.contains(&"flipper_subghz_write") {
        println!("✅ flipper_subghz_write registered");
    } else {
        println!("❌ flipper_subghz_write NOT registered");
    }

    println!("\n✅ All Sub-GHz tests complete!");
    Ok(())
}
