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

//! Resource monitoring and allocation functionality.

pub mod monitor;
pub mod allocation;

use crate::error::Error;
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt, ProcessorExt, DiskExt};

/// System resource information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResources {
    /// CPU usage as a percentage (0.0 - 100.0).
    pub cpu_usage: f32,
    /// Total CPU cores available.
    pub cpu_cores: u32,
    /// Total memory in bytes.
    pub total_memory: u64,
    /// Available memory in bytes.
    pub available_memory: u64,
    /// Total disk space in bytes.
    pub total_disk: u64,
    /// Available disk space in bytes.
    pub available_disk: u64,
    /// GPU information, if available.
    pub gpu_info: Option<GpuInfo>,
}

/// GPU information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    /// GPU name.
    pub name: String,
    /// GPU usage as a percentage (0.0 - 100.0).
    pub usage: f32,
    /// Total GPU memory in bytes.
    pub total_memory: u64,
    /// Available GPU memory in bytes.
    pub available_memory: u64,
}

/// The main resource manager for CatP2P.
pub struct ResourceManager {
    system: System,
    // Will add more fields as we implement the resource management functionality
}

impl ResourceManager {
    /// Creates a new ResourceManager.
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system,
        }
    }

    /// Gets the current system resources.
    pub fn get_system_resources(&mut self) -> SystemResources {
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

    /// Checks if the system has enough resources for a given task.
    pub fn has_enough_resources(&mut self, cpu: f32, memory: u64, disk: u64) -> bool {
        self.system.refresh_all();
        
        let available_memory = self.system.available_memory();
        let available_disk = self.system.disks().iter()
            .map(|disk| disk.available_space())
            .sum();
        
        // Simple check for now
        available_memory >= memory && available_disk >= disk
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}
