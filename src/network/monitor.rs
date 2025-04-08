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

//! Network monitoring functionality.

use crate::error::Error;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use tokio::time;

/// Network statistics.
#[derive(Debug, Clone)]
pub struct NetworkStats {
    /// Total bytes sent.
    pub bytes_sent: u64,
    /// Total bytes received.
    pub bytes_received: u64,
    /// Current upload speed in bytes per second.
    pub upload_speed: f64,
    /// Current download speed in bytes per second.
    pub download_speed: f64,
    /// Active connections.
    pub active_connections: usize,
}

/// Network monitor for tracking connection quality and performance.
pub struct NetworkMonitor {
    update_interval: Duration,
    stats: Arc<Mutex<NetworkStats>>,
    running: bool,
}

impl NetworkMonitor {
    /// Creates a new NetworkMonitor with the given update interval.
    pub fn new(update_interval: Duration) -> Self {
        Self {
            update_interval,
            stats: Arc::new(Mutex::new(NetworkStats {
                bytes_sent: 0,
                bytes_received: 0,
                upload_speed: 0.0,
                download_speed: 0.0,
                active_connections: 0,
            })),
            running: false,
        }
    }
    
    /// Creates a new NetworkMonitor with a default update interval of 5 seconds.
    pub fn new_with_default_interval() -> Self {
        Self::new(Duration::from_secs(5))
    }
    
    /// Starts monitoring the network.
    pub fn start(&mut self) -> Result<(), Error> {
        if self.running {
            return Err(Error::Network("Network monitor is already running".to_string()));
        }
        
        self.running = true;
        
        let stats = self.stats.clone();
        let interval = self.update_interval;
        
        tokio::spawn(async move {
            let mut last_bytes_sent = 0;
            let mut last_bytes_received = 0;
            let mut last_update = std::time::Instant::now();
            
            let mut timer = time::interval(interval);
            
            loop {
                timer.tick().await;
                
                // In a real implementation, we would collect actual network statistics here
                // For now, we'll just simulate some network activity
                
                let now = std::time::Instant::now();
                let elapsed = now.duration_since(last_update).as_secs_f64();
                
                let mut stats_lock = match stats.lock() {
                    Ok(lock) => lock,
                    Err(_) => continue,
                };
                
                // Calculate speeds
                let bytes_sent_diff = stats_lock.bytes_sent - last_bytes_sent;
                let bytes_received_diff = stats_lock.bytes_received - last_bytes_received;
                
                stats_lock.upload_speed = bytes_sent_diff as f64 / elapsed;
                stats_lock.download_speed = bytes_received_diff as f64 / elapsed;
                
                // Update last values
                last_bytes_sent = stats_lock.bytes_sent;
                last_bytes_received = stats_lock.bytes_received;
                last_update = now;
            }
        });
        
        Ok(())
    }
    
    /// Stops monitoring the network.
    pub fn stop(&mut self) -> Result<(), Error> {
        if !self.running {
            return Err(Error::Network("Network monitor is not running".to_string()));
        }
        
        self.running = false;
        
        // In a real implementation, we would signal the monitoring task to stop
        
        Ok(())
    }
    
    /// Gets the current network statistics.
    pub fn get_stats(&self) -> Result<NetworkStats, Error> {
        let stats = self.stats.lock()
            .map_err(|_| Error::Network("Failed to lock network stats".to_string()))?;
        
        Ok(stats.clone())
    }
    
    /// Updates the bytes sent counter.
    pub fn update_bytes_sent(&self, bytes: u64) -> Result<(), Error> {
        let mut stats = self.stats.lock()
            .map_err(|_| Error::Network("Failed to lock network stats".to_string()))?;
        
        stats.bytes_sent += bytes;
        
        Ok(())
    }
    
    /// Updates the bytes received counter.
    pub fn update_bytes_received(&self, bytes: u64) -> Result<(), Error> {
        let mut stats = self.stats.lock()
            .map_err(|_| Error::Network("Failed to lock network stats".to_string()))?;
        
        stats.bytes_received += bytes;
        
        Ok(())
    }
    
    /// Updates the active connections counter.
    pub fn update_active_connections(&self, connections: usize) -> Result<(), Error> {
        let mut stats = self.stats.lock()
            .map_err(|_| Error::Network("Failed to lock network stats".to_string()))?;
        
        stats.active_connections = connections;
        
        Ok(())
    }
}
