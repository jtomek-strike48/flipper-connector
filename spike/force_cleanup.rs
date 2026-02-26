use flipper_rpc::{
    proto::app::{AppExitRequest, StartRequest},
    proto::desktop::UnlockRequest,
    rpc::req::Request,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;
    println!("📱 Force cleanup at: {}\n", port);

    let mut cli = SerialRpcTransport::new(port)?;

    println!("Step 1: Desktop unlock...");
    for i in 0..3 {
        match cli.send_and_receive(Request::DesktopUnlock(UnlockRequest {})) {
            Ok(_) => {
                println!("  ✅ Unlocked (attempt {})", i+1);
                break;
            }
            Err(e) => println!("  Attempt {}: {}", i+1, e),
        }
        sleep(Duration::from_millis(200));
    }

    println!("\nStep 2: Exit app (multiple attempts)...");
    for i in 0..5 {
        match cli.send_and_receive(Request::AppExit(AppExitRequest {})) {
            Ok(_) => {
                println!("  ✅ App exited (attempt {})", i+1);
            }
            Err(e) => println!("  Attempt {}: {}", i+1, e),
        }
        sleep(Duration::from_millis(300));
    }

    println!("\nStep 3: Try starting NFC as a test...");
    match cli.send_and_receive(Request::AppStart(StartRequest {
        name: "NFC".to_string(),
        args: String::new(),
    })) {
        Ok(_) => {
            println!("  ✅ NFC started!");
            sleep(Duration::from_millis(500));
            cli.send_and_receive(Request::AppExit(AppExitRequest {}))?;
            println!("  ✅ NFC exited!");
        }
        Err(e) => println!("  ❌ Failed: {}", e),
    }

    println!("\n✅ Cleanup complete. Please manually check your Flipper Zero screen.");
    Ok(())
}
