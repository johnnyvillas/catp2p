//! Resource allocation functionality.

use crate::error::Error;
use crate::config::{ResourceMode, ResourceLimits};
use crate::resources::SystemResources;
use std::sync::{Arc, Mutex};

/// A resource allocator that manages system resources.
pub struct ResourceAllocator {
    mode: ResourceMode,
    limits: Option<ResourceLimits>,
    current_resources: Arc<Mutex<SystemResources>>,
}

impl ResourceAllocator {
    /// Creates a new ResourceAllocator with the given mode and limits.
    pub fn new(mode: ResourceMode, limits: Option<ResourceLimits>, resources: SystemResources) -> Self {
        Self {
            mode,
            limits,
            current_resources: Arc::new(Mutex::new(resources)),
        }
    }
    
    /// Updates the current system resources.
    pub fn update_resources(&self, resources: SystemResources) -> Result<(), Error> {
        let mut current = self.current_resources.lock().map_err(|_| {
            Error::Resource("Failed to lock resources".to_string())
        })?;
        
        *current = resources;
        
        Ok(())
    }
    
    /// Gets the current system resources.
    pub fn get_resources(&self) -> Result<SystemResources, Error> {
        let current = self.current_resources.lock().map_err(|_| {
            Error::Resource("Failed to lock resources".to_string())
        })?;
        
        Ok(current.clone())
    }
    
    /// Checks if there are enough resources available for the given requirements.
    pub fn has_enough_resources(&self, cpu_cores: u32, memory: u64, disk: u64) -> Result<bool, Error> {
        let current = self.current_resources.lock().map_err(|_| {
            Error::Resource("Failed to lock resources".to_string())
        })?;
        
        // Calculate available resources based on mode and limits
        let (available_cpu, available_memory, available_disk) = match self.mode {
            ResourceMode::Light => {
                let cpu_limit = current.cpu_cores / 4;
                let memory_limit = current.available_memory / 4;
                let disk_limit = current.available_disk / 4;
                (cpu_limit, memory_limit, disk_limit)
            },
            ResourceMode::Medium => {
                let cpu_limit = current.cpu_cores / 2;
                let memory_limit = current.available_memory / 2;
                let disk_limit = current.available_disk / 2;
                (cpu_limit, memory_limit, disk_limit)
            },
            ResourceMode::HighPerformance => {
                let cpu_limit = current.cpu_cores * 3 / 4;
                let memory_limit = current.available_memory * 3 / 4;
                let disk_limit = current.available_disk * 3 / 4;
                (cpu_limit, memory_limit, disk_limit)
            },
            ResourceMode::Custom => {
                if let Some(limits) = &self.limits {
                    let cpu_limit = (current.cpu_cores as f32 * limits.cpu_limit) as u32;
                    let memory_limit = limits.memory_limit.min(current.available_memory);
                    let disk_limit = limits.storage_limit.min(current.available_disk);
                    (cpu_limit, memory_limit, disk_limit)
                } else {
                    // Default to medium if no limits are specified
                    let cpu_limit = current.cpu_cores / 2;
                    let memory_limit = current.available_memory / 2;
                    let disk_limit = current.available_disk / 2;
                    (cpu_limit, memory_limit, disk_limit)
                }
            },
        };
        
        // Check if the requirements can be met
        Ok(cpu_cores <= available_cpu && memory <= available_memory && disk <= available_disk)
    }
    
    /// Sets the resource mode.
    pub fn set_mode(&mut self, mode: ResourceMode) {
        self.mode = mode;
    }
    
    /// Sets the resource limits for custom mode.
    pub fn set_limits(&mut self, limits: ResourceLimits) {
        self.limits = Some(limits);
    }
}
