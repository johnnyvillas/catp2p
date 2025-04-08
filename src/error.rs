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
