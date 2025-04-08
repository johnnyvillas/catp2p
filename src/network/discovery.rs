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

//! Peer discovery functionality.

use crate::error::Error;
use libp2p::PeerId;
use std::collections::HashSet;

/// The discovery manager for finding peers on the network.
pub struct DiscoveryManager {
    peer_ids: HashSet<PeerId>,
    // Will add more fields as we implement the discovery functionality
}

impl DiscoveryManager {
    /// Creates a new DiscoveryManager.
    pub fn new() -> Self {
        Self {
            peer_ids: HashSet::new(),
        }
    }

    /// Starts the discovery process.
    pub fn start(&mut self) -> Result<(), Error> {
        // Implementation will be added later
        Ok(())
    }

    /// Stops the discovery process.
    pub fn stop(&mut self) -> Result<(), Error> {
        // Implementation will be added later
        Ok(())
    }

    /// Returns the discovered peer IDs.
    pub fn discovered_peers(&self) -> Vec<PeerId> {
        self.peer_ids.iter().cloned().collect()
    }

    /// Adds a peer to the discovered peers list.
    pub fn add_peer(&mut self, peer_id: PeerId) {
        self.peer_ids.insert(peer_id);
    }

    /// Removes a peer from the discovered peers list.
    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        self.peer_ids.remove(peer_id);
    }
}

impl Default for DiscoveryManager {
    fn default() -> Self {
        Self::new()
    }
}
