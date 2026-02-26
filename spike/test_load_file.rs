use anyhow::Result;
use flipper_rpc::{
    proto::app::{AppLoadFileRequest, AppExitRequest},
    rpc::req::Request,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;
    println!("📱 Testing AppLoadFile at: {}\n", port);

    let mut cli = SerialRpcTransport::new(port)?;

    let test_apps = vec![
        "/ext/apps/NFC/nfc.fap",
        "/ext/apps/NFC/picopass.fap",
        "/ext/apps/RFID/lfrfid.fap",
        "/ext/apps/Sub-GHz/subghz.fap",
    ];

    for app_path in test_apps {
        println!("Testing: {}", app_path);

        let req = Request::AppLoadFile(AppLoadFileRequest {
            path: app_path.to_string(),
        });

        match cli.send_and_receive(req) {
            Ok(_) => {
                println!("  ✅ Loaded successfully!");
                sleep(Duration::from_millis(1000));

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
