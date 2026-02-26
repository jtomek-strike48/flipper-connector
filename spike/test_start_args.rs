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
    println!("📱 Testing AppStart with args at: {}\n", port);

    let mut cli = SerialRpcTransport::new(port)?;

    // Try different combinations of name and args
    let tests = vec![
        ("Loader", "/ext/apps/NFC/nfc.fap"),
        ("Loader", "/ext/apps/NFC/picopass.fap"),
        ("External", "/ext/apps/NFC/nfc.fap"),
        ("NFC", ""),  // This should work (built-in)
        ("RFID", ""), // Try RFID built-in
        ("Sub-GHz", ""), // Try Sub-GHz built-in
    ];

    for (name, args) in tests {
        println!("Testing: name='{}', args='{}'", name, args);

        let req = Request::AppStart(StartRequest {
            name: name.to_string(),
            args: args.to_string(),
        });

        match cli.send_and_receive(req) {
            Ok(_) => {
                println!("  ✅ Started successfully!");
                sleep(Duration::from_millis(800));

                // Exit
                match cli.send_and_receive(Request::AppExit(AppExitRequest {})) {
                    Ok(_) => println!("  ✅ Exited"),
                    Err(e) => println!("  ⚠️  Exit: {}", e),
                }
            }
            Err(e) => {
                println!("  ❌ Failed: {}", e);
            }
        }

        sleep(Duration::from_millis(500));
        println!();
    }

    Ok(())
}
