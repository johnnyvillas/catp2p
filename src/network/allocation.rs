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

//! Network resource allocation functionality.

use crate::error::Error;
use std::sync::{Arc, Mutex};

/// Network resource allocator for managing bandwidth and connections.
pub struct NetworkAllocator {
    max_bandwidth: u64,
    max_connections: usize,
    allocated_bandwidth: Arc<Mutex<u64>>,
    active_connections: Arc<Mutex<usize>>,
}

impl NetworkAllocator {
    /// Creates a new NetworkAllocator with the given limits.
    pub fn new(max_bandwidth: u64, max_connections: usize) -> Self {
        Self {
            max_bandwidth,
            max_connections,
            allocated_bandwidth: Arc::new(Mutex::new(0)),
            active_connections: Arc::new(Mutex::new(0)),
        }
    }
    
    /// Allocates bandwidth for a specific task or connection.
    pub fn allocate_bandwidth(&self, required_bandwidth: u64) -> Result<(), Error> {
        let mut allocated = self.allocated_bandwidth.lock()
            .map_err(|_| Error::Network("Failed to lock allocated bandwidth".to_string()))?;
        
        if *allocated + required_bandwidth > self.max_bandwidth {
            return Err(Error::Network("Insufficient bandwidth available".to_string()));
        }
        
        *allocated += required_bandwidth;
        Ok(())
    }
    
    /// Releases allocated bandwidth.
    pub fn release_bandwidth(&self, bandwidth: u64) -> Result<(), Error> {
        let mut allocated = self.allocated_bandwidth.lock()
            .map_err(|_| Error::Network("Failed to lock allocated bandwidth".to_string()))?;
        
        if *allocated < bandwidth {
            *allocated = 0;
        } else {
            *allocated -= bandwidth;
        }
        
        Ok(())
    }
    
    /// Allocates a connection.
    pub fn allocate_connection(&self) -> Result<(), Error> {
        let mut connections = self.active_connections.lock()
            .map_err(|_| Error::Network("Failed to lock active connections".to_string()))?;
        
        if *connections >= self.max_connections {
            return Err(Error::Network("Maximum connections reached".to_string()));
        }
        
        *connections += 1;
        Ok(())
    }
    
    /// Releases a connection.
    pub fn release_connection(&self) -> Result<(), Error> {
        let mut connections = self.active_connections.lock()
            .map_err(|_| Error::Network("Failed to lock active connections".to_string()))?;
        
        if *connections > 0 {
            *connections -= 1;
        }
        
        Ok(())
    }
    
    /// Gets the current available bandwidth.
    pub fn available_bandwidth(&self) -> Result<u64, Error> {
        let allocated = self.allocated_bandwidth.lock()
            .map_err(|_| Error::Network("Failed to lock allocated bandwidth".to_string()))?;
        
        Ok(self.max_bandwidth - *allocated)
    }
    
    /// Gets the current available connections.
    pub fn available_connections(&self) -> Result<usize, Error> {
        let connections = self.active_connections.lock()
            .map_err(|_| Error::Network("Failed to lock active connections".to_string()))?;
        
        Ok(self.max_connections - *connections)
    }
}
