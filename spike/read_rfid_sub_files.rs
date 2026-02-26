use anyhow::Result;
use flipper_protocol::FlipperClient;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Looking for .rfid and .sub files on Flipper Zero\n");

    let mut client = FlipperClient::new()?;
    println!("✅ Connected to Flipper Zero at: {}\n", client.port());

    // Check for RFID files
    let rfid_paths = vec!["/ext/lfrfid", "/ext/rfid", "/any/lfrfid"];

    println!("=== Searching for .rfid files ===");
    for path in rfid_paths {
        match client.list_directory(path, false).await {
            Ok(items) => {
                let rfid_files: Vec<_> = items.iter()
                    .filter(|i| format!("{:?}", i).contains(".rfid"))
                    .collect();

                if !rfid_files.is_empty() {
                    println!("\n✅ Found {} .rfid file(s) in {}:", rfid_files.len(), path);
                    for item in &rfid_files {
                        println!("  {:?}", item);
                    }

                    // Read first file as example
                    if let Some(first_file) = rfid_files.first() {
                        let debug_str = format!("{:?}", first_file);
                        if let Some(name_start) = debug_str.find("\"") {
                            if let Some(name_end) = debug_str[name_start+1..].find("\"") {
                                let filename = &debug_str[name_start+1..name_start+1+name_end];
                                let full_path = format!("{}/{}", path, filename);

                                println!("\n--- Example: {} ---", filename);
                                match client.read_file(&full_path).await {
                                    Ok(content) => {
                                        if let Ok(text) = String::from_utf8(content) {
                                            println!("{}", text);
                                        }
                                    }
                                    Err(e) => println!("Failed to read: {}", e),
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    // Check for Sub-GHz files
    let subghz_paths = vec!["/ext/subghz", "/ext/sub-ghz", "/any/subghz"];

    println!("\n=== Searching for .sub files ===");
    for path in subghz_paths {
        match client.list_directory(path, false).await {
            Ok(items) => {
                let sub_files: Vec<_> = items.iter()
                    .filter(|i| format!("{:?}", i).contains(".sub"))
                    .collect();

                if !sub_files.is_empty() {
                    println!("\n✅ Found {} .sub file(s) in {}:", sub_files.len(), path);
                    for item in &sub_files {
                        println!("  {:?}", item);
                    }

                    // Read first file as example
                    if let Some(first_file) = sub_files.first() {
                        let debug_str = format!("{:?}", first_file);
                        if let Some(name_start) = debug_str.find("\"") {
                            if let Some(name_end) = debug_str[name_start+1..].find("\"") {
                                let filename = &debug_str[name_start+1..name_start+1+name_end];
                                let full_path = format!("{}/{}", path, filename);

                                println!("\n--- Example: {} ---", filename);
                                match client.read_file(&full_path).await {
                                    Ok(content) => {
                                        if let Ok(text) = String::from_utf8(content) {
                                            println!("{}", text);
                                        }
                                    }
                                    Err(e) => println!("Failed to read: {}", e),
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    println!("\n✅ Search complete!");
    Ok(())
}
