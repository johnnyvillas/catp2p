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

//! Gradient calculation benchmark for GPU performance testing.

use crate::error::Error;
use crate::benchmark::gpu::{GpuTestResult, GRADIENT_REFERENCE};
use std::time::{Duration, Instant};
use wgpu::{Adapter, Device, Queue};
use wgpu::util::DeviceExt; // For create_buffer_init

/// Runs a gradient calculation benchmark on the GPU.
pub fn run_gradient_calc_benchmark(
    adapter: &Adapter,
    test_duration: Duration,
    data_size: u32,
) -> Result<GpuTestResult, Error> {
    // Create device and queue
    let (device, queue) = pollster::block_on(create_device_and_queue(adapter))?;
    
    // Run the benchmark with the created device and queue
    run_gradient_calc_benchmark_with_context(adapter, &device, &queue, test_duration, data_size)
}

/// Runs a gradient calculation benchmark on the GPU with provided device and queue.
pub fn run_gradient_calc_benchmark_with_context(
    _adapter: &Adapter,
    device: &Device,
    queue: &Queue,
    test_duration: Duration,
    data_size: u32,
) -> Result<GpuTestResult, Error> {
    // Create input data (weights, activations, and targets)
    let weights = create_random_data(data_size);
    let activations = create_random_data(data_size);
    let targets = create_random_data(data_size);
    
    let buffer_size = (data_size * std::mem::size_of::<f32>() as u32) as u64;
    
    // Create buffers
    let buffer_weights = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Weights Buffer"),
        contents: bytemuck::cast_slice(&weights),
        usage: wgpu::BufferUsages::STORAGE,
    });
    
    let buffer_activations = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Activations Buffer"),
        contents: bytemuck::cast_slice(&activations),
        usage: wgpu::BufferUsages::STORAGE,
    });
    
    let buffer_targets = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Targets Buffer"),
        contents: bytemuck::cast_slice(&targets),
        usage: wgpu::BufferUsages::STORAGE,
    });
    
    let buffer_gradients = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Gradients Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    
    // Create uniform buffer for data size
    let size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Size Buffer"),
        contents: bytemuck::cast_slice(&[data_size]),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    
    // Create bind group layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Gradient Bind Group Layout"),
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
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
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
        label: Some("Gradient Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer_weights.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: buffer_activations.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: buffer_targets.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: buffer_gradients.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: size_buffer.as_entire_binding(),
            },
        ],
    });
    
    // Create compute pipeline
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Gradient Calculation Shader"),
        source: wgpu::ShaderSource::Wgsl(GRADIENT_CALC_SHADER.into()),
    });
    
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Gradient Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Gradient Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
    });
    
    // Run the benchmark
    let start_time = Instant::now();
    let mut frame_times = Vec::new();
    let mut iterations = 0;
    
    while start_time.elapsed() < test_duration {
        let frame_start = Instant::now();
        
        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Gradient Encoder"),
        });
        
        // Execute compute pass
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Gradient Compute Pass"),
            });
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            
            // Dispatch workgroups
            let workgroup_size = 256; // Must match the shader
            let workgroup_count = (data_size + workgroup_size - 1) / workgroup_size;
            compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
        }
        
        // Submit command buffer
        queue.submit(std::iter::once(encoder.finish()));
        
        // Record frame time
        let frame_time = frame_start.elapsed();
        frame_times.push(frame_time.as_secs_f64() * 1000.0); // Convert to milliseconds
        
        iterations += 1;
    }
    
    // Calculate results
    if iterations == 0 || frame_times.is_empty() {
        return Err(Error::Benchmark("No gradient calculations were performed during the benchmark".to_string()));
    }
    
    // Calculate statistics
    let avg_frame_time = frame_times.iter().sum::<f64>() / frame_times.len() as f64;
    let min_frame_time = *frame_times.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&0.0);
    let max_frame_time = *frame_times.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&0.0);
    
    // Convert to operations per second
    let average_ops = if avg_frame_time > 0.0 { 1000.0 / avg_frame_time } else { 0.0 };
    let min_ops = if max_frame_time > 0.0 { 1000.0 / max_frame_time } else { 0.0 };
    let max_ops = if min_frame_time > 0.0 { 1000.0 / min_frame_time } else { 0.0 };
    
    // Calculate FLOPS (floating point operations per second)
    // For gradient calculation, we estimate about 10 operations per element
    let operations_per_element = 10.0;
    let flops = operations_per_element * (data_size as f64) * average_ops;
    
    // Calculate raw score based on FLOPS
    let raw_score = flops / 1_000_000.0; // Convert to MFLOPS for a more readable score
    
    // Calculate normalized score (0-100000 scale)
    let normalized_score = GpuTestResult::normalize_score(raw_score, GRADIENT_REFERENCE);
    
    Ok(GpuTestResult {
        test_name: "GradientCalculation".to_string(),
        average_fps: average_ops,
        min_fps: min_ops,
        max_fps: max_ops,
        score: raw_score,
        normalized_score,
    })
}

/// Creates a device and queue from an adapter.
async fn create_device_and_queue(adapter: &Adapter) -> Result<(Device, Queue), Error> {
    // Add a small delay to allow previous device to be released
    std::thread::sleep(std::time::Duration::from_millis(100));
    
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
}

/// Creates random data of the specified size.
fn create_random_data(size: u32) -> Vec<f32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(size as usize);
    
    for _ in 0..size {
        data.push(rng.gen_range(-1.0..1.0));
    }
    
    data
}

// Shader source for gradient calculation
const GRADIENT_CALC_SHADER: &str = r#"
struct Params {
    size: u32,
};

@group(0) @binding(0) var<storage, read> weights: array<f32>;
@group(0) @binding(1) var<storage, read> activations: array<f32>;
@group(0) @binding(2) var<storage, read> targets: array<f32>;
@group(0) @binding(3) var<storage, read_write> gradients: array<f32>;
@group(0) @binding(4) var<uniform> params: Params;

// Sigmoid function
fn sigmoid(x: f32) -> f32 {
    return 1.0 / (1.0 + exp(-x));
}

// Derivative of sigmoid
fn sigmoid_derivative(x: f32) -> f32 {
    let s = sigmoid(x);
    return s * (1.0 - s);
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    
    // Check if we're within bounds
    if (idx >= params.size) {
        return;
    }
    
    // Get the values for this element
    let weight = weights[idx];
    let activation = activations[idx];
    let target_value = targets[idx]; // Changed from "target" to "target_value"
    
    // Calculate weighted input
    let weighted_input = weight * activation;
    
    // Calculate output using sigmoid
    let output = sigmoid(weighted_input);
    
    // Calculate error (output - target)
    let error = output - target_value; // Use target_value instead of target
    
    // Calculate gradient using chain rule
    // dE/dW = dE/dO * dO/dI * dI/dW
    // where E is error, O is output, I is input, W is weight
    let dE_dO = error;  // derivative of error with respect to output
    let dO_dI = sigmoid_derivative(weighted_input);  // derivative of sigmoid
    let dI_dW = activation;  // derivative of weighted input with respect to weight
    
    // Combine using chain rule
    let gradient = dE_dO * dO_dI * dI_dW;
    
    // Store the result
    gradients[idx] = gradient;
}
"#;
