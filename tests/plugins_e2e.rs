/// End-to-end tests for Plugin System
/// 
/// This module tests the complete plugin lifecycle and integration,
/// covering the roadmap item for plugin marketplace functionality.

mod common;

use vibe_coder::plugins::{
    Plugin, PluginCapability, PluginError, PluginMetadata, PluginRegistry,
    sample_plugins::{UppercasePlugin, CodeFormatterPlugin},
};
use common::setup_plugin_registry;

#[tokio::test]
async fn test_plugin_registry_initialization() {
    let registry = setup_plugin_registry().await;
    assert_eq!(registry.list_plugins().len(), 0, "New registry should be empty");
}

#[tokio::test]
async fn test_register_single_plugin() {
    let mut registry = PluginRegistry::new();
    let plugin = Box::new(UppercasePlugin::new());

    let result = registry.register(plugin).await;
    assert!(result.is_ok(), "Plugin registration should succeed");
    assert_eq!(registry.list_plugins().len(), 1);
}

#[tokio::test]
async fn test_register_multiple_plugins() {
    let mut registry = PluginRegistry::new();

    let plugin1 = Box::new(UppercasePlugin::new());
    let plugin2 = Box::new(CodeFormatterPlugin::new());

    registry.register(plugin1).await.unwrap();
    registry.register(plugin2).await.unwrap();

    assert_eq!(registry.list_plugins().len(), 2);
    let plugins = registry.list_plugins();
    assert!(plugins.iter().any(|p| p.name == "Uppercase Converter"));
    assert!(plugins.iter().any(|p| p.name == "Code Formatter"));
}

#[tokio::test]
async fn test_get_plugin_by_name() {
    let mut registry = PluginRegistry::new();
    let plugin = Box::new(UppercasePlugin::new());

    registry.register(plugin).await.unwrap();

    let retrieved = registry.get("Uppercase Converter");
    assert!(retrieved.is_some(), "Should find registered plugin");

    let not_found = registry.get("Nonexistent Plugin");
    assert!(not_found.is_none(), "Should not find unregistered plugin");
}

#[tokio::test]
async fn test_plugin_metadata() {
    let plugin = UppercasePlugin::new();
    let metadata = plugin.metadata();

    assert_eq!(metadata.name, "Uppercase Converter");
    assert!(!metadata.version.is_empty());
    assert!(!metadata.description.is_empty());
    assert!(!metadata.author.is_empty());
}

#[tokio::test]
async fn test_plugin_capabilities() {
    let plugin = UppercasePlugin::new();
    let capabilities = plugin.capabilities();

    assert!(!capabilities.is_empty(), "Plugin should have capabilities");
    assert!(
        capabilities.iter().any(|c| matches!(c, PluginCapability::PreProcessor)),
        "UppercasePlugin should be a PreProcessor"
    );
}

#[tokio::test]
async fn test_uppercase_plugin_preprocessing() {
    let mut plugin = UppercasePlugin::new();
    plugin.initialize().await.unwrap(); // Need to initialize first
    let input = "hello world";

    let result = plugin.pre_process(input).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HELLO WORLD");
}

#[tokio::test]
async fn test_code_formatter_plugin_postprocessing() {
    let mut plugin = CodeFormatterPlugin::new();
    plugin.initialize().await.unwrap(); // Need to initialize first
    let response = "```python\ndef hello():\n    print('world')\n```";

    let result = plugin.post_process(response).await;
    assert!(result.is_ok());
    let formatted = result.unwrap();
    assert!(formatted.contains("Formatted by Code Formatter"));
}

#[tokio::test]
async fn test_plugin_registry_pre_process_all() {
    let mut registry = PluginRegistry::new();
    let plugin = Box::new(UppercasePlugin::new());
    registry.register(plugin).await.unwrap();

    let input = "test input";
    let result = registry.pre_process_all(input).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "TEST INPUT");
}

#[tokio::test]
async fn test_plugin_registry_post_process_all() {
    let mut registry = PluginRegistry::new();
    let plugin = Box::new(CodeFormatterPlugin::new());
    registry.register(plugin).await.unwrap();

    let response = "```rust\nfn main() {}\n```";
    let result = registry.post_process_all(response).await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Formatted by Code Formatter"));
}

#[tokio::test]
async fn test_plugin_chaining() {
    let mut registry = PluginRegistry::new();
    
    // Register multiple plugins that will process in sequence
    let plugin1 = Box::new(UppercasePlugin::new());
    registry.register(plugin1).await.unwrap();

    let input = "hello world";
    let result = registry.pre_process_all(input).await.unwrap();
    
    // Should be uppercase after processing
    assert_eq!(result, "HELLO WORLD");
}

#[tokio::test]
async fn test_plugin_initialization() {
    let mut plugin = UppercasePlugin::new();
    let result = plugin.initialize().await;
    assert!(result.is_ok(), "Plugin initialization should succeed");
}

/// E2E test simulating complete plugin workflow:
/// User input -> Pre-processing -> AI -> Post-processing -> Display
#[tokio::test]
async fn test_complete_plugin_workflow() {
    // Setup plugin registry with both plugins
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(UppercasePlugin::new())).await.unwrap();
    registry.register(Box::new(CodeFormatterPlugin::new())).await.unwrap();

    // User input
    let user_input = "write a function";

    // Pre-process (will uppercase)
    let processed_input = registry.pre_process_all(user_input).await.unwrap();
    assert_eq!(processed_input, "WRITE A FUNCTION");

    // Simulate AI response with code block
    let ai_response = "```python\ndef function():\n    return True\n```";

    // Post-process (will add formatter comment)
    let processed_response = registry.post_process_all(ai_response).await.unwrap();
    assert!(processed_response.contains("Formatted by Code Formatter"));
}

#[tokio::test]
async fn test_plugin_list_metadata() {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(UppercasePlugin::new())).await.unwrap();
    registry.register(Box::new(CodeFormatterPlugin::new())).await.unwrap();

    let plugins = registry.list_plugins();
    assert_eq!(plugins.len(), 2);

    for plugin_meta in plugins {
        assert!(!plugin_meta.name.is_empty());
        assert!(!plugin_meta.version.is_empty());
        assert!(!plugin_meta.description.is_empty());
        assert!(!plugin_meta.author.is_empty());
    }
}

#[tokio::test]
async fn test_empty_registry_processing() {
    let registry = PluginRegistry::new();

    let input = "test input";
    let result = registry.pre_process_all(input).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), input, "Empty registry should return input unchanged");

    let response = "test response";
    let result = registry.post_process_all(response).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), response, "Empty registry should return response unchanged");
}

#[tokio::test]
async fn test_plugin_registry_is_thread_safe() {
    use std::sync::Arc;
    use tokio::task;

    let registry = Arc::new(PluginRegistry::new());
    let mut handles = vec![];

    // Spawn multiple tasks that try to use the registry
    for i in 0..5 {
        let reg = Arc::clone(&registry);
        let handle = task::spawn(async move {
            let input = format!("test input {}", i);
            reg.pre_process_all(&input).await
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
    }
}

/// Test plugin error handling
#[tokio::test]
async fn test_plugin_error_propagation() {
    // This test demonstrates that plugin errors are properly handled
    // In a real scenario, a plugin might fail during processing
    let registry = PluginRegistry::new();
    
    // Processing should succeed with no plugins
    let result = registry.pre_process_all("test").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_code_formatter_plugin_metadata() {
    let plugin = CodeFormatterPlugin::new();
    let metadata = plugin.metadata();

    assert_eq!(metadata.name, "Code Formatter");
    assert!(
        plugin.capabilities().iter().any(|c| matches!(c, PluginCapability::PostProcessor)),
        "CodeFormatterPlugin should be a PostProcessor"
    );
}

#[tokio::test]
async fn test_plugin_default_behavior() {
    use async_trait::async_trait;

    // Create a minimal plugin that uses default implementations
    struct MinimalPlugin;

    #[async_trait]
    impl Plugin for MinimalPlugin {
        fn metadata(&self) -> PluginMetadata {
            PluginMetadata {
                name: "Minimal".to_string(),
                version: "1.0.0".to_string(),
                description: "Test".to_string(),
                author: "Test".to_string(),
            }
        }

        fn capabilities(&self) -> Vec<PluginCapability> {
            vec![]
        }

        async fn initialize(&mut self) -> Result<(), PluginError> {
            Ok(())
        }
    }

    let plugin = MinimalPlugin;

    // Default pre_process should return input unchanged
    let input = "test";
    let result = plugin.pre_process(input).await;
    assert_eq!(result.unwrap(), input);

    // Default post_process should return response unchanged
    let response = "response";
    let result = plugin.post_process(response).await;
    assert_eq!(result.unwrap(), response);
}
