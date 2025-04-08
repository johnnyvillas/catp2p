//! Custom protocols for peer communication.

use crate::error::Error;
use libp2p::{
    core::upgrade::{self, InboundUpgrade, OutboundUpgrade},
    identity,
    PeerId,
    Swarm,
};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

/// A simple protocol for exchanging messages between peers.
#[derive(Debug, Clone)]
pub struct MessageProtocol {
    protocol_name: String,
}

impl MessageProtocol {
    /// Creates a new MessageProtocol with the given name.
    pub fn new(protocol_name: String) -> Self {
        Self {
            protocol_name,
        }
    }

    /// Returns the protocol name.
    pub fn protocol_name(&self) -> &str {
        &self.protocol_name
    }
}

/// A trait for handling protocol messages.
#[async_trait]
pub trait MessageHandler {
    /// Handles an incoming message.
    async fn handle_message(&self, peer_id: &PeerId, message: &[u8]) -> Result<Vec<u8>, Error>;
}

/// A simple message format for peer communication.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    /// The message type.
    pub message_type: String,
    /// The message payload.
    pub payload: Vec<u8>,
}

impl Message {
    /// Creates a new Message.
    pub fn new(message_type: String, payload: Vec<u8>) -> Self {
        Self {
            message_type,
            payload,
        }
    }

    /// Serializes the message to bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        serde_json::to_vec(self)
            .map_err(|e| Error::Serialization(e))
    }

    /// Deserializes a message from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        serde_json::from_slice(bytes)
            .map_err(|e| Error::Serialization(e))
    }
}
