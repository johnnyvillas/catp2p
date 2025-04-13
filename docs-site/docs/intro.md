---
sidebar_position: 1
---

# Getting Started

> "The most powerful computer in the world is everyone's computer combined."

![CatP2P Logo](/img/catp2p_logo.svg)

## What is CatP2P?

CatP2P is a high-performance peer-to-peer library for distributed computing, written in Rust. It's designed to build the foundation for democratizing computing power by connecting devices across the globe into a seamless peer-to-peer network, unlocking the untapped potential of millions of computers sitting idle.

## Our Vision

We believe computing resources should be accessible to everyone, not just those with access to expensive infrastructure. By joining the CatP2P network, you're helping build a more equitable digital future where anyone can access the computational power they need.

## Key Features

- **P2P Networking**: Built on libp2p for robust peer discovery and communication
- **Task Distribution**: Efficiently distribute and execute tasks across the network
- **Resource Management**: Monitor and allocate CPU, GPU, memory, and storage resources
- **Benchmarking**: Assess node capabilities for optimal task allocation
- **Local Storage**: Persistent storage for task logs and peer interactions
- **Scoring System**: Track contributions and allocate rewards

## Getting Started

### Installation

Add catp2p to your Cargo.toml:

```toml
[dependencies]
catp2p = "0.1.0"
```

### Basic Usage

```rust
use catp2p::CatP2P;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new CatP2P instance with default configuration
    let mut node = CatP2P::new()?;
    
    // Start the node
    node.start()?;
    
    // The node is now running and will discover peers and process tasks
    
    // When done, stop the node
    node.stop()?;
    
    Ok(())
}
```

## Configuration

catp2p can be configured with different resource modes:

```rust
use catp2p::{CatP2P, config::{Config, ResourceMode}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom configuration
    let mut config = Config::default();
    config.resource_mode = ResourceMode::HighPerformance;
    
    // Create a CatP2P instance with custom configuration
    let mut node = CatP2P::with_config(config)?;
    
    // Start the node
    node.start()?;
    
    Ok(())
}
```

## Join the Movement

Whether you're a developer looking to contribute code, a user willing to share your idle computing resources, or an organization with computational needs, there's a place for you in the CatP2P ecosystem.

- 🌐 Share your unused computing power
- 🚀 Access distributed computing when you need it
- 🔧 Help build the infrastructure of tomorrow

Together, we can create a more resilient, accessible, and democratic computing landscape.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](https://github.com/johnnyvillas/catp2p/blob/main/LICENSE) file for details.
