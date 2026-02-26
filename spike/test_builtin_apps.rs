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
    println!("📱 Testing built-in apps at: {}\n", port);

    // Test each app one at a time with fresh connections
    let apps = vec![
        "NFC",
        "RFID",
        "Sub-GHz",
        "Infrared",
        "iButton",
        "GPIO",
        "BadUSB",
    ];

    for app_name in apps {
        println!("=== Testing: {} ===", app_name);

        // Fresh connection for each app
        let mut cli = SerialRpcTransport::new(port)?;

        // Launch app
        println!("  Starting {}...", app_name);
        let start_req = Request::AppStart(StartRequest {
            name: app_name.to_string(),
            args: String::new(),
        });

        match cli.send_and_receive(start_req) {
            Ok(_) => {
                println!("  ✅ {} started successfully!", app_name);

                // Wait a bit
                sleep(Duration::from_millis(1000));

                // Try to exit
                println!("  Exiting {}...", app_name);
                match cli.send_and_receive(Request::AppExit(AppExitRequest {})) {
                    Ok(_) => println!("  ✅ Exit command sent"),
                    Err(e) => println!("  ⚠️  Exit failed: {}", e),
                }

                // Wait for exit to complete
                sleep(Duration::from_millis(500));
            }
            Err(e) => {
                println!("  ❌ Failed to start: {}", e);
            }
        }

        drop(cli);
        println!("  Please check Flipper screen and press Back if needed, then press Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        println!();
    }

    println!("✅ Test complete!");
    Ok(())
}
