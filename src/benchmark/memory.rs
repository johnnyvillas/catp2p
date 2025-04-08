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
use std::time::Instant; // Removed unused Duration import

/// Runs a memory benchmark and returns a score.
pub fn run_memory_benchmark() -> Result<f64, Error> {
    // Run the benchmark
    let start_time = Instant::now();
    
    // Allocation benchmark
    let allocation_score = run_allocation_benchmark()?;
    
    // Read/write benchmark
    let read_write_score = run_read_write_benchmark()?;
    
    // Random access benchmark
    let random_access_score = run_random_access_benchmark()?;
    
    let elapsed = start_time.elapsed();
    
    // Calculate the score based on the individual benchmarks
    let score = (allocation_score + read_write_score + random_access_score) / 3.0;
    
    println!("Memory benchmark completed in {:?}", elapsed);
    
    Ok(score)
}

/// Runs a memory allocation benchmark.
fn run_allocation_benchmark() -> Result<f64, Error> {
    let start_time = Instant::now();
    
    // Allocate and deallocate memory in different sizes
    let iterations = 1000;
    let sizes = [
        1024,           // 1 KB
        10 * 1024,      // 10 KB
        100 * 1024,     // 100 KB
        1024 * 1024,    // 1 MB
        10 * 1024 * 1024, // 10 MB
    ];
    
    for _ in 0..iterations {
        for &size in &sizes {
            // Allocate memory
            let data = vec![0u8; size];
            
            // Prevent the compiler from optimizing away the allocation
            if data[0] == 42 {
                println!("Found the answer to life, the universe, and everything!");
            }
        }
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate score based on time (lower is better)
    let score = 1000.0 / elapsed.as_secs_f64();
    
    Ok(score)
}

/// Runs a memory read/write benchmark.
fn run_read_write_benchmark() -> Result<f64, Error> {
    let start_time = Instant::now();
    
    // Allocate a large buffer
    let size = 100 * 1024 * 1024; // 100 MB
    let mut data = vec![0u8; size];
    
    // Write to memory
    for i in 0..size {
        data[i] = (i % 256) as u8;
    }
    
    // Read from memory
    let mut sum = 0;
    for i in 0..size {
        sum += data[i] as usize;
    }
    
    // Prevent the compiler from optimizing away the calculation
    if sum == 42 {
        println!("Found the answer to life, the universe, and everything!");
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate score based on time (lower is better)
    let score = 1000.0 / elapsed.as_secs_f64();
    
    Ok(score)
}

/// Runs a memory random access benchmark.
fn run_random_access_benchmark() -> Result<f64, Error> {
    let start_time = Instant::now();
    
    // Allocate a large buffer
    let size = 100 * 1024 * 1024; // 100 MB
    let mut data = vec![0u8; size];
    
    // Initialize with some data
    for i in 0..size {
        data[i] = (i % 256) as u8;
    }
    
    // Generate random indices
    let num_accesses = 10_000_000;
    let mut indices = Vec::with_capacity(num_accesses);
    for _ in 0..num_accesses {
        indices.push(rand::random::<usize>() % size);
    }
    
    // Perform random accesses
    let mut sum = 0;
    for &idx in &indices {
        sum += data[idx] as usize;
    }
    
    // Prevent the compiler from optimizing away the calculation
    if sum == 42 {
        println!("Found the answer to life, the universe, and everything!");
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate score based on time (lower is better)
    let score = 1000.0 / elapsed.as_secs_f64();
    
    Ok(score)
}
