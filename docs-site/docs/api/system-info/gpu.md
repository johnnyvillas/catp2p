---
sidebar_position: 1
---

# GPU Information API Reference

This page provides detailed API reference for the GPU information functionality in CatP2P.

## Structures

### `GpuInfo`

Contains detailed information about a GPU.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `name` | String | The name/model of the GPU | `gpu_info.name` |
| `vendor` | String | The manufacturer of the GPU | `gpu_info.vendor` |
| `architecture` | String | GPU architecture/generation | `gpu_info.architecture` |
| `driver` | String | GPU driver information | `gpu_info.driver` |
| `vram` | String | Total VRAM formatted as string (e.g., "8.0 GB") | `gpu_info.vram` |
| `backend` | String | Backend used (Vulkan, DirectX, Metal, etc.) | `gpu_info.backend` |
| `is_integrated` | bool | Whether the GPU is integrated or discrete | `if gpu_info.is_integrated { ... }` |
| `temperature` | `Option<f32>` | Current GPU temperature in Celsius | `if let Some(temp) = gpu_info.temperature { ... }` |
| `additional_properties` | HashMap of String to String | Additional properties that don't fit in the standard fields | `gpu_info.additional_properties.get("Free VRAM")` |

### `GpuUsageInfo`

Contains real-time usage information about a GPU.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `name` | String | The name/model of the GPU | `usage_info.name` |
| `vendor` | String | The manufacturer of the GPU | `usage_info.vendor` |
| `total_vram_bytes` | u64 | Total VRAM in bytes | `usage_info.total_vram_bytes` |
| `total_vram` | String | Total VRAM formatted as string (e.g., "8.0 GB") | `usage_info.total_vram` |
| `used_vram_bytes` | u64 | Used VRAM in bytes | `usage_info.used_vram_bytes` |
| `used_vram` | String | Used VRAM formatted as string | `usage_info.used_vram` |
| `gpu_usage_percent` | f32 | GPU usage percentage (0-100) | `usage_info.gpu_usage_percent` |
| `timestamp` | Instant | Timestamp when this information was collected | `usage_info.timestamp` |

### `GpuUsageStats`

Contains statistics about GPU usage over time.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `name` | String | The name/model of the GPU | `stats.name` |
| `vendor` | String | The manufacturer of the GPU | `stats.vendor` |
| `total_vram` | String | Total VRAM formatted as string (e.g., "8.0 GB") | `stats.total_vram` |
| `avg_usage_percent` | f32 | Average GPU usage percentage | `stats.avg_usage_percent` |
| `min_usage_percent` | f32 | Minimum GPU usage percentage | `stats.min_usage_percent` |
| `max_usage_percent` | f32 | Maximum GPU usage percentage | `stats.max_usage_percent` |
| `avg_used_vram` | String | Average used VRAM formatted as string | `stats.avg_used_vram` |
| `min_used_vram` | String | Minimum used VRAM formatted as string | `stats.min_used_vram` |
| `max_used_vram` | String | Maximum used VRAM formatted as string | `stats.max_used_vram` |
| `sample_count` | usize | Number of samples taken | `stats.sample_count` |
| `duration` | Duration | Duration of monitoring | `stats.duration` |

### `TemperatureUnit`

Enum representing temperature units.

```rust
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}
```

## Functions

### Basic Information Functions

#### `get_info() -> Result<GpuInfo, Error>`

Gets information about the primary GPU.

```rust
use catp2p::hardware::gpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpu_info = gpu::get_info()?;
    println!("GPU: {} with {} VRAM", gpu_info.name, gpu_info.vram);
    Ok(())
}
```

#### `get_all_info() -> Result<Vec<GpuInfo>, Error>`

Gets information about all available GPUs.

```rust
use catp2p::hardware::gpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpu_infos = gpu::get_all_info()?;
    println!("Found {} GPUs:", gpu_infos.len());
    for (i, gpu) in gpu_infos.iter().enumerate() {
        println!("GPU {}: {}", i+1, gpu.name);
    }
    Ok(())
}
```

#### `is_available() -> bool`

Checks if a GPU is available on the system.

```rust
use catp2p::hardware::gpu;

fn main() {
    if gpu::is_available() {
        println!("GPU is available for acceleration");
    } else {
        println!("No GPU detected, falling back to CPU");
    }
}
```

#### `get_info_from_adapter(adapter: &Adapter) -> Result<GpuInfo, Error>`

Gets GPU information from a specific WGPU adapter.

```rust
use catp2p::hardware::gpu;
use wgpu::{Instance, InstanceDescriptor, Backends, RequestAdapterOptions, PowerPreference};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    
    let adapter = pollster::block_on(async {
        instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }).await
    }).ok_or("No adapter found")?;
    
    let gpu_info = gpu::get_info_from_adapter(&adapter)?;
    println!("Adapter GPU: {}", gpu_info.name);
    
    Ok(())
}
```

### Usage Monitoring Functions

#### `get_usage() -> Result<GpuUsageInfo, Error>`

Gets current usage information for the primary GPU.

```rust
use catp2p::hardware::gpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usage = gpu::get_usage()?;
    println!("GPU: {}", usage.name);
    println!("Usage: {:.1}%", usage.gpu_usage_percent);
    println!("VRAM: {} / {}", usage.used_vram, usage.total_vram);
    Ok(())
}
```

#### `get_usage_by_name(gpu_name: &str) -> Result<GpuUsageInfo, Error>`

Gets current usage information for a specific GPU by name.

```rust
use catp2p::hardware::gpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpu_name = "NVIDIA GeForce RTX 3080";
    let usage = gpu::get_usage_by_name(gpu_name)?;
    println!("GPU: {}", usage.name);
    println!("Usage: {:.1}%", usage.gpu_usage_percent);
    println!("VRAM: {} / {}", usage.used_vram, usage.total_vram);
    Ok(())
}
```

#### `monitor_usage(duration: Duration, sample_interval: Duration) -> Result<GpuUsageStats, Error>`

Monitors GPU usage over a specified duration.

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
    
    Ok(())
}
```

### Temperature Methods

#### `temperature_in(&self, unit: TemperatureUnit) -> Option<f32>`

Converts GPU temperature to the specified unit (Celsius or Fahrenheit).

```rust
use catp2p::hardware::gpu::{self, TemperatureUnit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpu_info = gpu::get_info()?;
    
    if let Some(temp_c) = gpu_info.temperature {
        println!("Temperature in Celsius: {:.1}°C", temp_c);
        
        if let Some(temp_f) = gpu_info.temperature_in(TemperatureUnit::Fahrenheit) {
            println!("Temperature in Fahrenheit: {:.1}°F", temp_f);
        }
    } else {
        println!("Temperature information not available");
    }
    
    Ok(())
}
```

## Error Handling

All functions that return a `Result` can produce the following errors:

- `Error::Benchmark`: Indicates an error during GPU detection or information retrieval
- `Error::Timeout`: Indicates that a GPU operation timed out

Example of proper error handling:

```rust
use catp2p::hardware::gpu;
use catp2p::error::Error;

fn main() {
    match gpu::get_info() {
        Ok(gpu_info) => {
            println!("Found GPU: {}", gpu_info.name);
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

- Uses WGPU, WMI, and NVIDIA-SMI (for NVIDIA GPUs)
- Provides detailed information for NVIDIA GPUs via nvidia-smi
- Uses DXGI for memory usage information on non-NVIDIA GPUs

### Linux

- Uses WGPU, NVIDIA-SMI, and various system files
- Provides detailed information for NVIDIA GPUs via nvidia-smi
- Uses sysfs for AMD GPU information
- Uses lspci as a fallback for other GPUs

### macOS

- Uses WGPU and system_profiler
- Provides detailed information for Apple Silicon GPUs
- Has limited information for discrete GPUs on Intel Macs

## Related Resources

- [GPU Information Documentation](/docs/docs/system-info/gpu_info)
- [GPU Information Examples](/docs/Examples/system-info/gpu-information)
