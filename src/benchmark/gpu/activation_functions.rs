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

//! Activation functions benchmark for GPU performance testing.

use crate::error::Error;
use crate::benchmark::gpu::GpuTestResult;
use std::time::{Duration, Instant};
use wgpu::{Adapter, Device, Queue};
use wgpu::util::DeviceExt;

/// Runs an activation functions benchmark on the GPU.
pub fn run_activation_functions_benchmark(
    adapter: &Adapter,
    test_duration: Duration,
    data_size: u32,
) -> Result<GpuTestResult, Error> {
    // Create device and queue
    let (device, queue) = pollster::block_on(create_device_and_queue(adapter))?;
    
    // Run the benchmark with the created device and queue
    run_activation_functions_benchmark_with_context(adapter, &device, &queue, test_duration, data_size)
}

/// Runs an activation functions benchmark on the GPU with provided device and queue.
pub fn run_activation_functions_benchmark_with_context(
    _adapter: &Adapter,
    device: &Device,
    queue: &Queue,
    test_duration: Duration,
    data_size: u32,
) -> Result<GpuTestResult, Error> {
    // Adjust data size based on available memory
    // For low-spec machines, we'll use a smaller data size
    let adjusted_data_size = std::cmp::min(data_size, 1_000_000); // Cap at 1 million elements
    
    // Create input data
    let input_data = create_random_data(adjusted_data_size);
    let result_size = (adjusted_data_size * std::mem::size_of::<f32>() as u32) as u64;
    
    // Create buffers with error handling
    let input_buffer = match device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Data Buffer"),
        contents: bytemuck::cast_slice(&input_data),
        usage: wgpu::BufferUsages::STORAGE,
    }) {
        buffer => buffer,
    };
    
    let result_buffer = match device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Result Buffer"),
        size: result_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    }) {
        buffer => buffer,
    };
    
    // Create uniform buffer for data size
    let size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Size Buffer"),
        contents: bytemuck::cast_slice(&[adjusted_data_size]),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    
    // Create bind group layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Activation Functions Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });
    
    // Create bind group
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Activation Functions Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: result_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: size_buffer.as_entire_binding(),
            },
        ],
    });
    
    // Create compute pipeline with a simpler shader for compatibility
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Activation Functions Shader"),
        source: wgpu::ShaderSource::Wgsl(ACTIVATION_FUNCTIONS_SHADER.into()),
    });
    
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Activation Functions Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Activation Functions Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
    });
    
    // Run the benchmark with adaptive timing
    let start_time = Instant::now();
    let mut frame_times = Vec::new();
    let mut iterations = 0;
    let max_iterations = 1000; // Cap iterations to prevent excessive runs on powerful machines
    
    // Adaptive workgroup size based on data size
    let workgroup_size = 64; // Smaller workgroup size for better compatibility
    let workgroup_count = (adjusted_data_size + workgroup_size - 1) / workgroup_size;
    
    while start_time.elapsed() < test_duration && iterations < max_iterations {
        let frame_start = Instant::now();
        
        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Activation Functions Encoder"),
        });
        
        // Execute compute pass
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Activation Functions Compute Pass"),
            });
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            
            // Dispatch workgroups
            compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
        }
        
        // Submit command buffer
        queue.submit(std::iter::once(encoder.finish()));
        
        // Record frame time
        let frame_time = frame_start.elapsed();
        frame_times.push(frame_time.as_secs_f64() * 1000.0); // Convert to milliseconds
        
        iterations += 1;
        
        // Add a small delay between iterations for low-spec machines
        if iterations % 10 == 0 {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    
    // Calculate results
    if iterations == 0 || frame_times.is_empty() {
        return Err(Error::Benchmark("No activation function operations were performed during the benchmark".to_string()));
    }
    
    // Calculate statistics with outlier removal for more stable results
    frame_times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    // Remove top and bottom 10% of measurements if we have enough samples
    let trim_count = if frame_times.len() >= 10 { frame_times.len() / 10 } else { 0 };
    let trimmed_times = if trim_count > 0 {
        &frame_times[trim_count..frame_times.len() - trim_count]
    } else {
        &frame_times
    };
    
    let avg_frame_time = trimmed_times.iter().sum::<f64>() / trimmed_times.len() as f64;
    let min_frame_time = *trimmed_times.first().unwrap_or(&0.0);
    let max_frame_time = *trimmed_times.last().unwrap_or(&0.0);
    
    // Convert to operations per second
    let average_ops = if avg_frame_time > 0.0 { 1000.0 / avg_frame_time } else { 0.0 };
    let min_ops = if max_frame_time > 0.0 { 1000.0 / max_frame_time } else { 0.0 };
    let max_ops = if min_frame_time > 0.0 { 1000.0 / min_frame_time } else { 0.0 };
    
    // Calculate operations per second
    // For activation functions, we're doing 4 different functions on each element
    let ops_per_second = 4.0 * (adjusted_data_size as f64) * average_ops;
    
    // Calculate score based on operations per second
    let score = ops_per_second / 1_000_000.0; // Convert to millions of operations per second
    
    Ok(GpuTestResult {
        test_name: "ActivationFunctions".to_string(),
        average_fps: average_ops,
        min_fps: min_ops,
        max_fps: max_ops,
        score,
    })
}

/// Creates a device and queue from an adapter.
async fn create_device_and_queue(adapter: &Adapter) -> Result<(Device, Queue), Error> {
    // Add a small delay to allow previous device to be released
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Request device with minimal features and relaxed limits for compatibility
    adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("GPU Benchmark Device"),
            features: wgpu::Features::empty(),
            // Use downlevel defaults for maximum compatibility
            limits: wgpu::Limits::downlevel_webgl2_defaults(),
        },
        None,
    )
    .await
    .map_err(|e| Error::Benchmark(format!("Failed to create device: {}", e)))
}

/// Creates random data of the specified size.
fn create_random_data(size: u32) -> Vec<f32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(size as usize);
    
    // Generate values in a more limited range for numerical stability
    for _ in 0..size {
        data.push(rng.gen_range(-2.0..2.0));
    }
    
    data
}

// Shader source for activation functions - simplified for compatibility
const ACTIVATION_FUNCTIONS_SHADER: &str = r#"
struct Params {
    size: u32,
};

@group(0) @binding(0) var<storage, read> input_data: array<f32>;
@group(0) @binding(1) var<storage, read_write> result: array<f32>;
@group(0) @binding(2) var<uniform> params: Params;

// ReLU activation function
fn relu(x: f32) -> f32 {
    return max(0.0, x);
}

// Sigmoid activation function - simplified for better numerical stability
fn sigmoid(x: f32) -> f32 {
    // Clamp input to avoid overflow
    let clamped_x = clamp(x, -10.0, 10.0);
    return 1.0 / (1.0 + exp(-clamped_x));
}

// Tanh activation function
fn tanh_activation(x: f32) -> f32 {
    // Clamp input to avoid overflow
    let clamped_x = clamp(x, -10.0, 10.0);
    return tanh(clamped_x);
}

// Leaky ReLU activation function
fn leaky_relu(x: f32) -> f32 {
    return select(0.01 * x, x, x > 0.0);
}

@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    
    // Check if we're within bounds
    if (idx >= params.size) {
        return;
    }
    
    let input_value = input_data[idx];
    
    // Apply all activation functions and combine results
    let relu_result = relu(input_value);
    let sigmoid_result = sigmoid(input_value);
    let tanh_result = tanh_activation(input_value);
    let leaky_relu_result = leaky_relu(input_value);
    
    // Store the combined result
    result[idx] = relu_result + sigmoid_result + tanh_result + leaky_relu_result;
}
"#;
