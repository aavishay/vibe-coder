# Vibe Coder End-to-End Test Suite

This directory contains comprehensive end-to-end (e2e) tests for the Vibe Coder project, covering all roadmap items and ensuring complete system integration.

## Test Structure

### Test Modules

1. **`ai_providers_e2e.rs`** - AI Provider System Tests
   - Provider configuration and management
   - Request/response handling
   - Multiple provider support (OpenAI, Anthropic, Local models)
   - Provider manager functionality
   - Complete AI workflow testing

2. **`plugins_e2e.rs`** - Plugin System Tests
   - Plugin registration and lifecycle
   - Pre-processor and post-processor plugins
   - Plugin chaining and composition
   - Plugin registry management
   - Thread-safe plugin operations

3. **`configuration_e2e.rs`** - Configuration System Tests
   - TOML configuration parsing
   - Provider settings management
   - Plugin configuration
   - UI and general settings
   - Configuration file persistence

4. **`session_history_e2e.rs`** - Session History Tests
   - Session entry management
   - History size limits
   - Search functionality
   - Timestamp ordering
   - Token usage tracking
   - Persistence to disk

5. **`export_e2e.rs`** - Export Functionality Tests
   - Export to Markdown format
   - Export to JSON format
   - Export to Plain Text format
   - Export to HTML format
   - File saving operations
   - Complete export workflows

6. **`integration_e2e.rs`** - Integration Tests
   - Multi-component workflows
   - AI Provider + Plugin integration
   - Parser integration with AI responses
   - Complete user session simulations
   - Performance testing
   - Error handling across components

7. **`common/mod.rs`** - Test Utilities
   - Helper functions for test setup
   - Mock provider creation
   - Request/response builders
   - Shared test fixtures

## Running Tests

### Run All E2E Tests
```bash
cargo test --test '*_e2e'
```

### Run Specific Test Module
```bash
# AI Provider tests
cargo test --test ai_providers_e2e

# Plugin system tests
cargo test --test plugins_e2e

# Configuration tests
cargo test --test configuration_e2e

# Session history tests
cargo test --test session_history_e2e

# Export tests
cargo test --test export_e2e

# Integration tests
cargo test --test integration_e2e
```

### Run Individual Tests
```bash
# Run specific test by name
cargo test --test ai_providers_e2e test_mock_provider_configuration

# Run with output
cargo test --test plugins_e2e -- --nocapture

# Run tests matching a pattern
cargo test --test integration_e2e complete
```

### Run All Tests (including unit tests)
```bash
cargo test
```

## Test Coverage

### Roadmap Features Tested

✅ **AI Provider System**
- Mock provider implementation
- Provider configuration
- Request/response handling
- Multiple provider management
- Provider switching
- Error handling

✅ **Plugin System**
- Plugin registration
- Plugin metadata
- Pre-processing and post-processing
- Plugin chaining
- Sample plugins (Uppercase, CodeFormatter)
- Thread safety

✅ **Configuration Support**
- TOML configuration parsing
- Provider settings
- Plugin settings
- UI preferences
- General settings
- File persistence

✅ **Session History**
- Entry management
- Size limits
- Search functionality
- Chronological ordering
- Token tracking
- JSON persistence

✅ **Export Functionality**
- Markdown export
- JSON export
- Plain text export
- HTML export
- File saving
- Special character handling

✅ **Integration Testing**
- Complete user workflows
- Component interactions
- Error handling
- Performance under load
- Concurrent operations

## Test Statistics

- **Total Test Files**: 7
- **Total Tests**: 100+ individual test cases
- **Test Categories**: Unit, Integration, E2E
- **Coverage Areas**: All major roadmap items

## Test Best Practices

### Writing New Tests

1. **Use Descriptive Names**
   ```rust
   #[tokio::test]
   async fn test_provider_handles_empty_response() { }
   ```

2. **Arrange-Act-Assert Pattern**
   ```rust
   // Arrange
   let provider = setup_mock_provider().await;
   
   // Act
   let response = provider.send_request(request).await;
   
   // Assert
   assert!(response.is_ok());
   ```

3. **Use Test Helpers**
   ```rust
   use common::{setup_mock_provider, create_test_request};
   ```

4. **Clean Up Resources**
   ```rust
   // Cleanup temporary files
   let _ = fs::remove_file(&test_file);
   ```

### Async Testing

All async tests use the `tokio::test` attribute:

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Temporary Files

Tests that create files use `std::env::temp_dir()`:

```rust
let temp_dir = std::env::temp_dir();
let test_file = temp_dir.join("test_file.txt");
// ... use file ...
let _ = fs::remove_file(&test_file); // cleanup
```

## Continuous Integration

These tests are designed to run in CI/CD pipelines:

- Fast execution time
- No external dependencies required
- Isolated test environments
- Automatic cleanup
- Clear pass/fail status

## Test Output

### Successful Test Run
```
running 100 tests
test ai_providers_e2e::test_mock_provider_configuration ... ok
test plugins_e2e::test_plugin_registry_initialization ... ok
test configuration_e2e::test_config_serialization ... ok
...
test result: ok. 100 passed; 0 failed; 0 ignored; 0 measured
```

### With Output Details
```bash
cargo test -- --nocapture --test-threads=1
```

## Future Test Additions

Planned test coverage for upcoming features:

- [ ] OpenAI provider integration tests (when implemented)
- [ ] Anthropic provider integration tests (when implemented)
- [ ] Local model (Ollama) integration tests (when implemented)
- [ ] Plugin marketplace tests (when implemented)
- [ ] Code execution sandbox tests (when implemented)
- [ ] UI component tests (when UI testing framework is added)

## Troubleshooting

### Test Failures

**Issue**: Test fails due to file permissions
```
Solution: Ensure temp directory is writable
```

**Issue**: Async test timeout
```
Solution: Increase timeout or check for deadlocks
```

**Issue**: Tests fail intermittently
```
Solution: Check for race conditions or shared state
```

### Running Specific Tests

Find test names:
```bash
cargo test -- --list
```

Run with filtering:
```bash
cargo test provider -- --nocapture
```

## Contributing

When adding new tests:

1. Follow existing patterns and naming conventions
2. Add tests to appropriate module
3. Update this README if adding new test category
4. Ensure tests are isolated and don't depend on order
5. Clean up any resources (files, connections, etc.)
6. Document complex test scenarios

## Documentation

Each test module contains detailed documentation:
- Purpose of the test suite
- Roadmap items covered
- Integration points tested
- Usage examples

## License

These tests are part of the Vibe Coder project and share the same MIT license.

---

**Last Updated**: 2024-01-01  
**Test Coverage**: 100+ tests across 7 modules  
**Status**: ✅ All tests passing
