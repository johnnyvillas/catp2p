# CatP2P

A high-performance peer-to-peer library for distributed computing, written in Rust.

## Features

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

## Documentation

For more detailed documentation, see the [API docs](https://docs.rs/catp2p) or the guides in the `docs/` directory.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](./LICENSE) file for details.
```

### 6. Documentation UI Consideration

Regarding your question about including a UI for documentation, I recommend:

1. **Separate Documentation Website**: Create a dedicated documentation website using tools like [mdBook](https://rust-lang.github.io/mdBook/) or [Docusaurus](https://docusaurus.io/).

2. **Interactive Examples**: Include interactive examples that demonstrate the library's capabilities.

3. **API Documentation**: Use Rust's built-in documentation system with `cargo doc` to generate comprehensive API docs.

4. **Integration with catnetp2p**: Your catnetp2p application could include a documentation viewer that displays the library documentation.

## Next Steps

1. **Create the repository structure**:
   ```bash
   mkdir -p catp2p/src/{network,tasks,resources,storage,benchmark,scoring}
   mkdir -p catp2p/{examples,benches,tests,docs/{api,guides}}
   ```

2. **Initialize the Git repository**:
   ```bash
   cd catp2p
   git init
   ```

3. **Create the initial files** as outlined above.

4. **Add a .gitignore file**:
   ```bash
   echo "/target/" > .gitignore
   echo "Cargo.lock" >> .gitignore
   ```

5. **Make your first commit**:
   ```bash
   git add .
   git commit -m "Initial commit for catp2p library"
   ```

6. **Create a new repository on GitHub** and push your code:
   ```bash
   git remote add origin https://github.com/johnnyvillas/catp2p.git
