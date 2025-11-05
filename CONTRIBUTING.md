# Contributing to Vibe Coder

Thank you for your interest in contributing to Vibe Coder! This document provides guidelines and instructions for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/vibe-coder.git
   cd vibe-coder
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Running the Application
```bash
cargo run
```

## Code Style

- Follow Rust's official style guide
- Use `cargo fmt` to format your code before committing
- Run `cargo clippy` to catch common mistakes

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

## Making Changes

### Adding a New AI Provider

1. Create a new struct that implements the `AIProvider` trait in `src/ai_providers/`
2. Add configuration support
3. Add tests
4. Update documentation

Example:
```rust
use async_trait::async_trait;
use crate::ai_providers::{AIProvider, AIRequest, AIResponse, ProviderConfig, AIProviderError};

pub struct MyProvider {
    config: Option<ProviderConfig>,
}

#[async_trait]
impl AIProvider for MyProvider {
    // Implement trait methods
}
```

### Creating a Plugin

1. Create a new struct that implements the `Plugin` trait in `src/plugins/`
2. Implement required methods
3. Add tests
4. Update documentation

Example:
```rust
use async_trait::async_trait;
use crate::plugins::{Plugin, PluginMetadata, PluginCapability, PluginError};

pub struct MyPlugin;

#[async_trait]
impl Plugin for MyPlugin {
    // Implement trait methods
}
```

### Adding Tests

- Add unit tests in the same file as your code using `#[cfg(test)]` modules
- Add integration tests in the `tests/` directory (if created)
- Ensure all tests pass before submitting

## Submitting Changes

1. **Commit your changes** with clear, descriptive commit messages:
   ```bash
   git commit -m "Add feature: description of your change"
   ```

2. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Create a Pull Request** on GitHub from your fork to the main repository

### Pull Request Guidelines

- Provide a clear title and description
- Reference any related issues
- Ensure all tests pass
- Update documentation if needed
- Keep PRs focused on a single feature or fix

## Code Review Process

1. Maintainers will review your PR
2. Address any requested changes
3. Once approved, your PR will be merged

## Reporting Issues

- Use GitHub Issues to report bugs
- Provide clear reproduction steps
- Include system information (OS, Rust version, etc.)
- Check if the issue already exists before creating a new one

## Feature Requests

- Open a GitHub Issue with the "enhancement" label
- Clearly describe the feature and its use case
- Be open to discussion and feedback

## Questions?

If you have questions, feel free to:
- Open a GitHub Issue
- Start a discussion in the repository

## License

By contributing to Vibe Coder, you agree that your contributions will be licensed under the same license as the project.

Thank you for contributing! 🎉
