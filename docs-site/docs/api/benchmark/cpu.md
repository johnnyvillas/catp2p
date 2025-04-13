---
sidebar_position: 1
---

# CPU Benchmarking API Reference

This page provides detailed API reference for the CPU benchmarking functionality in CatP2P.

## Structures

### `CpuInfo`

Contains detailed information about the CPU.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `name` | String | The model name of the CPU | `cpu_info.name` |
| `vendor` | String | The manufacturer of the CPU | `cpu_info.vendor` |
| `cores` | u32 | Number of physical CPU cores | `cpu_info.cores` |
| `logical_cores` | u32 | Number of logical CPU cores (including hyperthreading) | `cpu_info.logical_cores` |
| `frequency` | Option`<u64>` | CPU frequency in MHz, if available | `if let Some(freq) = cpu_info.frequency { ... }` |
| `usage` | f32 | Current CPU usage percentage (0-100) | `cpu_info.usage` |
| `cache_line_size` | Option`<u32>` | CPU cache line size in bytes, if available | `if let Some(size) = cpu_info.cache_line_size { ... }` |
| `l1_cache_size` | Option`<u32>` | L1 cache size in KB, if available | `if let Some(size) = cpu_info.l1_cache_size { ... }` |
| `l2_cache_size` | Option`<u32>` | L2 cache size in KB, if available | `if let Some(size) = cpu_info.l2_cache_size { ... }` |
| `l3_cache_size` | Option`<u32>` | L3 cache size in KB, if available | `if let Some(size) = cpu_info.l3_cache_size { ... }` |

### `CpuBenchmarkResult`

Contains the results of a CPU benchmark.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `score` | f64 | Overall benchmark score | `result.score` |
| `single_core_score` | f64 | Single-core performance score | `result.single_core_score` |
| `multi_core_score` | f64 | Multi-core performance score | `result.multi_core_score` |
| `floating_point_score` | f64 | Floating-point performance score | `result.floating_point_score` |
| `integer_score` | f64 | Integer performance score | `result.integer_score` |
| `memory_score` | f64 | Memory performance score | `result.memory_score` |
| `cpu_info` | CpuInfo | Information about the CPU that was benchmarked | `result.cpu_info` |

## Functions

### Information Functions

#### `get_cpu_info() -> Result<CpuInfo, Error>`

Gets detailed information about the CPU.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cpu_info = cpu::get_cpu_info()?;
    println!("CPU: {} with {} cores", cpu_info.name, cpu_info.cores);
    Ok(())
}
```

#### `get_cpu_usage() -> Result<f32, Error>`

Gets the current CPU usage percentage.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usage = cpu::get_cpu_usage()?;
    println!("Current CPU usage: {:.2}%", usage);
    Ok(())
}
```

### Benchmark Functions

#### `run_cpu_benchmark() -> Result<f64, Error>`

Runs a comprehensive CPU benchmark and returns an overall score.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let score = cpu::run_cpu_benchmark()?;
    println!("CPU benchmark score: {:.2}", score);
    Ok(())
}
```

#### `run_single_core_benchmark(iterations: u64) -> Result<std::time::Duration, Error>`

Runs a single-core benchmark with the specified number of iterations.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iterations = 50_000_000;
    let duration = cpu::run_single_core_benchmark(iterations)?;
    println!("Single-core benchmark took: {:?}", duration);
    Ok(())
}
```

#### `run_multi_core_benchmark(threads: u32, iterations_per_thread: u64) -> Result<std::time::Duration, Error>`

Runs a multi-core benchmark with the specified number of threads and iterations per thread.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let threads = 4;
    let iterations_per_thread = 50_000_000;
    let duration = cpu::run_multi_core_benchmark(threads, iterations_per_thread)?;
    println!("Multi-core benchmark with {} threads took: {:?}", threads, duration);
    Ok(())
}
```

#### `run_floating_point_benchmark(iterations: u64) -> Result<std::time::Duration, Error>`

Runs a floating-point benchmark with the specified number of iterations.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iterations = 10_000_000;
    let duration = cpu::run_floating_point_benchmark(iterations)?;
    println!("Floating-point benchmark took: {:?}", duration);
    Ok(())
}
```

#### `run_integer_benchmark(iterations: u64) -> Result<std::time::Duration, Error>`

Runs an integer benchmark with the specified number of iterations.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iterations = 100_000_000;
    let duration = cpu::run_integer_benchmark(iterations)?;
    println!("Integer benchmark took: {:?}", duration);
    Ok(())
}
```

#### `run_memory_benchmark(size_mb: u32) -> Result<std::time::Duration, Error>`

Runs a memory benchmark with the specified amount of memory in megabytes.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size_mb = 1024; // 1 GB
    let duration = cpu::run_memory_benchmark(size_mb)?;
    println!("Memory benchmark with {} MB took: {:?}", size_mb, duration);
    Ok(())
}
```

#### `run_averaged_benchmark<F, T, E>(runs: u32, benchmark_fn: F) -> Result<T, Error> where F: Fn() -> Result<T, E>, T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + From<u32> + Copy, E: std::fmt::Display`

Runs a benchmark function multiple times and returns the average result.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the single-core benchmark 3 times and average the results
    let avg_duration = cpu::run_averaged_benchmark(3, || {
        cpu::run_single_core_benchmark(10_000_000)
    })?;
    
    println!("Averaged single-core benchmark: {:?}", avg_duration);
    Ok(())
}
```

#### `run_detailed_cpu_benchmark() -> Result<CpuBenchmarkResult, Error>`

Runs a comprehensive CPU benchmark and returns detailed results.

```rust
use catp2p::benchmark::cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = cpu::run_detailed_cpu_benchmark()?;
    
    println!("CPU: {}", result.cpu_info.name);
    println!("Overall score: {:.2}", result.score);
    println!("Single-core score: {:.2}", result.single_core_score);
    println!("Multi-core score: {:.2}", result.multi_core_score);
    println!("Floating-point score: {:.2}", result.floating_point_score);
    println!("Integer score: {:.2}", result.integer_score);
    println!("Memory score: {:.2}", result.memory_score);
    
    Ok(())
}
```

## Error Handling

All functions that return a `Result` can produce the following errors:

- `Error::Benchmark`: Indicates an error during CPU benchmarking
- `Error::Timeout`: Indicates that a CPU operation timed out

Example of proper error handling:

```rust
use catp2p::benchmark::cpu;
use catp2p::error::Error;

fn main() {
    match cpu::get_cpu_info() {
        Ok(cpu_info) => {
            println!("CPU: {}", cpu_info.name);
        },
        Err(Error::Benchmark(msg)) => {
            println!("Benchmark error: {}", msg);
        },
        Err(Error::Timeout) => {
            println!("Operation timed out");
        },
        Err(e) => {
            println!("Other error: {}", e);
        }
    }
}
```

## Platform-Specific Considerations

### Windows

- Uses Windows Management Instrumentation (WMI) for CPU information
- May require administrative privileges for some detailed information

### Linux

- Uses `/proc/cpuinfo` and `/proc/stat` for CPU information
- May use `lscpu` as a fallback

### macOS

- Uses `sysctl` for CPU information
- May have limited access to some low-level CPU details

## Performance Considerations

- Benchmarks can be CPU-intensive and may affect system performance while running
- For accurate results, close other applications before running benchmarks
- Benchmark results can vary based on system load, power settings, and thermal conditions
- Consider running benchmarks multiple times for more consistent results

## Related Resources

- [CPU Benchmarking Documentation](/docs/docs/Benchmarking/cpu-benchmarking)
- [CPU Benchmark Examples](/docs/Examples/benchmarks/cpu-benchmark-example)
- [GPU Benchmarking API Reference](/docs/api/benchmark/gpu)
