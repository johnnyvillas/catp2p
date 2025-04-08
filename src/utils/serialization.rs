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

//! Serialization utilities.

use crate::error::Error;
use serde::{Serialize, Deserialize};

/// Serializes a value to JSON.
pub fn to_json<T: Serialize>(value: &T) -> Result<String, Error> {
    serde_json::to_string(value)
        .map_err(|e| Error::Serialization(format!("Failed to serialize to JSON: {}", e)))
}

/// Deserializes a value from JSON.
pub fn from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T, Error> {
    serde_json::from_str(json)
        .map_err(|e| Error::Serialization(format!("Failed to deserialize from JSON: {}", e)))
}

/// Serializes a value to CBOR.
pub fn to_cbor<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    serde_cbor::to_vec(value)
        .map_err(|e| Error::Serialization(format!("Failed to serialize to CBOR: {}", e)))
}

/// Deserializes a value from CBOR.
pub fn from_cbor<T: for<'de> Deserialize<'de>>(cbor: &[u8]) -> Result<T, Error> {
    serde_cbor::from_slice(cbor)
        .map_err(|e| Error::Serialization(format!("Failed to deserialize from CBOR: {}", e)))
}

/// Serializes a value to MessagePack.
pub fn to_msgpack<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    rmp_serde::to_vec(value)
        .map_err(|e| Error::Serialization(format!("Failed to serialize to MessagePack: {}", e)))
}

/// Deserializes a value from MessagePack.
pub fn from_msgpack<T: for<'de> Deserialize<'de>>(msgpack: &[u8]) -> Result<T, Error> {
    rmp_serde::from_slice(msgpack)
        .map_err(|e| Error::Serialization(format!("Failed to deserialize from MessagePack: {}", e)))
}
