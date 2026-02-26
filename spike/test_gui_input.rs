use anyhow::Result;
use flipper_rpc::{
    proto::app::StartRequest,
    proto::gui::{SendInputEventRequest, InputKey, InputType},
    rpc::req::Request,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;
    println!("📱 Testing GuiSendInputEvent at: {}\n", port);

    let mut cli = SerialRpcTransport::new(port)?;

    println!("Step 1: Launch NFC app...");
    cli.send_and_receive(Request::AppStart(StartRequest {
        name: "NFC".to_string(),
        args: String::new(),
    }))?;
    println!("  ✅ NFC launched!");
    sleep(Duration::from_secs(1));

    println!("\nStep 2: Send GUI Back button (Short press)...");
    match cli.send_and_receive(Request::GuiSendInputEvent(SendInputEventRequest {
        key: InputKey::Back as i32,
        r#type: InputType::Short as i32,
    })) {
        Ok(_) => println!("  ✅ Back button sent!"),
        Err(e) => println!("  ❌ Failed: {}", e),
    }

    println!("\nStep 3: Wait 1 second...");
    sleep(Duration::from_secs(1));

    println!("\nStep 4: Check Flipper screen - did the app close?");
    println!("What do you see? Press Enter...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // If it didn't work, try press/release sequence
    println!("\nStep 5: Trying Press then Release...");
    println!("  Sending Back Press...");
    cli.send_and_receive(Request::GuiSendInputEvent(SendInputEventRequest {
        key: InputKey::Back as i32,
        r#type: InputType::Press as i32,
    }))?;

    sleep(Duration::from_millis(100));

    println!("  Sending Back Release...");
    cli.send_and_receive(Request::GuiSendInputEvent(SendInputEventRequest {
        key: InputKey::Back as i32,
        r#type: InputType::Release as i32,
    }))?;

    println!("\nStep 6: Wait and check again...");
    sleep(Duration::from_secs(1));
    println!("What do you see now? Press Enter...");
    input.clear();
    std::io::stdin().read_line(&mut input)?;

    println!("\n✅ Test complete!");
    Ok(())
}
