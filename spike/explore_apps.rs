use flipper_rpc::{
    fs::FsReadDir,
    transport::serial::{list_flipper_ports, rpc::SerialRpcTransport},
};

fn main() -> anyhow::Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;

    println!("📱 Exploring Flipper apps at: {}\n", port);

    let mut cli = SerialRpcTransport::new(port)?;

    let categories = vec!["NFC", "RFID", "Sub-GHz", "Infrared", "iButton", "GPIO", "USB", "Bluetooth"];

    for category in categories {
        let path = format!("/ext/apps/{}", category);
        println!("=== {} ===", category);
        match cli.fs_read_dir(&path, false) {
            Ok(items) => {
                let items_vec: Vec<_> = items.collect();
                if items_vec.is_empty() {
                    println!("  (empty)");
                } else {
                    for item in items_vec {
                        println!("  {:?}", item);
                    }
                }
            }
            Err(e) => println!("  Error: {}", e),
        }
        println!();
    }

    Ok(())
}
