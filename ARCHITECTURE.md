# Vibe Coder Architecture

## Overview

Vibe Coder is a fast, extensible AI coding console built with Rust. The application follows a modular architecture with clear separation of concerns.

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────┐
│                    Vibe Coder UI                         │
│                   (iced Framework)                       │
│  ┌────────────────────────────────────────────────────┐  │
│  │  Input Line 1: [___________________________]      │  │
│  │  Input Line 2: [___________________________]      │  │
│  │               [Send Request Button]               │  │
│  └────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────┐  │
│  │         Response Display Area                     │  │
│  │  ┌──────────────────────────────────────────────┐ │  │
│  │  │ # Title (parsed from markdown)              │ │  │
│  │  │ Paragraph text...                           │ │  │
│  │  │ ┌────────────────────────────────────────┐  │ │  │
│  │  │ │ ```rust                                │  │ │  │
│  │  │ │ fn main() {                            │  │ │  │
│  │  │ │     println!("Hello");                 │  │ │  │
│  │  │ │ }                                      │  │ │  │
│  │  │ │ ```                                    │  │ │  │
│  │  │ └────────────────────────────────────────┘  │ │  │
│  │  └──────────────────────────────────────────────┘ │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
                            ↓
┌──────────────────────────────────────────────────────────┐
│                   Core Application                       │
│  ┌────────────┐   ┌──────────────┐   ┌──────────────┐  │
│  │  Plugin    │   │  AI Provider │   │   Response   │  │
│  │  Registry  │   │   Manager    │   │    Parser    │  │
│  └────────────┘   └──────────────┘   └──────────────┘  │
└──────────────────────────────────────────────────────────┘
        ↓                   ↓                   ↓
┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│   Plugins    │   │ AI Providers │   │    Parser    │
│   ┌────────┐ │   │ ┌──────────┐ │   │  Markdown    │
│   │ Upper  │ │   │ │  Mock    │ │   │    to        │
│   │ case   │ │   │ │ Provider │ │   │ ContentBlock │
│   └────────┘ │   │ └──────────┘ │   │              │
│   ┌────────┐ │   │ ┌──────────┐ │   │  - Titles    │
│   │  Code  │ │   │ │  OpenAI  │ │   │  - Code      │
│   │Formatter│ │   │ │  (TODO)  │ │   │  - Lists     │
│   └────────┘ │   │ └──────────┘ │   │  - Quotes    │
│              │   │ ┌──────────┐ │   │              │
│    (More)    │   │ │Anthropic │ │   │              │
│              │   │ │  (TODO)  │ │   │              │
│              │   │ └──────────┘ │   │              │
└──────────────┘   └──────────────┘   └──────────────┘
```

## Component Details

### 1. UI Module (`src/ui/mod.rs`)

**Responsibilities:**
- Render the user interface using the iced framework
- Handle user input from two text fields
- Display parsed AI responses
- Manage application state

**Key Features:**
- Two input lines for user queries
- Send button to submit requests
- Scrollable response area with formatted content
- Real-time status updates

**Technologies:**
- `iced` v0.12 - Cross-platform GUI framework
- Async message handling with tokio

### 2. AI Provider System (`src/ai_providers/mod.rs`)

**Responsibilities:**
- Abstract interface for multiple AI providers
- Provider configuration and management
- Request/response handling

**Key Traits:**
```rust
pub trait AIProvider: Send + Sync {
    fn name(&self) -> String;
    async fn configure(&mut self, config: ProviderConfig) -> Result<(), AIProviderError>;
    async fn send_request(&self, request: AIRequest) -> Result<AIResponse, AIProviderError>;
    fn is_ready(&self) -> bool;
}
```

**Implemented Providers:**
- **MockAIProvider**: For testing and demonstration
- **OpenAI** (planned)
- **Anthropic Claude** (planned)
- **Local models** (planned)

### 3. Response Parser (`src/parser/mod.rs`)

**Responsibilities:**
- Parse markdown-formatted AI responses
- Extract structured content blocks
- Provide helper methods for accessing specific content types

**Supported Content Types:**
- **Titles**: Multi-level headings (h1-h6)
- **Paragraphs**: Regular text blocks
- **Code Blocks**: With language detection
- **Lists**: Bullet and numbered lists
- **Quotes**: Block quotes

**Technologies:**
- `pulldown-cmark` - Fast markdown parser
- Serde for serialization

### 4. Plugin System (`src/plugins/mod.rs`)

**Responsibilities:**
- Extensible architecture for custom functionality
- Plugin lifecycle management
- Pre/post processing hooks

**Key Traits:**
```rust
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> PluginMetadata;
    fn capabilities(&self) -> Vec<PluginCapability>;
    async fn initialize(&mut self) -> Result<(), PluginError>;
    async fn pre_process(&self, input: &str) -> Result<String, PluginError>;
    async fn post_process(&self, response: &str) -> Result<String, PluginError>;
}
```

**Plugin Types:**
- **PreProcessor**: Modify input before sending to AI
- **PostProcessor**: Modify AI responses
- **CodeFormatter**: Format code blocks
- **CustomCommand**: Add custom commands

**Sample Plugins:**
- **UppercasePlugin**: Converts input to uppercase (demo)
- **CodeFormatterPlugin**: Adds formatting markers to code

## Data Flow

### Request Flow
```
User Input (2 lines)
    ↓
Plugin Pre-Processing
    ↓
AI Provider
    ↓
AI Response
    ↓
Plugin Post-Processing
    ↓
Response Parser
    ↓
UI Rendering
```

### Message Flow (UI)
```
User Action → Message::SendRequest
    ↓
Create AIRequest
    ↓
Async Command (tokio)
    ↓
Call AI Provider
    ↓
Message::ResponseReceived
    ↓
Update UI State
```

## Technology Stack

### Core
- **Rust** 2021 Edition - Safe, fast, concurrent
- **Cargo** - Build system and package manager

### UI
- **iced** v0.12 - Cross-platform GUI framework
- **tokio** v1.35 - Async runtime

### Parsing
- **pulldown-cmark** v0.9 - Markdown parser
- **regex** v1.10 - Regular expressions

### Serialization
- **serde** v1.0 - Serialization framework
- **serde_json** v1.0 - JSON support

### HTTP
- **reqwest** v0.11 - HTTP client (for AI APIs)

### Error Handling
- **anyhow** v1.0 - Error handling
- **thiserror** v1.0 - Error derive macros

### Async
- **async-trait** v0.1 - Async traits

## Extension Points

### Adding a New AI Provider

1. Implement the `AIProvider` trait
2. Add provider-specific configuration
3. Handle API authentication
4. Parse provider-specific response formats
5. Add to `AIProviderManager`

### Creating a Plugin

1. Implement the `Plugin` trait
2. Define plugin metadata
3. Specify capabilities
4. Implement processing hooks
5. Register with `PluginRegistry`

### Custom Response Formatting

1. Extend `ContentBlock` enum
2. Update parser to recognize new formats
3. Add UI rendering for new content types

## Performance Considerations

- **Async I/O**: All AI requests are non-blocking
- **Streaming**: UI remains responsive during long operations
- **Efficient Parsing**: Single-pass markdown parsing
- **Memory Management**: Rust's ownership system prevents leaks

## Security Considerations

- **API Keys**: Should be stored securely (config files, env vars)
- **Input Validation**: User input is sanitized
- **Plugin Isolation**: Plugin errors don't crash the app
- **Type Safety**: Rust's type system prevents common bugs

## Testing Strategy

- **Unit Tests**: Each module has comprehensive tests
- **Integration Tests**: Test component interactions
- **Mock Providers**: Enable offline testing
- **Sample Plugins**: Demonstrate plugin development

## Future Enhancements

1. **OpenAI Integration**: Add GPT-4 support
2. **Anthropic Integration**: Add Claude support
3. **Local Models**: Ollama, llama.cpp support
4. **Configuration UI**: In-app settings management
5. **Plugin Marketplace**: Share and discover plugins
6. **Code Execution**: Sandbox for running code snippets
7. **Session History**: Save and restore conversations
8. **Export Functionality**: Save responses to files
9. **Themes**: Dark mode and custom themes
10. **Keyboard Shortcuts**: Power user features

## Build and Deployment

### Development Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Running the Application
```bash
cargo run
```

### Distribution
- Single binary (no runtime dependencies)
- Platform-specific builds (macOS, Linux, Windows)
- ~10-20 MB binary size (release mode)

## License

MIT License - See LICENSE file for details
