# Vibe Coder

An AI coding console built with Go (Wails) and React. Vibe Coder provides a modern, VS Code/Zed-inspired interface for interacting with AI coding assistants locally.

## Features

- 🎨 **Modern Editor UI**: VS Code/Zed-inspired layout with activity bar, sidebar, tab bar, editor pane, and status bar
- 🖋️ **Font Customization**: Cycle through developer fonts (JetBrains Mono, Fira Code, SF Mono, Cascadia Code, Menlo) and adjust font size
- 🤖 **Multi-Provider AI Support**: 
  - Built-in support for Ollama (local AI)
  - Mock provider for testing
  - Easy to extend with additional providers (Copilot, Gemini, Claude)
  - Provider configuration dialog with type selection
- 🎭 **Theme Support**: Dark/light modes with VS Code and Zed color palettes
- 📝 **Smart Response Display**: Monaco Editor for syntax-highlighted code and markdown rendering
- ⚡ **Fast & Native**: Go backend with embedded React frontend using Wails

## Tech Stack

- **Backend**: Go 1.22+ with Wails v2
- **Frontend**: React 18 + TypeScript + Vite
- **Styling**: Tailwind CSS with shadcn/ui design tokens
- **Icons**: Lucide React
- **Editor**: Monaco Editor for code display

## Installation

### Prerequisites

- Go 1.22 or higher
- Node.js 18+ and npm
- Wails CLI: `go install github.com/wailsapp/wails/v2/cmd/wails@latest`

### Building from Source

```bash
# Clone the repository
git clone https://github.com/aavishay/vibe-coder.git
cd vibe-coder/wails-app

# Install frontend dependencies
cd frontend
npm install
cd ..

# Build and run
wails dev    # Development mode with live reload
wails build  # Production build
```

## Usage

1. **Launch the application**:
   ```bash
   cd wails-app
   wails dev
   ```

2. **Customize appearance**:
   - Click **VS Code/Zed** button to toggle editor style
   - Click **Dark/Light** button to toggle theme
   - Click **Font** button to cycle through fonts
   - Click **A-** / **A+** buttons to adjust font size

3. **Configure AI providers**:
   - Click **Providers** button in status bar
   - Click **Cycle Type** to select provider (Ollama, Copilot, Gemini, Claude, Mock)
   - Enter configuration:
     - Name (optional, defaults to provider type)
     - API Key
     - Endpoint (defaults provided)
     - Model (defaults provided)
   - Click **Add Provider**
   
4. **For Ollama** (local AI):
   - Install Ollama: `brew install ollama` or visit https://ollama.ai
   - Pull a model: `ollama pull llama3`
   - In the app, add Ollama provider with:
     - Endpoint: `http://localhost:11434` (default)
     - Model: `llama3` (or any model you have)

5. **Send prompts**:
   - Type your coding question in the textarea
   - Click **Send** or press Enter
   - Responses appear in Monaco Editor with syntax highlighting

## Architecture

### Backend (Go)

```
wails-app/
├── main.go              # Wails app, provider management, API endpoints
├── go.mod               # Go dependencies
└── wails.json           # Wails configuration
```

**Key Components**:
- `Provider` interface: Generic AI provider abstraction
- `OllamaProvider`: HTTP client for Ollama API
- `MockProvider`: Testing fallback
- `App` struct: Provider manager with thread-safe operations

**API Methods**:
- `AddProvider(config)` - Register new provider
- `ListProviders()` - Get all provider names  
- `SetActiveProvider(index)` - Switch active provider
- `SendPrompt(prompt)` - Send request to active provider

### Frontend (React + TypeScript)

```
wails-app/frontend/
├── src/
│   ├── ui/App.tsx       # Main app component
│   ├── main.tsx         # React entry point
│   └── index.css        # Tailwind + CSS variables
├── package.json         # Frontend dependencies
├── vite.config.ts       # Vite bundler config
└── tailwind.config.js   # Tailwind + shadcn/ui tokens
```

**Features**:
- Monaco Editor for response display
- Lucide icons for UI elements
- Tailwind CSS for styling
- Provider dialog modal
- Font and theme customization

## Development

### Project Structure

```
vibe-coder/
├── wails-app/           # Main application
│   ├── main.go          # Go backend
│   ├── frontend/        # React frontend
│   └── build/           # Build artifacts (gitignored)
├── .github/             # GitHub workflows
├── Makefile             # Build automation
└── README.md
```

### Adding New AI Providers

Implement the `Provider` interface in Go:

```go
type MyProvider struct {
    config ProviderConfig
    client *http.Client
}

func (p *MyProvider) GetName() string {
    return "My Provider"
}

func (p *MyProvider) SendRequest(prompt string, temperature float64, maxTokens int) (string, error) {
    // Implement your API call
    return "response", nil
}
```

Then add to the provider factory in `AddProvider()`:

```go
case "MyProvider":
    provider = NewMyProvider(config)
```

## Testing

Frontend tests (in development):
```bash
cd wails-app/frontend
npm test
```

Backend tests (in development):
```bash
cd wails-app
go test ./...
```

## Roadmap

- [x] Ollama provider implementation
- [x] Font customization
- [x] Provider management UI
- [x] Monaco Editor integration
- [ ] OpenAI/Copilot provider
- [ ] Anthropic Claude provider  
- [ ] Gemini provider
- [ ] Configuration file persistence
- [ ] Session history
- [ ] Export conversations
- [ ] Multiple chat tabs
- [ ] shadcn/ui component library integration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the MIT License.

## Support

For issues, questions, or contributions, please visit the [GitHub repository](https://github.com/aavishay/vibe-coder).
