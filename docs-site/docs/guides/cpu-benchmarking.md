---
sidebar_position: 1
---

# CPU Benchmarking

CatP2P provides comprehensive CPU benchmarking capabilities to help you understand your system's processing power. This is crucial for distributed computing tasks that may require significant CPU resources.

## CPU Information vs. Performance Testing

CatP2P offers two approaches to CPU assessment:

1. **Information Gathering**: Extracting CPU details like core count and current usage without running performance tests
2. **Performance Testing**: Running actual computations to measure real-world performance

## Getting CPU Information

You can quickly retrieve basic CPU information using the `ResourceMonitor`:

```rust
use catp2p::resources::monitor::ResourceMonitor;

// Create a resource monitor
let mut resource_monitor = ResourceMonitor::new_with_default_interval();

// Get current system resources
let system_resources = resource_monitor.get_current_resources();

// Access CPU information
println!("CPU Cores: {}", system_resources.cpu_cores);
println!("Current CPU Usage: {:.2}%", system_resources.cpu_usage);
```

## Running CPU Performance Benchmarks

For a comprehensive assessment of CPU performance, you can use the benchmarking functions:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Run the overall CPU benchmark
    let cpu_score = cpu::run_cpu_benchmark()?;
    println!("CPU Benchmark Score: {:.2}", cpu_score);
    
    // The score represents overall CPU performance
    // Higher scores indicate better performance
    
    Ok(())
}
```

## Single-Core vs. Multi-Core Performance

CatP2P allows you to test both single-core and multi-core performance:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Test single-core performance with different workloads
    let iterations = 50_000_000;
    let duration = cpu::run_single_core_benchmark(iterations)?;
    println!("Single-core time: {:?}", duration);
    
    // Test multi-core performance with different thread counts
    let threads = 4; // Use 4 CPU cores
    let iterations_per_thread = 50_000_000;
    let duration = cpu::run_multi_core_benchmark(threads, iterations_per_thread)?;
    println!("Multi-core time with {} threads: {:?}", threads, duration);
    
    Ok(())
}
```

## Understanding CPU Benchmark Results

The CPU benchmark in CatP2P measures several aspects of CPU performance:

1. **Raw computational power**: How quickly your CPU can perform calculations
2. **Multi-threading efficiency**: How well performance scales with multiple cores
3. **Workload handling**: Performance under different types of computational tasks

### Interpreting the Score

The overall CPU benchmark score is a composite value that represents:

- Higher scores indicate better CPU performance
- Scores are influenced by:
  - Number of CPU cores
  - CPU clock speed
  - CPU architecture and efficiency
  - Multi-threading capabilities

### Comparing Single-Core vs. Multi-Core Results

By comparing single-core and multi-core benchmark results, you can understand:

- The raw power of each CPU core
- How efficiently your CPU scales with multiple threads
- The optimal number of threads for your specific CPU

## Complete CPU Benchmarking Example

Here's a complete example that demonstrates all CPU benchmarking capabilities:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;
use catp2p::resources::monitor::ResourceMonitor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get CPU information
    let mut resource_monitor = ResourceMonitor::new_with_default_interval();
    let system_resources = resource_monitor.get_current_resources();
    
    println!("CPU Cores: {}", system_resources.cpu_cores);
    println!("Current CPU Usage: {:.2}%", system_resources.cpu_usage);
    
    // Run overall CPU benchmark
    let cpu_score = cpu::run_cpu_benchmark()?;
    println!("CPU Benchmark Score: {:.2}", cpu_score);
    
    // Run single-core benchmark
    let iterations = 50_000_000;
    let duration = cpu::run_single_core_benchmark(iterations)?;
    println!("Single-core benchmark: {:?}", duration);
    
    // Run multi-core benchmark with different thread counts
    let max_cores = system_resources.cpu_cores as usize;
    let iterations_per_thread = 50_000_000;
    
    for cores in [1, 2, 4, max_cores.min(8) as usize] {
        if cores > max_cores as usize {
            continue;
        }
        
        let duration = cpu::run_multi_core_benchmark(cores, iterations_per_thread)?;
        println!("Multi-core benchmark ({} cores): {:?}", cores, duration);
    }
    
    Ok(())
}
```

## Using CPU Benchmark Results

The results from CPU benchmarking can help you:

1. Determine if your system is suitable for CPU-intensive distributed tasks
2. Configure optimal thread counts for parallel processing
3. Compare your node's capabilities with other nodes in the network
4. Set appropriate CPU resource limits in your CatP2P configuration
```

### Step 5: Create the Example File

```bash
mkdir -p catp2p/examples
touch catp2p/examples/cpu_benchmark.rs
```

Add the example code:

```rust
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
```

### Step 6: Update the Cargo.toml to Include the Example

Make sure the example is properly registered in your Cargo.toml:

```toml
[[example]]
name = "cpu_benchmark"
path = "examples/cpu_benchmark.rs"
```

### Step 7: Update the Introduction Page

Let's also update the introduction page to mention our new documentation:

Edit `catp2p/docs-site/docs/intro.md` to include a reference to our new guides:

```markdown
---
sidebar_position: 1
---

# Introduction to CatP2P

CatP2P is a high-performance peer-to-peer library for distributed computing, written in Rust.

## Features

- **P2P Networking**: Built on libp2p for robust peer discovery and communication
- **Task Distribution**: Efficiently distribute and execute tasks across the network
- **Resource Management**: Monitor and allocate CPU, GPU, memory, and storage resources
- **Benchmarking**: Assess node capabilities for optimal task allocation
- **Local Storage**: Persistent storage for task logs and peer interactions
- **Scoring System**: Track contributions and allocate rewards

## Getting Started

Check out our guides to learn how to use CatP2P:

- [CPU Benchmarking](./guides/cpu-benchmarking.md): Learn how to benchmark CPU performance
- More guides coming soon!

## Installation

Add catp2p to your Cargo.toml:

```toml
[dependencies]
catp2p = "0.1.0"
```

## Basic Usage

```rust
use catp2p::CatP2P;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new CatP2P instance with default configuration
    let mut node = CatP2P::new()?;
    
    // Start the node
    node.start()?;
    
    // The node is now running and will discover peers and process tasks
    
    // When done, stop the node
    node.stop()?;
    
    Ok(())
}
```
```

### Step 8: Test the Documentation

To test your Docusaurus site locally:

```bash
cd catp2p/docs-site
npm start
