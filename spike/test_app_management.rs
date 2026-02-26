use anyhow::Result;
use flipper_protocol::FlipperClient;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing App Management Tools\n");

    let mut client = FlipperClient::new()?;
    println!("✅ Connected to Flipper Zero at: {}\n", client.port());

    // Test 1: List all NFC apps
    println!("=== Test 1: List NFC Apps ===");
    match client.list_directory("/ext/apps/NFC", false).await {
        Ok(items) => {
            println!("✅ Found {} items in /ext/apps/NFC:", items.len());
            for item in &items {
                println!("   {:?}", item);
            }
        }
        Err(e) => {
            println!("❌ Failed: {}", e);
        }
    }

    // Test 2: Get info on a specific app
    println!("\n=== Test 2: Get App Info ===");
    let test_app = "/ext/apps/NFC/nfc.fap";
    println!("Getting info for: {}", test_app);

    match client.get_metadata(test_app).await {
        Ok(size) => {
            println!("✅ App info retrieved!");
            println!("   Path: {}", test_app);
            println!("   Size: {} bytes ({:.2} KB)", size, size as f64 / 1024.0);
        }
        Err(e) => {
            println!("❌ Failed: {}", e);
        }
    }

    // Test 3: List all app categories
    println!("\n=== Test 3: List App Categories ===");
    let categories = vec!["NFC", "RFID", "Sub-GHz", "Infrared", "iButton", "GPIO", "USB", "Bluetooth"];

    for cat in categories {
        let path = format!("/ext/apps/{}", cat);
        match client.list_directory(&path, false).await {
            Ok(items) => {
                let fap_count = items.iter().filter(|i| format!("{:?}", i).contains(".fap")).count();
                if fap_count > 0 {
                    println!("   {} - {} app(s)", cat, fap_count);
                }
            }
            Err(_) => {
                // Category doesn't exist, skip
            }
        }
    }

    println!("\n✅ All tests complete!");
    Ok(())
}
