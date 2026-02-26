use flipper_rpc::{
    fs::FsReadDir,
    transport::{Transport, serial::{list_flipper_ports, rpc::SerialRpcTransport}},
};

fn main() -> anyhow::Result<()> {
    let ports = list_flipper_ports()?;
    let port = &ports[0].port_name;
    
    println!("📱 Connected to: {}\n", port);
    
    let mut cli = SerialRpcTransport::new(port)?;
    
    // List what's in /ext/apps - this might give us app names
    println!("=== Apps directory ===");
    match cli.fs_read_dir("/ext/apps", false) {
        Ok(items) => {
            for item in items {
                println!("  {:?}", item);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    
    // Also check /int/apps
    println!("\n=== Internal apps ===");
    match cli.fs_read_dir("/int/apps", false) {
        Ok(items) => {
            for item in items {
                println!("  {:?}", item);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    
    Ok(())
}
