use anyhow::Result;
use flipper_protocol::FlipperClient;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Looking for .nfc files on Flipper Zero\n");

    let mut client = FlipperClient::new()?;
    println!("✅ Connected to Flipper Zero at: {}\n", client.port());

    // Common NFC storage locations
    let nfc_paths = vec![
        "/ext/nfc",
        "/any/nfc",
        "/int/nfc",
    ];

    for path in nfc_paths {
        println!("=== Checking {} ===", path);
        match client.list_directory(path, false).await {
            Ok(items) => {
                let nfc_files: Vec<_> = items.iter()
                    .filter(|i| format!("{:?}", i).contains(".nfc"))
                    .collect();

                if nfc_files.is_empty() {
                    println!("No .nfc files found\n");
                } else {
                    println!("Found {} .nfc file(s):", nfc_files.len());
                    for item in &nfc_files {
                        println!("  {:?}", item);
                    }
                    println!();

                    // Try to read the first .nfc file we find
                    if let Some(first_file) = nfc_files.first() {
                        let debug_str = format!("{:?}", first_file);
                        if let Some(name_start) = debug_str.find("\"") {
                            if let Some(name_end) = debug_str[name_start+1..].find("\"") {
                                let filename = &debug_str[name_start+1..name_start+1+name_end];
                                let full_path = format!("{}/{}", path, filename);

                                println!("=== Reading {} ===", full_path);
                                match client.read_file(&full_path).await {
                                    Ok(content) => {
                                        match String::from_utf8(content.clone()) {
                                            Ok(text) => {
                                                println!("Content ({} bytes):\n", content.len());
                                                println!("{}", text);
                                                println!("\n✅ File read successfully!");
                                                return Ok(());
                                            }
                                            Err(_) => {
                                                println!("Binary content ({} bytes), first 100 bytes:", content.len());
                                                println!("{:?}", &content[..content.len().min(100)]);
                                            }
                                        }
                                    }
                                    Err(e) => println!("❌ Failed to read: {}", e),
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {
                println!("Path does not exist or is empty\n");
            }
        }
    }

    println!("\n⚠️  No .nfc files found on device. You may need to scan a tag first.");
    Ok(())
}
