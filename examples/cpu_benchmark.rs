use catp2p::benchmark::cpu;
use catp2p::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("=== CatP2P CPU Information and Benchmarking ===\n");
    
    // Part 1: Get CPU information without performance testing
    println!("--- CPU Information ---");
    
    // Get detailed CPU information
    let cpu_info = cpu::get_cpu_info()?;
    
    println!("CPU Model: {}", cpu_info.name);
    println!("CPU Vendor: {}", cpu_info.vendor);
    println!("CPU Cores: {} (Logical: {})", cpu_info.cores, cpu_info.logical_cores);
    if let Some(freq) = cpu_info.frequency {
        println!("CPU Frequency: {} MHz", freq);
    }
    println!("Current CPU Usage: {:.2}%", cpu_info.usage);
    
    // Part 2: Run performance benchmarks
    println!("\n--- CPU Performance Benchmark ---");
    println!("Running CPU benchmark...");
    
    // Run the CPU benchmark
    let cpu_score = cpu::run_cpu_benchmark()?;
    println!("CPU Benchmark Score: {:.2}", cpu_score);
    
    // Part 3: Run single-core benchmark with different workloads
    println!("\n--- Single Core Performance ---");
    
    let iterations = [1_000_000, 10_000_000, 100_000_000];
    for &iter in &iterations {
        let duration = cpu::run_single_core_benchmark(iter)?;
        println!(
            "Single-core benchmark ({} iterations): {} ms", 
            iter, 
            duration.as_millis()
        );
    }
    
    // Part 4: Run multi-core benchmark with different thread counts
    println!("\n--- Multi-Core Scaling ---");
    
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
        
        let duration = cpu::run_multi_core_benchmark(cores, iterations_per_thread)?;
        println!(
            "Multi-core benchmark ({} cores): {} ms", 
            cores, 
            duration.as_millis()
        );
        
        results.push((cores, duration.as_millis()));
    }
    
    // Part 5: Run floating-point benchmark
    println!("\n--- Floating-Point Performance ---");
    let fp_iterations = 10_000_000;
    let duration = cpu::run_floating_point_benchmark(fp_iterations)?;
    println!("Floating-point benchmark ({} iterations): {} ms", fp_iterations, duration.as_millis());
    
    // Part 6: Run averaged benchmarks for more consistent results
    println!("\n--- Averaged Benchmarks (3 iterations each) ---");
    
    let avg_single = cpu::run_averaged_benchmark(3, || cpu::run_single_core_benchmark(10_000_000))?;
    println!("Averaged single-core benchmark: {} ms", avg_single.as_millis());
    
    let avg_multi = cpu::run_averaged_benchmark(3, || cpu::run_multi_core_benchmark(max_cores, 10_000_000))?;
    println!("Averaged multi-core benchmark ({} cores): {} ms", max_cores, avg_multi.as_millis());
    
    let avg_fp = cpu::run_averaged_benchmark(3, || cpu::run_floating_point_benchmark(10_000_000))?;
    println!("Averaged floating-point benchmark: {} ms", avg_fp.as_millis());
    
    // Part 7: Visualize multi-core scaling with ASCII chart
    println!("\n--- Multi-Core Scaling Visualization ---");
    println!("Lower is better:");
    
    // Find max duration for scaling the chart
    let max_duration = results.iter().map(|&(_, d)| d).max().unwrap_or(1);
    let scale = 50.0 / max_duration as f64;
    
    // Use &results to avoid moving the vector
    for &(cores, duration) in &results {
        let bar_length = (duration as f64 * scale) as usize;
        let bar = "#".repeat(bar_length);
        println!("{:2} cores: {:5} ms |{}|", cores, duration, bar);
    }
    
    // Part 8: Calculate and display speedup ratios
    if !results.is_empty() {
        println!("\n--- Speedup Ratios ---");
        println!("Relative to single-core performance:");
        
        let single_core_time = results[0].1;
        
        // Use &results to avoid moving the vector
        for &(cores, duration) in &results {
            let speedup = single_core_time as f64 / duration as f64;
            println!("{:2} cores: {:.2}x speedup", cores, speedup);
        }
        
        // Calculate ideal vs. actual scaling
        println!("\nIdeal vs. Actual Scaling:");
        println!("Cores | Ideal Speedup | Actual Speedup | Efficiency");
        println!("------|---------------|----------------|----------");
        
        // Use &results to avoid moving the vector
        for &(cores, duration) in &results {
            if cores == 1 {
                println!("{:5} | {:13.2} | {:14.2} | {:8.2}%", cores, 1.0, 1.0, 100.0);
            } else {
                let ideal_speedup = cores as f64;
                let actual_speedup = single_core_time as f64 / duration as f64;
                let efficiency = (actual_speedup / ideal_speedup) * 100.0;
                println!("{:5} | {:13.2} | {:14.2} | {:8.2}%", cores, ideal_speedup, actual_speedup, efficiency);
            }
        }
    }
    
    println!("\nCPU benchmarking completed!");
    
    Ok(())
}
