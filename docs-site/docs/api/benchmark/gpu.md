---
sidebar_position: 3
---

# GPU Benchmarking API Reference

This page provides detailed API reference for the GPU benchmarking functionality in CatP2P.

## Structures

### `GpuBenchmarkContext`

A context for GPU benchmarks that manages GPU resources and allows reusing them across multiple benchmarks.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `gpu_info` | GpuInfo | Information about the GPU | `context.gpu_info.name` |
| `adapter` | Adapter | GPU adapter | (internal use) |
| `device` | Device | GPU device | (internal use) |
| `queue` | Queue | GPU command queue | (internal use) |

### `GpuBenchmarkResult`

Contains detailed information about the results of a GPU benchmark.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `gpu_model` | String | GPU model name | `result.gpu_model` |
| `gpu_vendor` | String | GPU vendor | `result.gpu_vendor` |
| `vram_estimate` | String | Estimated VRAM in GB | `result.vram_estimate` |
| `compute_score` | f64 | Compute performance score (MFLOPS) | `result.compute_score` |
| `texture_score` | f64 | Texture sampling performance score | `result.texture_score` |
| `geometry_score` | f64 | Geometry processing performance score | `result.geometry_score` |
| `memory_score` | f64 | Memory bandwidth performance score | `result.memory_score` |
| `overall_score` | f64 | Overall benchmark score (higher is better) | `result.overall_score` |
| `average_fps` | f64 | Average frames per second across all tests | `result.average_fps` |
| `test_results` | Vec&lt;GpuTestResult&gt; | Detailed results for each test | `result.test_results[0].score` |

### `GpuTestResult`

Contains detailed information about the results of a specific GPU test.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `test_name` | String | Name of the test | `test_result.test_name` |
| `average_fps` | f64 | Average frames per second | `test_result.average_fps` |
| `min_fps` | f64 | Minimum frames per second | `test_result.min_fps` |
| `max_fps` | f64 | Maximum frames per second | `test_result.max_fps` |
| `score` | f64 | Test score in MFLOPS (higher is better) | `test_result.score` |

### `GpuBenchmarkConfig`

Configuration options for GPU benchmarks.

| Field | Type | Description | Default Value | Example Access |
|-------|------|-------------|---------------|----------------|
| `test_duration_secs` | u64 | Duration of each test in seconds | 5 | `config.test_duration_secs` |
| `include_compute_test` | bool | Whether to include compute test | true | `config.include_compute_test` |
| `include_texture_test` | bool | Whether to include texture test | true | `config.include_texture_test` |
| `include_geometry_test` | bool | Whether to include geometry test | true | `config.include_geometry_test` |
| `include_memory_test` | bool | Whether to include memory test | true | `config.include_memory_test` |
| `complexity` | u32 | Test complexity (1-10) affecting matrix size | 5 | `config.complexity` |
| `window_width` | u32 | Width of benchmark window | 800 | `config.window_width` |
| `window_height` | u32 | Height of benchmark window | 600 | `config.window_height` |
| `show_window` | bool | Whether to show the benchmark window | false | `config.show_window` |

### `GpuInfo`

Information about a GPU.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `name` | String | GPU model name | `gpu_info.name` |
| `vendor` | String | GPU vendor | `gpu_info.vendor` |
| `driver` | String | GPU driver version | `gpu_info.driver` |
| `vram` | String | Estimated VRAM | `gpu_info.vram` |
| `backend` | String | Graphics API backend | `gpu_info.backend` |
| `is_integrated` | bool | Whether the GPU is integrated | `gpu_info.is_integrated` |

## Functions

### Context Management

| Function | Return Type | Description | Example Usage | Possible Errors |
|----------|-------------|-------------|--------------|-----------------|
| `GpuBenchmarkContext::new()` | Result&lt;GpuBenchmarkContext, Error&gt; | Creates a new GPU benchmark context | `let context = GpuBenchmarkContext::new()?;` | No compatible GPU found, device creation failed |
| `context.run_matrix_mult(duration, matrix_size)` | Result&lt;GpuTestResult, Error&gt; | Runs a matrix multiplication benchmark | `let result = context.run_matrix_mult(Duration::from_secs(5), 1024)?;` | Device lost, out of memory |
| `context.run_activation_functions(duration, data_size)` | Result&lt;GpuTestResult, Error&gt; | Runs a neural network activation functions benchmark | `let result = context.run_activation_functions(Duration::from_secs(2), 1_000_000)?;` | Device lost, out of memory |

### Information Gathering

| Function | Return Type | Description | Example Usage | Possible Errors |
|----------|-------------|-------------|--------------|-----------------|
| `get_gpu_info()` | Result&lt;GpuInfo, Error&gt; | Gets information about the GPU | `let gpu_info = gpu::get_gpu_info()?;` | No compatible GPU found |
| `is_gpu_available()` | bool | Checks if a compatible GPU is available | `if gpu::is_gpu_available() { ... }` | None |

### Performance Testing

| Function | Return Type | Description | Example Usage | Performance Impact |
|----------|-------------|-------------|--------------|-------------------|
| `run_gpu_benchmark()` | Result&lt;f64, Error&gt; | Runs a benchmark with default settings | `let score = gpu::run_gpu_benchmark()?;` | High - runs matrix multiplication test with default duration |
| `run_gpu_benchmark_with_config(config)` | Result&lt;GpuBenchmarkResult, Error&gt; | Runs a benchmark with custom configuration | `let result = gpu::run_gpu_benchmark_with_config(&config)?;` | Varies based on configuration |
| `run_matrix_mult_benchmark(adapter, duration, size)` | Result&lt;GpuTestResult, Error&gt; | Runs only the matrix multiplication test | `let result = gpu::run_matrix_mult_benchmark(&adapter, duration, 1024)?;` | Medium - runs only matrix multiplication test |
| `run_activation_functions_benchmark(adapter, duration, data_size)` | Result&lt;GpuTestResult, Error&gt; | Runs only the activation functions test | `let result = gpu::run_activation_functions_benchmark(&adapter, duration, 1_000_000)?;` | Medium - runs only activation functions test |

## Understanding GPU Benchmark Results

The GPU benchmark in CatP2P includes two main tests:

1. **Matrix Multiplication**: How efficiently the GPU can multiply large matrices
2. **Neural Network Activation Functions**: How efficiently the GPU can compute common activation functions used in neural networks

### Matrix Multiplication Benchmark

This benchmark measures the GPU's ability to perform matrix multiplication operations, which are fundamental to many GPU computing tasks:

- **Score in MFLOPS**: Millions of floating-point operations per second
- **Higher scores indicate better GPU compute performance**
- **Matrix size**: Calculated as 512 + (complexity * 128), where complexity ranges from 1 to 10

### Activation Functions Benchmark

This benchmark measures the GPU's ability to compute common neural network activation functions:

- **Operations tested**: ReLU, Sigmoid, Tanh, and Leaky ReLU
- **Score**: Based on millions of operations per second
- **Higher scores indicate better performance for AI and deep learning applications**

### Interpreting the Score

The GPU benchmark score represents:

- Higher scores indicate better GPU compute performance
- Scores are influenced by:
  - GPU architecture and generation
  - Number of compute units/cores
  - Memory bandwidth and capacity
  - Driver optimization
  - System configuration

### Typical Performance Ranges

| GPU Type | Typical Matrix Multiplication Score (MFLOPS) | Expected Performance |
|----------|---------------------------------------------|---------------------|
| High-end Desktop GPU | 5,000,000 - 15,000,000 | Excellent for complex parallel computing |
| Mid-range Desktop GPU | 1,000,000 - 5,000,000 | Good for most parallel computing tasks |
| Entry-level Desktop GPU | 200,000 - 1,000,000 | Suitable for basic parallel computing |
| High-end Integrated GPU | 50,000 - 200,000 | Limited parallel computing capability |
| Basic Integrated GPU | 5,000 - 50,000 | Minimal parallel computing capability |

Note: Actual performance can vary significantly based on specific hardware, system conditions, and benchmark parameters.

### Factors Affecting Benchmark Results

| Factor | Impact | Notes |
|--------|--------|-------|
| Matrix Size | High | Larger matrices provide more accurate results but may hit memory limits |
| Data Size | Medium | Larger data sizes for activation functions provide more accurate results |
| System Activity | Medium | Other processes using the GPU can reduce benchmark scores |
| Driver Version | Medium | Updated drivers can provide performance improvements |
| Thermal Throttling | High | GPUs may slow down if they overheat during benchmarking |
| Power Limits | Medium | Power-limited systems (like laptops) may show lower performance |
| API Overhead | Low | Different graphics APIs have different overheads |

## Implementation Details

### Matrix Multiplication Benchmark

The matrix multiplication benchmark measures how quickly the GPU can multiply two large matrices:

1. Two random matrices of size NxN are created (where N is determined by the complexity parameter)
2. The matrices are uploaded to GPU memory
3. A compute shader multiplies the matrices
4. The process is repeated for the specified duration
5. Performance is measured in MFLOPS (Millions of Floating Point Operations Per Second)

The matrix size is calculated as: 512 + (complexity * 128), where complexity ranges from 1 to 10.

### Activation Functions Benchmark

The activation functions benchmark measures how quickly the GPU can compute common neural network activation functions:

1. Random input data of the specified size is created
2. The data is uploaded to GPU memory
3. A compute shader applies four activation functions (ReLU, Sigmoid, Tanh, Leaky ReLU) to each element
4. The process is repeated for the specified duration
5. Performance is measured in millions of operations per second

### Resource Management

The GPU benchmarking module uses a context-based approach to manage GPU resources:

1. A `GpuBenchmarkContext` is created once
2. This context holds the GPU device and command queue
3. Multiple benchmarks can be run using the same context
4. This approach avoids device creation/destruction overhead and potential driver issues

### Graphics API Selection

The GPU benchmarking functionality uses wgpu, which automatically selects the best available graphics API:

1. Vulkan on supported systems (Linux, Windows, Android)
2. Metal on macOS and iOS
3. DirectX 12 on Windows
4. DirectX 11 on older Windows systems
5. OpenGL as a fallback

## Error Handling

The GPU benchmarking functions use Rust's `Result` type to handle errors gracefully. Common errors include:

- `Error::Benchmark("No suitable GPU adapter found")`: System lacks required GPU capabilities
- `Error::Benchmark("Failed to create device: {error}")`: GPU initialization failed
- `Error::Benchmark("No matrix multiplications were performed during the benchmark")`: Benchmark failed to run any iterations
- `Error::Benchmark("No activation function operations were performed during the benchmark")`: Benchmark failed to run any iterations
```