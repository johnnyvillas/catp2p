// examples/cpu_benchmark.rs
use catp2p::benchmark::cpu;
use catp2p::error::Error;
use catp2p::resources::monitor::ResourceMonitor;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("=== CatP2P CPU Information and Benchmarking ===\n");
    
    // Part 1: Get CPU information without performance testing
    println!("--- CPU Information ---");
    
    // Create a resource monitor to get CPU details
    let mut resource_monitor = ResourceMonitor::new_with_default_interval();
    let system_resources = resource_monitor.get_current_resources();
    
    println!("CPU Cores: {}", system_resources.cpu_cores);
    println!("Current CPU Usage: {:.2}%", system_resources.cpu_usage);
    
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
            "Single-core benchmark ({} iterations): {:.2} ms", 
            iter, 
            duration.as_millis()
        );
    }
    
    // Part 4: Run multi-core benchmark with different thread counts
    println!("\n--- Multi-Core Scaling ---");
    
    // Get available CPU cores
    let max_cores = num_cpus::get();
    let core_counts = [1, 2, 4, max_cores.min(8), max_cores];
    
    // Use a fixed workload per thread
    let iterations_per_thread = 50_000_000;
    
    for &cores in &core_counts {
        if cores > max_cores {
            continue;
        }
        
        let duration = cpu::run_multi_core_benchmark(cores, iterations_per_thread)?;
        println!(
            "Multi-core benchmark ({} cores): {:.2} ms", 
            cores, 
            duration.as_millis()
        );
    }
    
    println!("\nCPU benchmarking completed!");
    
    Ok(())
}
