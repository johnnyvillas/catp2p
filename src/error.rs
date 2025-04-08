//! Error types for the CatP2P library.

use thiserror::Error;

/// Error type for the CatP2P library.
#[derive(Error, Debug)]
pub enum Error {
    /// Network-related errors.
    #[error("Network error: {0}")]
    Network(String),

    /// Task-related errors.
    #[error("Task error: {0}")]
    Task(String),

    /// Resource-related errors.
    #[error("Resource error: {0}")]
    Resource(String),

    /// Storage-related errors.
    #[error("Storage error: {0}")]
    Storage(String),

    /// Configuration errors.
    #[error("Configuration error: {0}")]
    Config(String),

    /// I/O errors.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization/deserialization errors.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Database errors.
    #[error("Database error: {0}")]
    Database(String),

    /// Feature not implemented.
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Other errors.
    #[error("Other error: {0}")]
    Other(String),
}
