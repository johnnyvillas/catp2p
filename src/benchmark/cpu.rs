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

//! CPU benchmarking functionality.

use crate::error::Error;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, CpuExt};

/// CPU information structure containing details about the processor.
#[derive(Debug, Clone)]
pub struct CpuInfo {
    /// The name/model of the CPU.
    pub name: String,
    /// The number of physical cores.
    pub cores: usize,
    /// The number of logical processors (including hyperthreading).
    pub logical_cores: usize,
    /// Current CPU usage as a percentage (0-100).
    pub usage: f32,
    /// CPU vendor (e.g., "Intel", "AMD").
    pub vendor: String,
    /// CPU frequency in MHz, if available.
    pub frequency: Option<u64>,
}

/// Gets detailed information about the CPU.
pub fn get_cpu_info() -> Result<CpuInfo, Error> {
    let mut system = System::new_all();
    system.refresh_all();
    
    let cpus = system.cpus();
    if cpus.is_empty() {
        return Err(Error::Benchmark("Failed to retrieve CPU information".to_string()));
    }
    
    // Get CPU name from the first core (all cores should have the same name)
    let name = cpus[0].brand().to_string();
    
    // Get vendor information
    let vendor = cpus[0].vendor_id().to_string();
    
    // Get frequency if available
    let frequency = cpus[0].frequency();
    
    // Calculate average CPU usage across all cores
    let usage = system.global_cpu_info().cpu_usage();
    
    // Get the number of physical and logical cores
    let logical_cores = cpus.len();
    
    // Note: Getting the number of physical cores (without hyperthreading) 
    // is OS-specific and not directly available in sysinfo.
    // For simplicity, we'll use logical_cores for now.
    let cores = logical_cores;
    
    Ok(CpuInfo {
        name,
        cores,
        logical_cores,
        usage,
        vendor,
        frequency: Some(frequency),
    })
}

/// Runs a CPU benchmark and returns a score.
///
/// The score is calculated based on how quickly the CPU can perform a series of
/// computations across all available cores. Higher scores indicate better performance.
pub fn run_cpu_benchmark() -> Result<f64, Error> {
    // Get the number of CPU cores
    let num_cores = rayon::current_num_threads();

    // Run the benchmark
    let start_time = Instant::now();

    // Create a parallel iterator to utilize all cores
    let result: Result<Vec<u64>, Error> = (0..num_cores)
        .into_par_iter()
        .map(|_core_id| {
            // Run a CPU-intensive task on each core
            let mut sum: u64 = 0;
            for i in 0..10_000_000 {
                sum = sum.wrapping_add(i);
                // Add some complexity to prevent optimization
                if i % 1000 == 0 {
                    sum = sum.wrapping_mul(i);
                }
            }
            Ok(sum)
        })
        .collect();

    // Check for errors
    result?;

    let elapsed = start_time.elapsed();

    // Calculate the score based on the elapsed time and number of cores
    // Lower time is better, so we invert it
    let base_score = 1000.0 / elapsed.as_secs_f64();

    // Scale the score based on the number of cores
    let score = base_score * (num_cores as f64).sqrt();

    Ok(score)
}

/// Runs a single-threaded CPU benchmark for a specific number of iterations.
///
/// This function measures how quickly a single CPU core can perform a series of
/// computations. The returned duration represents the time taken to complete the task.
pub fn run_single_core_benchmark(iterations: u64) -> Result<Duration, Error> {
    let start_time = Instant::now();

    let mut sum: u64 = 0;
    for i in 0..iterations {
        sum = sum.wrapping_add(i);
        // Add some complexity to prevent optimization
        if i % 1000 == 0 {
            sum = sum.wrapping_mul(i);
        }
    }

    // Prevent the compiler from optimizing away the calculation
    if sum == 42 {
        println!("The answer to life, the universe, and everything!");
    }

    let elapsed = start_time.elapsed();

    Ok(elapsed)
}

/// Runs a multi-threaded CPU benchmark with the specified number of threads.
///
/// This function measures how well the CPU performs with parallel workloads.
/// It distributes a fixed amount of work across the specified number of threads
/// and measures the time taken to complete all tasks.
pub fn run_multi_core_benchmark(
    threads: usize,
    iterations_per_thread: u64,
) -> Result<Duration, Error> {
    // Calculate total iterations based on threads and iterations per thread
    let total_iterations = iterations_per_thread * threads as u64;
    
    // Create a thread pool with the specified number of threads
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .map_err(|e| Error::Benchmark(format!("Failed to create thread pool: {}", e)))?;
    
    // Use an atomic counter to ensure work is evenly distributed
    let counter = Arc::new(AtomicU64::new(0));
    let chunk_size = 10_000; // Process in smaller chunks for better distribution
    
    let start_time = Instant::now();
    
    // Run the benchmark on the thread pool
    pool.install(|| {
        (0..threads).into_par_iter().for_each(|_| {
            let counter = Arc::clone(&counter);
            let mut local_sum: u64 = 0;
            
            loop {
                // Get the next chunk of work
                let start = counter.fetch_add(chunk_size, Ordering::Relaxed);
                if start >= total_iterations {
                    break;
                }
                
                // Calculate the end of this chunk (capped at total_iterations)
                let end = (start + chunk_size).min(total_iterations);
                
                // Process this chunk with CPU-intensive operations
                for i in start..end {
                    local_sum = local_sum.wrapping_add(i);
                    
                    // Add more complex operations that benefit from parallelism
                    if i % 100 == 0 {
                        local_sum = local_sum.wrapping_mul(i.wrapping_add(1));
                        for _ in 0..10 {
                            local_sum = local_sum.wrapping_add(local_sum.wrapping_mul(7));
                        }
                    }
                }
                
                // If we've reached the end, break
                if end == total_iterations {
                    break;
                }
            }
            
            // Prevent the compiler from optimizing away the calculation
            if local_sum == 42 {
                println!("The answer to life, the universe, and everything!");
            }
        });
    });
    
    let elapsed = start_time.elapsed();
    
    Ok(elapsed)
}

/// Runs a CPU benchmark that tests floating-point performance.
///
/// This function measures how quickly the CPU can perform floating-point
/// operations, which are common in scientific computing and graphics.
pub fn run_floating_point_benchmark(iterations: u64) -> Result<Duration, Error> {
    let start_time = Instant::now();
    
    let mut result: f64 = 1.0;
    for i in 1..iterations {
        let i_f64 = i as f64;
        // Mix of operations: addition, multiplication, division, and square root
        result += i_f64 / 1000.0;
        result *= 1.0 + (i_f64 / 1_000_000.0);
        if i % 1000 == 0 {
            result = result.sqrt() * 10.0;
        }
    }
    
    // Prevent the compiler from optimizing away the calculation
    if result == 42.0 {
        println!("The answer to life, the universe, and everything!");
    }
    
    let elapsed = start_time.elapsed();
    
    Ok(elapsed)
}

/// Runs multiple iterations of a benchmark and returns the average duration.
///
/// This function helps get more consistent benchmark results by running
/// the benchmark multiple times and averaging the results.
pub fn run_averaged_benchmark<F>(iterations: usize, benchmark_fn: F) -> Result<Duration, Error>
where
    F: Fn() -> Result<Duration, Error>,
{
    if iterations == 0 {
        return Err(Error::Benchmark("Iterations must be greater than 0".to_string()));
    }
    
    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..iterations {
        let duration = benchmark_fn()?;
        total_duration += duration;
    }
    
    // Calculate the average duration
    let nanos = total_duration.as_nanos() / iterations as u128;
    let average_duration = Duration::from_nanos(nanos as u64);
    
    Ok(average_duration)
}
