---
sidebar_position: 1
---

# CPU Benchmarking

CatP2P provides comprehensive CPU benchmarking capabilities to help you understand your system's processing power. This is crucial for distributed computing tasks that may require significant CPU resources.

## CPU Information vs. Performance Testing

CatP2P offers two approaches to CPU assessment:

1. **Information Gathering**: Extracting CPU details like model name, core count, and current usage without running performance tests
2. **Performance Testing**: Running actual computations to measure real-world performance

## Getting CPU Information

You can retrieve detailed CPU information using the `get_cpu_info` function:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Get detailed CPU information
    let cpu_info = cpu::get_cpu_info()?;
    
    // Access CPU information
    println!("CPU Model: {}", cpu_info.name);
    println!("CPU Vendor: {}", cpu_info.vendor);
    println!("CPU Cores: {} (Logical: {})", cpu_info.cores, cpu_info.logical_cores);
    if let Some(freq) = cpu_info.frequency {
        println!("CPU Frequency: {} MHz", freq);
    }
    println!("Current CPU Usage: {:.2}%", cpu_info.usage);
    
    Ok(())
}
```

This provides comprehensive information including the CPU model name, vendor, and frequency.

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

## Floating-Point Performance

CatP2P also provides a benchmark for floating-point operations, which are common in scientific computing and graphics:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    let iterations = 10_000_000;
    let duration = cpu::run_floating_point_benchmark(iterations)?;
    println!("Floating-point benchmark: {:?}", duration);
    
    Ok(())
}
```

## Averaged Benchmarks for Consistency

To get more consistent results, you can run benchmarks multiple times and average the results:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Run the single-core benchmark 3 times and average the results
    let avg_duration = cpu::run_averaged_benchmark(3, || {
        cpu::run_single_core_benchmark(10_000_000)
    })?;
    
    println!("Averaged single-core benchmark: {:?}", avg_duration);
    
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

### Understanding Multi-Core Performance

When running multi-core benchmarks, you might observe that adding more cores doesn't always improve performance proportionally. This is due to several factors:

1. **Thread Creation Overhead**: Creating and managing threads has a cost
2. **Memory Contention**: Multiple cores accessing memory simultaneously can cause bottlenecks
3. **Task Distribution**: The current implementation might not distribute work evenly
4. **System Load**: Other processes running on your system can affect benchmark results
5. **Cache Coherence**: Maintaining consistent cache state across cores can add overhead

For example, on an AMD Ryzen 7 3700X with 16 logical cores, we observed the following results:

```
Multi-core benchmark (1 cores): 442 ms
Multi-core benchmark (2 cores): 432 ms
Multi-core benchmark (4 cores): 486 ms
Multi-core benchmark (8 cores): 605 ms
Multi-core benchmark (16 cores): 918 ms
```

This shows that performance actually degrades as more cores are added beyond 2 cores. The efficiency drops significantly:

```
Cores | Ideal Speedup | Actual Speedup | Efficiency
------|---------------|----------------|----------
    1 |          1.00 |           1.00 |   100.00%
    2 |          2.00 |           1.02 |    51.16%
    4 |          4.00 |           0.91 |    22.74%
    8 |          8.00 |           0.73 |     9.13%
   16 |         16.00 |           0.48 |     3.01%
```

This decreasing efficiency suggests that the benchmark workload is not well-suited for parallelization in its current form.

## Complete CPU Benchmarking Example

Here's a complete example that demonstrates all CPU benchmarking capabilities:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("=== CatP2P CPU Information and Benchmarking ===\n");
    
    // Get CPU information
    let cpu_info = cpu::get_cpu_info()?;
    
    println!("CPU Model: {}", cpu_info.name);
    println!("CPU Vendor: {}", cpu_info.vendor);
    println!("CPU Cores: {} (Logical: {})", cpu_info.cores, cpu_info.logical_cores);
    if let Some(freq) = cpu_info.frequency {
        println!("CPU Frequency: {} MHz", freq);
    }
    println!("Current CPU Usage: {:.2}%", cpu_info.usage);
    
    // Run overall CPU benchmark
    let cpu_score = cpu::run_cpu_benchmark()?;
    println!("CPU Benchmark Score: {:.2}", cpu_score);
    
    // Run single-core benchmark
    let iterations = 50_000_000;
    let duration = cpu::run_single_core_benchmark(iterations)?;
    println!("Single-core benchmark: {:?}", duration);
    
    // Run multi-core benchmark with different thread counts
    let max_cores = cpu_info.logical_cores;
    let iterations_per_thread = 50_000_000;
    
    for cores in [1, 2, 4, max_cores.min(8), max_cores] {
        if cores > max_cores {
            continue;
        }
        
        let duration = cpu::run_multi_core_benchmark(cores, iterations_per_thread)?;
        println!("Multi-core benchmark ({} cores): {:?}", cores, duration);
    }
    
    // Run floating-point benchmark
    let fp_iterations = 10_000_000;
    let duration = cpu::run_floating_point_benchmark(fp_iterations)?;
    println!("Floating-point benchmark: {:?}", duration);
    
    // Run averaged benchmarks
    let avg_single = cpu::run_averaged_benchmark(3, || cpu::run_single_core_benchmark(10_000_000))?;
    println!("Averaged single-core benchmark: {:?}", avg_single);
    
    Ok(())
}
```

## Best Practices for CPU Benchmarking

To get the most accurate results from your CPU benchmarks:

1. **Run benchmarks when the system is idle**: Close other applications and background processes
2. **Run benchmarks multiple times**: Take the average of several runs to account for variations
3. **Be consistent with system conditions**: Power settings, CPU frequency scaling, and temperature can affect results
4. **Interpret results in context**: Compare results only between similar hardware and configurations
5. **Consider your specific workload**: The benchmark is a general test and might not reflect the performance of your specific application

## Using CPU Benchmark Results

The results from CPU benchmarking can help you:

1. Determine if your system is suitable for CPU-intensive distributed tasks
2. Configure optimal thread counts for parallel processing
3. Compare your node's capabilities with other nodes in the network
4. Set appropriate CPU resource limits in your CatP2P configuration

## Optimizing Multi-Core Performance

If you're developing CPU-intensive applications with CatP2P, consider these strategies to improve multi-core performance:

1. **Reduce synchronization points**: Minimize the use of locks, atomic operations, and shared data
2. **Use larger work chunks**: Increase the amount of work done between synchronization points
3. **Implement work stealing**: Allow idle threads to "steal" work from busy threads
4. **Consider NUMA awareness**: On systems with Non-Uniform Memory Access, ensure threads work with memory close to their CPU
5. **Use thread-local storage**: Minimize contention by giving each thread its own data structures

## Related Resources

For more information about CPU benchmarking in CatP2P, check out these resources:

- [CPU Benchmarking Examples](/docs/Examples/benchmarks/cpu-benchmark-example)
- [CPU Benchmarking API Reference](/docs/api/benchmark/cpu)