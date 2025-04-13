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

//! Main entry point for the CatP2P library, defining the public API and core functionality.

#![warn(missing_docs)]

pub mod config;
pub mod error;
pub mod network;
pub mod tasks;
pub mod resources;
pub mod storage;
pub mod benchmark;
pub mod hardware;
pub mod scoring;

use error::Error;
use config::Config;

/// The main entry point for the catp2p library.
#[allow(dead_code)]
pub struct CatP2P {
    config: Config,
    // Other fields will be added as we implement the components
}

impl CatP2P {
    /// Creates a new CatP2P instance with the default configuration.
    pub fn new() -> Result<Self, Error> {
        Self::with_config(Config::default())
    }

    /// Creates a new CatP2P instance with a custom configuration.
    pub fn with_config(config: Config) -> Result<Self, Error> {
        Ok(Self {
            config,
        })
    }

    /// Starts the CatP2P node.
    pub fn start(&self) -> Result<(), Error> {
        // Implementation will be added later
        Ok(())
    }

    /// Stops the CatP2P node.
    pub fn stop(&self) -> Result<(), Error> {
        // Implementation will be added later
        Ok(())
    }

    /// Runs a system benchmark to assess the node's capabilities.
    pub fn run_benchmark(&self) -> Result<benchmark::BenchmarkResult, Error> {
        // Implementation will be added later
        Err(Error::NotImplemented("Benchmarking not yet implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instance() {
        let catp2p = CatP2P::new().expect("Failed to create CatP2P instance");
        assert!(catp2p.config.is_valid());
    }
}
