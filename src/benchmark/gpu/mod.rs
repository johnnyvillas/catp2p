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
mod matrix_multiplications;
mod activation_functions;
mod gradient_calculations;

// Import GPU info from hardware module
use crate::hardware::gpu;

// Re-export structures and functions
pub use gpu::GpuInfo;
pub use gpu::{
    get_info as get_gpu_info,
    is_available as is_gpu_available,
    get_all_info as get_all_gpu_info,
    get_info_from_adapter as get_gpu_info_from_adapter,
};
pub use matrix_multiplications::run_matrix_mult_benchmark;
pub use activation_functions::run_activation_functions_benchmark;
pub use gradient_calculations::run_gradient_calc_benchmark;

// Reference values for normalizing scores to 0-100000 scale
/// Reference value for matrix multiplication (in MFLOPS)
pub const MATRIX_MULT_REFERENCE: f64 = 5_000_000.0;
/// Reference value for activation functions
pub const ACTIVATION_REFERENCE: f64 = 10_000.0;
/// Reference value for gradient calculations
pub const GRADIENT_REFERENCE: f64 = 15_000.0;

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
    /// Test-specific raw score
    pub score: f64,
    /// Normalized score (0-100000 scale)
    pub normalized_score: f64,
}

impl GpuTestResult {
    /// Gets the appropriate reference value for this test type
    pub fn get_reference_value(&self) -> f64 {
        match self.test_name.as_str() {
            "MatrixMultiplication" => MATRIX_MULT_REFERENCE,
            "ActivationFunctions" => ACTIVATION_REFERENCE,
            "GradientCalculation" => GRADIENT_REFERENCE,
            _ => 10000.0, // Default reference value
        }
    }
    
    /// Normalizes a raw score to the 0-100000 scale
    pub fn normalize_score(raw_score: f64, reference: f64) -> f64 {
        (raw_score / reference * 50000.0).min(100000.0)
    }
    
    /// Creates a new GpuTestResult with automatically calculated normalized score
    pub fn new(test_name: String, average_fps: f64, min_fps: f64, max_fps: f64, score: f64) -> Self {
        let reference = match test_name.as_str() {
            "MatrixMultiplication" => MATRIX_MULT_REFERENCE,
            "ActivationFunctions" => ACTIVATION_REFERENCE,
            "GradientCalculation" => GRADIENT_REFERENCE,
            _ => 10000.0, // Default reference value
        };
        
        let normalized_score = Self::normalize_score(score, reference);
        
        Self {
            test_name,
            average_fps,
            min_fps,
            max_fps,
            score,
            normalized_score,
        }
    }
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
        
        // Get GPU information using the hardware module
        let gpu_info = gpu::get_info_from_adapter(&adapter)?;
        
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
    
    /// Runs a gradient calculation benchmark.
    pub fn run_gradient_calc(&self, test_duration: Duration, data_size: u32) -> Result<GpuTestResult, Error> {
        gradient_calculations::run_gradient_calc_benchmark_with_context(
            &self.adapter,
            &self.device,
            &self.queue,
            test_duration,
            data_size,
        )
    }
    
    /// Runs selected benchmarks based on the provided configuration.
    pub fn run_selected_benchmarks(&self, config: &GpuBenchmarkConfig) -> Result<Vec<GpuTestResult>, Error> {
        let mut results = Vec::new();
        
        // Calculate matrix size based on complexity
        let matrix_size = 512 + (config.complexity * 128);
        
        // Calculate data size for neural network benchmarks
        let data_size = 100_000 * config.complexity;
        
        // Run matrix multiplication benchmark
        if config.include_matrix_test {
            match self.run_matrix_mult(
                Duration::from_secs(config.test_duration_secs),
                matrix_size,
            ) {
                Ok(result) => results.push(result),
                Err(e) => eprintln!("Warning: Matrix multiplication benchmark failed: {}", e),
            }
        }
        
        // Run activation function benchmark
        if config.include_compute_test {
            match self.run_activation_functions(
                Duration::from_secs(config.test_duration_secs),
                data_size,
            ) {
                Ok(result) => results.push(result),
                Err(e) => eprintln!("Warning: Activation functions benchmark failed: {}", e),
            }
            
            // Run gradient calculation benchmark
            match self.run_gradient_calc(
                Duration::from_secs(config.test_duration_secs),
                data_size,
            ) {
                Ok(result) => results.push(result),
                Err(e) => eprintln!("Warning: Gradient calculation benchmark failed: {}", e),
            }
        }
        
        Ok(results)
    }
    
    /// Calculates the overall benchmark score from a set of test results
    pub fn calculate_overall_score(results: &[GpuTestResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }
        
        // Calculate average of normalized scores
        let sum: f64 = results.iter()
            .map(|result| result.normalized_score)
            .sum();
        
        sum / results.len() as f64
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
    
    // Get GPU information using the hardware module
    let gpu_info = gpu::get_info_from_adapter(&adapter)?;
    
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
    
    // Run selected benchmarks
    let test_results = context.run_selected_benchmarks(config)?;
    
    if test_results.is_empty() {
        return Err(Error::Benchmark("No benchmarks were successfully completed".to_string()));
    }
    
    // Calculate scores by category
    let mut compute_score = 0.0;
    let mut compute_count = 0;
    let texture_score = 0.0;
    let geometry_score = 0.0;
    let memory_score = 0.0;
    
    // Process test results
    for result in &test_results {
        match result.test_name.as_str() {
            "MatrixMultiplication" | "ActivationFunctions" | "GradientCalculation" => {
                compute_score += result.normalized_score;
                compute_count += 1;
            },
            _ => {}
        }
    }
    
    // Calculate average compute score
    if compute_count > 0 {
        compute_score /= compute_count as f64;
    }
    
    // Calculate overall score
    let overall_score = GpuBenchmarkContext::calculate_overall_score(&test_results);
    
    // Calculate average FPS
    let average_fps = test_results.iter()
        .map(|r| r.average_fps)
        .sum::<f64>() / test_results.len() as f64;
    
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
    /// Whether to include matrix multiplication test
    pub include_matrix_test: bool,
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
            include_matrix_test: true,
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
