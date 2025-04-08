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

//! Resource monitoring functionality.

use crate::error::Error;
use crate::resources::SystemResources;
// Remove unused import
// use crate::resources::GpuInfo;
use sysinfo::{System, SystemExt, CpuExt, DiskExt}; // Added DiskExt
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

/// A resource monitor that periodically checks system resources.
pub struct ResourceMonitor {
    system: System,
    update_interval: Duration,
    running: bool,
}

impl ResourceMonitor {
    /// Creates a new ResourceMonitor with the given update interval.
    pub fn new(update_interval: Duration) -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system,
            update_interval,
            running: false,
        }
    }
    
    /// Creates a new ResourceMonitor with a default update interval of 1 second.
    pub fn new_with_default_interval() -> Self {
        Self::new(Duration::from_secs(1))
    }
    
    /// Gets the current system resources.
    pub fn get_current_resources(&mut self) -> SystemResources {
        self.system.refresh_all();
        
        // Use global_cpu_info() instead of global_processor_info()
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        // Use cpus() to get the list of CPUs
        let cpu_cores = self.system.cpus().len() as u32;
        
        let total_memory = self.system.total_memory();
        let available_memory = self.system.available_memory();
        
        // Add explicit type annotations for sum operations
        let total_disk: u64 = self.system.disks().iter()
            .map(|disk| disk.total_space())
            .sum();
        let available_disk: u64 = self.system.disks().iter()
            .map(|disk| disk.available_space())
            .sum();
        
        // GPU info is not directly available through sysinfo
        // We'll need to implement this using wgpu or another library
        let gpu_info = None;
        
        SystemResources {
            cpu_usage,
            cpu_cores,
            total_memory,
            available_memory,
            total_disk,
            available_disk,
            gpu_info,
        }
    }
    
    /// Starts the resource monitor and returns a channel for receiving resource updates.
    pub fn start(&mut self) -> Result<mpsc::Receiver<SystemResources>, Error> {
        if self.running {
            return Err(Error::Resource("Resource monitor is already running".to_string()));
        }
        
        self.running = true;
        
        let (tx, rx) = mpsc::channel(100);
        let update_interval = self.update_interval;
        
        // Clone the system for the monitoring task
        let mut system = System::new_all();
        
        tokio::spawn(async move {
            let mut interval = time::interval(update_interval);
            
            loop {
                interval.tick().await;
                
                system.refresh_all();
                
                // Use global_cpu_info() instead of global_processor_info()
                let cpu_usage = system.global_cpu_info().cpu_usage();
                // Use cpus() to get the list of CPUs
                let cpu_cores = system.cpus().len() as u32;
                
                let total_memory = system.total_memory();
                let available_memory = system.available_memory();
                
                // Add explicit type annotations for sum operations
                let total_disk: u64 = system.disks().iter()
                    .map(|disk| disk.total_space())
                    .sum();
                let available_disk: u64 = system.disks().iter()
                    .map(|disk| disk.available_space())
                    .sum();
                
                // GPU info is not directly available through sysinfo
                let gpu_info = None;
                
                let resources = SystemResources {
                    cpu_usage,
                    cpu_cores,
                    total_memory,
                    available_memory,
                    total_disk,
                    available_disk,
                    gpu_info,
                };
                
                // Send the resources update, but don't block if the channel is full
                let _ = tx.try_send(resources);
            }
        });
        
        Ok(rx)
    }
    
    /// Stops the resource monitor.
    pub fn stop(&mut self) {
        self.running = false;
    }
}
