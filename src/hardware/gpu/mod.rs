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

//! GPU hardware information and utilities.
//!
//! This module provides functionality for retrieving detailed information about
//! the GPU hardware available on the system, including specifications, capabilities,
//! and current status.
//!
//! # Examples
//!
//! ```
//! use catp2p::hardware::gpu;
//!
//! // Get information about the primary GPU
//! if let Ok(gpu_info) = gpu::get_info() {
//!     println!("GPU: {} with {} VRAM", gpu_info.name, gpu_info.vram);
//!     println!("Vendor: {}", gpu_info.vendor);
//!     println!("Architecture: {}", gpu_info.architecture);
//!     println!("Driver: {}", gpu_info.driver);
//! }
//!
//! Get information about all available GPUs
//! if let Ok(all_gpus) = gpu::get_all_info() {
//!     println!("Found {} GPUs:", all_gpus.len());
//!     for (i, gpu) in all_gpus.iter().enumerate() {
//!         println!("GPU {}: {}", i+1, gpu.name);
//!     }
//! }
//!
//! // Monitor GPU usage in real-time
//! if let Ok(mut usage) = gpu::get_usage() {
//!     println!("GPU: {} - Usage: {:.1}%, VRAM: {}/{}", 
//!         usage.name, usage.gpu_usage_percent, usage.used_vram, usage.total_vram);
//! }
//! ```

pub mod info;

use crate::error::Error;
use wgpu::Adapter;
use std::time::Duration;

// Re-export the GpuInfo struct for convenience
pub use info::GpuInfo;
pub use info::GpuUsageInfo;
pub use info::TemperatureUnit;

/// Gets information about the primary GPU.
///
/// This function attempts to detect and retrieve detailed information about
/// the primary GPU in the system. It uses the highest performance GPU available.
///
/// # Returns
///
/// Returns a `Result` containing either a `GpuInfo` struct with detailed
/// information about the GPU, or an `Error` if no suitable GPU was found
/// or if there was an error retrieving the information.
///
/// # Examples
///
/// ```
/// use catp2p::hardware::gpu;
///
/// match gpu::get_info() {
///     Ok(info) => println!("Found GPU: {}", info.name),
///     Err(e) => println!("Error detecting GPU: {}", e),
/// }
/// ```
pub fn get_info() -> Result<GpuInfo, Error> {
    info::get_gpu_info()
}

/// Gets information about all available GPUs in the system.
///
/// This function attempts to detect and retrieve detailed information about
/// all GPUs available in the system, including both discrete and integrated GPUs.
///
/// # Returns
///
/// Returns a `Result` containing either a `Vec<GpuInfo>` with information about
/// all detected GPUs, or an `Error` if no GPUs were found or if there was an
/// error retrieving the information.
///
/// # Examples
///
/// ```
/// use catp2p::hardware::gpu;
///
/// match gpu::get_all_info() {
///     Ok(gpus) => {
///         println!("Found {} GPUs:", gpus.len());
///         for (i, gpu) in gpus.iter().enumerate() {
///             println!("GPU {}: {} with {} VRAM", i+1, gpu.name, gpu.vram);
///         }
///     },
///     Err(e) => println!("Error detecting GPUs: {}", e),
/// }
/// ```
pub fn get_all_info() -> Result<Vec<GpuInfo>, Error> {
    info::get_all_gpu_info()
}

/// Gets current usage information for the primary GPU.
///
/// This function retrieves real-time usage information about the primary GPU,
/// including memory usage and utilization percentage.
///
/// # Returns
///
/// Returns a `Result` containing either a `GpuUsageInfo` struct with current
/// usage information, or an `Error` if no suitable GPU was found or if there
/// was an error retrieving the information.
///
/// # Examples
///
/// ```
/// use catp2p::hardware::gpu;
///
/// match gpu::get_usage() {
///     Ok(usage) => {
///         println!("GPU: {}", usage.name);
///         println!("Usage: {:.1}%", usage.gpu_usage_percent);
///         println!("VRAM: {}/{}", usage.used_vram, usage.total_vram);
///     },
///     Err(e) => println!("Error getting GPU usage: {}", e),
/// }
/// ```
pub fn get_usage() -> Result<GpuUsageInfo, Error> {
    info::get_gpu_usage()
}

/// Gets current usage information for a specific GPU by name.
///
/// This function retrieves real-time usage information about a specific GPU
/// identified by its name, including memory usage and utilization percentage.
///
/// # Arguments
///
/// * `gpu_name` - The name of the GPU to get usage information for
///
/// # Returns
///
/// Returns a `Result` containing either a `GpuUsageInfo` struct with current
/// usage information, or an `Error` if the specified GPU was not found or if
/// there was an error retrieving the information.
///
/// # Examples
///
/// ```
/// use catp2p::hardware::gpu;
///
/// let gpu_name = "NVIDIA GeForce RTX 3080";
/// match gpu::get_usage_by_name(gpu_name) {
///     Ok(usage) => {
///         println!("GPU: {}", usage.name);
///         println!("Usage: {:.1}%", usage.gpu_usage_percent);
///         println!("VRAM: {}/{}", usage.used_vram, usage.total_vram);
///     },
///     Err(e) => println!("Error getting GPU usage: {}", e),
/// }
/// ```
pub fn get_usage_by_name(gpu_name: &str) -> Result<GpuUsageInfo, Error> {
    info::get_gpu_usage_by_name(gpu_name)
}

/// Monitors GPU usage over a specified duration.
///
/// This function monitors the primary GPU's usage over the specified duration,
/// taking samples at the specified interval, and returns the average, minimum,
/// and maximum values.
///
/// # Arguments
///
/// * `duration` - The total duration to monitor for
/// * `sample_interval` - The interval between samples
///
/// # Returns
///
/// Returns a `Result` containing either a `GpuUsageStats` struct with usage
/// statistics, or an `Error` if there was an error monitoring the GPU.
///
/// # Examples
///
/// ```
/// use catp2p::hardware::gpu;
/// use std::time::Duration;
///
/// match gpu::monitor_usage(Duration::from_secs(5), Duration::from_millis(500)) {
///     Ok(stats) => {
///         println!("GPU: {}", stats.name);
///         println!("Average usage: {:.1}%", stats.avg_usage_percent);
///         println!("Min/Max usage: {:.1}%/{:.1}%", stats.min_usage_percent, stats.max_usage_percent);
///         println!("Average VRAM: {}/{}", stats.avg_used_vram, stats.total_vram);
///     },
///     Err(e) => println!("Error monitoring GPU: {}", e),
/// }
/// ```
pub fn monitor_usage(duration: Duration, sample_interval: Duration) -> Result<GpuUsageStats, Error> {
    info::monitor_gpu_usage(duration, sample_interval)
}

/// Checks if a GPU is available on the system.
///
/// This function attempts to detect if any GPU is available on the system
/// that can be used for GPU-accelerated tasks.
///
/// # Returns
///
/// Returns `true` if a GPU is available, `false` otherwise.
///
/// # Examples
///
/// ```
/// use catp2p::hardware::gpu;
///
/// if gpu::is_available() {
///     println!("GPU is available for acceleration");
/// } else {
///     println!("No GPU detected, falling back to CPU");
/// }
/// ```
pub fn is_available() -> bool {
    info::is_gpu_available()
}

/// Gets GPU information from a specific wgpu adapter.
///
/// This function is useful when you already have a wgpu adapter instance
/// and want to retrieve detailed information about the corresponding GPU.
///
/// # Arguments
///
/// * `adapter` - A reference to a wgpu Adapter instance
///
/// # Returns
///
/// Returns a `Result` containing either a `GpuInfo` struct with detailed
/// information about the GPU, or an `Error` if there was an error retrieving
/// the information.
///
/// # Examples
///
/// ```
/// use catp2p::hardware::gpu;
/// use wgpu::{Instance, InstanceDescriptor, Backends, RequestAdapterOptions, PowerPreference};
///
/// // Create a wgpu instance and request an adapter
/// let instance = Instance::new(InstanceDescriptor {
///     backends: Backends::all(),
///     dx12_shader_compiler: Default::default(),
/// });
///
/// let adapter = pollster::block_on(async {
///     instance.request_adapter(&RequestAdapterOptions {
///         power_preference: PowerPreference::HighPerformance,
///         compatible_surface: None,
///         force_fallback_adapter: false,
///     }).await
/// });
///
/// if let Some(adapter) = adapter {
///     match gpu::get_info_from_adapter(&adapter) {
///         Ok(info) => println!("Adapter GPU: {}", info.name),
///         Err(e) => println!("Error: {}", e),
///     }
/// }
/// ```
pub fn get_info_from_adapter(adapter: &Adapter) -> Result<GpuInfo, Error> {
    info::get_gpu_info_from_adapter(adapter)
}

/// Statistics about GPU usage over time.
#[derive(Debug, Clone)]
pub struct GpuUsageStats {
    /// GPU name
    pub name: String,
    /// GPU vendor
    pub vendor: String,
    /// Total VRAM formatted as string (e.g., "8.0 GB")
    pub total_vram: String,
    /// Average GPU usage percentage
    pub avg_usage_percent: f32,
    /// Minimum GPU usage percentage
    pub min_usage_percent: f32,
    /// Maximum GPU usage percentage
    pub max_usage_percent: f32,
    /// Average used VRAM formatted as string
    pub avg_used_vram: String,
    /// Minimum used VRAM formatted as string
    pub min_used_vram: String,
    /// Maximum used VRAM formatted as string
    pub max_used_vram: String,
    /// Number of samples taken
    pub sample_count: usize,
    /// Duration of monitoring
    pub duration: Duration,
}
