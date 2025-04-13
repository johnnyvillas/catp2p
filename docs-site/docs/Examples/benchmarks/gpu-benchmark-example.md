---
sidebar_position: 1
---

# GPU Benchmark Examples

This page provides examples of how to use the GPU benchmarking functionality in CatP2P.

## Basic GPU Benchmark Example

The `examples/gpu_benchmark.rs` file in the CatP2P repository demonstrates comprehensive GPU benchmarking capabilities.

### Running the Example

```bash
cargo run --example gpu_benchmark
```

### Sample Output

```
=== CatP2P GPU Benchmarking Example ===

--- GPU Information ---
GPU Model: NVIDIA GeForce GTX 1060 6GB
GPU Vendor: 4318
GPU Driver: Driver: "NVIDIA", Backend: Vulkan
Estimated VRAM: 8+ GB
Backend: Vulkan
Type: Discrete

--- GPU Matrix Multiplication Benchmark ---
Running matrix multiplication benchmark... Done! (took 5.21s)
Test: Matrix Multiplication (1024x1024)
Score: 5043598.09 MFLOPS
Average FPS: 1824.87
Min/Max FPS: 1532.49/2105.26

--- GPU Scaling with Complexity ---
Running benchmarks with different complexity levels...
Complexity level 3: Done! (took 2.13s)
  Matrix size: 896x896
  Score: 3421587.32 MFLOPS
  Average FPS: 4812.76
Complexity level 5: Done! (took 2.18s)
  Matrix size: 1152x1152
  Score: 7865432.18 MFLOPS
  Average FPS: 5134.21
Complexity level 8: Done! (took 2.25s)
  Matrix size: 1536x1536
  Score: 12876543.21 MFLOPS
  Average FPS: 3567.89

--- Intensive GPU Benchmark ---
Running intensive benchmark with complexity level 10... Done! (took 3.15s)
Test: Matrix Multiplication (1792x1792)
Score: 17637699.80 MFLOPS
Average FPS: 3087.54

--- GPU Performance Scaling with Complexity ---
Higher is better:
Complexity  3:  3421587.32 MFLOPS |####################                              |
Complexity  5:  7865432.18 MFLOPS |###########################################       |
Complexity  8: 12876543.21 MFLOPS |################################################  |
Complexity 10: 17637699.80 MFLOPS |##################################################|

--- Neural Network Activation Functions Benchmark ---
Preparing benchmark data (1000000 elements)... Done! (took 0.05s)
Running activation functions benchmark... Done! (took 2.08s)
Test: Neural Network Activation Functions
Data size: 1000000 elements
Score: 4532876.54
Average FPS: 1132.45
Min/Max FPS: 987.65/1243.21
Operations: ReLU, Sigmoid, Tanh, Leaky ReLU

--- GPU Benchmark Summary ---
GPU benchmarking completed!
Tested GPU: NVIDIA GeForce GTX 1060 6GB (Discrete)
Average matrix multiplication performance: 8959077.89 MFLOPS
Peak matrix multiplication performance: 17637699.80 MFLOPS
```

### What This Example Demonstrates

1. **GPU Information Gathering**: Querying and displaying detailed GPU information
2. **Matrix Multiplication Benchmark**: Testing basic compute performance
3. **Complexity Scaling**: Testing how performance scales with different workload sizes
4. **Activation Functions Benchmark**: Testing neural network operations performance
5. **Performance Visualization**: Using ASCII charts to visualize performance scaling
6. **Benchmark Summary**: Providing an overview of the GPU's capabilities

## Creating Your Own Benchmarks

You can create your own GPU benchmark scripts based on the examples. Here's a minimal example:

```rust
use catp2p::benchmark::gpu::GpuBenchmarkContext;
use std::time::Duration;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Create a benchmark context
    let context = GpuBenchmarkContext::new()?;
    
    // Run a matrix multiplication benchmark
    let result = context.run_matrix_mult(Duration::from_secs(3), 1024)?;
    
    println!("GPU: {}", context.gpu_info.name);
    println!("Score: {:.2} MFLOPS", result.score);
    
    Ok(())
}
```

