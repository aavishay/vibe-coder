/// AI Provider integration framework
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub prompt: String,
    pub context: Option<String>,
    pub temperature: f32,
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub model: String,
    pub tokens_used: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub api_key: Option<String>,
    pub api_endpoint: Option<String>,
    pub model: String,
}

/// Trait for AI providers
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Get provider name
    fn name(&self) -> String;
    
    /// Configure the provider
    async fn configure(&mut self, config: ProviderConfig) -> Result<(), AIProviderError>;
    
    /// Send a request to the AI provider
    async fn send_request(&self, request: AIRequest) -> Result<AIResponse, AIProviderError>;
    
    /// Check if provider is configured and ready
    fn is_ready(&self) -> bool;
}

#[derive(Debug, thiserror::Error)]
pub enum AIProviderError {
    #[error("Provider not configured")]
    NotConfigured,
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Mock AI Provider for testing and demonstration
pub struct MockAIProvider {
    config: Option<ProviderConfig>,
}

impl MockAIProvider {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Default for MockAIProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AIProvider for MockAIProvider {
    fn name(&self) -> String {
        "Mock Provider".to_string()
    }
    
    async fn configure(&mut self, config: ProviderConfig) -> Result<(), AIProviderError> {
        self.config = Some(config);
        Ok(())
    }
    
    async fn send_request(&self, request: AIRequest) -> Result<AIResponse, AIProviderError> {
        if !self.is_ready() {
            return Err(AIProviderError::NotConfigured);
        }
        
        // Simulate AI response with formatted content
        let response_content = format!(
            "# AI Response\n\n\
            You asked: {}\n\n\
            ## Code Example\n\n\
            ```rust\n\
            fn hello_world() {{\n    \
                println!(\"Hello from Vibe Coder!\");\n\
            }}\n\
            ```\n\n\
            ## Explanation\n\n\
            This is a mock response demonstrating the parsing capabilities.",
            request.prompt
        );
        
        Ok(AIResponse {
            content: response_content,
            model: self.config.as_ref().unwrap().model.clone(),
            tokens_used: Some(150),
        })
    }
    
    fn is_ready(&self) -> bool {
        self.config.is_some()
    }
}

/// AI Provider Manager
pub struct AIProviderManager {
    providers: Vec<Box<dyn AIProvider>>,
    active_provider_index: Option<usize>,
}

impl AIProviderManager {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            active_provider_index: None,
        }
    }
    
    pub async fn add_provider(&mut self, provider: Box<dyn AIProvider>) {
        self.providers.push(provider);
        if self.active_provider_index.is_none() && !self.providers.is_empty() {
            self.active_provider_index = Some(0);
        }
    }
    
    pub fn set_active_provider(&mut self, index: usize) -> Result<(), AIProviderError> {
        if index >= self.providers.len() {
            return Err(AIProviderError::ConfigurationError(
                "Provider index out of bounds".to_string(),
            ));
        }
        self.active_provider_index = Some(index);
        Ok(())
    }
    
    pub fn get_active_provider(&self) -> Option<&dyn AIProvider> {
        self.active_provider_index
            .and_then(|idx| self.providers.get(idx))
            .map(|p| p.as_ref())
    }
    
    pub fn list_providers(&self) -> Vec<String> {
        self.providers.iter().map(|p| p.name()).collect()
    }
}

impl Default for AIProviderManager {
    fn default() -> Self {
        Self::new()
    }
}
