---
sidebar_position: 2
---

# CPU Benchmark Examples

This page provides examples of how to use the CPU benchmarking functionality in CatP2P.

## Basic CPU Benchmark Example

The `examples/cpu_benchmark.rs` file in the CatP2P repository demonstrates comprehensive CPU benchmarking capabilities.

### Running the Example

```bash
cargo run --example cpu_benchmark
```

### Sample Output

```
=== CatP2P CPU Information and Benchmarking ===

--- CPU Information ---
CPU Model: AMD Ryzen 7 3700X 8-Core Processor
CPU Vendor: AuthenticAMD
CPU Cores: 16 (Logical: 16)
CPU Frequency: 3600 MHz
Current CPU Usage: 43%
Time to query CPU info: 591.23ms

--- CPU Performance Benchmark ---
Running CPU benchmark...
CPU Benchmark Score: 22834.19
Benchmark completed in: 175.18ms

--- Single Core Performance ---
Single-core benchmark (1000000 iterations): 6 ms (took 6.87ms)
Single-core benchmark (10000000 iterations): 69 ms (took 69.04ms)
Single-core benchmark (100000000 iterations): 687 ms (took 687.42ms)

--- Multi-Core Scaling ---
Multi-core benchmark (1 cores): 411 ms (took 411.64ms)
Multi-core benchmark (2 cores): 420 ms (took 420.37ms)
Multi-core benchmark (4 cores): 424 ms (took 424.89ms)
Multi-core benchmark (8 cores): 594 ms (took 595.39ms)
Multi-core benchmark (16 cores): 907 ms (took 907.63ms)

--- Floating-Point Performance ---
Floating-point benchmark (10000000 iterations): 104 ms (took 104.31ms)

--- Averaged Benchmarks (3 iterations each) ---
Averaged single-core benchmark: 73 ms (took 220.03ms)
Averaged multi-core benchmark (16 cores): 185 ms (took 559.96ms)
Averaged floating-point benchmark: 96 ms (took 290.68ms)

--- Multi-Core Scaling Visualization ---
Lower is better:
1  cores: 411   ms |######################|
2  cores: 420   ms |#######################|
4  cores: 424   ms |#######################|
8  cores: 594   ms |################################|
16 cores: 907   ms |##################################################|

--- Speedup Ratios ---
Relative to single-core performance:
1  cores: 1x speedup
2  cores: 0.98x speedup
4  cores: 0.97x speedup
8  cores: 0.69x speedup
16 cores: 0.45x speedup

Ideal vs. Actual Scaling:
Cores | Ideal Speedup | Actual Speedup | Efficiency
------|---------------|----------------|----------
1     | 1.00          | 1.00           | 100.00%
2     | 2.00          | 0.98           | 48.93%
4     | 4.00          | 0.97           | 24.24%
8     | 8.00          | 0.69           | 8.65%
16    | 16.00         | 0.45           | 2.83%

CPU benchmarking completed!
```

### What This Example Demonstrates

1. **CPU Information Gathering**: Querying and displaying detailed CPU information
2. **Overall CPU Benchmark**: Testing general CPU performance
3. **Single-Core Performance**: Testing performance with different workload sizes
4. **Multi-Core Scaling**: Testing how performance scales with different thread counts
5. **Floating-Point Performance**: Testing floating-point computation performance
6. **Averaged Benchmarks**: Running multiple iterations for more consistent results
7. **Performance Visualization**: Using ASCII charts to visualize multi-core scaling
8. **Speedup Analysis**: Analyzing actual vs. ideal multi-core performance scaling

## Understanding the Results

### CPU Benchmark Score

The overall CPU benchmark score is a composite metric that represents general CPU performance. Higher scores indicate better performance.

### Single-Core Performance

This test measures how quickly the CPU can perform a fixed number of operations on a single core. Lower times (in milliseconds) indicate better performance.

### Multi-Core Scaling

This test shows how performance scales when using multiple CPU cores. Ideally, using more cores should reduce execution time proportionally, but real-world scaling is affected by factors like:

- Thread synchronization overhead
- Memory bandwidth limitations
- Cache contention
- Operating system scheduling

### Efficiency Percentage

The efficiency percentage shows how well the CPU utilizes multiple cores compared to the theoretical ideal:

- 100% efficiency means perfect scaling (doubling cores halves execution time)
- Lower percentages indicate diminishing returns when adding more cores
- Very low efficiency with many cores may indicate a bottleneck in the workload

In this example, we can see that the efficiency drops significantly as more cores are added, which is common for many workloads due to synchronization overhead and memory bottlenecks.

## Creating Your Own CPU Benchmarks

You can create your own CPU benchmark scripts based on the examples. Here's a minimal example:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Get CPU information
    let cpu_info = cpu::get_cpu_info()?;
    println!("CPU: {} with {} cores", cpu_info.name, cpu_info.logical_cores);
    
    // Run a simple benchmark
    let score = cpu::run_cpu_benchmark()?;
    println!("CPU Benchmark Score: {:.2}", score);
    
    // Test multi-core scaling
    let cores_to_test = [1, 2, 4, cpu_info.logical_cores];
    
    for &cores in &cores_to_test {
        if cores <= cpu_info.logical_cores {
            let duration = cpu::run_multi_core_benchmark(cores, 10_000_000)?;
            println!("{} cores: {} ms", cores, duration.as_millis());
        }
    }
    
    Ok(())
}
```

For more detailed examples, see the [CPU Benchmarking](/catp2p/docs/Benchmarking/cpu-benchmarking) and [API Reference](/catp2p/docs/api/benchmark/cpu) pages.
