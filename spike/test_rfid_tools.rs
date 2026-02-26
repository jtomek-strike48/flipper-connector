use anyhow::Result;
use flipper_tools::{RfidReadTool, RfidWriteTool};
use flipper_core::tools::{PentestTool, ToolContext};
use flipper_protocol::FlipperClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing RFID Tools\n");

    let ctx = ToolContext::default();

    // Test 1: Read real RFID files
    println!("========================================");
    println!("Test 1: Read RFID Files");
    println!("========================================");

    let test_files = vec![
        "/ext/lfrfid/De_office_3.rfid",
        "/ext/lfrfid/De_office_4.rfid",
        "/ext/lfrfid/2.rfid",
    ];

    let read_tool = RfidReadTool;

    for path in test_files {
        println!("\n--- Reading: {} ---", path);
        let params = json!({"path": path});

        match read_tool.execute(params, &ctx).await {
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
    }

    // Test 2: Write RFID file with facility/card
    println!("\n========================================");
    println!("Test 2: Write H10301 from Facility/Card");
    println!("========================================");

    let write_tool = RfidWriteTool;

    let params = json!({
        "path": "/ext/lfrfid/test_badge.rfid",
        "key_type": "H10301",
        "facility_code": 42,
        "card_number": 12345
    });

    match write_tool.execute(params, &ctx).await {
        Ok(result) => {
            if result.success {
                println!("✅ Success!");
                println!("{}", serde_json::to_string_pretty(&result.data)?);

                // Read back
                println!("\n--- Reading back ---");
                let mut client = FlipperClient::new()?;
                let content = client.read_file("/ext/lfrfid/test_badge.rfid").await?;
                println!("{}", String::from_utf8(content)?);

                // Parse with read tool
                println!("--- Parsing with read tool ---");
                let params = json!({"path": "/ext/lfrfid/test_badge.rfid"});
                if let Ok(result) = read_tool.execute(params, &ctx).await {
                    println!("{}", serde_json::to_string_pretty(&result.data)?);
                }
            } else {
                println!("❌ Failed: {:?}", result.error);
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Test 3: Write with direct hex data
    println!("\n========================================");
    println!("Test 3: Write H10301 with Direct Hex");
    println!("========================================");

    let params = json!({
        "path": "/ext/lfrfid/test_hex_badge.rfid",
        "key_type": "H10301",
        "data": "1C 69 CE"
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

    println!("\n✅ All RFID tests complete!");
    Ok(())
}
