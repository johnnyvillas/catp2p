//! CPU benchmarking functionality.

use crate::error::Error;
use std::time::{Duration, Instant};
use rayon::prelude::*;

/// Runs a CPU benchmark and returns a score.
pub fn run_cpu_benchmark() -> Result<f64, Error> {
    // Get the number of CPU cores
    let num_cores = rayon::current_num_threads();
    
    // Run the benchmark
    let start_time = Instant::now();
    
    // Create a parallel iterator to utilize all cores
    let result: Result<Vec<u64>, Error> = (0..num_cores).into_par_iter().map(|core_id| {
        // Run a CPU-intensive task on each core
        let mut sum = 0;
        for i in 0..10_000_000 {
            sum = sum.wrapping_add(i);
            // Add some complexity to prevent optimization
            if i % 1000 == 0 {
                sum = sum.wrapping_mul(i);
            }
        }
        Ok(sum)
    }).collect();
    
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

/// Runs a single-threaded CPU benchmark for a specific core.
pub fn run_single_core_benchmark(iterations: u64) -> Result<Duration, Error> {
    let start_time = Instant::now();
    
    let mut sum = 0;
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
pub fn run_multi_core_benchmark(threads: usize, iterations_per_thread: u64) -> Result<Duration, Error> {
    let start_time = Instant::now();
    
    // Create a thread pool with the specified number of threads
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .map_err(|e| Error::Benchmark(format!("Failed to create thread pool: {}", e)))?;
    
    // Run the benchmark on the thread pool
    pool.install(|| {
        (0..threads).into_par_iter().for_each(|_| {
            let mut sum = 0;
            for i in 0..iterations_per_thread {
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
        });
    });
    
    let elapsed = start_time.elapsed();
    
    Ok(elapsed)
}
