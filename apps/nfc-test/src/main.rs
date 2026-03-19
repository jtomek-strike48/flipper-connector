// apps/nfc-test/src/main.rs
use flipper_tools::{NfcReadTool, NfcWriteTool, NfcCloneTool, NfcDetectTool, NfcDictAttackTool};
use flipper_core::tools::{PentestTool, ToolContext};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];
    let ctx = ToolContext::default();

    match command.as_str() {
        "read" => {
            if args.len() < 3 {
                println!("Usage: nfc-test read <path>");
                return;
            }
            let path = &args[2];

            println!("📖 Reading NFC file: {}", path);
            let tool = NfcReadTool;
            let params = json!({"path": path});

            match tool.execute(params, &ctx).await {
                Ok(result) => {
                    println!("✅ Success!");
                    println!("{}", serde_json::to_string_pretty(&result.data).unwrap());
                }
                Err(e) => println!("❌ Error: {}", e),
            }
        }

        "write" => {
            if args.len() < 5 {
                println!("Usage: nfc-test write <path> <device_type> <uid>");
                return;
            }
            let path = &args[2];
            let device_type = &args[3];
            let uid = &args[4];

            println!("✍️  Writing NFC file: {}", path);
            let tool = NfcWriteTool;
            let params = json!({
                "path": path,
                "device_type": device_type,
                "uid": uid,
                "atqa": "44 00",
                "sak": "08"
            });

            match tool.execute(params, &ctx).await {
                Ok(result) => println!("✅ File created: {}", path),
                Err(e) => println!("❌ Error: {}", e),
            }
        }

        "clone" => {
            if args.len() < 5 {
                println!("Usage: nfc-test clone <source> <dest> <new_uid>");
                return;
            }
            let source = &args[2];
            let dest = &args[3];
            let new_uid = &args[4];

            println!("📋 Cloning: {} -> {}", source, dest);
            let tool = NfcCloneTool;
            let params = json!({
                "source_path": source,
                "dest_path": dest,
                "new_uid": new_uid
            });

            match tool.execute(params, &ctx).await {
                Ok(result) => println!("✅ Clone created with UID: {}", new_uid),
                Err(e) => println!("❌ Error: {}", e),
            }
        }

        "detect" => {
            println!("🔍 Detecting card type...");
            let tool = NfcDetectTool;
            let params = json!({"timeout": 5});

            match tool.execute(params, &ctx).await {
                Ok(result) => {
                    println!("✅ Detection complete:");
                    println!("{}", serde_json::to_string_pretty(&result.data).unwrap());
                }
                Err(e) => println!("❌ Error: {}", e),
            }
        }

        "dict-attack" => {
            if args.len() < 3 {
                println!("Usage: nfc-test dict-attack <uid>");
                return;
            }
            let uid = &args[2];

            println!("🔐 Running dictionary attack on UID: {}", uid);
            let tool = NfcDictAttackTool;
            let params = json!({
                "card_uid": uid,
                "sectors": "0-15"
            });

            match tool.execute(params, &ctx).await {
                Ok(result) => {
                    println!("✅ Attack complete:");
                    println!("{}", serde_json::to_string_pretty(&result.data).unwrap());
                }
                Err(e) => println!("❌ Error: {}", e),
            }
        }

        _ => print_usage(),
    }
}

fn print_usage() {
    println!("🐬 Flipper Zero NFC Test Tool");
    println!();
    println!("Usage:");
    println!("  nfc-test read <path>");
    println!("  nfc-test write <path> <device_type> <uid>");
    println!("  nfc-test clone <source> <dest> <new_uid>");
    println!("  nfc-test detect");
    println!("  nfc-test dict-attack <uid>");
    println!();
    println!("Examples:");
    println!("  nfc-test read /ext/nfc/my_card.nfc");
    println!("  nfc-test write /ext/nfc/test.nfc \"Mifare Classic\" \"04 11 22 33\"");
    println!("  nfc-test clone /ext/nfc/card.nfc /ext/nfc/clone.nfc \"04 AA BB CC\"");
    println!("  nfc-test detect");
    println!("  nfc-test dict-attack \"04 A1 B2 C3\"");
}
