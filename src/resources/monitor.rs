//! Resource monitoring functionality.

use crate::error::Error;
use crate::resources::{SystemResources, GpuInfo};
use sysinfo::{System, SystemExt, ProcessorExt, DiskExt};
use std::time::{Duration, Instant};
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
        
        let cpu_usage = self.system.global_processor_info().cpu_usage();
        let cpu_cores = self.system.processors().len() as u32;
        
        let total_memory = self.system.total_memory();
        let available_memory = self.system.available_memory();
        
        let total_disk = self.system.disks().iter()
            .map(|disk| disk.total_space())
            .sum();
        let available_disk = self.system.disks().iter()
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
                
                let cpu_usage = system.global_processor_info().cpu_usage();
                let cpu_cores = system.processors().len() as u32;
                
                let total_memory = system.total_memory();
                let available_memory = system.available_memory();
                
                let total_disk = system.disks().iter()
                    .map(|disk| disk.total_space())
                    .sum();
                let available_disk = system.disks().iter()
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
