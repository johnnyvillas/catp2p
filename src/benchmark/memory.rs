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

//! Memory benchmarking functionality.

use crate::error::Error;
use std::time::{Duration, Instant};

/// Runs a memory benchmark and returns a score.
pub fn run_memory_benchmark() -> Result<f64, Error> {
    // Run the benchmark
    let start_time = Instant::now();
    
    // Allocate and write to memory
    let memory_size = 1024 * 1024 * 100; // 100 MB
    let mut data = Vec::with_capacity(memory_size);
    
    // Fill the vector with data
    for i in 0..memory_size {
        data.push((i % 256) as u8);
    }
    
    // Read from memory
    let mut sum = 0;
    for &byte in &data {
        sum = sum.wrapping_add(byte as u64);
    }
    
    // Prevent the compiler from optimizing away the calculation
    if sum == 42 {
        println!("The answer to life, the universe, and everything!");
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate the score based on the elapsed time
    // Lower time is better, so we invert it
    let score = 1000.0 / elapsed.as_secs_f64();
    
    Ok(score)
}

/// Runs a memory allocation benchmark.
pub fn run_memory_allocation_benchmark(size_mb: usize) -> Result<Duration, Error> {
    let size = size_mb * 1024 * 1024;
    
    let start_time = Instant::now();
    
    // Allocate memory
    let mut data = Vec::with_capacity(size);
    
    // Fill the vector with data
    for i in 0..size {
        data.push((i % 256) as u8);
    }
    
    let elapsed = start_time.elapsed();
    
    // Prevent the compiler from optimizing away the allocation
    if data.len() != size {
        return Err(Error::Benchmark("Memory allocation failed".to_string()));
    }
    
    Ok(elapsed)
}

/// Runs a memory bandwidth benchmark.
pub fn run_memory_bandwidth_benchmark(size_mb: usize, iterations: usize) -> Result<(Duration, f64), Error> {
    let size = size_mb * 1024 * 1024;
    
    // Allocate memory
    let mut src = Vec::with_capacity(size);
    let mut dst = Vec::with_capacity(size);
    
    // Fill the source vector with data
    for i in 0..size {
        src.push((i % 256) as u8);
        dst.push(0);
    }
    
    let start_time = Instant::now();
    
    // Copy data between vectors
    for _ in 0..iterations {
        dst.copy_from_slice(&src);
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate bandwidth in MB/s
    let total_bytes = size * iterations;
    let seconds = elapsed.as_secs_f64();
    let bandwidth = (total_bytes as f64) / seconds / (1024.0 * 1024.0);
    
    // Prevent the compiler from optimizing away the copy
    if dst[0] != src[0] {
        return Err(Error::Benchmark("Memory copy failed".to_string()));
    }
    
    Ok((elapsed, bandwidth))
}
