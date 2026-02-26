use anyhow::Result;
use flipper_protocol::FlipperClient;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Searching for Sub-GHz files\n");

    let mut client = FlipperClient::new()?;
    println!("✅ Connected\n");

    let paths = vec!["/ext/subghz", "/ext/sub-ghz", "/any/subghz"];

    for path in paths {
        println!("Checking: {}", path);
        match client.list_directory(path, false).await {
            Ok(items) => {
                let sub_files: Vec<_> = items.iter()
                    .filter(|i| format!("{:?}", i).contains(".sub"))
                    .collect();

                if !sub_files.is_empty() {
                    println!("✅ Found {} .sub files:", sub_files.len());
                    for item in sub_files.iter().take(10) {
                        println!("  {:?}", item);
                    }
                    
                    // Read first file
                    if let Some(first) = sub_files.first() {
                        let debug_str = format!("{:?}", first);
                        if let Some(start) = debug_str.find("\"") {
                            if let Some(end) = debug_str[start+1..].find("\"") {
                                let name = &debug_str[start+1..start+1+end];
                                let full = format!("{}/{}", path, name);
                                
                                println!("\n--- Example: {} ---", name);
                                if let Ok(content) = client.read_file(&full).await {
                                    if let Ok(text) = String::from_utf8(content) {
                                        println!("{}", text);
                                    }
                                }
                            }
                        }
                    }
                    return Ok(());
                }
            }
            Err(_) => {}
        }
    }

    println!("⚠️  No .sub files found. Try capturing a remote signal first.");
    Ok(())
}
