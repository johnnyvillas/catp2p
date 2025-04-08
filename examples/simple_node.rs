/* Copyright 2025 Joao Guimaraes, Catp2p Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
