# CatP2P Project File Structure

This document provides an overview of the CatP2P project's file structure with descriptions.

```
catp2p/
├── CONTRIBUTING.md # Contribution guidelines
├── Cargo.toml # Package manifest
├── LICENSE # License file
├── README.md # Project overview
├── docs-site/ # Project documentation at https://johnnyvillas.github.io/catp2p/
│   └── README.md # Project overview
├── examples/
│   ├── cpu_benchmark.rs # No description available
│   ├── drive_benchmark.rs # Example demonstrating drive benchmarking functionality.
│   ├── gpu_benchmark.rs # Example demonstrating GPU benchmarking functionality.
│   ├── hardware/
│   │   └── gpu_info.rs # Example demonstrating GPU information retrieval functionality.
│   └── memory_benchmark.rs # No description available
└── src/
    ├── benchmark/
    │   ├── cpu.rs # CPU benchmarking functionality.
    │   ├── drives.rs # Drive benchmarking functionality for assessing storage performance.
    │   ├── gpu/
    │   │   ├── activation_functions.rs # Activation functions benchmark for GPU performance testing.
    │   │   ├── gradient_calculations.rs # Gradient calculation benchmark for GPU performance testing.
    │   │   ├── matrix_multiplications.rs # Matrix multiplication benchmark for GPU performance testing.
    │   │   └── mod.rs # GPU benchmark test implementations.
    │   ├── memory.rs # Memory benchmarking functionality.
    │   ├── mod.rs # Benchmarking functionality for assessing system capabilities.
    │   └── network.rs # Network benchmarking functionality.
    ├── config.rs # Configuration for the CatP2P library.
    ├── error.rs # Error types for the CatP2P library.
    ├── hardware/
    │   ├── gpu/
    │   │   ├── info/
    │   │   │   ├── common.rs # Common GPU information utilities shared across platforms. 
    │   │   │   ├── linux.rs # Linux-specific GPU information utilities.
    │   │   │   ├── macos.rs # macOS-specific GPU information utilities.
    │   │   │   ├── mod.rs # GPU information utilities.
    │   │   │   └── windows.rs # Windows-specific GPU information utilities.
    │   │   └── mod.rs # GPU hardware information and utilities.  This module provides functionality for retrieving detailed information about the GPU hardware available on the system, including specifications, capabilities, and current status.  # Examples  ``` use catp2p::hardware::gpu;  // Get information about the primary GPU if let Ok(gpu_info) = gpu::get_info() { println!("GPU: {} with {} VRAM", gpu_info.name, gpu_info.vram); println!("Vendor: {}", gpu_info.vendor); println!("Architecture: {}", gpu_info.architecture); println!("Driver: {}", gpu_info.driver); }  Get information about all available GPUs if let Ok(all_gpus) = gpu::get_all_info() { println!("Found {} GPUs:", all_gpus.len()); for (i, gpu) in all_gpus.iter().enumerate() { println!("GPU {}: {}", i+1, gpu.name); } }  // Monitor GPU usage in real-time if let Ok(mut usage) = gpu::get_usage() { println!("GPU: {} - Usage: {:.1}%, VRAM: {}/{}", usage.name, usage.gpu_usage_percent, usage.used_vram, usage.total_vram); } ```
    │   └── mod.rs # Hardware information and utilities.  This module provides functionality for retrieving detailed information about the hardware available on the system, including GPUs, CPUs, memory, and storage.
    ├── lib.rs # Main entry point for the CatP2P library, defining the public API and core functionality.
    ├── network/
    │   ├── allocation.rs # Network resource allocation functionality.
    │   ├── discovery.rs # Peer discovery functionality.
    │   ├── mod.rs # Resource monitoring and allocation functionality.
    │   ├── monitor.rs # Network monitoring functionality.
    │   ├── protocol.rs # Custom protocols for peer communication.
    │   └── transport.rs # Network transport functionality.
    ├── resources/
    │   ├── allocation.rs # Resource allocation functionality.
    │   ├── mod.rs # Resource monitoring and allocation functionality.
    │   └── monitor.rs # Resource monitoring functionality.
    ├── scoring/
    │   ├── mod.rs # Scoring and rewards system for tracking contributions.
    │   └── points.rs # Points system for tracking and rewarding contributions.
    ├── storage/
    │   ├── db.rs # Database functionality for persisting data.
    │   └── mod.rs # Storage functionality for persisting data.
    ├── tasks/
    │   ├── cpu.rs # CPU task execution functionality.
    │   ├── gpu.rs # GPU task execution functionality.
    │   ├── mod.rs # Task management functionality for distributing and executing tasks.
    │   └── scheduler.rs # Task scheduling functionality.
    └── utils/
        ├── crypto.rs # Cryptographic utilities.
        ├── logging.rs # Logging utilities.
        ├── mod.rs # Utility modules.
        ├── serialization.rs # Serialization utilities.
        └── time.rs # Time utilities.
```
