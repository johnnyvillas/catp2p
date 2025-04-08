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
│   └── memory_benchmark.rs # No description available
└── src/
    ├── benchmark/
    │   ├── cpu.rs # CPU benchmarking functionality.
    │   ├── drives.rs # Drive benchmarking functionality for assessing storage performance.
    │   ├── gpu.rs # GPU benchmarking functionality.
    │   ├── memory.rs # Memory benchmarking functionality.
    │   ├── mod.rs # Benchmarking functionality for assessing system capabilities.
    │   └── network.rs # Network benchmarking functionality.
    ├── config.rs # Configuration for the CatP2P library.
    ├── error.rs # Error types for the CatP2P library.
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
