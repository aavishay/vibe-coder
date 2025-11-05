# Quick Start Guide

Get up and running with Vibe Coder in minutes!

## Prerequisites

Before you begin, ensure you have:
- **Rust** 1.70 or higher installed ([Install Rust](https://rustup.rs/))
- **macOS** 10.15+ (or Linux/Windows for cross-platform use)

## Installation

### Option 1: Build from Source

```bash
# Clone the repository
git clone https://github.com/aavishay/vibe-coder.git
cd vibe-coder

# Build the project (this may take a few minutes the first time)
cargo build --release

# Run the application
cargo run --release
```

### Option 2: Using Cargo Install (when published)

```bash
# Install directly from crates.io
cargo install vibe-coder

# Run the application
vibe-coder
```

## First Launch

When you first launch Vibe Coder:

1. You'll see two input fields at the top
2. A response area below (initially empty)
3. The app starts with a mock AI provider for testing

## Basic Usage

### Making Your First Query

1. **Enter your query** in the two input fields:
   - **Line 1**: "Write a Rust function"
   - **Line 2**: "that calculates fibonacci numbers"

2. **Click "Send Request"** or press Enter in the second field

3. **View the response** in the display area below:
   - Titles and headings will be formatted
   - Code blocks will be syntax-highlighted
   - Lists and quotes will be properly displayed

### Example Queries

Try these examples to see Vibe Coder in action:

**Example 1: Code Generation**
- Line 1: `Create a function in Rust`
- Line 2: `that reverses a string`

**Example 2: Explanation**
- Line 1: `Explain how async/await works`
- Line 2: `in Rust programming`

**Example 3: Debug Help**
- Line 1: `How to fix a borrow checker error`
- Line 2: `when passing variables to closures`

## Understanding the Interface

### Input Section
- **Input Line 1**: Main query or topic
- **Input Line 2**: Additional context or specifics
- **Send Button**: Submit your request
- **Status Message**: Shows current state (Ready, Processing, Error)

### Response Section
- **Scrollable Area**: View AI responses
- **Formatted Content**:
  - # Large headings (h1)
  - ## Medium headings (h2)
  - ### Smaller headings (h3)
  - Regular paragraphs
  - ```code blocks``` with language labels
  - • Bulleted lists
  - ❝ Quoted text

## Configuration

### Using the Mock Provider (Default)

The mock provider is active by default and requires no configuration. It generates sample responses to test the UI.

### Adding a Real AI Provider (Future)

1. Copy `config.example.toml` to `config.toml`
2. Edit `config.toml` with your AI provider settings
3. Restart the application

Example configuration:
```toml
[ai_provider]
active = "openai"

[ai_provider.openai]
api_key = "your-api-key-here"
model = "gpt-4"
```

## Tips for Best Results

1. **Be Specific**: Use both input lines to provide context
2. **Iterate**: Build on previous responses
3. **Clear Requests**: Start with clear, focused questions
4. **Use Keywords**: Programming language, framework names

## Troubleshooting

### Application Won't Start

**Issue**: Error when running `cargo run`
```bash
# Solution 1: Update Rust
rustup update

# Solution 2: Clean and rebuild
cargo clean
cargo build --release
```

### Compilation Errors

**Issue**: Build fails with dependency errors
```bash
# Solution: Update dependencies
cargo update
cargo build --release
```

### UI Not Responding

**Issue**: Application freezes or doesn't respond
- The mock provider should respond quickly
- Check the status message for errors
- Restart the application

### Code Blocks Not Formatted

**Issue**: Code appears as plain text
- Ensure the AI response uses markdown code blocks
- Example: \`\`\`rust\ncode here\n\`\`\`
- The mock provider already does this

## Next Steps

### Explore Sample Plugins

Check out `src/plugins/sample_plugins.rs` to see example plugins:
- **UppercasePlugin**: Converts input to uppercase
- **CodeFormatterPlugin**: Adds formatting markers

### Create Your Own Plugin

See the [CONTRIBUTING.md](CONTRIBUTING.md) guide for:
- Plugin development
- Adding AI providers
- Extending functionality

### Read the Documentation

- [README.md](README.md) - Full project documentation
- [ARCHITECTURE.md](ARCHITECTURE.md) - System design and architecture
- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute

## Getting Help

If you encounter issues:

1. **Check the FAQ** (coming soon)
2. **Search Issues**: [GitHub Issues](https://github.com/aavishay/vibe-coder/issues)
3. **Ask a Question**: Open a new issue
4. **Join Discussions**: Participate in GitHub Discussions

## What's Next?

Now that you're set up:

- ✅ Try different query combinations
- ✅ Explore the source code
- ✅ Create a custom plugin
- ✅ Contribute to the project
- ✅ Star the repo if you like it!

Happy coding with Vibe Coder! 🚀
