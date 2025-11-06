# Vibe Coder - AI Coding Agent Instructions

## Project Overview

Vibe Coder is a fast, extensible AI coding console built with Rust. It provides a GUI interface for interacting with AI coding assistants, with a focus on parsing and displaying structured responses.

The application follows a modular architecture with clear separation of concerns:
- **UI Layer**: Built with `iced` framework for cross-platform GUI
- **AI Provider System**: Trait-based abstraction for multiple AI providers
- **Response Parser**: Markdown parsing using `pulldown-cmark`
- **Plugin System**: Extensible architecture for custom functionality

## Architecture

### Core Components

1. **UI Layer** (`src/ui/`) - Built with `iced` framework
   - Two input fields for user queries (single input field in current implementation)
   - Response display area with formatted content
   - Dark/light theme support
   - Provider configuration dialog
   - Uses Elm-like architecture with messages driving state changes

2. **AI Provider System** (`src/ai_providers/`)
   - Trait-based abstraction for multiple AI providers (`AIProvider` trait)
   - Mock provider for testing/demonstration (`MockAIProvider`)
   - Provider manager for handling multiple services (`AIProviderManager`)
   - Request/response structures (`AIRequest`, `AIResponse`)
   - Configuration system (`ProviderConfig`)

3. **Response Parser** (`src/parser/`)
   - Markdown parsing using `pulldown-cmark`
   - Extracts structured content blocks (titles, code, lists, quotes, paragraphs)
   - Content block types: `Title`, `Paragraph`, `CodeBlock`, `List`, `Quote`
   - Helper methods for accessing specific content types (`get_code_blocks`, `get_titles`)

4. **Plugin System** (`src/plugins/`)
   - Trait-based extensible architecture (`Plugin` trait)
   - Pre-processors (modify input) and post-processors (modify responses)
   - Plugin registry for lifecycle management (`PluginRegistry`)
   - Plugin capabilities: `PreProcessor`, `PostProcessor`, `CodeFormatter`, `CustomCommand`
   - Sample plugins: `UppercasePlugin`, `CodeFormatterPlugin`

## Key Patterns and Conventions

### Error Handling
- Use `anyhow` for general errors and `thiserror` for structured errors
- Errors are propagated with `Result<T, Error>` types
- Custom error enums for each module:
  - `AIProviderError` for AI provider issues
  - `PluginError` for plugin system issues
- Error variants include `NotConfigured`, `ApiError`, `NetworkError`, etc.

### Async Patterns
- All AI operations are async using `tokio`
- UI uses `iced`'s async message handling with `Command` and `Message` types
- Plugin methods are async even if they don't need it (for future extensibility)
- Async traits are implemented using `async-trait` crate

### Configuration
- TOML-based configuration (see `config.example.toml`)
- Provider configuration with API keys, endpoints, models
- UI settings (window dimensions, theme)
- General settings (temperature, max_tokens, debug mode)

### Testing
- Unit tests in each module file using `cfg(test)` modules
- Integration tests in `tests/` directory with end-to-end coverage
- Mock providers for testing without external dependencies
- Test helpers in `tests/common/` for setup and utilities
- Async tests use `tokio::test` attribute

## Development Workflows

### Building
```bash
cargo build          # Development build
cargo build --release # Release build
```

### Running
```bash
cargo run            # Run the application
```

### Testing
```bash
cargo test           # Run all tests
cargo test --test ai_providers_e2e  # Run specific test suite
```

### Code Quality
```bash
cargo fmt            # Format code
cargo clippy         # Lint code
```

## Adding New Features

### AI Providers
1. Implement the `AIProvider` trait
2. Add to `AIProviderManager`
3. Update configuration support
4. Add tests

### Plugins
1. Implement the `Plugin` trait
2. Register with `PluginRegistry`
3. Add to configuration
4. Add tests

### UI Components
1. Add messages to the `Message` enum
2. Handle messages in the `update` function
3. Add UI elements in the `view` function
4. Update styling as needed

### Adding a New Content Block Type
1. Add variant to `ContentBlock` enum in `src/parser/mod.rs`
2. Update parsing logic in `parse_response`
3. Add rendering in `render_content_block` in `src/ui/mod.rs`
4. Add tests

### Adding Configuration Options
1. Update `config.example.toml`
2. Add struct definitions as needed
3. Implement parsing logic
4. Use in appropriate components

### Extending AI Providers
1. Implement `AIProvider` trait
2. Add to provider manager
3. Update configuration parsing
4. Add tests and documentation

## Important Implementation Details

### Response Parsing
- Uses `pulldown-cmark` for efficient markdown parsing
- Handles titles, paragraphs, code blocks, lists, quotes
- Preserves formatting while extracting structure

### Plugin System
- Supports pre-processing (modify input) and post-processing (modify output)
- Plugins are chained together for processing
- Sample plugins demonstrate the pattern

### UI State Management
- Uses `iced`'s Elm-like architecture
- Messages drive all state changes
- Async commands for background operations

## Common Tasks

### Adding a New Content Block Type
1. Add variant to `ContentBlock` enum in `src/parser/mod.rs`
2. Update parsing logic in `parse_response`
3. Add rendering in `render_content_block` in `src/ui/mod.rs`
4. Add tests

### Adding Configuration Options
1. Update `config.example.toml`
2. Add struct definitions as needed
3. Implement parsing logic
4. Use in appropriate components

### Extending AI Providers
1. Implement `AIProvider` trait
2. Add to provider manager
3. Update configuration parsing
4. Add tests and documentation

## Dependencies

Key external crates:
- `iced` - GUI framework (v0.12)
- `tokio` - Async runtime (v1.35)
- `pulldown-cmark` - Markdown parser (v0.9)
- `serde` - Serialization framework (v1.0)
- `reqwest` - HTTP client (for real providers) (v0.11)
- `async-trait` - Async trait support (v0.1)
- `anyhow`/`thiserror` - Error handling (v1.0)
- `regex` - Regular expressions (v1.10)

## Testing Strategy

- Unit tests for each module using `cfg(test)` modules
- Integration tests for component interactions in `tests/` directory
- End-to-end tests for complete workflows (e2e test suite)
- Mock providers for testing without external dependencies
- Sample plugins demonstrate plugin testing patterns
- Test helpers in `tests/common/` for setup and utilities
- Async tests use `tokio::test` attribute
- Comprehensive test coverage for roadmap items