# Vibe Coder

An AI coding console for macOS (and other platforms) built with Rust. Vibe Coder is a fast, extensible, and scalable application that allows users to run coding tasks locally with AI assistance.

## Features

- 🚀 **Fast & Efficient**: Built with Rust for maximum performance
- 🎨 **Clean UI**: Two input lines for queries with a responsive display area
- 🤖 **Multi-Provider AI Support**: Integrate multiple AI providers (OpenAI, Anthropic, etc.)
- 🔌 **Plugin System**: Extensible architecture for custom functionality
- 📝 **Smart Response Parsing**: Automatically parses and formats AI responses including:
  - Titles and headings
  - Code blocks with syntax highlighting
  - Lists and quotes
  - Paragraphs and formatted text

## Installation

### Prerequisites

- Rust 1.70 or higher
- macOS 10.15+ (or Linux/Windows for cross-platform support)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/aavishay/vibe-coder.git
cd vibe-coder

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

1. Launch the application
2. Enter your coding query in the two input fields:
   - **Input Line 1**: Main query or task description
   - **Input Line 2**: Additional context or parameters
3. Click "Send Request" or press Enter
4. View the parsed AI response with syntax-highlighted code blocks

## Architecture

### Core Components

#### 1. AI Provider System (`src/ai_providers/`)
- Trait-based architecture for multiple AI providers
- Easy integration of new providers (OpenAI, Anthropic, local models)
- Built-in mock provider for testing

#### 2. Response Parser (`src/parser/`)
- Markdown-based parsing using `pulldown-cmark`
- Extracts structured content:
  - Headings (multiple levels)
  - Code blocks with language detection
  - Lists and quotes
  - Paragraphs

#### 3. Plugin System (`src/plugins/`)
- Trait-based plugin architecture
- Support for:
  - Pre-processors (modify input before AI)
  - Post-processors (modify AI responses)
  - Code formatters
  - Custom commands
- Sample plugins included

#### 4. UI Layer (`src/ui/`)
- Built with `iced` GUI framework
- Responsive and native look & feel
- Real-time response rendering

## Plugin Development

Create custom plugins by implementing the `Plugin` trait:

```rust
use async_trait::async_trait;
use vibe_coder::plugins::{Plugin, PluginMetadata, PluginCapability, PluginError};

pub struct MyPlugin;

#[async_trait]
impl Plugin for MyPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "My Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Does something awesome".to_string(),
            author: "Your Name".to_string(),
        }
    }
    
    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![PluginCapability::PreProcessor]
    }
    
    async fn initialize(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
    
    async fn pre_process(&self, input: &str) -> Result<String, PluginError> {
        // Your custom logic here
        Ok(input.to_string())
    }
}
```

## Adding AI Providers

Implement the `AIProvider` trait to add new AI services:

```rust
use async_trait::async_trait;
use vibe_coder::ai_providers::{AIProvider, AIRequest, AIResponse, ProviderConfig, AIProviderError};

pub struct MyAIProvider;

#[async_trait]
impl AIProvider for MyAIProvider {
    fn name(&self) -> String {
        "My AI Provider".to_string()
    }
    
    async fn configure(&mut self, config: ProviderConfig) -> Result<(), AIProviderError> {
        // Configure your provider
        Ok(())
    }
    
    async fn send_request(&self, request: AIRequest) -> Result<AIResponse, AIProviderError> {
        // Implement API call
        todo!()
    }
    
    fn is_ready(&self) -> bool {
        true
    }
}
```

## Testing

Run the test suite:

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

## Development

### Project Structure

```
vibe-coder/
├── src/
│   ├── main.rs              # Application entry point
│   ├── ui/                  # GUI components
│   │   └── mod.rs
│   ├── parser/              # Response parser
│   │   └── mod.rs
│   ├── ai_providers/        # AI provider integrations
│   │   └── mod.rs
│   └── plugins/             # Plugin system
│       ├── mod.rs
│       └── sample_plugins.rs
├── Cargo.toml               # Dependencies and metadata
└── README.md
```

### Key Dependencies

- `iced`: Modern cross-platform GUI framework
- `tokio`: Async runtime
- `pulldown-cmark`: Markdown parser
- `serde`: Serialization framework
- `reqwest`: HTTP client for API calls

## Roadmap

- [ ] OpenAI provider implementation
- [ ] Anthropic Claude provider implementation
- [ ] Local model support (Ollama, etc.)
- [ ] Configuration file support
- [ ] Plugin marketplace
- [ ] Code execution sandbox
- [ ] Session history
- [ ] Export to file functionality

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the MIT License.

## Support

For issues, questions, or contributions, please visit the [GitHub repository](https://github.com/aavishay/vibe-coder).
