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

// Reference values for normalizing scores to 0-10000 scale
const MATRIX_MULT_REFERENCE: f64 = 5_000_000.0; // 5 million MFLOPS as reference
const ACTIVATION_REFERENCE: f64 = 10_000.0;     // 10,000 as reference
const GRADIENT_REFERENCE: f64 = 15_000.0;       // 15,000 as reference

// Function to normalize a score to 0-10000 scale
fn normalize_score(score: f64, reference: f64) -> f64 {
    // Cap at 10000 to avoid extremely high values
    (score / reference * 5000.0).min(10000.0)
}

fn main() -> Result<(), Error> {
    // Enable colored output
    colored::control::set_override(true);

    println!("{}\n", "=== CatP2P GPU Benchmarking Example ===".bright_green().bold());

    // Part 1: Get GPU information without performance testing
    println!("{}", "--- GPU Information ---".yellow().bold());
    println!("{}", "┌─ Description ───────────────────────────────────────────────┐".bright_blue());
    println!("{}", "│ This section queries basic information about your GPU        │".bright_blue());
    println!("{}", "│ hardware. This helps identify the capabilities of your       │".bright_blue());
    println!("{}", "│ system for distributed computing tasks.                      │".bright_blue());
    println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_blue());
    
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
    println!("{}", "┌─ Description ───────────────────────────────────────────────┐".bright_blue());
    println!("{}", "│ Matrix multiplication is a fundamental operation in many     │".bright_blue());
    println!("{}", "│ computational tasks, particularly in machine learning,       │".bright_blue());
    println!("{}", "│ scientific simulations, and data processing. It tests the    │".bright_blue());
    println!("{}", "│ GPU's ability to perform parallel floating-point operations. │".bright_blue());
    println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_blue());
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
            println!("{} {:.2}", "Average FPS:".cyan(), result.average_fps);
            println!("{} {:.2}/{:.2}", "Min/Max FPS:".cyan(), result.min_fps, result.max_fps);
            
            // Normalize the score to 0-10000 scale
            let normalized_score = normalize_score(result.score, MATRIX_MULT_REFERENCE);
            println!("{} {:.2} MFLOPS (Normalized: {:.2}/10000)", "Score:".bright_green(), result.score, normalized_score);
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
    println!("{}", "┌─ Description ───────────────────────────────────────────────┐".bright_blue());
    println!("{}", "│ This test measures how GPU performance scales with           │".bright_blue());
    println!("{}", "│ increasing workload complexity. Understanding scaling        │".bright_blue());
    println!("{}", "│ behavior is crucial for optimizing distributed computing     │".bright_blue());
    println!("{}", "│ tasks.                                                       │".bright_blue());
    println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_blue());
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
                println!("  {}: {:.2}", "Average FPS".cyan(), result.average_fps);
                
                // Normalize the score to 0-10000 scale
                let normalized_score = normalize_score(result.score, MATRIX_MULT_REFERENCE);
                println!("  {}: {:.2} MFLOPS (Normalized: {:.2}/10000)", "Score".bright_green(), result.score, normalized_score);
                
                results.push((complexity, result.score, normalized_score));
            },
            Err(e) => {
                println!("{}: {}", "Failed".red().bold(), e);
            }
        }
    }
    
    // Part 4: Run a more intensive benchmark
    println!("\n{}", "--- Intensive GPU Benchmark ---".yellow().bold());
    println!("{}", "┌─ Description ───────────────────────────────────────────────┐".bright_blue());
    println!("{}", "│ This benchmark tests your GPU under high computational load  │".bright_blue());
    println!("{}", "│ to evaluate its performance ceiling and stability with       │".bright_blue());
    println!("{}", "│ large-scale parallel operations.                             │".bright_blue());
    println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_blue());
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
            println!("{} {:.2}", "Average FPS:".cyan(), result.average_fps);
            
            // Normalize the score to 0-10000 scale
            let normalized_score = normalize_score(result.score, MATRIX_MULT_REFERENCE);
            println!("{} {:.2} MFLOPS (Normalized: {:.2}/10000)", "Score:".bright_green(), result.score, normalized_score);
            
            // Add to results for visualization
            results.push((complexity, result.score, normalized_score));
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
        }
    }
    
    // Part 5: Visualize complexity scaling with ASCII chart
    if !results.is_empty() {
        println!("\n{}", "--- GPU Performance Scaling with Complexity ---".yellow().bold());
        println!("{}", "┌─ Description ───────────────────────────────────────────────┐".bright_blue());
        println!("{}", "│ This visualization shows how performance scales with         │".bright_blue());
        println!("{}", "│ increasing workload size. Ideally, a GPU should maintain     │".bright_blue());
        println!("{}", "│ consistent performance as complexity increases.              │".bright_blue());
        println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_blue());
        println!("{}", "Higher is better:".magenta().italic());
        
        // Find max score for scaling the chart
        let max_score = results.iter().map(|&(_, score, _)| score as usize).max().unwrap_or(1);
        let scale = 50.0 / max_score as f64;
        
        for &(complexity, score, normalized) in &results {
            let bar_length = (score as f64 * scale) as usize;
            let bar = "#".repeat(bar_length);
            println!("Complexity {:2}: {:10.2} MFLOPS (Normalized: {:.2}/10000) |{}|", 
                     complexity.to_string().white(), 
                     score.to_string().bright_green(),
                     normalized.to_string().yellow(),
                     bar.bright_cyan());
        }
    }
    
    // Part 6: Run activation functions benchmark
    println!("\n{}", "--- Neural Network Activation Functions Benchmark ---".yellow().bold());
    println!("{}", "┌─ Description ───────────────────────────────────────────────┐".bright_blue());
    println!("{}", "│ Activation functions are essential components of neural      │".bright_blue());
    println!("{}", "│ networks that introduce non-linearity into the model. This   │".bright_blue());
    println!("{}", "│ benchmark tests the GPU's ability to compute complex         │".bright_blue());
    println!("{}", "│ mathematical functions like sigmoid, tanh, and ReLU at scale.│".bright_blue());
    println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_blue());
    
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
            println!("{} {:.2}", "Average FPS:".cyan(), result.average_fps);
            println!("{} {:.2}/{:.2}", "Min/Max FPS:".cyan(), result.min_fps, result.max_fps);
            println!("{} ReLU, Sigmoid, Tanh, Leaky ReLU", "Operations:".cyan());
            
            // Normalize the score to 0-10000 scale
            let normalized_score = normalize_score(result.score, ACTIVATION_REFERENCE);
            println!("{} {:.2} (Normalized: {:.2}/10000)", "Score:".bright_green(), result.score, normalized_score);
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
        }
    }
    
       // Part 7: Run gradient calculation benchmark (NEW)
       println!("\n{}", "--- Neural Network Gradient Calculation Benchmark ---".yellow().bold());
       println!("{}", "┌─ Description ───────────────────────────────────────────────┐".bright_blue());
       println!("{}", "│ The gradient calculation benchmark simulates neural network  │".bright_blue());
       println!("{}", "│ training, which is a key workload for distributed machine    │".bright_blue());
       println!("{}", "│ learning tasks. It tests the GPU's performance with          │".bright_blue());
       println!("{}", "│ backpropagation algorithms used to train models.             │".bright_blue());
       println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_blue());
       
       // Use the same data size for consistency
       print!("{} ({} elements)... ", "Preparing benchmark data".blue(), data_size);
       io::stdout().flush().unwrap();
       
       let prep_start = Instant::now();
       // Preparation is done inside the benchmark function
       let prep_time = prep_start.elapsed();
       println!("{} (took {:.2?})", "Done!".green().bold(), prep_time);
       
       print!("{}", "Running gradient calculation benchmark... ".blue());
       io::stdout().flush().unwrap();
       
       let benchmark_start = Instant::now();
       match context.run_gradient_calc(test_duration, data_size) {
           Ok(result) => {
               let benchmark_time = benchmark_start.elapsed();
               println!("{} (took {:.2?})", "Done!".green().bold(), benchmark_time);
               println!("{}", "Test: Neural Network Gradient Calculation".cyan());
               println!("{} {} elements", "Data size:".cyan(), data_size);
               println!("{} {:.2}", "Average FPS:".cyan(), result.average_fps);
               println!("{} {:.2}/{:.2}", "Min/Max FPS:".cyan(), result.min_fps, result.max_fps);
               println!("{} Backpropagation with sigmoid activation", "Operations:".cyan());
               
               // Normalize the score to 0-10000 scale
               let normalized_score = normalize_score(result.score, GRADIENT_REFERENCE);
               println!("{} {:.2} (Normalized: {:.2}/10000)", "Score:".bright_green(), result.score, normalized_score);
           },
           Err(e) => {
               println!("{}: {}", "Failed".red().bold(), e);
           }
       }
       
       // Part 8: Summary
       println!("\n{}", "--- GPU Benchmark Summary ---".yellow().bold());
       println!("{}", "┌─ Description ───────────────────────────────────────────────┐".bright_blue());
       println!("{}", "│ This summary provides an overview of your GPU's performance  │".bright_blue());
       println!("{}", "│ across different computational tasks relevant to distributed │".bright_blue());
       println!("{}", "│ computing and machine learning.                              │".bright_blue());
       println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_blue());
       println!("{}", "GPU benchmarking completed!".green().bold());
       
       println!("{} {} ({})", "Tested GPU:".cyan(), 
                context.gpu_info.name.white(), 
                if context.gpu_info.is_integrated { "Integrated" } else { "Discrete" });
       
       // Collect all normalized test scores
       let mut all_scores = Vec::new();
       let mut all_normalized_scores = Vec::new();
       
       // Add matrix multiplication scores
       if !results.is_empty() {
           // Calculate average raw score
           let avg_score = results.iter().map(|&(_, score, _)| score).sum::<f64>() / results.len() as f64;
           println!("{} {:.2} MFLOPS", "Average matrix multiplication performance:".cyan(), avg_score);
           
           // Find peak score and add to collection
           if let Some(&(_, max_score, max_normalized)) = results.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)) {
               println!("{} {:.2} MFLOPS (Normalized: {:.2}/10000)", 
                        "Peak matrix multiplication performance:".cyan(), 
                        max_score, 
                        max_normalized);
               all_scores.push(max_score);
               all_normalized_scores.push(max_normalized);
           }
       }
       
       // Try to run activation functions benchmark for final score if not already done
       if let Ok(result) = context.run_activation_functions(Duration::from_secs(1), 500_000) {
           let normalized = normalize_score(result.score, ACTIVATION_REFERENCE);
           println!("{} {:.2} (Normalized: {:.2}/10000)", "Activation functions score:".cyan(), result.score, normalized);
           all_scores.push(result.score);
           all_normalized_scores.push(normalized);
       }
       
       // Try to run gradient calculation benchmark for final score if not already done
       if let Ok(result) = context.run_gradient_calc(Duration::from_secs(1), 500_000) {
           let normalized = normalize_score(result.score, GRADIENT_REFERENCE);
           println!("{} {:.2} (Normalized: {:.2}/10000)", "Gradient calculation score:".cyan(), result.score, normalized);
           all_scores.push(result.score);
           all_normalized_scores.push(normalized);
       }
       
       // Calculate final score (average of normalized scores)
       if !all_normalized_scores.is_empty() {
           let final_score = all_normalized_scores.iter().sum::<f64>() / all_normalized_scores.len() as f64;
           
           println!("\n{} {:.2}/10000", "FINAL SCORE:".bright_green().bold(), final_score);
           
           // Provide a rating based on the normalized score
           let rating = if final_score > 9000.0 {
               "Exceptional".bright_magenta().bold()
           } else if final_score > 7500.0 {
               "Excellent".bright_green().bold()
           } else if final_score > 5000.0 {
               "Very Good".green().bold()
           } else if final_score > 2500.0 {
               "Good".yellow().bold()
           } else if final_score > 1000.0 {
               "Average".yellow()
           } else if final_score > 500.0 {
               "Below Average".red()
           } else {
               "Low".red().bold()
           };
           
           println!("{} {}", "Performance Rating:".cyan(), rating);
       }
       
       Ok(())
   }
   