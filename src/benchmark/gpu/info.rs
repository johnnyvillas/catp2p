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

//! GPU information utilities.

use crate::error::Error;
use wgpu::Adapter;

/// GPU information structure.
#[derive(Debug, Clone)]
pub struct GpuInfo {
    /// GPU model name
    pub name: String,
    /// GPU vendor
    pub vendor: String,
    /// GPU driver info
    pub driver: String,
    /// Estimated VRAM in GB
    pub vram: String,
    /// Backend used (Vulkan, DirectX, Metal, etc.)
    pub backend: String,
    /// Whether the GPU is integrated or discrete
    pub is_integrated: bool,
}

/// Gets GPU information from the adapter.
pub fn get_gpu_info_from_adapter(adapter: &Adapter) -> Result<GpuInfo, Error> {
    let info = adapter.get_info();
    
    // Estimate VRAM based on adapter limits
    let vram_estimate = pollster::block_on(estimate_vram(adapter));
    
    // Determine if integrated or discrete
    let is_integrated = match info.device_type {
        wgpu::DeviceType::IntegratedGpu => true,
        _ => false,
    };
    
    Ok(GpuInfo {
        name: info.name,
        vendor: format!("{:?}", info.vendor),
        driver: format!("Driver: {:?}, Backend: {:?}", info.driver, info.backend),
        vram: vram_estimate,
        backend: format!("{:?}", info.backend),
        is_integrated,
    })
}

/// Gets information about the GPU.
pub fn get_gpu_info() -> Result<GpuInfo, Error> {
    // Initialize wgpu
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    
    // Request adapter without surface (headless)
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .ok_or_else(|| Error::Benchmark("No suitable GPU adapter found".to_string()))?;
    
    get_gpu_info_from_adapter(&adapter)
}

/// Estimates VRAM based on adapter limits.
async fn estimate_vram(adapter: &Adapter) -> String {
    let limits = adapter.limits();
    
    // Get some indication of memory based on maximum buffer binding size
    let max_buffer_size = limits.max_buffer_size;
    let vram_estimate = match max_buffer_size {
        size if size >= 8 * 1024 * 1024 * 1024 => format!("8+ GB"),
        size if size >= 4 * 1024 * 1024 * 1024 => format!("4-8 GB"),
        size if size >= 2 * 1024 * 1024 * 1024 => format!("2-4 GB"),
        size if size >= 1 * 1024 * 1024 * 1024 => format!("1-2 GB"),
        _ => format!("Less than 1 GB"),
    };
    
    vram_estimate
}

/// Checks if GPU is available.
pub fn is_gpu_available() -> bool {
    match get_gpu_info() {
        Ok(_) => true,
        Err(_) => false,
    }
}
