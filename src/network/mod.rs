//! Network functionality for peer discovery and communication.

pub mod discovery;
pub mod transport;
pub mod protocol;

use crate::error::Error;
use crate::config::NetworkConfig;
use libp2p::{
    core::transport::Transport,
    identity,
    PeerId,
    Swarm,
};
use std::collections::HashSet;

/// Represents a peer in the network.
#[derive(Debug, Clone)]
pub struct Peer {
    /// The peer's ID.
    pub id: PeerId,
    /// The peer's addresses.
    pub addresses: Vec<String>,
    /// Whether the peer is connected.
    pub connected: bool,
    /// The peer's reported capabilities.
    pub capabilities: Vec<String>,
}

/// The main network manager for CatP2P.
pub struct NetworkManager {
    config: NetworkConfig,
    local_peer_id: PeerId,
    known_peers: HashSet<PeerId>,
    // Will add more fields as we implement the network functionality
}

impl NetworkManager {
    /// Creates a new NetworkManager with the given configuration.
    pub fn new(config: NetworkConfig) -> Result<Self, Error> {
        // Generate a random peer ID for now
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        Ok(Self {
            config,
            local_peer_id,
            known_peers: HashSet::new(),
        })
    }

    /// Starts the network manager.
    pub fn start(&mut self) -> Result<(), Error> {
        // Implementation will be added later
        Ok(())
    }

    /// Stops the network manager.
    pub fn stop(&mut self) -> Result<(), Error> {
        // Implementation will be added later
        Ok(())
    }

    /// Returns the local peer ID.
    pub fn local_peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }

    /// Returns a list of known peers.
    pub fn known_peers(&self) -> Vec<PeerId> {
        self.known_peers.iter().cloned().collect()
    }
}
