use anyhow::Result;
use flipper_rpc::{
    proto::app::{StartRequest, AppButtonPressRequest, AppButtonReleaseRequest},
    rpc::req::Request,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;
    println!("📱 Testing button simulation at: {}\n", port);

    let mut cli = SerialRpcTransport::new(port)?;

    println!("Step 1: The NFC app should still be running from the previous test.");
    println!("Check Flipper screen - is NFC app showing? Press Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // If NFC isn't running, launch it
    println!("\nStep 2: Ensuring NFC is running...");
    match cli.send_and_receive(Request::AppStart(StartRequest {
        name: "NFC".to_string(),
        args: String::new(),
    })) {
        Ok(_) => println!("  ✅ NFC launched"),
        Err(e) => println!("  ℹ️  Launch result: {}", e),
    }
    sleep(Duration::from_secs(1));

    println!("\nStep 3: Now trying to press the Back button (index 5)...");

    // Try pressing Back button
    match cli.send_and_receive(Request::AppButtonPress(AppButtonPressRequest {
        args: String::new(),
        index: 5, // Back button
    })) {
        Ok(_) => {
            println!("  ✅ Back button press sent!");
            sleep(Duration::from_millis(100));

            // Release the button
            match cli.send_and_receive(Request::AppButtonRelease(AppButtonReleaseRequest {})) {
                Ok(_) => println!("  ✅ Back button released!"),
                Err(e) => println!("  ❌ Release failed: {}", e),
            }
        }
        Err(e) => {
            println!("  ❌ Back button press failed: {}", e);
        }
    }

    println!("\nStep 4: Wait 1 second...");
    sleep(Duration::from_secs(1));

    println!("\nStep 5: Check Flipper screen - did the app close?");
    println!("What do you see? Press Enter when done checking...");
    input.clear();
    std::io::stdin().read_line(&mut input)?;

    println!("\n✅ Test complete!");
    Ok(())
}
