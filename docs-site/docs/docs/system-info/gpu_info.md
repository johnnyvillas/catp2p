---
sidebar_position: 1
---

# GPU Information System

The GPU Information System in CatP2P provides comprehensive functionality for detecting, analyzing, and monitoring GPUs across different platforms. This system enables applications to make informed decisions about GPU resource allocation and utilization.

## Overview

The GPU Information System offers capabilities for:

- Detecting available GPUs on the system
- Retrieving detailed specifications of GPUs
- Monitoring real-time GPU usage and temperature
- Tracking GPU performance over time
- Supporting multiple platforms (Windows, Linux, macOS)

## Key Concepts

### GPU Detection

CatP2P uses a multi-layered approach to detect GPUs:

1. **Primary Detection**: Uses WGPU (WebGPU) to detect GPUs through hardware acceleration APIs
2. **Fallback Methods**: Platform-specific fallback methods when WGPU detection fails
3. **Multiple GPU Support**: Can detect and work with multiple GPUs in the same system

### GPU Information

The system collects comprehensive information about GPUs:

- **Basic Information**: Name, vendor, architecture, driver version
- **Memory Details**: VRAM capacity, memory type, bus width
- **Performance Characteristics**: Clock speeds, compute capabilities
- **Hardware Details**: Integrated vs. discrete, form factor
- **Thermal Information**: Current temperature in Celsius or Fahrenheit
- **Platform-Specific Information**: Additional details based on the operating system

### GPU Monitoring

Real-time monitoring capabilities include:

- **Memory Usage**: Total and used VRAM
- **Utilization**: GPU processing load percentage
- **Temperature**: GPU temperature in Celsius or Fahrenheit
- **Power Usage**: Power consumption (where available)

## Available Functions

### Basic Information Functions

- **`get_info()`**: Retrieves detailed information about the primary GPU
- **`get_all_info()`**: Gets information about all available GPUs
- **`is_available()`**: Checks if a GPU is available on the system
- **`get_info_from_adapter()`**: Gets GPU information from a specific WGPU adapter

### Usage Monitoring Functions

- **`get_usage()`**: Gets current usage information for the primary GPU
- **`get_usage_by_name()`**: Gets current usage information for a specific GPU by name
- **`monitor_usage()`**: Monitors GPU usage over a specified duration

### Temperature Functions

- **`temperature_in(unit)`**: Converts GPU temperature to the specified unit (Celsius or Fahrenheit)

## Platform Support

The GPU Information System works across multiple platforms with platform-specific optimizations:

- **Windows**: Uses WGPU, WMI, and NVIDIA-SMI (for NVIDIA GPUs)
- **Linux**: Uses WGPU, NVIDIA-SMI, and various system files
- **macOS**: Uses WGPU and system_profiler

## Usage Examples

### Basic GPU Detection

```rust
use catp2p::hardware::gpu;

fn main() {
    // Check if a GPU is available
    if gpu::is_available() {
        println!("GPU is available for acceleration");
    } else {
        println!("No GPU detected, falling back to CPU");
    }
}
```

### Getting GPU Information

```rust
use catp2p::hardware::gpu::{self, TemperatureUnit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get information about the primary GPU
    let gpu_info = gpu::get_info()?;
    
    println!("GPU: {} with {} VRAM", gpu_info.name, gpu_info.vram);
    println!("Vendor: {}", gpu_info.vendor);
    println!("Architecture: {}", gpu_info.architecture);
    println!("Driver: {}", gpu_info.driver);
    
    // Display temperature if available (in both Celsius and Fahrenheit)
    if let Some(temp) = gpu_info.temperature {
        println!("Temperature: {:.1}°C / {:.1}°F", 
                 temp, 
                 gpu_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
    }
    
    Ok(())
}
```

### Monitoring GPU Usage

```rust
use catp2p::hardware::gpu;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Monitor GPU usage for 5 seconds, sampling every 500ms
    let stats = gpu::monitor_usage(
        Duration::from_secs(5),
        Duration::from_millis(500)
    )?;
    
    println!("GPU: {}", stats.name);
    println!("Average usage: {:.1}%", stats.avg_usage_percent);
    println!("Min/Max usage: {:.1}%/{:.1}%", 
             stats.min_usage_percent, 
             stats.max_usage_percent);
    println!("Average VRAM: {}/{}", 
             stats.avg_used_vram, 
             stats.total_vram);
    
    // Check current temperature
    let current_info = gpu::get_info()?;
    if let Some(temp) = current_info.temperature {
        println!("Current temperature: {:.1}°C", temp);
    }
    
    Ok(())
}
```

## Best Practices

- **Cache Information**: Cache GPU information when possible to avoid repeated queries
- **Check Availability**: Use `is_available()` for quick checks before attempting GPU operations
- **Monitor Sparingly**: Use monitoring functions sparingly to minimize performance impact
- **Handle Errors**: Always handle errors from GPU information functions, as hardware detection can fail
- **Temperature Awareness**: For applications that stress the GPU, monitor temperature to ensure safe operation

## Related Resources

- [GPU Information API Reference](/docs/api/system-info/gpu)
- [GPU Information Examples](/docs/Examples/system-info/gpu-information)
```