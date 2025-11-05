/// Common utilities and helpers for e2e tests
use vibe_coder::ai_providers::{AIProvider, AIRequest, ProviderConfig, MockAIProvider};
use vibe_coder::plugins::PluginRegistry;
use vibe_coder::parser::{parse_response, ParsedResponse};

/// Test helper to create a default AI request
pub fn create_test_request(prompt: &str) -> AIRequest {
    AIRequest {
        prompt: prompt.to_string(),
        context: None,
        temperature: 0.7,
        max_tokens: Some(1000),
    }
}

/// Test helper to create a provider config
pub fn create_test_config(name: &str, model: &str) -> ProviderConfig {
    ProviderConfig {
        name: name.to_string(),
        api_key: Some("test-api-key".to_string()),
        api_endpoint: Some("https://api.test.com".to_string()),
        model: model.to_string(),
    }
}

/// Test helper to create and configure a mock provider
pub async fn setup_mock_provider() -> MockAIProvider {
    let mut provider = MockAIProvider::new();
    let config = create_test_config("Test Provider", "test-model-v1");
    provider.configure(config).await.expect("Failed to configure provider");
    provider
}

/// Test helper to create a plugin registry with sample plugins
pub async fn setup_plugin_registry() -> PluginRegistry {
    PluginRegistry::new()
}

/// Test helper to parse a response
pub fn parse_test_response(content: &str) -> ParsedResponse {
    parse_response(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_request() {
        let request = create_test_request("Hello, world!");
        assert_eq!(request.prompt, "Hello, world!");
        assert_eq!(request.temperature, 0.7);
        assert_eq!(request.max_tokens, Some(1000));
    }

    #[test]
    fn test_create_test_config() {
        let config = create_test_config("TestProvider", "model-1");
        assert_eq!(config.name, "TestProvider");
        assert_eq!(config.model, "model-1");
        assert!(config.api_key.is_some());
    }

    #[tokio::test]
    async fn test_setup_mock_provider() {
        let provider = setup_mock_provider().await;
        assert!(provider.is_ready());
        assert_eq!(provider.name(), "Mock Provider");
    }

    #[tokio::test]
    async fn test_setup_plugin_registry() {
        let registry = setup_plugin_registry().await;
        assert_eq!(registry.list_plugins().len(), 0);
    }
}
