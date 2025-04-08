//! Network transport functionality.

use crate::error::Error;
use libp2p::{
    core::transport::Transport,
    dns::DnsConfig,
    identity,
    mplex,
    noise::{self, NoiseConfig},
    tcp::TcpConfig,
    websocket::WsConfig,
    yamux,
    PeerId,
    Swarm,
    Transport as _,
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
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&keypair)
        .map_err(|e| Error::Network(format!("Failed to create noise keys: {}", e)))?;

    let noise = NoiseConfig::xx(noise_keys).into_authenticated();

    // Create a TCP transport with DNS resolution
    let transport = TcpConfig::new()
        .nodelay(true)
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise)
        .multiplex(yamux::YamuxConfig::default())
        .timeout(Duration::from_secs(20))
        .boxed();

    Ok(transport)
}
