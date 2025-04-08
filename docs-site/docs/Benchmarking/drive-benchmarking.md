---
sidebar_position: 2
---

# Drive Benchmarking

CatP2P provides comprehensive drive benchmarking functionality to assess the performance of storage devices. This is crucial for distributed applications that need to store and retrieve data efficiently.

## Overview

The drive benchmarking module allows you to:

- Benchmark individual drives or all available drives
- Measure write speed, read speed, and random access performance
- Get detailed information about available drives
- Compare performance across different storage devices
- Customize benchmark parameters

## Basic Usage

### Simple Benchmark

To run a simple benchmark on the system's temporary directory:

```rust
use catp2p::benchmark::drives;
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // Run a simple benchmark and get the overall score
    let score = drives::run_drive_benchmark()?;
    println!("Drive benchmark score: {:.2}", score);
    
    Ok(())
}
```

### Benchmarking a Specific Drive

To benchmark a specific drive with custom parameters:

```rust
use catp2p::benchmark::drives::{DriveBenchmarkConfig, run_drive_benchmark_with_config};
use std::path::Path;

fn main() -> Result<(), catp2p::error::Error> {
    // Create a custom configuration
    let config = DriveBenchmarkConfig {
        file_size_mb: 100,           // Size of test file in MB
        random_access_ops: 1000,     // Number of random access operations
        include_random_access: true, // Include random access test
    };
    
    // Benchmark a specific drive
    let drive_path = Path::new("C:\\"); // Windows example
    // let drive_path = Path::new("/"); // Unix example
    
    let result = run_drive_benchmark_with_config(drive_path, &config)?;
    
    println!("Benchmark results for {}:", drive_path.display());
    println!("  Write Speed: {:.2} MB/s", result.write_speed);
    println!("  Read Speed: {:.2} MB/s", result.read_speed);
    println!("  Random Access: {:.2} ops/s", result.random_access_speed);
    println!("  Overall Score: {:.2}", result.overall_score);
    
    Ok(())
}
```

### Benchmarking All Drives

To benchmark all available drives:

```rust
use catp2p::benchmark::drives::run_all_drives_benchmark;

fn main() -> Result<(), catp2p::error::Error> {
    // Benchmark all available drives
    let results = run_all_drives_benchmark()?;
    
    for (i, result) in results.iter().enumerate() {
        println!("Drive {}: {}", i + 1, result.path.display());
        println!("  Write Speed: {:.2} MB/s", result.write_speed);
        println!("  Read Speed: {:.2} MB/s", result.read_speed);
        println!("  Random Access: {:.2} ops/s", result.random_access_speed);
        println!("  Overall Score: {:.2}", result.overall_score);
    }
    
    Ok(())
}
```

## Getting Drive Information

To get information about all available drives without running benchmarks:

```rust
use catp2p::benchmark::drives::get_drives_info;

fn main() {
    let drives_info = get_drives_info();
    
    for drive_info in drives_info {
        println!("Drive: {} ({})", drive_info.name, drive_info.path.display());
        println!("  File System: {}", drive_info.file_system);
        println!("  Total Capacity: {:.2} GB", 
            drive_info.total_capacity as f64 / (1024.0 * 1024.0 * 1024.0));
        println!("  Available Space: {:.2} GB", 
            drive_info.available_space as f64 / (1024.0 * 1024.0 * 1024.0));
        println!("  Removable: {}", drive_info.is_removable);
    }
}
```

## Advanced Usage

### Custom Benchmark Configuration

The `DriveBenchmarkConfig` struct allows you to customize the benchmark parameters:

```rust
let config = DriveBenchmarkConfig {
    // Size of the test file in MB (larger files provide more accurate results but take longer)
    file_size_mb: 500,
    
    // Number of random access operations to perform
    random_access_ops: 5000,
    
    // Whether to include random access test (can be disabled for faster benchmarks)
    include_random_access: true,
};
```

### Finding the Best Drive for Storage

A common use case is to find the best drive for storing data:

```rust
use catp2p::benchmark::drives::{get_drives_info, run_drive_benchmark_with_config, DriveBenchmarkConfig};
use std::path::Path;

fn main() -> Result<(), catp2p::error::Error> {
    // Get all drives with at least 10GB free space
    let min_free_space = 10 * 1024 * 1024 * 1024; // 10GB in bytes
    let suitable_drives: Vec<_> = get_drives_info()
        .into_iter()
        .filter(|drive| drive.available_space >= min_free_space)
        .collect();
    
    if suitable_drives.is_empty() {
        println!("No drives with sufficient free space found.");
        return Ok(());
    }
    
    // Create a benchmark configuration
    let config = DriveBenchmarkConfig {
        file_size_mb: 100,
        random_access_ops: 1000,
        include_random_access: true,
    };
    
    // Benchmark each suitable drive
    let mut best_drive = None;
    let mut best_score = 0.0;
    
    for drive_info in suitable_drives {
        match run_drive_benchmark_with_config(&drive_info.path, &config) {
            Ok(result) => {
                println!("Benchmarked {}: Score {:.2}", 
                    drive_info.path.display(), result.overall_score);
                
                if result.overall_score > best_score {
                    best_score = result.overall_score;
                    best_drive = Some((drive_info, result));
                }
            },
            Err(e) => {
                println!("Failed to benchmark {}: {}", drive_info.path.display(), e);
            }
        }
    }
    
    // Report the best drive
    if let Some((drive_info, result)) = best_drive {
        println!("\nBest drive for storage: {}", drive_info.path.display());
        println!("  Overall Score: {:.2}", result.overall_score);
        println!("  Write Speed: {:.2} MB/s", result.write_speed);
        println!("  Read Speed: {:.2} MB/s", result.read_speed);
        println!("  Available Space: {:.2} GB", 
            drive_info.available_space as f64 / (1024.0 * 1024.0 * 1024.0));
    } else {
        println!("No suitable drives could be benchmarked.");
    }
    
    Ok(())
}
```

## Performance Considerations

- Benchmarking drives can be I/O intensive and may temporarily impact system performance
- For the most accurate results, close other applications during benchmarking
- System drives (like C: on Windows) may show lower performance if the OS is running from them
- SSD performance can vary based on factors like TRIM status and drive fullness
- External drives connected via USB may show different performance based on the USB port type (2.0, 3.0, etc.)

## Error Handling

The drive benchmarking functions return `Result<T, Error>` types, allowing you to handle errors gracefully:

```rust
match run_drive_benchmark() {
    Ok(score) => println!("Benchmark score: {:.2}", score),
    Err(e) => println!("Benchmark failed: {}", e),
}
```

Common errors include:
- Permission issues when trying to write to protected directories
- Insufficient disk space for the benchmark file
- Drive disconnection during the benchmark

The library includes fallback mechanisms to find writable locations for temporary files, but some errors may still occur in restricted environments.

## API Reference

For detailed API information, see the [Drive Benchmarking API Reference](/catp2p/docs/api/benchmark/drives).
```