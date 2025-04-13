---
sidebar_position: 1
---

# CPU API Reference

This page provides detailed API reference for the CPU benchmarking functionality in CatP2P.

## Structures

### `CpuInfo`

Contains detailed information about the system's CPU.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `name` | String | The name/model of the CPU | `cpu_info.name` |
| `cores` | usize | The number of physical cores | `cpu_info.cores` |
| `logical_cores` | usize | The number of logical processors (including hyperthreading) | `cpu_info.logical_cores` |
| `usage` | f32 | Current CPU usage as a percentage (0-100) | `cpu_info.usage` |
| `vendor` | String | CPU vendor (e.g., "Intel", "AMD") | `cpu_info.vendor` |
| `frequency` | Option\<u64\> | CPU frequency in MHz, if available | `if let Some(freq) = cpu_info.frequency { ... }` |

## Functions

### Information Gathering

| Function | Return Type | Description | Example Usage | Possible Errors |
|----------|-------------|-------------|--------------|-----------------|
| `get_cpu_info()` | Result\<CpuInfo, Error\> | Retrieves detailed information about the system's CPU | `let info = cpu::get_cpu_info()?;` | Failed to retrieve CPU information |

### Performance Testing

| Function | Return Type | Description | Example Usage | Performance Impact |
|----------|-------------|-------------|--------------|-------------------|
| `run_cpu_benchmark()` | Result\<f64, Error\> | Runs a comprehensive CPU benchmark and returns an overall score (higher is better) | `let score = cpu::run_cpu_benchmark()?;` | High - utilizes all CPU cores |
| `run_single_core_benchmark(iterations: u64)` | Result\<Duration, Error\> | Measures performance of a single CPU core with specified iterations | `let duration = cpu::run_single_core_benchmark(50_000_000)?;` | Medium - runs on a single core |
| `run_multi_core_benchmark(threads: usize, iterations_per_thread: u64)` | Result\<Duration, Error\> | Measures performance with multiple threads and specified iterations per thread | `let duration = cpu::run_multi_core_benchmark(4, 50_000_000)?;` | High - utilizes specified number of cores |
| `run_floating_point_benchmark(iterations: u64)` | Result\<Duration, Error\> | Measures floating-point computation performance | `let duration = cpu::run_floating_point_benchmark(10_000_000)?;` | Medium - primarily tests FPU |
| `run_averaged_benchmark(iterations: usize, benchmark_fn: F)` | Result\<Duration, Error\> | Runs a benchmark multiple times and returns the average duration | `let avg = cpu::run_averaged_benchmark(3, || cpu::run_single_core_benchmark(10_000_000))?;` | Depends on the benchmark function |

### Function Relationships

| Function | Related Functions | Notes |
|----------|-------------------|-------|
| `run_cpu_benchmark()` | `run_single_core_benchmark()`, `run_multi_core_benchmark()` | Provides an overall CPU score based on performance across all cores |
| `run_averaged_benchmark()` | Any benchmark function | Takes a closure that returns a benchmark result and runs it multiple times |
| `get_cpu_info()` | `memory::get_memory_info()` | Often used together to get a complete system overview |

### Parameter Details

| Function | Parameter | Description | Recommended Values |
|----------|-----------|-------------|-------------------|
| `run_single_core_benchmark()` | `iterations` | Number of computational iterations to perform | 10,000,000 to 100,000,000 |
| `run_multi_core_benchmark()` | `threads` | Number of threads to use | 1 to number of logical cores |
| `run_multi_core_benchmark()` | `iterations_per_thread` | Iterations per thread | 10,000,000 to 100,000,000 |
| `run_floating_point_benchmark()` | `iterations` | Number of floating-point operations to perform | 1,000,000 to 50,000,000 |
| `run_averaged_benchmark()` | `iterations` | Number of times to run the benchmark | 3 to 10 |

## Understanding CPU Benchmark Results

### Score Interpretation

The `run_cpu_benchmark()` function returns a score that represents overall CPU performance:

- Higher scores indicate better performance
- The score is calculated as: `1000.0 / elapsed_time * sqrt(num_cores)`
- This formula balances raw speed with parallel processing capability

### Typical Score Ranges

| CPU Type | Typical Score Range | Notes |
|----------|---------------------|-------|
| High-end Desktop (16+ cores) | 500-1000+ | Modern Threadripper, Core i9 |
| Mid-range Desktop (8-12 cores) | 300-600 | Modern Core i7, Ryzen 7 |
| Entry-level Desktop (4-6 cores) | 150-350 | Modern Core i5, Ryzen 5 |
| Laptop (4-8 cores) | 100-400 | Varies widely based on power limits |
| Older Desktop (2-4 cores) | 50-150 | Core i3/i5, older generations |
| Low-power devices | 10-100 | Mobile CPUs, SBCs like Raspberry Pi |

### Factors Affecting Benchmark Results

| Factor | Impact | Notes |
|--------|--------|-------|
| System Load | High | Other processes using CPU can significantly reduce benchmark scores |
| CPU Frequency Scaling | High | Power-saving modes can reduce performance |
| Thermal Throttling | High | CPUs may reduce frequency when they get too hot |
| Memory Speed | Medium | CPU benchmarks are also affected by memory performance |
| Operating System | Low to Medium | Different OS scheduling can affect multi-threaded performance |
| Background Services | Medium | System services can consume CPU resources |
| BIOS/UEFI Settings | Medium to High | Power limits and other settings can affect performance |

## Implementation Details

### Benchmark Methodology

The CPU benchmarking in CatP2P uses a combination of techniques to measure CPU performance:

1. **Integer Operations**: The single-core and multi-core benchmarks primarily test integer arithmetic and logic operations
2. **Floating-Point Operations**: The floating-point benchmark tests FPU performance with transcendental functions
3. **Memory Access**: The benchmarks include memory access patterns to test cache performance
4. **Thread Scaling**: Multi-core tests evaluate how well performance scales with additional cores

### Benchmark Workload Characteristics

The benchmark workloads are designed to:

1. Be deterministic (same input always produces same result)
2. Prevent compiler optimization from eliminating the work
3. Exercise different CPU subsystems (ALU, FPU, cache, etc.)
4. Scale appropriately with iteration count
5. Provide consistent results across different hardware

### Multi-threading Implementation

The multi-core benchmark uses Rayon's thread pool to:

1. Create the specified number of worker threads
2. Distribute work evenly across threads
3. Synchronize at completion to measure total execution time
4. Prevent false sharing and other concurrency issues

## Error Handling

The CPU benchmarking functions use Rust's `Result` type to handle errors gracefully. Common errors include:

- `Error::Benchmark("Failed to create thread pool: {error}")`: Thread creation issues
- `Error::Benchmark("Task execution failed: {error}")`: Errors during benchmark execution
- `Error::Benchmark("Failed to retrieve CPU information: {error}")`: System API errors

## Performance Considerations

When running CPU benchmarks, be aware of these performance considerations:

1. **Warm-up Effects**: The first run of a benchmark may be slower due to cache warming
2. **Frequency Scaling**: Modern CPUs dynamically adjust frequency based on workload and temperature
3. **Background Activity**: System background tasks can introduce variance in results
4. **Power Management**: Different power profiles can significantly affect results
5. **Thermal Constraints**: Extended benchmarking can cause thermal throttling

For the most accurate results:

1. Run benchmarks multiple times and average the results
2. Ensure the system is in a consistent state (temperature, background tasks)
3. Consider using a high-performance power profile
4. Allow adequate cooling for your system
5. Close unnecessary applications and services

## Comparing Results

When comparing CPU benchmark results:

1. **Same Hardware**: Results should be consistent within 5-10% on the same hardware
2. **Different Hardware**: Compare relative performance rather than absolute scores
3. **Different Versions**: Benchmark scores may not be comparable across different CatP2P versions
4. **Different Workloads**: The general benchmark may not reflect performance for specific workloads

## Advanced Usage

### Custom Benchmark Functions

You can create custom CPU benchmark functions and use them with the averaging utility:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;
use std::time::{Duration, Instant};

fn custom_cpu_benchmark() -> Result<Duration, Error> {
    let start = Instant::now();
    
    // Your custom CPU-intensive workload here
    let mut result = 0;
    for i in 0..50_000_000 {
        result = result.wrapping_add(i);
    }
    
    // Prevent compiler optimization
    if result == 42 {
        println!("Unlikely result: {}", result);
    }
    
    Ok(start.elapsed())
}

fn main() -> Result<(), Error> {
    // Run your custom benchmark with averaging
    let avg_duration = cpu::run_averaged_benchmark(5, custom_cpu_benchmark)?;
    println!("Custom benchmark average: {:?}", avg_duration);
    
    Ok(())
}
```

### Benchmark Comparison

To compare performance across different configurations:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Get CPU info for context
    let cpu_info = cpu::get_cpu_info()?;
    println!("CPU: {} with {} cores", cpu_info.name, cpu_info.logical_cores);
    
    // Compare single-core vs multi-core performance
    let single_core_time = cpu::run_single_core_benchmark(50_000_000)?;
    
    let thread_counts = [1, 2, 4, 8, cpu_info.logical_cores];
    for &threads in thread_counts.iter() {
        if threads <= cpu_info.logical_cores {
            let multi_core_time = cpu::run_multi_core_benchmark(threads, 50_000_000)?;
            
            let speedup = single_core_time.as_secs_f64() / multi_core_time.as_secs_f64();
            let efficiency = speedup / threads as f64 * 100.0;
            
            println!("Threads: {}, Time: {:?}, Speedup: {:.2}x, Efficiency: {:.2}%", 
                     threads, multi_core_time, speedup, efficiency);
        }
    }
    
    Ok(())
}
```
