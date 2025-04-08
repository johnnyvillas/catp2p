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
use std::time::{Duration, Instant};

#[cfg(feature = "gpu")]
use wgpu;

/// Runs a GPU benchmark and returns a score.
pub async fn run_gpu_benchmark() -> Result<Option<f64>, Error> {
    #[cfg(feature = "gpu")]
    {
        // Initialize GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        // Try to find a suitable GPU adapter
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            },
        ).await;
        
        // If no adapter is found, return None
        let adapter = match adapter {
            Some(adapter) => adapter,
            None => return Ok(None),
        };
        
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
        
        // Create a compute pipeline
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(r#"
                @group(0) @binding(0)
                var<storage, read_write> output: array<u32>;

                @compute @workgroup_size(64)
                fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                    let index = global_id.x;
                    if (index >= 1000000) {
                        return;
                    }
                    
                    var sum: u32 = 0;
                    for (var i: u32 = 0; i < 1000; i = i + 1) {
                        sum = sum + i;
                    }
                    
                    output[index] = sum;
                }
            "#)),
        });
        
        // Create a buffer to store the results
        let buffer_size = 1000000 * std::mem::size_of::<u32>() as u64;
        let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Storage Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        
        // Create a buffer to read back the results
        let result_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Result Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        
        // Create a bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        // Create a bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: storage_buffer.as_entire_binding(),
                },
            ],
        });
        
        // Create a pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        
        // Create a compute pipeline
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
        });
        
        // Create a command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        });
        
        // Dispatch the compute shader
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
            });
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(1000000 / 64 + 1, 1, 1);
        }
        
        // Copy the results to the result buffer
        encoder.copy_buffer_to_buffer(&storage_buffer, 0, &result_buffer, 0, buffer_size);
        
        // Submit the commands
        queue.submit(std::iter::once(encoder.finish()));
        
        // Read back the results
        let buffer_slice = result_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        
        // Wait for the GPU to finish
        device.poll(wgpu::Maintain::Wait);
        
        // Check the result
        if let Ok(Ok(())) = receiver.receive().await {
            let data = buffer_slice.get_mapped_range();
            let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
            drop(data);
            result_buffer.unmap();
            
            // Calculate the elapsed time
            let elapsed = start_time.elapsed();
            
            // Calculate the score based on the elapsed time
            // Lower time is better, so we invert it
            let score = 1000.0 / elapsed.as_secs_f64();
            
            Ok(Some(score))
        } else {
            Err(Error::Benchmark("Failed to read GPU results".to_string()))
        }
    }
    
    #[cfg(not(feature = "gpu"))]
    {
        // GPU support is not enabled
        Ok(None)
    }
}
