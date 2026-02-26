use anyhow::Result;
use flipper_protocol::FlipperClient;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Reading multiple .nfc file examples\n");

    let mut client = FlipperClient::new()?;
    println!("✅ Connected to Flipper Zero at: {}\n", client.port());

    let nfc_path = "/ext/nfc";

    // Get list of all .nfc files
    let items = client.list_directory(nfc_path, false).await?;
    let nfc_files: Vec<String> = items.iter()
        .filter_map(|i| {
            let debug_str = format!("{:?}", i);
            if debug_str.contains(".nfc") {
                if let Some(name_start) = debug_str.find("\"") {
                    if let Some(name_end) = debug_str[name_start+1..].find("\"") {
                        return Some(debug_str[name_start+1..name_start+1+name_end].to_string());
                    }
                }
            }
            None
        })
        .collect();

    println!("Found {} .nfc files. Reading first 5...\n", nfc_files.len());

    for (idx, filename) in nfc_files.iter().take(5).enumerate() {
        let full_path = format!("{}/{}", nfc_path, filename);

        println!("========================================");
        println!("File {}: {}", idx + 1, filename);
        println!("========================================");

        match client.read_file(&full_path).await {
            Ok(content) => {
                match String::from_utf8(content.clone()) {
                    Ok(text) => {
                        println!("{}", text);
                        println!();
                    }
                    Err(_) => {
                        println!("❌ Binary content, {} bytes\n", content.len());
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to read: {}\n", e);
            }
        }
    }

    println!("✅ Done reading examples!");
    Ok(())
}
