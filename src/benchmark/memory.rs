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
use std::time::Instant;
use sysinfo::{System, SystemExt};
use serde::{Deserialize, Serialize};

/// Memory information structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Total physical memory in bytes.
    pub total_memory: u64,
    /// Available memory in bytes.
    pub available_memory: u64,
    /// Used memory in bytes.
    pub used_memory: u64,
    /// Memory usage as a percentage (0.0 - 100.0).
    pub usage_percent: f64,
    /// Memory per CPU core in bytes.
    pub memory_per_core: u64,
}

/// Gets detailed information about the system's memory.
pub fn get_memory_info() -> Result<MemoryInfo, Error> {
    let mut system = System::new_all();
    system.refresh_all();
    
    let total_memory = system.total_memory();
    let available_memory = system.available_memory();
    let used_memory = total_memory - available_memory;
    let usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
    
    // Get CPU core count for memory-per-core calculation
    let cpu_cores = system.cpus().len() as u64;
    let memory_per_core = if cpu_cores > 0 {
        total_memory / cpu_cores
    } else {
        0
    };
    
    Ok(MemoryInfo {
        total_memory,
        available_memory,
        used_memory,
        usage_percent,
        memory_per_core,
    })
}

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
pub fn run_allocation_benchmark() -> Result<f64, Error> {
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
pub fn run_read_write_benchmark() -> Result<f64, Error> {
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
pub fn run_random_access_benchmark() -> Result<f64, Error> {
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
