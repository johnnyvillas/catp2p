---
sidebar_position: 3
---

# GPU Benchmarking

CatP2P provides GPU benchmarking functionality to assess the compute capabilities of graphics processing units. This is essential for distributed applications that need to allocate GPU-intensive tasks efficiently.

## Overview

The GPU benchmarking module allows you to:

- Benchmark GPU compute performance using matrix multiplication
- Test neural network activation function performance
- Get detailed information about the GPU
- Compare performance across different graphics cards
- Customize benchmark parameters

## Basic Usage

### Simple Benchmark

To run a simple GPU benchmark with default settings:

```rust
use catp2p::benchmark::gpu;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Run a simple benchmark and get the overall score
    let score = gpu::run_gpu_benchmark()?;
    println!("GPU benchmark score: {:.2} MFLOPS", score);
    
    Ok(())
}
```

### Getting GPU Information

To get information about the GPU without running a benchmark:

```rust
use catp2p::benchmark::gpu::get_gpu_info;

fn main() -> Result<(), catp2p::error::Error> {
    // Get GPU information
    let gpu_info = get_gpu_info()?;
    
    println!("GPU: {} ({})", gpu_info.name, gpu_info.vendor);
    println!("Driver: {}", gpu_info.driver);
    println!("Estimated VRAM: {}", gpu_info.vram);
    println!("Backend: {}", gpu_info.backend);
    println!("Type: {}", if gpu_info.is_integrated { "Integrated" } else { "Discrete" });
    
    Ok(())
}
```

### Using the GPU Benchmark Context

For more control and better performance, use the `GpuBenchmarkContext`:

```rust
use catp2p::benchmark::gpu::GpuBenchmarkContext;
use std::time::Duration;

fn main() -> Result<(), catp2p::error::Error> {
    // Create a benchmark context
    let context = GpuBenchmarkContext::new()?;
    
    println!("Testing GPU: {}", context.gpu_info.name);
    
    // Run matrix multiplication benchmark with different sizes
    let sizes = [512, 1024, 2048];
    
    for &size in &sizes {
        println!("Running benchmark with {}x{} matrices...", size, size);
        
        let result = context.run_matrix_mult(Duration::from_secs(3), size)?;
        
        println!("  Score: {:.2} MFLOPS", result.score);
        println!("  Average FPS: {:.2}", result.average_fps);
    }
    
    // Run activation functions benchmark
    let data_size = 1_000_000; // 1 million elements
    println!("Running activation functions benchmark with {} elements...", data_size);
    
    let result = context.run_activation_functions(Duration::from_secs(2), data_size)?;
    
    println!("  Score: {:.2}", result.score);
    println!("  Average FPS: {:.2}", result.average_fps);
    
    Ok(())
}
```

### Custom Benchmark Configuration

To run a GPU benchmark with custom configuration:

```rust
use catp2p::benchmark::gpu::{GpuBenchmarkConfig, run_gpu_benchmark_with_config};

fn main() -> Result<(), catp2p::error::Error> {
    // Create a custom configuration
    let config = GpuBenchmarkConfig {
        test_duration_secs: 5,           // Duration of each test in seconds
        complexity: 8,                   // Higher complexity (larger matrices)
        ..Default::default()
    };
    
    // Run the benchmark with custom configuration
    let result = run_gpu_benchmark_with_config(&config)?;
    
    println!("Benchmark results for {}:", result.gpu_model);
    println!("  Compute Score: {:.2} MFLOPS", result.compute_score);
    println!("  Overall Score: {:.2}", result.overall_score);
    println!("  Average FPS: {:.2}", result.average_fps);
    
    Ok(())
}
```

## Advanced Usage

### Analyzing Performance Scaling

A common use case is to analyze how GPU performance scales with workload size:

```rust
use catp2p::benchmark::gpu::GpuBenchmarkContext;
use std::time::Duration;
use std::io::{self, Write};

fn main() -> Result<(), catp2p::error::Error> {
    // Create a benchmark context
    let context = GpuBenchmarkContext::new()?;
    
    // Run benchmarks with different complexity levels
    let complexity_levels = [1, 3, 5, 8, 10];
    let mut results = Vec::new();
    
    for &complexity in &complexity_levels {
        // Calculate matrix size based on complexity
        let matrix_size = 512 + (complexity * 128);
        
        print!("Testing {}x{} matrices... ", matrix_size, matrix_size);
        io::stdout().flush().unwrap();
        
        let result = context.run_matrix_mult(Duration::from_secs(2), matrix_size)?;
        
        println!("Score: {:.2} MFLOPS", result.score);
        results.push((complexity, result.score));
    }
    
    // Find optimal complexity for this GPU
    if let Some(&(optimal_complexity, max_score)) = results.iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)) {
        
        println!("Optimal complexity for this GPU: {}", optimal_complexity);
        println!("Peak performance: {:.2} MFLOPS", max_score);
        
        // Calculate optimal matrix size
        let optimal_size = 512 + (optimal_complexity * 128);
        println!("Optimal matrix size: {}x{}", optimal_size, optimal_size);
    }
    
    Ok(())
}
```

### Visualizing Performance Results

You can create a simple ASCII chart to visualize performance scaling:

```rust
// After collecting results as (complexity, score) pairs:
let max_score = results.iter().map(|&(_, score)| score as usize).max().unwrap_or(1);
let scale = 50.0 / max_score as f64;

println!("Performance scaling (higher is better):");
for &(complexity, score) in &results {
    let bar_length = (score as f64 * scale) as usize;
    let bar = "#".repeat(bar_length);
    println!("Complexity {:2}: {:10.2} MFLOPS |{}|", complexity, score, bar);
}
```

### Comparing Matrix Multiplication and Activation Functions

You can compare different types of GPU workloads to understand your GPU's strengths:

```rust
use catp2p::benchmark::gpu::GpuBenchmarkContext;
use std::time::Duration;

fn main() -> Result<(), catp2p::error::Error> {
    // Create a benchmark context
    let context = GpuBenchmarkContext::new()?;
    
    println!("GPU: {}", context.gpu_info.name);
    
    // Run matrix multiplication benchmark
    let matrix_size = 1024;
    println!("Running matrix multiplication benchmark ({}x{})...", matrix_size, matrix_size);
    let matrix_result = context.run_matrix_mult(Duration::from_secs(3), matrix_size)?;
    println!("Matrix multiplication score: {:.2} MFLOPS", matrix_result.score);
    
    // Run activation functions benchmark
    let data_size = 1_000_000;
    println!("Running activation functions benchmark ({} elements)...", data_size);
    let activation_result = context.run_activation_functions(Duration::from_secs(3), data_size)?;
    println!("Activation functions score: {:.2}", activation_result.score);
    
    // Compare the results (note: scores are in different units, this is just for relative performance)
    if matrix_result.average_fps > activation_result.average_fps {
        println!("This GPU performs better at matrix operations relative to activation functions");
    } else {
        println!("This GPU performs better at activation functions relative to matrix operations");
    }
    
    Ok(())
}
```

### Determining Task Suitability

You can use the benchmark results to determine if a GPU is suitable for specific tasks:

```rust
use catp2p::benchmark::gpu::GpuBenchmarkContext;
use std::time::Duration;

fn main() -> Result<(), catp2p::error::Error> {
    // Create a benchmark context
    let context = GpuBenchmarkContext::new()?;
    
    // Run a benchmark with a moderate workload
    let matrix_size = 1024;
    let result = context.run_matrix_mult(Duration::from_secs(3), matrix_size)?;
    
    println!("GPU: {}", context.gpu_info.name);
    println!("Compute Score: {:.2} MFLOPS", result.score);
    
    // Determine suitability for different tasks
    if result.score > 5_000_000.0 {
        println!("This GPU is excellent for:");
        println!("- Complex matrix operations");
        println!("- Machine learning workloads");
        println!("- Scientific simulations");
    } else if result.score > 1_000_000.0 {
        println!("This GPU is good for:");
        println!("- Moderate matrix operations");
        println!("- Basic machine learning tasks");
        println!("- Data processing");
    } else if result.score > 100_000.0 {
        println!("This GPU has limited capabilities for:");
        println!("- Simple matrix operations");
        println!("- Basic data processing");
    } else {
        println!("This GPU is not recommended for compute tasks");
        println!("Consider using CPU for these workloads instead");
    }
    
    Ok(())
}
```

## Understanding the Benchmarks

CatP2P includes two main GPU benchmarks:

### Matrix Multiplication Benchmark

The matrix multiplication benchmark is a standard way to measure GPU compute performance:

1. **What it measures**: How quickly the GPU can multiply two large matrices
2. **Why it matters**: Matrix multiplication is a fundamental operation in many GPU workloads:
   - Machine learning (neural network layers)
   - Scientific computing
   - Data processing
   - Computer graphics (transformations)

3. **How it works**:
   - Two random matrices of size NxN are created
   - The matrices are uploaded to GPU memory
   - A compute shader multiplies the matrices
   - The process is repeated for the specified duration
   - Performance is measured in MFLOPS (Millions of Floating Point Operations Per Second)

4. **Matrix size and complexity**:
   - Matrix size is calculated as: 512 + (complexity * 128)
   - Complexity ranges from 1 to 10
   - Larger matrices provide more accurate results but may hit memory limits

### Activation Functions Benchmark

The activation functions benchmark measures the GPU's ability to compute common neural network activation functions:

1. **What it measures**: How quickly the GPU can compute activation functions on large arrays of data
2. **Why it matters**: Activation functions are essential operations in neural networks:
   - Used in every layer of neural networks
   - Critical for deep learning performance
   - Representative of many AI workloads

3. **How it works**:
   - A large array of random input data is created
   - The data is uploaded to GPU memory
   - A compute shader applies four activation functions to each element:
     - ReLU (Rectified Linear Unit)
     - Sigmoid
     - Tanh (Hyperbolic Tangent)
     - Leaky ReLU
   - The process is repeated for the specified duration
   - Performance is measured in millions of operations per second

4. **Data size**:
   - Controlled by the `data_size` parameter
   - Larger data sizes provide more accurate results but require more memory

## Performance Considerations

- GPU benchmarking can be resource-intensive and may temporarily impact system performance
- For the most accurate results, close other GPU-intensive applications during benchmarking
- Results may vary based on driver versions and system conditions
- Performance can be affected by thermal throttling during extended tests
- Different GPUs have different optimal workload sizes - a GPU might perform better with certain matrix sizes
- The benchmark uses compute shaders, which may not be available on very old GPUs

## Error Handling

The GPU benchmarking functions return `Result<T, Error>` types, allowing you to handle errors gracefully:

```rust
match GpuBenchmarkContext::new() {
    Ok(context) => {
        // Use the context for benchmarking
    },
    Err(e) => println!("GPU benchmarking not available: {}", e),
}
```

Common errors include:
- No compatible GPU found
- Insufficient GPU capabilities for compute shaders
- Driver or API compatibility issues
- System resource limitations

## Tracking Benchmark Progress

For long-running benchmarks, you might want to track progress:

```rust
use catp2p::benchmark::gpu::GpuBenchmarkContext;
use std::time::{Duration, Instant};
use std::io::{self, Write};

fn main() -> Result<(), catp2p::error::Error> {
    let context = GpuBenchmarkContext::new()?;
    
    // Show progress indicators
    print!("Running matrix multiplication benchmark... ");
    io::stdout().flush().unwrap();
    
    let benchmark_start = Instant::now();
    let result = context.run_matrix_mult(Duration::from_secs(5), 1024)?;
    let benchmark_time = benchmark_start.elapsed();
    
    println!("Done! (took {:.2?})", benchmark_time);
    println!("Score: {:.2} MFLOPS", result.score);
    
    Ok(())
}
```

## API Reference

For detailed API information, see the [GPU Benchmarking API Reference](/catp2p/docs/api/benchmark/gpu).

## Examples

For practical examples of GPU benchmarking, see the [GPU Benchmark Examples](/catp2p/docs/Examples/gpu-benchmark-example) page.
```
