---
sidebar_position: 2
---

# Memory Benchmarking API Reference

This page provides detailed API reference for the memory benchmarking functionality in CatP2P.

## Structures

### `MemoryInfo`

Contains detailed information about the system's memory.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `total_memory` | u64 | Total physical memory in bytes | `memory_info.total_memory` |
| `available_memory` | u64 | Available memory in bytes | `memory_info.available_memory` |
| `used_memory` | u64 | Used memory in bytes (total - available) | `memory_info.used_memory` |
| `usage_percent` | f64 | Memory usage as a percentage (0.0 - 100.0) | `memory_info.usage_percent` |
| `memory_per_core` | u64 | Memory per CPU core in bytes | `memory_info.memory_per_core` |

## Functions

### Information Gathering

| Function | Return Type | Description | Example Usage | Possible Errors |
|----------|-------------|-------------|--------------|-----------------|
| `get_memory_info()` | Result\<MemoryInfo, Error\> | Retrieves detailed information about the system's memory | `let info = memory::get_memory_info()?;` | System access errors |

### Performance Testing

| Function | Return Type | Description | Example Usage | Performance Impact |
|----------|-------------|-------------|--------------|-------------------|
| `run_memory_benchmark()` | Result\<f64, Error\> | Runs a comprehensive memory benchmark and returns an overall score (higher is better) | `let score = memory::run_memory_benchmark()?;` | High - runs all benchmarks, takes several seconds |
| `run_allocation_benchmark()` | Result\<f64, Error\> | Tests memory allocation/deallocation performance (higher is better) | `let score = memory::run_allocation_benchmark()?;` | Medium - allocates various memory sizes |
| `run_read_write_benchmark()` | Result\<f64, Error\> | Tests sequential memory read/write performance (higher is better) | `let score = memory::run_read_write_benchmark()?;` | High - allocates ~100MB buffer |
| `run_random_access_benchmark()` | Result\<f64, Error\> | Tests random memory access performance (higher is better) | `let score = memory::run_random_access_benchmark()?;` | High - allocates ~100MB buffer |

### Function Relationships

| Function | Related Functions | Notes |
|----------|-------------------|-------|
| `run_memory_benchmark()` | `run_allocation_benchmark()`, `run_read_write_benchmark()`, `run_random_access_benchmark()` | Calls all three specific benchmarks and combines their scores |
| `get_memory_info()` | `cpu::get_cpu_info()` | Often used together to get a complete system overview |

### Parameter Details

All memory benchmark functions take no parameters, making them simple to use.

## Understanding Memory Benchmark Results

### Score Interpretation

The memory benchmark functions return scores where higher values indicate better performance:

- `run_allocation_benchmark()`: Measures how quickly memory can be allocated and deallocated
- `run_read_write_benchmark()`: Measures sequential memory access performance
- `run_random_access_benchmark()`: Measures random memory access performance
- `run_memory_benchmark()`: Combines the above scores into an overall memory performance score

### Typical Score Ranges

| Memory Type | Allocation Score | Read/Write Score | Random Access Score | Overall Score |
|-------------|-----------------|------------------|---------------------|---------------|
| High-end DDR4/DDR5 | 800-1500+ | 500-1000+ | 300-800+ | 500-1000+ |
| Mid-range DDR4 | 500-800 | 300-500 | 200-300 | 300-500 |
| Basic DDR3/DDR4 | 300-500 | 200-300 | 100-200 | 200-300 |
| Low-power systems | 100-300 | 50-200 | 50-100 | 50-200 |

Note: Actual scores can vary significantly based on specific hardware, system conditions, and memory configuration.

### Factors Affecting Benchmark Results

| Factor | Impact | Notes |
|--------|--------|-------|
| Memory Speed (MHz) | High | Higher frequency memory generally performs better |
| Memory Channels | High | Dual/quad channel configurations improve bandwidth |
| Memory Timings | Medium | Lower CAS latency and other timings improve performance |
| CPU Cache | High | Larger CPU caches can mask memory performance issues |
| System Load | Medium | Other processes using memory can affect benchmark results |
| Memory Fragmentation | Medium | Long-running systems may have fragmented memory |
| NUMA Configuration | Medium | Non-Uniform Memory Access affects performance on multi-socket systems |

## Implementation Details

### Benchmark Methodology

The memory benchmarking in CatP2P uses a combination of techniques to measure memory performance:

1. **Allocation Benchmark**: Tests allocation and deallocation of memory blocks of various sizes
2. **Read/Write Benchmark**: Tests sequential reading and writing to a large memory buffer
3. **Random Access Benchmark**: Tests random access patterns across a large memory buffer

Each benchmark is designed to:
- Prevent compiler optimizations from skewing results
- Provide consistent results across different hardware
- Test realistic memory access patterns

### Score Calculation

All benchmark scores are calculated using the formula:
```
score = 1000.0 / elapsed_time_in_seconds
```

This means that faster execution results in higher scores. The overall memory benchmark score is the average of the three individual benchmark scores.

## Error Handling

The memory benchmarking functions use Rust's `Result` type to handle errors gracefully. Common errors include:

- `Error::Resource("Failed to retrieve system memory information: {error}")`: System API errors
- `Error::Resource("Memory allocation failed: {error}")`: Out of memory errors

## Performance Considerations

When running memory benchmarks, be aware of these performance considerations:

1. **Memory Usage**: The benchmarks allocate significant amounts of memory, especially the read/write and random access tests
2. **System Load**: Running benchmarks on a system with high memory usage may affect results
3. **Warm-up Effects**: The first run of a benchmark may be slower due to cache and TLB warming
4. **Background Activity**: System background tasks can introduce variance in results

For the most accurate results:

1. Run benchmarks multiple times and average the results
2. Ensure the system has sufficient free memory
3. Close unnecessary applications and services
4. Be consistent in your testing environment

## Advanced Usage

### Combining with CPU Benchmarks

Memory and CPU performance are closely related. For a complete system assessment, combine memory benchmarks with CPU benchmarks:

```rust
use catp2p::benchmark::{memory, cpu};
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Get system information
    let cpu_info = cpu::get_cpu_info()?;
    let memory_info = memory::get_memory_info()?;
    
    println!("System Information:");
    println!("CPU: {} ({} cores)", cpu_info.name, cpu_info.logical_cores);
    println!("Memory: {:.2} GB", memory_info.total_memory as f64 / 1_073_741_824.0);
    
    // Run benchmarks
    let cpu_score = cpu::run_cpu_benchmark()?;
    let memory_score = memory::run_memory_benchmark()?;
    
    println!("Benchmark Results:");
    println!("CPU Score: {:.2}", cpu_score);
    println!("Memory Score: {:.2}", memory_score);
    
    // Calculate a combined system score
    let system_score = (cpu_score * 0.6) + (memory_score * 0.4);
    println!("Combined System Score: {:.2}", system_score);
    
    Ok(())
}
```

### Memory-to-CPU Ratio Analysis

The ratio of memory to CPU cores is an important metric for distributed computing:

```rust
use catp2p::benchmark::{memory, cpu};
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    let cpu_info = cpu::get_cpu_info()?;
    let memory_info = memory::get_memory_info()?;
    
    // Calculate memory per core in GB
    let memory_per_core_gb = memory_info.memory_per_core as f64 / 1_073_741_824.0;
    
    println!("Memory per CPU core: {:.2} GB", memory_per_core_gb);
    
    // Analyze the ratio
    if memory_per_core_gb < 1.0 {
        println!("Warning: Limited memory per core. Consider reducing parallel workloads.");
    } else if memory_per_core_gb > 4.0 {
        println!("Excellent memory-to-CPU ratio. Suitable for memory-intensive tasks.");
    } else {
        println!("Good memory-to-CPU ratio. Suitable for most workloads.");
    }
    
    Ok(())
}
```