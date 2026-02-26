use anyhow::Result;
use flipper_tools::{create_tool_registry, NfcReadTool};
use flipper_core::tools::{PentestTool, ToolContext};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing NFC Read Tool\n");

    // Test with a real NFC file
    let test_files = vec![
        "/ext/nfc/Citi_costo.nfc",
        "/ext/nfc/Marriott_ga_01.nfc",
        "/ext/nfc/11074.nfc",
    ];

    let tool = NfcReadTool;
    let ctx = ToolContext::default();

    for (idx, path) in test_files.iter().enumerate() {
        println!("========================================");
        println!("Test {}: {}", idx + 1, path);
        println!("========================================");

        let params = json!({
            "path": path
        });

        match tool.execute(params, &ctx).await {
            Ok(result) => {
                if result.success {
                    println!("✅ Success!");
                    println!("\nParsed data:");
                    println!("{}", serde_json::to_string_pretty(&result.data)?);
                } else {
                    println!("❌ Failed: {:?}", result.error);
                }
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }

        println!();
    }

    // Verify tool is registered
    println!("========================================");
    println!("Registry Check");
    println!("========================================");
    let registry = create_tool_registry();
    let names = registry.names();
    println!("Total tools: {}", names.len());

    if names.contains(&"flipper_nfc_read") {
        println!("✅ flipper_nfc_read registered");
    } else {
        println!("❌ flipper_nfc_read NOT registered");
    }

    if names.contains(&"flipper_nfc_write") {
        println!("✅ flipper_nfc_write registered");
    } else {
        println!("❌ flipper_nfc_write NOT registered");
    }

    Ok(())
}
