use anyhow::Result;
use flipper_tools::{BatchReadTool, create_tool_registry};
use flipper_core::tools::{PentestTool, ToolContext};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing Batch Read Tool\n");

    let tool = BatchReadTool;
    let ctx = ToolContext::default();

    // Test 1: Read multiple NFC files
    println!("========================================");
    println!("Test 1: Batch Read NFC Files");
    println!("========================================");

    let params = json!({
        "paths": [
            "/ext/nfc/Citi_costo.nfc",
            "/ext/nfc/11074.nfc",
            "/ext/nfc/I1.nfc"
        ],
        "parse": true
    });

    match tool.execute(params, &ctx).await {
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

    // Test 2: Read multiple RFID files
    println!("\n========================================");
    println!("Test 2: Batch Read RFID Files");
    println!("========================================");

    let params = json!({
        "paths": [
            "/ext/lfrfid/De_office_3.rfid",
            "/ext/lfrfid/De_office_4.rfid",
            "/ext/lfrfid/2.rfid"
        ],
        "parse": true
    });

    match tool.execute(params, &ctx).await {
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

    // Test 3: Mixed file types with one error
    println!("\n========================================");
    println!("Test 3: Mixed Files (including error)");
    println!("========================================");

    let params = json!({
        "paths": [
            "/ext/nfc/Citi_costo.nfc",
            "/ext/lfrfid/2.rfid",
            "/ext/nonexistent.file",
            "/ext/subghz/test_garage.sub"
        ],
        "parse": true
    });

    match tool.execute(params, &ctx).await {
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

    // Test 4: Verify registration
    println!("\n========================================");
    println!("Registry Check");
    println!("========================================");

    let registry = create_tool_registry();
    let names = registry.names();
    println!("Total tools: {}", names.len());

    if names.contains(&"flipper_batch_read") {
        println!("✅ flipper_batch_read registered");
    } else {
        println!("❌ flipper_batch_read NOT registered");
    }

    println!("\n✅ All batch read tests complete!");
    Ok(())
}
