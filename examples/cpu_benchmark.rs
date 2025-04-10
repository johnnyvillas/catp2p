use catp2p::benchmark::cpu;
use catp2p::error::Error;
use colored::*;
use std::io::{self, Write};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Enable colored output
    colored::control::set_override(true);

    println!("{}\n", "=== CatP2P CPU Information and Benchmarking ===".bright_green().bold());
    
    // Part 1: Get CPU information without performance testing
    println!("{}", "--- CPU Information ---".yellow().bold());
    
    // Get detailed CPU information
    let cpu_info_start = Instant::now();
    let cpu_info = cpu::get_cpu_info()?;
    let cpu_info_time = cpu_info_start.elapsed();
    
    println!("{} {}", "CPU Model:".cyan(), cpu_info.name.white());
    println!("{} {}", "CPU Vendor:".cyan(), cpu_info.vendor.white());
    println!("{} {}", "CPU Cores:".cyan(), format!("{} (Logical: {})", cpu_info.cores, cpu_info.logical_cores).white());
    if let Some(freq) = cpu_info.frequency {
        println!("{} {} MHz", "CPU Frequency:".cyan(), freq.to_string().white());
    }
    println!("{} {:.2}%", "Current CPU Usage:".cyan(), cpu_info.usage.to_string().white());
    println!("{} {:.2?}", "Time to query CPU info:".cyan(), cpu_info_time);
    
    // Part 2: Run performance benchmarks
    println!("\n{}", "--- CPU Performance Benchmark ---".yellow().bold());
    println!("{}", "Running CPU benchmark...".blue());
    
    // Run the CPU benchmark
    let benchmark_start = Instant::now();
    let cpu_score = cpu::run_cpu_benchmark()?;
    let benchmark_time = benchmark_start.elapsed();
    
    println!("{} {:.2}", "CPU Benchmark Score:".bright_green().bold(), cpu_score);
    println!("{} {:.2?}", "Benchmark completed in:".cyan(), benchmark_time);
    
    // Part 3: Run single-core benchmark with different workloads
    println!("\n{}", "--- Single Core Performance ---".yellow().bold());
    
    let iterations = [1_000_000, 10_000_000, 100_000_000];
    for &iter in &iterations {
        print!("Single-core benchmark ({} iterations): ", iter);
        io::stdout().flush().unwrap();
        
        let benchmark_start = Instant::now();
        let duration = cpu::run_single_core_benchmark(iter)?;
        let benchmark_time = benchmark_start.elapsed();
        
        println!("{} ms (took {:.2?})", 
            duration.as_millis().to_string().bright_green(), 
            benchmark_time
        );
    }
    
    // Part 4: Run multi-core benchmark with different thread counts
    println!("\n{}", "--- Multi-Core Scaling ---".yellow().bold());
    
    // Get available CPU cores from the cpu_info instead of system_resources
    let max_cores = cpu_info.logical_cores;
    let core_counts = [1, 2, 4, max_cores.min(8), max_cores];
    
    // Use a fixed workload per thread
    let iterations_per_thread = 50_000_000;
    
    // Store results for visualization
    let mut results = Vec::new();
    
    for &cores in &core_counts {
        if cores > max_cores {
            continue;
        }
        
        print!("Multi-core benchmark ({} cores): ", cores);
        io::stdout().flush().unwrap();
        
        let benchmark_start = Instant::now();
        let duration = cpu::run_multi_core_benchmark(cores, iterations_per_thread)?;
        let benchmark_time = benchmark_start.elapsed();
        
        println!("{} ms (took {:.2?})", 
            duration.as_millis().to_string().bright_green(), 
            benchmark_time
        );
        
        results.push((cores, duration.as_millis()));
    }
    
    // Part 5: Run floating-point benchmark
    println!("\n{}", "--- Floating-Point Performance ---".yellow().bold());
    let fp_iterations = 10_000_000;
    
    print!("Floating-point benchmark ({} iterations): ", fp_iterations);
    io::stdout().flush().unwrap();
    
    let benchmark_start = Instant::now();
    let duration = cpu::run_floating_point_benchmark(fp_iterations)?;
    let benchmark_time = benchmark_start.elapsed();
    
    println!("{} ms (took {:.2?})", 
        duration.as_millis().to_string().bright_green(), 
        benchmark_time
    );
    
    // Part 6: Run averaged benchmarks for more consistent results
    println!("\n{}", "--- Averaged Benchmarks (3 iterations each) ---".yellow().bold());
    
    print!("Averaged single-core benchmark: ");
    io::stdout().flush().unwrap();
    
    let benchmark_start = Instant::now();
    let avg_single = cpu::run_averaged_benchmark(3, || cpu::run_single_core_benchmark(10_000_000))?;
    let benchmark_time = benchmark_start.elapsed();
    
    println!("{} ms (took {:.2?})", 
        avg_single.as_millis().to_string().bright_green(), 
        benchmark_time
    );
    
    print!("Averaged multi-core benchmark ({} cores): ", max_cores);
    io::stdout().flush().unwrap();
    
    let benchmark_start = Instant::now();
    let avg_multi = cpu::run_averaged_benchmark(3, || cpu::run_multi_core_benchmark(max_cores, 10_000_000))?;
    let benchmark_time = benchmark_start.elapsed();
    
    println!("{} ms (took {:.2?})", 
        avg_multi.as_millis().to_string().bright_green(), 
        benchmark_time
    );
    
    print!("Averaged floating-point benchmark: ");
    io::stdout().flush().unwrap();
    
    let benchmark_start = Instant::now();
    let avg_fp = cpu::run_averaged_benchmark(3, || cpu::run_floating_point_benchmark(10_000_000))?;
    let benchmark_time = benchmark_start.elapsed();
    
    println!("{} ms (took {:.2?})", 
        avg_fp.as_millis().to_string().bright_green(), 
        benchmark_time
    );
    
    // Part 7: Visualize multi-core scaling with ASCII chart
    println!("\n{}", "--- Multi-Core Scaling Visualization ---".yellow().bold());
    println!("{}", "Lower is better:".cyan().italic());
    
    // Find max duration for scaling the chart
    let max_duration = results.iter().map(|&(_, d)| d).max().unwrap_or(1);
    let scale = 50.0 / max_duration as f64;
    
    // Use &results to avoid moving the vector
    for &(cores, duration) in &results {
        let bar_length = (duration as f64 * scale) as usize;
        let bar = "#".repeat(bar_length);
        println!("{:2} cores: {:5} ms |{}|", 
            cores.to_string().white(), 
            duration.to_string().bright_green(), 
            bar.bright_cyan()
        );
    }
    
    // Part 8: Calculate and display speedup ratios
    if !results.is_empty() {
        println!("\n{}", "--- Speedup Ratios ---".yellow().bold());
        println!("{}", "Relative to single-core performance:".cyan());
        
        let single_core_time = results[0].1;
        
        // Use &results to avoid moving the vector
        for &(cores, duration) in &results {
            let speedup = single_core_time as f64 / duration as f64;
            println!("{:2} cores: {:.2}x speedup", 
                cores.to_string().white(), 
                speedup.to_string().bright_green()
            );
        }
        
        // Calculate ideal vs. actual scaling
        println!("\n{}", "Ideal vs. Actual Scaling:".cyan());
        println!("{}", "Cores | Ideal Speedup | Actual Speedup | Efficiency".bright_white());
        println!("{}", "------|---------------|----------------|----------".bright_white());
        
        // Use &results to avoid moving the vector
        for &(cores, duration) in &results {
            if cores == 1 {
                println!("{:5} | {:13.2} | {:14.2} | {:8.2}%", 
                    cores.to_string().white(), 
                    "1.00".bright_green(), 
                    "1.00".bright_green(), 
                    "100.00".bright_green()
                );
            } else {
                let ideal_speedup = cores as f64;
                let actual_speedup = single_core_time as f64 / duration as f64;
                let efficiency = (actual_speedup / ideal_speedup) * 100.0;
                
                let efficiency_color = if efficiency > 75.0 {
                    efficiency.to_string().bright_green()
                } else if efficiency > 50.0 {
                    efficiency.to_string().yellow()
                } else if efficiency > 25.0 {
                    efficiency.to_string().bright_yellow()
                } else {
                    efficiency.to_string().red()
                };
                
                println!("{:5} | {:13.2} | {:14.2} | {:8.2}%", 
                    cores.to_string().white(), 
                    ideal_speedup.to_string().bright_green(), 
                    actual_speedup.to_string().bright_green(), 
                    efficiency_color
                );
            }
        }
    }
    
    println!("\n{}", "CPU benchmarking completed!".green().bold());
    
    Ok(())
}
