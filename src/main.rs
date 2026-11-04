//! SigmaOS-apm-server - SigmaOS Component

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting SigmaOS-apm-server");
    println!("SigmaOS SigmaOS-apm-server v0.1.0");
    Ok(())
}
