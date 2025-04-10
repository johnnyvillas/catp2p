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

//! GPU benchmark test implementations.

use crate::error::Error;
use std::time::Duration;
use wgpu::{Adapter, Device, Queue};

// Import individual benchmark modules
mod info;
mod matrix_multiplications;
mod activation_functions;

// Re-export structures and functions
pub use info::{GpuInfo, get_gpu_info, is_gpu_available};
pub use matrix_multiplications::run_matrix_mult_benchmark;
pub use activation_functions::run_activation_functions_benchmark;

/// Result of a GPU test.
#[derive(Debug, Clone)]
pub struct GpuTestResult {
    /// Name of the test
    pub test_name: String,
    /// Average FPS during the test
    pub average_fps: f64,
    /// Minimum FPS during the test
    pub min_fps: f64,
    /// Maximum FPS during the test
    pub max_fps: f64,
    /// Test-specific score
    pub score: f64,
}

/// Context for GPU benchmarks to reuse resources.
pub struct GpuBenchmarkContext {
    /// GPU information
    pub gpu_info: GpuInfo,
    /// GPU adapter
    pub adapter: Adapter,
    /// GPU device
    pub device: Device,
    /// GPU queue
    pub queue: Queue,
}

impl GpuBenchmarkContext {
    /// Creates a new GPU benchmark context.
    pub fn new() -> Result<Self, Error> {
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
        
        // Get GPU information
        let gpu_info = info::get_gpu_info_from_adapter(&adapter)?;
        
        // Create device and queue
        let (device, queue) = pollster::block_on(async {
            adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("GPU Benchmark Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| Error::Benchmark(format!("Failed to create device: {}", e)))
        })?;
        
        Ok(Self {
            gpu_info,
            adapter,
            device,
            queue,
        })
    }
    
    /// Runs a matrix multiplication benchmark.
    pub fn run_matrix_mult(&self, test_duration: Duration, matrix_size: u32) -> Result<GpuTestResult, Error> {
        matrix_multiplications::run_matrix_mult_benchmark_with_context(
            &self.adapter,
            &self.device,
            &self.queue,
            test_duration,
            matrix_size,
        )
    }
    
    /// Runs an activation functions benchmark.
    pub fn run_activation_functions(&self, test_duration: Duration, data_size: u32) -> Result<GpuTestResult, Error> {
        activation_functions::run_activation_functions_benchmark_with_context(
            &self.adapter,
            &self.device,
            &self.queue,
            test_duration,
            data_size,
        )
    }
}

/// Initializes the GPU and returns adapter information.
pub fn initialize_gpu(_show_window: bool) -> Result<(GpuInfo, Adapter), Error> {
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
    
    // Get GPU information
    let gpu_info = info::get_gpu_info_from_adapter(&adapter)?;
    
    Ok((gpu_info, adapter))
}

/// Runs a GPU benchmark.
pub fn run_gpu_benchmark() -> Result<f64, Error> {
    // Create a benchmark context
    let context = GpuBenchmarkContext::new()?;
    
    // Run matrix multiplication benchmark
    let matrix_size = 1024; // 1024x1024 matrix
    let test_result = context.run_matrix_mult(
        Duration::from_secs(5),
        matrix_size,
    )?;
    
    Ok(test_result.score)
}

/// Runs a GPU benchmark with custom configuration.
pub fn run_gpu_benchmark_with_config(
    config: &GpuBenchmarkConfig,
) -> Result<GpuBenchmarkResult, Error> {
    // Create a benchmark context
    let context = GpuBenchmarkContext::new()?;
    
    // Calculate matrix size based on complexity
    let matrix_size = 512 + (config.complexity * 128);
    
    // Run matrix multiplication benchmark
    let matrix_result = context.run_matrix_mult(
        Duration::from_secs(config.test_duration_secs),
        matrix_size,
    )?;
    
    // Initialize scores
    let mut compute_score = matrix_result.score;
    let texture_score = 0.0;
    let geometry_score = 0.0;
    let memory_score = 0.0;
    let mut test_results = vec![matrix_result.clone()];
    let mut test_count = 1;
    
    // Run activation function benchmark if compute test is included
    if config.include_compute_test {
        // Calculate data size based on complexity
        let data_size = 100_000 * config.complexity;
        
        // Run activation functions benchmark
        match context.run_activation_functions(
            Duration::from_secs(config.test_duration_secs),
            data_size,
        ) {
            Ok(activation_result) => {
                compute_score = (compute_score + activation_result.score) / 2.0;
                test_results.push(activation_result);
                test_count += 1;
            },
            Err(e) => {
                eprintln!("Warning: Activation functions benchmark failed: {}", e);
            }
        }
    }
    
    // Calculate overall score
    let overall_score = compute_score;
    
    // Calculate average FPS
    let average_fps = test_results.iter().map(|r| r.average_fps).sum::<f64>() / test_count as f64;
    
    // Create benchmark result
    let result = GpuBenchmarkResult {
        gpu_model: context.gpu_info.name,
        gpu_vendor: context.gpu_info.vendor,
        vram_estimate: context.gpu_info.vram,
        compute_score,
        texture_score,
        geometry_score,
        memory_score,
        overall_score,
        average_fps,
        test_results,
    };
    
    Ok(result)
}

/// Configuration options for GPU benchmarks.
#[derive(Debug, Clone)]
pub struct GpuBenchmarkConfig {
    /// Duration of each test in seconds
    pub test_duration_secs: u64,
    /// Whether to include compute test
    pub include_compute_test: bool,
    /// Whether to include texture sampling test
    pub include_texture_test: bool,
    /// Whether to include geometry processing test
    pub include_geometry_test: bool,
    /// Whether to include memory bandwidth test
    pub include_memory_test: bool,
    /// Complexity level (1-10) affecting workload intensity
    pub complexity: u32,
    /// Window width for rendering tests
    pub window_width: u32,
    /// Window height for rendering tests
    pub window_height: u32,
    /// Whether to show the benchmark window
    pub show_window: bool,
}

impl Default for GpuBenchmarkConfig {
    fn default() -> Self {
        Self {
            test_duration_secs: 5,
            include_compute_test: true,
            include_texture_test: true,
            include_geometry_test: true,
            include_memory_test: true,
            complexity: 5,
            window_width: 800,
            window_height: 600,
            show_window: false,
        }
    }
}

/// Result of a GPU benchmark containing detailed performance metrics.
#[derive(Debug, Clone)]
pub struct GpuBenchmarkResult {
    /// GPU model name
    pub gpu_model: String,
    /// GPU vendor
    pub gpu_vendor: String,
    /// Estimated VRAM in GB
    pub vram_estimate: String,
    /// Compute performance score
    pub compute_score: f64,
    /// Texture sampling performance score
    pub texture_score: f64,
    /// Geometry processing performance score
    pub geometry_score: f64,
    /// Memory bandwidth performance score
    pub memory_score: f64,
    /// Overall benchmark score (higher is better)
    pub overall_score: f64,
    /// Average FPS across all tests
    pub average_fps: f64,
    /// Detailed results for each test
    pub test_results: Vec<GpuTestResult>,
}
