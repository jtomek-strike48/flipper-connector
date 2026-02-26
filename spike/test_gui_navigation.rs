use anyhow::Result;
use flipper_rpc::{
    proto::gui::{SendInputEventRequest, InputKey, InputType},
    rpc::req::Request,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;
    println!("📱 Testing GUI navigation at: /dev/ttyACM0\n");

    let mut cli = SerialRpcTransport::new(port)?;

    println!("NFC app should still be showing from previous test.");
    println!("Current menu shows: Read, Extract MF Keys, Saved, Extra Actions");
    println!("\nPress Enter to start test...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    println!("\nTest 1: Press Down to move selection...");
    cli.send_and_receive(Request::GuiSendInputEvent(SendInputEventRequest {
        key: InputKey::Down as i32,
        r#type: InputType::Short as i32,
    }))?;
    sleep(Duration::from_millis(500));
    println!("  Did the selection move? Press Enter...");
    input.clear();
    std::io::stdin().read_line(&mut input)?;

    println!("\nTest 2: Press Up to move back...");
    cli.send_and_receive(Request::GuiSendInputEvent(SendInputEventRequest {
        key: InputKey::Up as i32,
        r#type: InputType::Short as i32,
    }))?;
    sleep(Duration::from_millis(500));
    println!("  Did selection move back? Press Enter...");
    input.clear();
    std::io::stdin().read_line(&mut input)?;

    println!("\nTest 3: Try multiple Back presses (to exit nested menu)...");
    for i in 1..=5 {
        println!("  Back press {}...", i);
        cli.send_and_receive(Request::GuiSendInputEvent(SendInputEventRequest {
            key: InputKey::Back as i32,
            r#type: InputType::Short as i32,
        }))?;
        sleep(Duration::from_millis(300));
    }

    sleep(Duration::from_secs(1));
    println!("\nDid the app exit? What do you see? Press Enter...");
    input.clear();
    std::io::stdin().read_line(&mut input)?;

    println!("\n✅ Test complete!");
    Ok(())
}
