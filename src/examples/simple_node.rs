//! A simple example of a CatP2P node.

use catp2p::{CatP2P, config::{Config, ResourceMode}};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();

    println!("Starting a simple CatP2P node...");

    // Create a custom configuration
    let mut config = Config::default();
    config.resource_mode = ResourceMode::Medium;

    // Create a CatP2P instance with custom configuration
    let catp2p = CatP2P::with_config(config)?;

    // Start the node
    catp2p.start()?;

    println!("CatP2P node started. Press Ctrl+C to exit.");

    // Keep the node running until interrupted
    tokio::signal::ctrl_c().await?;

    // Stop the node
    catp2p.stop()?;

    println!("CatP2P node stopped.");

    Ok(())
}
