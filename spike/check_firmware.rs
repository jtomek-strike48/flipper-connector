use flipper_rpc::{
    rpc::req::Request,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};

fn main() -> anyhow::Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;

    println!("📱 Checking Flipper Zero firmware info...\n");

    let mut cli = SerialRpcTransport::new(port)?;

    // Get device info
    println!("Requesting device information...\n");
    match cli.send_and_receive(Request::SystemDeviceInfo) {
        Ok(response) => {
            println!("✅ Device Info Response:");
            println!("{:#?}\n", response);
        }
        Err(e) => {
            println!("❌ Failed to get device info: {}", e);
        }
    }

    // Get protobuf version
    println!("Requesting protobuf version...\n");
    match cli.send_and_receive(Request::SystemProtobufVersion) {
        Ok(response) => {
            println!("✅ Protobuf Version:");
            println!("{:#?}\n", response);
        }
        Err(e) => {
            println!("❌ Failed to get protobuf version: {}", e);
        }
    }

    Ok(())
}
