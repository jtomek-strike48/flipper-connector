use anyhow::Result;
use flipper_protocol::FlipperClient;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing Week 2 Tools\n");

    let mut client = FlipperClient::new()?;
    println!("✅ Connected to Flipper Zero at: {}\n", client.port());

    // Test 1: Create a directory
    println!("=== Test 1: Create Directory ===");
    let test_dir = "/ext/week2_test";
    println!("Creating directory: {}", test_dir);

    match client.create_directory(test_dir).await {
        Ok(created) => {
            if created {
                println!("✅ Directory created successfully!");
            } else {
                println!("ℹ️  Directory already exists");
            }
        }
        Err(e) => {
            println!("❌ Failed to create directory: {}", e);
        }
    }

    // Test 2: Get metadata on the directory
    println!("\n=== Test 2: Get Directory Metadata ===");
    println!("Getting metadata for: {}", test_dir);

    match client.get_metadata(test_dir).await {
        Ok(size) => {
            println!("✅ Metadata retrieved!");
            println!("   Size: {} bytes", size);
            println!("   Type: {}", if size == 0 { "directory or empty file" } else { "file" });
        }
        Err(e) => {
            println!("❌ Failed to get metadata: {}", e);
        }
    }

    // Test 3: Create a file in the directory
    println!("\n=== Test 3: Create File in Directory ===");
    let test_file = format!("{}/test.txt", test_dir);
    let test_content = b"Hello from Week 2 testing!";
    println!("Creating file: {}", test_file);

    match client.write_file(&test_file, test_content.to_vec()).await {
        Ok(_) => println!("✅ File created successfully!"),
        Err(e) => println!("❌ Failed to create file: {}", e),
    }

    // Test 4: Get metadata on the file
    println!("\n=== Test 4: Get File Metadata ===");
    println!("Getting metadata for: {}", test_file);

    match client.get_metadata(&test_file).await {
        Ok(size) => {
            println!("✅ Metadata retrieved!");
            println!("   Size: {} bytes", size);
            println!("   Expected: {} bytes", test_content.len());
            println!("   Type: {}", if size == 0 { "directory or empty file" } else { "file" });
        }
        Err(e) => {
            println!("❌ Failed to get metadata: {}", e);
        }
    }

    // Test 5: List directory contents
    println!("\n=== Test 5: List Directory Contents ===");
    println!("Listing contents of: {}", test_dir);

    match client.list_directory(test_dir, false).await {
        Ok(items) => {
            println!("✅ Directory listing successful!");
            println!("   Found {} item(s):", items.len());
            for item in items {
                println!("     {:?}", item);
            }
        }
        Err(e) => {
            println!("❌ Failed to list directory: {}", e);
        }
    }

    // Test 6: Cleanup - delete the test file and directory
    println!("\n=== Test 6: Cleanup ===");
    println!("Deleting test file: {}", test_file);
    match client.delete_path(&test_file, false).await {
        Ok(_) => println!("✅ File deleted"),
        Err(e) => println!("⚠️  Failed to delete file: {}", e),
    }

    println!("Deleting test directory: {}", test_dir);
    match client.delete_path(test_dir, false).await {
        Ok(_) => println!("✅ Directory deleted"),
        Err(e) => println!("⚠️  Failed to delete directory: {}", e),
    }

    println!("\n✅ All tests complete!");
    Ok(())
}
