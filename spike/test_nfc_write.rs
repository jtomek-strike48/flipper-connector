use anyhow::Result;
use flipper_tools::NfcWriteTool;
use flipper_core::tools::{PentestTool, ToolContext};
use flipper_protocol::FlipperClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing NFC Write Tool\n");

    let tool = NfcWriteTool;
    let ctx = ToolContext::default();

    // Test: Create a simple UID card
    println!("========================================");
    println!("Test 1: Create UID-only card");
    println!("========================================");

    let params = json!({
        "path": "/ext/nfc/test_uid_card.nfc",
        "device_type": "UID",
        "uid": "04 12 34 56",
        "atqa": "44 00",
        "sak": "00"
    });

    match tool.execute(params, &ctx).await {
        Ok(result) => {
            if result.success {
                println!("✅ Success!");
                println!("Result: {}", serde_json::to_string_pretty(&result.data)?);

                // Read back the file to verify
                println!("\n--- Reading back file ---");
                let mut client = FlipperClient::new()?;
                let content = client.read_file("/ext/nfc/test_uid_card.nfc").await?;
                let text = String::from_utf8(content)?;
                println!("{}", text);
            } else {
                println!("❌ Failed: {:?}", result.error);
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }

    // Test: Create an NTAG203 card
    println!("\n========================================");
    println!("Test 2: Create NTAG203 card");
    println!("========================================");

    let params = json!({
        "path": "/ext/nfc/test_ntag.nfc",
        "device_type": "NTAG203",
        "uid": "04 AA BB CC DD EE FF",
        "atqa": "44 00",
        "sak": "00"
    });

    match tool.execute(params, &ctx).await {
        Ok(result) => {
            if result.success {
                println!("✅ Success!");
                println!("Result: {}", serde_json::to_string_pretty(&result.data)?);
            } else {
                println!("❌ Failed: {:?}", result.error);
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }

    println!("\n✅ All write tests complete!");
    Ok(())
}
