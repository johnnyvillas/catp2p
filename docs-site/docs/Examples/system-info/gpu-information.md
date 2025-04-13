# GPU Information Examples

This section provides examples of how to use the CatP2P library to retrieve and monitor GPU information.

## Available Examples

CatP2P provides several examples demonstrating different aspects of GPU information retrieval:

1. **Basic GPU Information**: Retrieves detailed information about the primary GPU.
2. **All GPUs Information**: Gets information about all available GPUs in the system.
3. **GPU Usage Information**: Retrieves real-time usage information about the GPU.
4. **GPU Usage Monitoring**: Monitors GPU usage and temperature over time.
5. **Performance Comparison**: Compares the performance of different GPU information methods.
6. **Run All Examples**: Interactive example that allows you to run any or all of the above examples.

## Running the Examples

You can run any of these examples using Cargo:

```bash
# Run the interactive example that lets you choose which examples to run
cargo run --example hardware_gpu_run_all

# Or run individual examples directly:
cargo run --example hardware_gpu_basic_info
cargo run --example hardware_gpu_all_gpus
cargo run --example hardware_gpu_usage_info
cargo run --example hardware_gpu_monitoring
cargo run --example hardware_gpu_performance
```

## Example: Basic GPU Information

The [basic_info.rs](https://github.com/johnnyvillas/catp2p/blob/main/examples/hardware/gpu/basic_info.rs) example demonstrates how to retrieve detailed information about the primary GPU in your system.

```rust
use catp2p::hardware::gpu::{get_info, TemperatureUnit};
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    let gpu_info = get_info()?;
    
    println!("Primary GPU Details:");
    println!("Name: {}", gpu_info.name);
    println!("Vendor: {}", gpu_info.vendor);
    println!("Architecture: {}", gpu_info.architecture);
    println!("Driver: {}", gpu_info.driver);
    println!("VRAM: {}", gpu_info.vram);
    println!("Type: {}", if gpu_info.is_integrated { "Integrated" } else { "Discrete" });
    
    // Display temperature if available (in both Celsius and Fahrenheit)
    if let Some(temp) = gpu_info.temperature {
        println!("Temperature: {:.1}°C / {:.1}°F", 
                 temp, 
                 gpu_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
    }
    
    // Display additional properties if available
    if !gpu_info.additional_properties.is_empty() {
        println!("\nAdditional Properties:");
        for (key, value) in &gpu_info.additional_properties {
            println!("{}: {}", key, value);
        }
    }
    
    Ok(())
}
```

## Example: All GPUs Information

The [all_gpus.rs](https://github.com/johnnyvillas/catp2p/blob/main/examples/hardware/gpu/all_gpus.rs) example shows how to retrieve information about all GPUs available in your system.

```rust
use catp2p::hardware::gpu::{get_all_info, TemperatureUnit};
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    let gpu_infos = get_all_info()?;
    
    println!("Number of GPUs detected: {}", gpu_infos.len());
    
    for (i, gpu_info) in gpu_infos.iter().enumerate() {
        println!("\nGPU {}: {}", i+1, gpu_info.name);
        println!("Vendor: {}", gpu_info.vendor);
        println!("Architecture: {}", gpu_info.architecture);
        println!("Driver: {}", gpu_info.driver);
        println!("VRAM: {}", gpu_info.vram);
        println!("Type: {}", if gpu_info.is_integrated { "Integrated" } else { "Discrete" });
        
        // Display temperature if available
        if let Some(temp) = gpu_info.temperature {
            println!("Temperature: {:.1}°C / {:.1}°F", 
                     temp, 
                     gpu_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
        }
        
        // Display key properties if available
        if !gpu_info.additional_properties.is_empty() {
            let important_props = ["Total VRAM", "Free VRAM", "GPU Utilization"];
            let mut has_props = false;
            
            for prop in important_props.iter() {
                if let Some(value) = gpu_info.additional_properties.get(*prop) {
                    if !has_props {
                        println!("\nKey Properties:");
                        has_props = true;
                    }
                    println!("{}: {}", prop, value);
                }
            }
        }
    }
    
    Ok(())
}
```

## Example: GPU Usage Information

The [usage_info.rs](https://github.com/johnnyvillas/catp2p/blob/main/examples/hardware/gpu/usage_info.rs) example demonstrates how to retrieve real-time usage information about the GPU.

```rust
use catp2p::hardware::gpu::{get_info, get_usage, TemperatureUnit};
use catp2p::error::Error;

fn main() -> Result<(), Error> {
    // First get basic GPU info
    let gpu_info = get_info()?;
    
    println!("GPU: {}", gpu_info.name);
    
    // Display temperature if available
    if let Some(temp) = gpu_info.temperature {
        println!("Temperature: {:.1}°C / {:.1}°F", 
                 temp, 
                 gpu_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
    }
    
    // Get real-time usage information
    let usage = get_usage()?;
    
    println!("\nGPU Usage Details:");
    println!("GPU Utilization: {:.1}%", usage.gpu_usage_percent);
    println!("VRAM Usage: {} / {}", usage.used_vram, usage.total_vram);
    
    // Calculate percentage of VRAM used
    if usage.total_vram_bytes > 0 {
        let vram_percent = (usage.used_vram_bytes as f32 / usage.total_vram_bytes as f32) * 100.0;
        println!("VRAM Utilization: {:.1}%", vram_percent);
    }
    
    Ok(())
}
```

## Example: GPU Usage Monitoring

The [monitoring.rs](https://github.com/johnnyvillas/catp2p/blob/main/examples/hardware/gpu/monitoring.rs) example demonstrates how to monitor GPU usage and temperature over time.

```rust
use catp2p::hardware::gpu::{monitor_usage, get_info, TemperatureUnit};
use catp2p::error::Error;
use std::time::Duration;

fn main() -> Result<(), Error> {
    // Get initial temperature
    let initial_info = get_info()?;
    println!("GPU: {}", initial_info.name);
    
    if let Some(temp) = initial_info.temperature {
        println!("Initial Temperature: {:.1}°C / {:.1}°F", 
                 temp, 
                 initial_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
    }
    
    println!("\nMonitoring GPU usage for 3 seconds...");
    
    let stats = monitor_usage(Duration::from_secs(3), Duration::from_millis(500))?;
    
    println!("\nGPU Usage Statistics:");
    println!("Name: {}", stats.name);
    println!("Average GPU Utilization: {:.1}%", stats.avg_usage_percent);
    println!("Min/Max GPU Utilization: {:.1}% / {:.1}%", 
            stats.min_usage_percent, stats.max_usage_percent);
    println!("Average VRAM Usage: {}", stats.avg_used_vram);
    println!("Min/Max VRAM Usage: {} / {}", 
            stats.min_used_vram, stats.max_used_vram);
    println!("Total VRAM: {}", stats.total_vram);
    println!("Samples Collected: {}", stats.sample_count);
    println!("Monitoring Duration: {:?}", stats.duration);
    
    // Check temperature after monitoring
    let final_info = get_info()?;
    if let Some(final_temp) = final_info.temperature {
        println!("\nFinal Temperature: {:.1}°C / {:.1}°F", 
                 final_temp, 
                 final_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
        
        // Show temperature change
        if let Some(initial_temp) = initial_info.temperature {
            let temp_diff = final_temp - initial_temp;
            println!("Temperature Change: {:.1}°C", temp_diff);
        }
    }
    
    Ok(())
}
```

## Example: Performance Comparison

The [performance.rs](https://github.com/johnnyvillas/catp2p/blob/main/examples/hardware/gpu/performance.rs) example compares the performance of different GPU information methods.

```rust
use catp2p::hardware::gpu::{get_info, get_all_info, is_available, get_usage};
use catp2p::error::Error;
use std::time::Instant;

fn main() -> Result<(), Error> {
    let iterations = 5;
    println!("Running {} iterations of each method:", iterations);
    
    // Measure performance of different methods
    let mut total_time_info = std::time::Duration::new(0, 0);
    let mut total_time_all_info = std::time::Duration::new(0, 0);
    let mut total_time_available = std::time::Duration::new(0, 0);
    let mut total_time_usage = std::time::Duration::new(0, 0);
    
    for i in 1..=iterations {
        // Measure get_info performance
        let start = Instant::now();
        let _ = get_info()?;
        total_time_info += start.elapsed();
        
        // Measure get_all_info performance
        let start = Instant::now();
        let _ = get_all_info()?;
        total_time_all_info += start.elapsed();
        
        // Measure is_available performance
        let start = Instant::now();
        let _ = is_available();
        total_time_available += start.elapsed();
        
        // Measure get_usage performance
        let start = Instant::now();
        let _ = get_usage()?;
        total_time_usage += start.elapsed();
    }
    
    // Calculate averages
    let avg_time_info = total_time_info / iterations;
    let avg_time_all_info = total_time_all_info / iterations;
    let avg_time_available = total_time_available / iterations;
    let avg_time_usage = total_time_usage / iterations;
    
    // Print performance summary
    println!("\nPerformance Summary:");
    println!("Average time for get_info(): {:?}", avg_time_info);
    println!("Average time for get_all_info(): {:?}", avg_time_all_info);
    println!("Average time for is_available(): {:?}", avg_time_available);
    println!("Average time for get_usage(): {:?}", avg_time_usage);
    
    Ok(())
}
```

## Example: Run All Examples

The [run_all.rs](https://github.com/johnnyvillas/catp2p/blob/main/examples/hardware/gpu/run_all.rs) example provides an interactive interface to run any or all of the GPU information examples.

```rust
use catp2p::hardware::gpu::{get_info, get_all_info, is_available, get_usage, monitor_usage, TemperatureUnit};
use catp2p::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Error> {
    println!("=== CatP2P GPU Information Examples ===");
    
    let examples = [
        ("GPU Availability Check", "cargo run --example hardware_gpu_availability"),
        ("Basic GPU Information", "cargo run --example hardware_gpu_basic_info"),
        ("All GPUs Information", "cargo run --example hardware_gpu_all_gpus"),
        ("GPU Usage Information", "cargo run --example hardware_gpu_usage_info"),
        ("GPU Usage Monitoring", "cargo run --example hardware_gpu_monitoring"),
        ("Performance Comparison", "cargo run --example hardware_gpu_performance"),
    ];
    
    println!("\nAvailable Examples:");
    for (i, (name, command)) in examples.iter().enumerate() {
        println!("{}. {} - Run with: {}", i+1, name, command);
    }
    
    println!("\nOptions:");
    println!("A. Run all examples");
    for (i, (name, _)) in examples.iter().enumerate() {
        println!("{}. {}", i+1, name);
    }
    println!("Q. Quit");
    
    // Interactive menu to choose examples to run
    // (Implementation details omitted for brevity)
    
    Ok(())
}
```

## Best Practices

When working with GPU information in your applications, consider these best practices:

1. **Check Availability First**: Always check if a GPU is available before attempting GPU operations.
2. **Cache Information**: Cache GPU information when possible to avoid repeated queries.
3. **Handle Errors**: Always handle errors from GPU information functions, as hardware detection can fail.
4. **Monitor Sparingly**: Use monitoring functions sparingly to minimize performance impact.
5. **Consider Multiple GPUs**: If your application might run on systems with multiple GPUs, use `get_all_info()` to detect them.
6. **Performance Considerations**: Choose the appropriate method based on your needs - `is_available()` is fastest for simple checks.
7. **Temperature Monitoring**: For applications that stress the GPU, monitor temperature to ensure safe operation.

## Related Resources

- [GPU Information Documentation](/docs/docs/system-info/gpu_info)
- [GPU Information API Reference](/docs/api/system-info/gpu)
```