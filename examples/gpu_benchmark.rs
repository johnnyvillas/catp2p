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
use std::time::{Duration, Instant};
use std::io::{self, Write};
use colored::*;

fn main() -> Result<(), Error> {
    // Enable colored output
    colored::control::set_override(true);

    println!("{}\n", "=== CatP2P GPU Benchmarking Example ===".bright_green().bold());

    // Part 1: Get GPU information without performance testing
    println!("{}", "--- GPU Information ---".yellow().bold());
    
    // Get GPU information
    let gpu_info_start = Instant::now();
    match get_gpu_info() {
        Ok(gpu_info) => {
            let gpu_info_time = gpu_info_start.elapsed();
            println!("{} {}", "GPU Model:".cyan(), gpu_info.name.white());
            println!("{} {}", "GPU Vendor:".cyan(), gpu_info.vendor.white());
            println!("{} {}", "GPU Driver:".cyan(), gpu_info.driver.white());
            println!("{} {}", "Estimated VRAM:".cyan(), gpu_info.vram.white());
            println!("{} {}", "Backend:".cyan(), gpu_info.backend.white());
            println!("{} {}", "Type:".cyan(), 
                if gpu_info.is_integrated { "Integrated".white() } else { "Discrete".white() });
            println!("{} {:.2?}", "Time to query GPU info:".cyan(), gpu_info_time);
            println!();
        },
        Err(e) => {
            println!("{}: {}", "Failed to get GPU information".red().bold(), e);
            println!("{}", "This might indicate that no compatible GPU is available.".yellow());
            return Ok(());
        }
    }

    // Create a single benchmark context for all tests
    println!("{}", "Creating GPU benchmark context...".yellow());
    let context_start = Instant::now();
    let context = match GpuBenchmarkContext::new() {
        Ok(ctx) => {
            let context_time = context_start.elapsed();
            println!("{} {:.2?}", "Context created in".green(), context_time);
            ctx
        },
        Err(e) => {
            println!("{}: {}", "Failed to create GPU benchmark context".red().bold(), e);
            return Ok(());
        }
    };

    // Part 2: Run standard matrix multiplication benchmark
    println!("\n{}", "--- GPU Matrix Multiplication Benchmark ---".yellow().bold());
    print!("{}", "Running matrix multiplication benchmark... ".blue());
    io::stdout().flush().unwrap();
    
    // Run the standard benchmark with our context
    let matrix_size = 1024; // 1024x1024 matrix
    let benchmark_start = Instant::now();
    match context.run_matrix_mult(Duration::from_secs(5), matrix_size) {
        Ok(result) => {
            let benchmark_time = benchmark_start.elapsed();
            println!("{} (took {:.2?})", "Done!".green().bold(), benchmark_time);
            println!("{} ({}x{})", "Test: Matrix Multiplication".cyan(), matrix_size, matrix_size);
            println!("{} {:.2} MFLOPS", "Score:".bright_green(), result.score);
            println!("{} {:.2}", "Average FPS:".cyan(), result.average_fps);
            println!("{} {:.2}/{:.2}", "Min/Max FPS:".cyan(), result.min_fps, result.max_fps);
            println!();
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
            println!("{}", "This might be due to GPU compatibility issues or insufficient resources.".yellow());
            return Ok(());
        }
    }

    // Part 3: Run benchmarks with different complexity levels
    println!("{}", "--- GPU Scaling with Complexity ---".yellow().bold());
    println!("{}", "Running benchmarks with different complexity levels...".blue());
    
    let complexity_levels = [3, 5, 8];
    let mut results = Vec::new();
    
    for &complexity in &complexity_levels {
        print!("{} {}: ", "Complexity level".blue(), complexity.to_string().white());
        io::stdout().flush().unwrap();
        
        // Calculate matrix size based on complexity
        let matrix_size = 512 + (complexity * 128);
        
        let benchmark_start = Instant::now();
        match context.run_matrix_mult(Duration::from_secs(2), matrix_size) {
            Ok(result) => {
                let benchmark_time = benchmark_start.elapsed();
                println!("{} (took {:.2?})", "Done!".green().bold(), benchmark_time);
                println!("  {}: {}x{}", "Matrix size".cyan(), matrix_size, matrix_size);
                println!("  {}: {:.2} MFLOPS", "Score".bright_green(), result.score);
                println!("  {}: {:.2}", "Average FPS".cyan(), result.average_fps);
                
                results.push((complexity, result.score));
            },
            Err(e) => {
                println!("{}: {}", "Failed".red().bold(), e);
            }
        }
    }
    
    // Part 4: Run a more intensive benchmark
    println!("\n{}", "--- Intensive GPU Benchmark ---".yellow().bold());
    print!("{}", "Running intensive benchmark with complexity level 10... ".blue());
    io::stdout().flush().unwrap();
    
    let complexity = 10;
    let matrix_size = 512 + (complexity * 128);
    
    let benchmark_start = Instant::now();
    match context.run_matrix_mult(Duration::from_secs(3), matrix_size) {
        Ok(result) => {
            let benchmark_time = benchmark_start.elapsed();
            println!("{} (took {:.2?})", "Done!".green().bold(), benchmark_time);
            println!("{} ({}x{})", "Test: Matrix Multiplication".cyan(), matrix_size, matrix_size);
            println!("{} {:.2} MFLOPS", "Score:".bright_green(), result.score);
            println!("{} {:.2}", "Average FPS:".cyan(), result.average_fps);
            
            // Add to results for visualization
            results.push((complexity, result.score));
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
        }
    }
    
    // Part 5: Visualize complexity scaling with ASCII chart
    if !results.is_empty() {
        println!("\n{}", "--- GPU Performance Scaling with Complexity ---".yellow().bold());
        println!("{}", "Higher is better:".cyan().italic());
        
        // Find max score for scaling the chart
        let max_score = results.iter().map(|&(_, score)| score as usize).max().unwrap_or(1);
        let scale = 50.0 / max_score as f64;
        
        for &(complexity, score) in &results {
            let bar_length = (score as f64 * scale) as usize;
            let bar = "#".repeat(bar_length);
            println!("Complexity {:2}: {:10.2} MFLOPS |{}|", 
                     complexity.to_string().white(), 
                     score.to_string().bright_green(), 
                     bar.bright_cyan());
        }
    }
    
    // Part 6: Run activation functions benchmark
    println!("\n{}", "--- Neural Network Activation Functions Benchmark ---".yellow().bold());
    
    // Use a medium data size for the benchmark
    let data_size = 1_000_000; // 1 million elements
    let test_duration = Duration::from_secs(2);
    
    // Show progress indicators
    print!("{} ({} elements)... ", "Preparing benchmark data".blue(), data_size);
    io::stdout().flush().unwrap();
    
    let prep_start = Instant::now();
    // Preparation is actually done inside the benchmark function
    let prep_time = prep_start.elapsed();
    println!("{} (took {:.2?})", "Done!".green().bold(), prep_time);
    
    print!("{}", "Running activation functions benchmark... ".blue());
    io::stdout().flush().unwrap();
    
    let benchmark_start = Instant::now();
    match context.run_activation_functions(test_duration, data_size) {
        Ok(result) => {
            let benchmark_time = benchmark_start.elapsed();
            println!("{} (took {:.2?})", "Done!".green().bold(), benchmark_time);
            println!("{}", "Test: Neural Network Activation Functions".cyan());
            println!("{} {} elements", "Data size:".cyan(), data_size);
            println!("{} {:.2}", "Score:".bright_green(), result.score);
            println!("{} {:.2}", "Average FPS:".cyan(), result.average_fps);
            println!("{} {:.2}/{:.2}", "Min/Max FPS:".cyan(), result.min_fps, result.max_fps);
            println!("{} ReLU, Sigmoid, Tanh, Leaky ReLU", "Operations:".cyan());
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
        }
    }
    
    // Part 7: Summary
    println!("\n{}", "--- GPU Benchmark Summary ---".yellow().bold());
    println!("{}", "GPU benchmarking completed!".green().bold());
    
    println!("{} {} ({})", "Tested GPU:".cyan(), 
             context.gpu_info.name.white(), 
             if context.gpu_info.is_integrated { "Integrated" } else { "Discrete" });
    
    if !results.is_empty() {
        let avg_score = results.iter().map(|&(_, score)| score).sum::<f64>() / results.len() as f64;
        println!("{} {:.2} MFLOPS", "Average matrix multiplication performance:".cyan(), avg_score);
        
        if let Some(&(_, max_score)) = results.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)) {
            println!("{} {:.2} MFLOPS", "Peak matrix multiplication performance:".cyan(), max_score);
        }
    }
    
    Ok(())
}
