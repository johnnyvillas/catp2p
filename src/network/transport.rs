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

//! Network transport functionality.

use crate::error::Error;
use libp2p::{
    core::upgrade,
    identity,
    noise,
    tcp,
    yamux,
    PeerId,
    Transport,
};
use std::time::Duration;

/// Creates a libp2p transport with the given identity.
pub fn create_transport(
    keypair: identity::Keypair,
) -> Result<
    impl Transport<Output = (PeerId, impl libp2p::core::muxing::StreamMuxer)> + Clone,
    Error,
> {
    // Create a noise authentication configuration
    let noise_config = noise::Config::new(&keypair)
        .map_err(|e| Error::Network(format!("Failed to create noise keys: {}", e)))?;

    // Create a TCP transport with DNS resolution
    let transport = tcp::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise_config)
        .multiplex(yamux::Config::default())
        .timeout(Duration::from_secs(20))
        .boxed();

    Ok(transport)
}
