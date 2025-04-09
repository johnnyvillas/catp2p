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

//! Example demonstrating GPU benchmarking functionality.

use catp2p::benchmark::gpu::{
    get_gpu_info, GpuBenchmarkContext
};
use catp2p::error::Error;
use std::time::Duration;

fn main() -> Result<(), Error> {
    println!("=== CatP2P GPU Benchmarking Example ===\n");

    // Part 1: Get GPU information without performance testing
    println!("--- GPU Information ---");
    
    // Get GPU information
    match get_gpu_info() {
        Ok(gpu_info) => {
            println!("GPU Model: {}", gpu_info.name);
            println!("GPU Vendor: {}", gpu_info.vendor);
            println!("GPU Driver: {}", gpu_info.driver);
            println!("Estimated VRAM: {}", gpu_info.vram);
            println!("Backend: {}", gpu_info.backend);
            println!("Type: {}", if gpu_info.is_integrated { "Integrated" } else { "Discrete" });
            println!();
        },
        Err(e) => {
            println!("Failed to get GPU information: {}", e);
            println!("This might indicate that no compatible GPU is available.");
            return Ok(());
        }
    }

    // Create a single benchmark context for all tests
    println!("Creating GPU benchmark context...");
    let context = match GpuBenchmarkContext::new() {
        Ok(ctx) => ctx,
        Err(e) => {
            println!("Failed to create GPU benchmark context: {}", e);
            return Ok(());
        }
    };

    // Part 2: Run standard benchmark
    println!("--- GPU Performance Benchmark ---");
    println!("Running standard GPU benchmark...");
    
    // Run the standard benchmark with our context
    let matrix_size = 1024; // 1024x1024 matrix
    match context.run_matrix_mult(Duration::from_secs(5), matrix_size) {
        Ok(result) => {
            println!("Benchmark completed successfully!");
            println!("Standard GPU benchmark score: {:.2} MFLOPS", result.score);
            println!("Test type: Matrix Multiplication");
            println!("Matrix size: {}x{}", matrix_size, matrix_size);
            println!("Higher scores indicate better GPU performance.");
            println!();
        },
        Err(e) => {
            println!("Benchmark failed: {}", e);
            println!("This might be due to GPU compatibility issues or insufficient resources.");
            return Ok(());
        }
    }

    // Part 3: Run benchmarks with different complexity levels
    println!("--- GPU Scaling with Complexity ---");
    println!("Running benchmarks with different complexity levels...");
    
    let complexity_levels = [3, 5, 8];
    let mut results = Vec::new();
    
    for &complexity in &complexity_levels {
        println!("Running benchmark with complexity level {}...", complexity);
        
        // Calculate matrix size based on complexity
        let matrix_size = 512 + (complexity * 128);
        
        match context.run_matrix_mult(Duration::from_secs(2), matrix_size) {
            Ok(result) => {
                println!("  Matrix size: {}x{}", matrix_size, matrix_size);
                println!("  Compute Score: {:.2} MFLOPS", result.score);
                println!("  Average FPS: {:.2}", result.average_fps);
                
                results.push((complexity, result.score));
            },
            Err(e) => {
                println!("  Failed: {}", e);
            }
        }
    }
    
    // Part 4: Run a more intensive benchmark
    println!("\n--- Intensive GPU Benchmark ---");
    println!("Running intensive benchmark with higher complexity...");
    
    let complexity = 10;
    let matrix_size = 512 + (complexity * 128);
    
    match context.run_matrix_mult(Duration::from_secs(3), matrix_size) {
        Ok(result) => {
            println!("Intensive benchmark results:");
            println!("  Test type: Matrix Multiplication");
            println!("  Matrix size: {}x{}", matrix_size, matrix_size);
            println!("  Compute Score: {:.2} MFLOPS", result.score);
            println!("  Average FPS: {:.2}", result.average_fps);
            
            // Add to results for visualization
            results.push((complexity, result.score));
        },
        Err(e) => {
            println!("Intensive benchmark failed: {}", e);
        }
    }
    
    // Part 5: Visualize complexity scaling with ASCII chart
    if !results.is_empty() {
        println!("\n--- GPU Performance Scaling with Complexity ---");
        println!("Higher is better:");
        
        // Find max score for scaling the chart
        let max_score = results.iter().map(|&(_, score)| score as usize).max().unwrap_or(1);
        let scale = 50.0 / max_score as f64;
        
        for &(complexity, score) in &results {
            let bar_length = (score as f64 * scale) as usize;
            let bar = "#".repeat(bar_length);
            println!("Complexity {:2}: {:10.2} MFLOPS |{}|", complexity, score, bar);
        }
    }
    
    // Part 6: Summary
    println!("\n--- GPU Benchmark Summary ---");
    println!("GPU benchmarking completed!");
    
    println!("Tested GPU: {} ({})", context.gpu_info.name, 
             if context.gpu_info.is_integrated { "Integrated" } else { "Discrete" });
    
    if !results.is_empty() {
        let avg_score = results.iter().map(|&(_, score)| score).sum::<f64>() / results.len() as f64;
        println!("Average performance score: {:.2} MFLOPS", avg_score);
        
        if let Some(&(_, max_score)) = results.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)) {
            println!("Peak performance score: {:.2} MFLOPS", max_score);
        }
    }
    
    Ok(())
}
