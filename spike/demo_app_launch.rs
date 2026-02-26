use anyhow::Result;
use flipper_rpc::{
    proto::app::StartRequest,
    rpc::req::Request,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;

    println!("🎬 Flipper Zero App Launch Demo");
    println!("================================\n");
    println!("This demo shows:");
    println!("  ✅ Apps launching successfully via RPC");
    println!("  ❌ But we cannot exit them programmatically");
    println!("  ⚠️  You'll need to manually press Back after each app\n");
    println!("Connected to: {}\n", port);
    println!("Press Enter to start...");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let apps = vec![
        ("NFC", "NFC card reader and emulator"),
        ("Sub-GHz", "Sub-GHz transceiver (315/433/868 MHz)"),
        ("Infrared", "IR remote control"),
        ("iButton", "Dallas iButton reader"),
        ("GPIO", "General purpose input/output"),
    ];

    for (i, (app_name, description)) in apps.iter().enumerate() {
        println!("\n╔═══════════════════════════════════════════════════════╗");
        println!("║ App {}/{}                                              ║", i + 1, apps.len());
        println!("╚═══════════════════════════════════════════════════════╝");
        println!("\n📱 App: {}", app_name);
        println!("📝 Description: {}", description);
        println!("\n🚀 Launching...");

        let mut cli = SerialRpcTransport::new(port)?;

        match cli.send_and_receive(Request::AppStart(StartRequest {
            name: app_name.to_string(),
            args: String::new(),
        })) {
            Ok(_) => {
                println!("✅ {} launched successfully!", app_name);
                sleep(Duration::from_secs(1));

                println!("\n👀 CHECK YOUR FLIPPER SCREEN NOW!");
                println!("   You should see the {} app running.", app_name);
                println!("\n⚠️  NOTE: We CANNOT exit this app via RPC!");
                println!("   This is the limitation we discovered.\n");
                println!("📋 To continue:");
                println!("   1. Look at your Flipper screen");
                println!("   2. Manually press the BACK button (left side)");
                println!("   3. Press Enter here to launch the next app\n");
                println!("Press Enter when ready...");

                input.clear();
                std::io::stdin().read_line(&mut input)?;
            }
            Err(e) => {
                println!("❌ Failed to launch {}: {}", app_name, e);
                println!("Skipping to next app...\n");
                sleep(Duration::from_millis(500));
            }
        }

        drop(cli);
        sleep(Duration::from_millis(500));
    }

    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║ Demo Complete!                                        ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");
    println!("✅ What we proved:");
    println!("   • Apps CAN be launched via RPC");
    println!("   • Apps display correctly on Flipper screen");
    println!("   • Multiple different apps work (NFC, Sub-GHz, etc.)\n");
    println!("❌ What doesn't work:");
    println!("   • Cannot exit apps programmatically");
    println!("   • Cannot send button commands to control apps");
    println!("   • Must manually press Back to exit\n");
    println!("🔄 Conclusion:");
    println!("   • App launching works perfectly");
    println!("   • But apps are display-only during RPC");
    println!("   • This is why we pivot to file-based workflows\n");
    println!("✨ Demo complete! Press Enter to exit...");

    input.clear();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}
