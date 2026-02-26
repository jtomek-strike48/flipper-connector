//! Week 1.5: Research Flipper Zero apps and button simulation
//!
//! This program tests:
//! 1. Launching various built-in apps
//! 2. Button simulation (press/release)
//! 3. App state checking
//! 4. App loading files

use anyhow::{Context, Result};
use flipper_rpc::{
    proto::app::{AppButtonPressRequest, AppButtonReleaseRequest, StartRequest, AppExitRequest, AppLoadFileRequest},
    proto::desktop::UnlockRequest,
    rpc::req::Request,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    println!("🔬 Week 1.5: App Control Research\n");

    // Find Flipper Zero
    let ports = list_flipper_ports().context("Failed to list ports")?;
    if ports.is_empty() {
        anyhow::bail!("No Flipper Zero found!");
    }

    let port = &ports[0].port_name;
    println!("📱 Found Flipper Zero at: {}\n", port);

    // First: Unlock desktop and exit any running app
    println!("=== Step 0: Cleanup ===\n");
    let mut cli = SerialRpcTransport::new(port)?;

    println!("  Unlocking desktop...");
    match cli.send_and_receive(Request::DesktopUnlock(UnlockRequest {})) {
        Ok(_) => println!("  ✅ Desktop unlocked"),
        Err(e) => println!("  ℹ️  Desktop unlock: {}", e),
    }
    sleep(Duration::from_millis(300));

    println!("  Exiting any running app...");
    match cli.send_and_receive(Request::AppExit(AppExitRequest {})) {
        Ok(_) => println!("  ✅ App exited"),
        Err(e) => println!("  ℹ️  No app to exit: {}", e),
    }
    drop(cli);
    sleep(Duration::from_millis(500));

    // Test 1: Launch various built-in apps
    println!("\n=== Test 1: App Launching ===\n");
    test_app_launches(port)?;

    // Test 2: Button simulation (with an app running)
    println!("\n=== Test 2: Button Simulation ===\n");
    test_button_simulation(port)?;

    // Test 3: App with file loading
    println!("\n=== Test 3: App File Loading ===\n");
    test_app_file_loading(port)?;

    println!("\n✅ Research complete!");
    Ok(())
}

fn test_app_launches(port: &str) -> Result<()> {
    // These are actual .fap app names from /ext/apps
    let app_names = vec![
        // NFC apps
        "nfc",
        "picopass",
        // RFID apps
        "lfrfid",
        // Sub-GHz apps
        "subghz",
        // Infrared
        "infrared",
        // iButton
        "ibutton",
        // GPIO
        "gpio",
        // USB apps
        "bad_usb",
        "hid_usb",
    ];

    for app_name in app_names {
        let mut cli = SerialRpcTransport::new(port)?;

        println!("  Testing app: '{}'", app_name);

        let req = Request::AppStart(StartRequest {
            name: app_name.to_string(),
            args: String::new(),
        });

        match cli.send_and_receive(req) {
            Ok(_) => {
                println!("    ✅ '{}' launched successfully!", app_name);

                // Exit the app using the same connection
                sleep(Duration::from_millis(300));
                match cli.send_and_receive(Request::AppExit(AppExitRequest {})) {
                    Ok(_) => println!("    ✅ App exited successfully"),
                    Err(e) => println!("    ⚠️  Exit failed: {}", e),
                }
            }
            Err(e) => {
                println!("    ❌ '{}' failed: {}", app_name, e);
            }
        }

        drop(cli);
        sleep(Duration::from_millis(500)); // Longer delay between tests
    }

    Ok(())
}

fn test_button_simulation(port: &str) -> Result<()> {
    // First, launch an app so buttons have something to interact with
    println!("  Launching NFC app for button testing...");
    let mut cli = SerialRpcTransport::new(port)?;
    cli.send_and_receive(Request::AppStart(StartRequest {
        name: "NFC".to_string(),
        args: String::new(),
    }))?;
    println!("  ✅ NFC app launched\n");
    sleep(Duration::from_millis(500));

    // Button indices based on common patterns:
    // Typically: 0=Up, 1=Down, 2=Left, 3=Right, 4=Ok, 5=Back
    // But this needs verification

    let button_names = vec![
        ("Up", 0),
        ("Down", 1),
        ("Left", 2),
        ("Right", 3),
        ("Ok/Center", 4),
        ("Back", 5),
    ];

    println!("  Testing button press/release...");
    println!("  Note: These indices are educated guesses and need verification\n");

    for (name, index) in button_names {
        println!("  Button '{}' (index {})", name, index);

        // Press
        let press_req = Request::AppButtonPress(AppButtonPressRequest {
            args: String::new(),
            index: index,
        });

        match cli.send_and_receive(press_req) {
            Ok(_) => {
                println!("    ✅ Press succeeded");

                sleep(Duration::from_millis(100));

                // Release
                let release_req = Request::AppButtonRelease(AppButtonReleaseRequest {});
                match cli.send_and_receive(release_req) {
                    Ok(_) => println!("    ✅ Release succeeded"),
                    Err(e) => println!("    ❌ Release failed: {}", e),
                }
            }
            Err(e) => {
                println!("    ❌ Press failed: {}", e);
            }
        }

        sleep(Duration::from_millis(200));
    }

    // Exit the app
    println!("\n  Exiting NFC app...");
    cli.send_and_receive(Request::AppExit(AppExitRequest {}))?;
    println!("  ✅ App exited");
    drop(cli);
    sleep(Duration::from_millis(500));

    Ok(())
}

fn test_app_file_loading(port: &str) -> Result<()> {
    println!("  Testing AppLoadFile request...");

    // First, let's try to load an NFC file (if one exists)
    let test_paths = vec![
        "/ext/nfc/test.nfc",
        "/any/subghz/test.sub",
        "/ext/rfid/test.rfid",
    ];

    for path in test_paths {
        let mut cli = SerialRpcTransport::new(port)?;

        println!("  Trying to load: {}", path);

        let req = Request::AppLoadFile(AppLoadFileRequest {
            path: path.to_string(),
        });

        match cli.send_and_receive(req) {
            Ok(_) => {
                println!("    ✅ File load request succeeded");

                // Exit after loading
                sleep(Duration::from_millis(500));
                let mut cli2 = SerialRpcTransport::new(port)?;
                let _ = cli2.send_and_receive(Request::AppExit(AppExitRequest {}));
            }
            Err(e) => {
                println!("    ℹ️  File load failed (expected if file doesn't exist): {}", e);
            }
        }

        drop(cli);
        sleep(Duration::from_millis(200));
    }

    Ok(())
}
