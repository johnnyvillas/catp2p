# Contributing to CatP2P

Thank you for your interest in contributing to CatP2P! This document provides guidelines and instructions for contributing to this project.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please be respectful and considerate of others.

## Getting Started

1. **Fork the Repository**
    
    - Fork the [CatP2P repository](https://github.com/johnnyvillas/catp2p) on GitHub
    - Clone your fork locally: `git clone https://github.com/YOUR-USERNAME/catp2p.git`
    - Add the original repository as upstream: `git remote add upstream https://github.com/johnnyvillas/catp2p.git`
2. **Set Up Development Environment**
    
    - Make sure you have Rust installed (latest stable version recommended)
    - Run `cargo build` to compile the project
    - Run `cargo test` to ensure everything is working correctly

## Making Contributions

### Types of Contributions

We welcome the following types of contributions:

- Bug fixes
- Feature implementations
- Documentation improvements
- Performance optimizations
- Test additions

### Development Workflow

1. **Create a Branch**
    
    - Create a new branch for your work: `git checkout -b feature/your-feature-name`
    - Use descriptive branch names (e.g., `fix/connection-timeout`, `feature/resource-monitoring`)
2. **Make Your Changes**
    
    - Follow the [Rust style guidelines](command:_cody.vscode.open?%22https%3A%2F%2Fdoc.rust-lang.org%2F1.0.0%2Fstyle%2FREADME.html%22)
    - Add appropriate license headers to new files
    - Write tests for new functionality
    - Update documentation as needed
3. **Commit Your Changes**
    
    - Use clear and descriptive commit messages
    - Reference issue numbers in commit messages when applicable
    - Keep commits focused on single changes when possible
4. **Submit a Pull Request**
    
    - Push your branch to your fork: `git push origin feature/your-feature-name`
    - Submit a pull request to the main repository
    - Provide a clear description of the changes in your PR
    - Link any related issues

### Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing to ensure consistent formatting
- Run `cargo clippy` to catch common mistakes and improve code quality

### Testing

- Add tests for new functionality
- Ensure all tests pass with `cargo test`
- Consider adding integration tests for significant features

## Documentation

- Update the documentation in the `docs-site` directory for any user-facing changes
- Add inline documentation (comments) for code
- Include examples for new features

## Pull Request Process

1. Update the README.md or documentation with details of changes if appropriate
2. Update the version numbers in Cargo.toml following [Semantic Versioning](command:_cody.vscode.open?%22https%3A%2F%2Fsemver.org%2F%22)
3. Your PR will be reviewed by maintainers, who may request changes
4. Once approved, your PR will be merged

## License

By contributing to CatP2P, you agree that your contributions will be licensed under the project's [Apache License 2.0](vscode-webview://053gn7o36ebi4sma99sli1496acqnqp3qpj7vev179e1mt1c6lgi/index.html?id=bc6b3a0d-d7be-4e14-bc79-ca200074be8a&origin=0cd70bf0-dddd-4d1c-83be-f536f292e6d3&swVersion=4&extensionId=sourcegraph.cody-ai&platform=electron&vscode-resource-base-authority=vscode-resource.vscode-cdn.net&parentOrigin=vscode-file%3A%2F%2Fvscode-app&purpose=webviewView).

## Questions?

If you have questions about contributing, please open an issue or reach out to the maintainers.

Thank you for contributing to CatP2P!