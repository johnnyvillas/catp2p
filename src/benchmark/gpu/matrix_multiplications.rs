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

//! Matrix multiplication benchmark for GPU performance testing.

use crate::error::Error;
use crate::benchmark::gpu::GpuTestResult;
use std::time::{Duration, Instant};
use wgpu::{Adapter, Device, Queue};
use wgpu::util::DeviceExt; // Add this import for create_buffer_init

/// Runs a matrix multiplication benchmark on the GPU.
pub fn run_matrix_mult_benchmark(
    adapter: &Adapter,
    test_duration: Duration,
    matrix_size: u32,
) -> Result<GpuTestResult, Error> {
    // Create device and queue
    let (device, queue) = pollster::block_on(create_device_and_queue(adapter))?;
    
    // Run the benchmark with the created device and queue
    run_matrix_mult_benchmark_with_context(adapter, &device, &queue, test_duration, matrix_size)
}

/// Runs a matrix multiplication benchmark on the GPU with provided device and queue.
pub fn run_matrix_mult_benchmark_with_context(
    _adapter: &Adapter,
    device: &Device,
    queue: &Queue,
    test_duration: Duration,
    matrix_size: u32,
) -> Result<GpuTestResult, Error> {
    // Create matrices
    let matrix_a = create_random_matrix(matrix_size);
    let matrix_b = create_random_matrix(matrix_size);
    let result_size = (matrix_size * matrix_size * std::mem::size_of::<f32>() as u32) as u64;
    
    // Create buffers
    let buffer_a = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Matrix A Buffer"),
        contents: bytemuck::cast_slice(&matrix_a),
        usage: wgpu::BufferUsages::STORAGE,
    });
    
    let buffer_b = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Matrix B Buffer"),
        contents: bytemuck::cast_slice(&matrix_b),
        usage: wgpu::BufferUsages::STORAGE,
    });
    
    let buffer_result = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Result Buffer"),
        size: result_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    
    // Create uniform buffer for matrix size
    let size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Size Buffer"),
        contents: bytemuck::cast_slice(&[matrix_size]),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    
    // Create bind group layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Matrix Bind Group Layout"),
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
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
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
        label: Some("Matrix Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer_a.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: buffer_b.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: buffer_result.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: size_buffer.as_entire_binding(),
            },
        ],
    });
    
    // Create compute pipeline
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Matrix Multiplication Shader"),
        source: wgpu::ShaderSource::Wgsl(MATRIX_MULT_SHADER.into()),
    });
    
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Matrix Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Matrix Compute Pipeline"),
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
            label: Some("Matrix Encoder"),
        });
        
        // Execute compute pass
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Matrix Compute Pass"),
            });
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            
            // Dispatch workgroups
            let workgroup_size = 16; // Must match the shader
            let workgroup_count = (matrix_size + workgroup_size - 1) / workgroup_size;
            compute_pass.dispatch_workgroups(workgroup_count, workgroup_count, 1);
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
        return Err(Error::Benchmark("No matrix multiplications were performed during the benchmark".to_string()));
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
    // For matrix multiplication, the number of operations is 2 * N^3
    let flops = 2.0 * (matrix_size as f64).powi(3) * average_ops;
    
    // Calculate score based on FLOPS
    let score = flops / 1_000_000.0; // Convert to MFLOPS for a more readable score
    
    Ok(GpuTestResult {
        test_name: "MatrixMultiplication".to_string(),
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

/// Creates a random matrix of the specified size.
fn create_random_matrix(size: u32) -> Vec<f32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut matrix = Vec::with_capacity((size * size) as usize);
    
    for _ in 0..(size * size) {
        matrix.push(rng.gen_range(-1.0..1.0));
    }
    
    matrix
}

// Shader source for matrix multiplication
const MATRIX_MULT_SHADER: &str = r#"
struct Params {
    size: u32,
};

@group(0) @binding(0) var<storage, read> matrix_a: array<f32>;
@group(0) @binding(1) var<storage, read> matrix_b: array<f32>;
@group(0) @binding(2) var<storage, read_write> result: array<f32>;
@group(0) @binding(3) var<uniform> params: Params;

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let size = params.size;
    let row = global_id.x;
    let col = global_id.y;
    
    // Check if we're within the matrix bounds
    if (row >= size || col >= size) {
        return;
    }
    
    // Compute the dot product of row from A and column from B
    var sum: f32 = 0.0;
    for (var i: u32 = 0u; i < size; i = i + 1u) {
        let a_idx = row * size + i;
        let b_idx = i * size + col;
        sum = sum + matrix_a[a_idx] * matrix_b[b_idx];
    }
    
    // Store the result
    let result_idx = row * size + col;
    result[result_idx] = sum;
}
"#;