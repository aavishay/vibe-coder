/// End-to-end tests for AI Provider system
/// 
/// This module tests the complete flow of AI provider integration,
/// covering the roadmap items for OpenAI, Anthropic, and local model support.

mod common;

use vibe_coder::ai_providers::{
    AIProvider, AIProviderError, AIProviderManager, AIRequest, MockAIProvider,
    ProviderConfig,
};
use common::{create_test_config, create_test_request, setup_mock_provider};

#[tokio::test]
async fn test_mock_provider_configuration() {
    let mut provider = MockAIProvider::new();
    assert!(!provider.is_ready(), "Provider should not be ready before configuration");

    let config = create_test_config("Mock Provider", "mock-v1");
    let result = provider.configure(config).await;
    assert!(result.is_ok(), "Configuration should succeed");
    assert!(provider.is_ready(), "Provider should be ready after configuration");
}

#[tokio::test]
async fn test_mock_provider_send_request() {
    let provider = setup_mock_provider().await;
    let request = create_test_request("Write a hello world function in Rust");

    let result = provider.send_request(request).await;
    assert!(result.is_ok(), "Request should succeed");

    let response = result.unwrap();
    assert!(!response.content.is_empty(), "Response content should not be empty");
    assert_eq!(response.model, "test-model-v1");
    assert!(response.tokens_used.is_some(), "Token usage should be tracked");
}

#[tokio::test]
async fn test_mock_provider_unconfigured_request() {
    let provider = MockAIProvider::new();
    let request = create_test_request("Test prompt");

    let result = provider.send_request(request).await;
    assert!(result.is_err(), "Unconfigured provider should fail");

    if let Err(AIProviderError::NotConfigured) = result {
        // Expected error
    } else {
        panic!("Expected NotConfigured error");
    }
}

#[tokio::test]
async fn test_provider_manager_add_provider() {
    let mut manager = AIProviderManager::new();
    assert_eq!(manager.list_providers().len(), 0);

    let provider = setup_mock_provider().await;
    manager.add_provider(Box::new(provider)).await;

    assert_eq!(manager.list_providers().len(), 1);
    assert!(manager.get_active_provider().is_some());
}

#[tokio::test]
async fn test_provider_manager_multiple_providers() {
    let mut manager = AIProviderManager::new();

    // Add first provider
    let provider1 = setup_mock_provider().await;
    manager.add_provider(Box::new(provider1)).await;

    // Add second provider
    let mut provider2 = MockAIProvider::new();
    let config2 = create_test_config("Second Provider", "mock-v2");
    provider2.configure(config2).await.unwrap();
    manager.add_provider(Box::new(provider2)).await;

    assert_eq!(manager.list_providers().len(), 2);
    assert_eq!(manager.list_providers()[0], "Mock Provider");
    assert_eq!(manager.list_providers()[1], "Mock Provider");
}

#[tokio::test]
async fn test_provider_manager_set_active_provider() {
    let mut manager = AIProviderManager::new();

    let provider1 = setup_mock_provider().await;
    manager.add_provider(Box::new(provider1)).await;

    let provider2 = setup_mock_provider().await;
    manager.add_provider(Box::new(provider2)).await;

    // Set second provider as active
    let result = manager.set_active_provider(1);
    assert!(result.is_ok());

    // Try to set invalid index
    let result = manager.set_active_provider(10);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_provider_response_parsing_integration() {
    let provider = setup_mock_provider().await;
    let request = create_test_request("Explain Rust ownership");

    let response = provider.send_request(request).await.unwrap();

    // Verify response contains expected markdown sections
    assert!(response.content.contains("# AI Response"));
    assert!(response.content.contains("## Code Example"));
    assert!(response.content.contains("```rust"));
    assert!(response.content.contains("## Explanation"));
}

#[tokio::test]
async fn test_provider_with_context() {
    let provider = setup_mock_provider().await;
    let mut request = create_test_request("Continue the code");
    request.context = Some("Previously we defined a struct User".to_string());

    let result = provider.send_request(request).await;
    assert!(result.is_ok());
    assert!(!result.unwrap().content.is_empty());
}

#[tokio::test]
async fn test_provider_with_temperature_settings() {
    let provider = setup_mock_provider().await;

    // Test with low temperature (more deterministic)
    let mut request1 = create_test_request("Write code");
    request1.temperature = 0.1;
    let result1 = provider.send_request(request1).await;
    assert!(result1.is_ok());

    // Test with high temperature (more creative)
    let mut request2 = create_test_request("Write code");
    request2.temperature = 1.5;
    let result2 = provider.send_request(request2).await;
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_provider_with_max_tokens() {
    let provider = setup_mock_provider().await;

    let mut request = create_test_request("Generate code");
    request.max_tokens = Some(100);

    let result = provider.send_request(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    if let Some(tokens) = response.tokens_used {
        assert!(tokens > 0, "Should report token usage");
    }
}

#[tokio::test]
async fn test_provider_name_consistency() {
    let provider = MockAIProvider::new();
    assert_eq!(provider.name(), "Mock Provider");
}

/// E2E test simulating complete workflow:
/// User query -> Provider processing -> Response
#[tokio::test]
async fn test_complete_ai_workflow() {
    // Setup
    let mut manager = AIProviderManager::new();
    let provider = setup_mock_provider().await;
    manager.add_provider(Box::new(provider)).await;

    // Create user request
    let request = AIRequest {
        prompt: "Implement a binary search function".to_string(),
        context: Some("Using Rust with proper error handling".to_string()),
        temperature: 0.7,
        max_tokens: Some(500),
    };

    // Get active provider and send request
    let active_provider = manager.get_active_provider();
    assert!(active_provider.is_some());

    let response = active_provider.unwrap().send_request(request).await;
    assert!(response.is_ok());

    // Verify response structure
    let ai_response = response.unwrap();
    assert!(!ai_response.content.is_empty());
    assert!(!ai_response.model.is_empty());
    assert!(ai_response.tokens_used.is_some());
}

/// Test provider configuration with various settings
#[tokio::test]
async fn test_provider_configuration_variants() {
    let mut provider = MockAIProvider::new();

    // Test with minimal config
    let config1 = ProviderConfig {
        name: "Minimal".to_string(),
        api_key: None,
        api_endpoint: None,
        model: "default".to_string(),
    };
    assert!(provider.configure(config1).await.is_ok());

    // Test with full config
    let config2 = ProviderConfig {
        name: "Full Config".to_string(),
        api_key: Some("sk-test-key-123".to_string()),
        api_endpoint: Some("https://api.example.com/v1".to_string()),
        model: "gpt-4".to_string(),
    };
    let mut provider2 = MockAIProvider::new();
    assert!(provider2.configure(config2).await.is_ok());
}
