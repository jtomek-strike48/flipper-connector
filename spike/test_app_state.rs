use anyhow::Result;
use flipper_rpc::{
    proto::app::{StartRequest, AppExitRequest},
    rpc::req::Request,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;
    println!("📱 Testing app state at: {}\n", port);

    let mut cli = SerialRpcTransport::new(port)?;

    println!("Step 1: Launch NFC app...");
    cli.send_and_receive(Request::AppStart(StartRequest {
        name: "NFC".to_string(),
        args: String::new(),
    }))?;
    println!("  ✅ Launch command sent");

    println!("\nStep 2: Wait 2 seconds...");
    sleep(Duration::from_secs(2));

    println!("\nStep 3: Check what's on Flipper screen, then press Enter...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    println!("\nStep 4: Try to exit...");
    match cli.send_and_receive(Request::AppExit(AppExitRequest {})) {
        Ok(_) => println!("  ✅ Exit succeeded"),
        Err(e) => println!("  ❌ Exit failed: {}", e),
    }

    println!("\nStep 5: Wait 1 second...");
    sleep(Duration::from_secs(1));

    println!("\nStep 6: Check Flipper screen again - is app still running?");
    println!("Press Enter when done checking...");
    input.clear();
    std::io::stdin().read_line(&mut input)?;

    println!("\n✅ Test complete!");
    Ok(())
}
