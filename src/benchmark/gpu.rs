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

//! GPU benchmarking functionality.

use crate::error::Error;
// Remove the following line:
// use std::time::Instant;

#[cfg(feature = "gpu")]
use std::time::Instant;
#[cfg(feature = "gpu")]
use wgpu;

/// Checks if a GPU is available.
pub async fn is_gpu_available() -> bool {
    #[cfg(feature = "gpu")]
    {
        // Initialize wgpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        // Try to get an adapter
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            },
        ).await;
        
        adapter.is_some()
    }
    
    #[cfg(not(feature = "gpu"))]
    {
        false
    }
}

/// Runs a GPU benchmark and returns a score.
pub async fn run_gpu_benchmark() -> Result<f64, Error> {
    #[cfg(feature = "gpu")]
    {
        // Check if GPU is available
        if !is_gpu_available().await {
            return Err(Error::Benchmark("No GPU available".to_string()));
        }
        
        // Initialize wgpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        // Get the adapter
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            },
        ).await.ok_or_else(|| Error::Benchmark("Failed to get GPU adapter".to_string()))?;
        
        // Get the device and queue
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.map_err(|e| Error::Benchmark(format!("Failed to get GPU device: {}", e)))?;
        
        // Run the benchmark
        let start_time = Instant::now();
        
        // This is a placeholder for a real GPU benchmark
        // In a real implementation, we would create a compute shader and execute it
        
        // For now, we'll just simulate some GPU work
        let mut score = 0.0;
        
        // Get GPU info
        let info = adapter.get_info();
        
        // Adjust score based on GPU info
        match info.device_type {
            wgpu::DeviceType::DiscreteGpu => score += 1000.0,
            wgpu::DeviceType::IntegratedGpu => score += 500.0,
            _ => score += 100.0,
        }
        
        // Add some points for the backend
        match info.backend {
            wgpu::Backend::Vulkan => score += 200.0,
            wgpu::Backend::Metal => score += 180.0,
            wgpu::Backend::Dx12 => score += 190.0,
            _ => score += 100.0,
        }
        
        // Simulate some GPU work
        device.poll(wgpu::Maintain::Wait);
        
        let elapsed = start_time.elapsed();
        
        // Adjust score based on time
        // Lower time is better, so we invert it
        score *= 1.0 / elapsed.as_secs_f64().max(0.001);
        
        Ok(score)
    }
    
    #[cfg(not(feature = "gpu"))]
    {
        Err(Error::Benchmark("GPU support is not enabled".to_string()))
    }
}
