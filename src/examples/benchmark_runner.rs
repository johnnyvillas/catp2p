//! An example that runs the CatP2P benchmarks.

use catp2p::{CatP2P, benchmark::BenchmarkResult};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();

    println!("Running CatP2P benchmarks...");

    // Create a CatP2P instance with default configuration
    let catp2p = CatP2P::new()?;

    // Run the benchmark
    let result = catp2p.run_benchmark()?;

    // Print the results
    print_benchmark_results(&result);

    Ok(())
}

fn print_benchmark_results(result: &BenchmarkResult) {
    println!("\n=== Benchmark Results ===");
    println!("CPU Score: {:.2}", result.cpu_score);
    println!("Memory Score: {:.2}", result.memory_score);
    println!("Disk Score: {:.2}", result.disk_score);
    
    if let Some(gpu_score) = result.gpu_score {
        println!("GPU Score: {:.2}", gpu_score);
    } else {
        println!("GPU Score: N/A (No compatible GPU found)");
    }
    
    println!("Overall Score: {:.2}", result.overall_score);
    println!("========================\n");
    
    // Provide some context for the scores
    println!("Score Interpretation:");
    println!("< 50: Low performance");
    println!("50-100: Average performance");
    println!("100-200: Good performance");
    println!("> 200: Excellent performance");
}
