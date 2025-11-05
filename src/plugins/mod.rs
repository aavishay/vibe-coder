/// Plugin system for extensibility
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod sample_plugins;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Clone)]
pub enum PluginCapability {
    PreProcessor,
    PostProcessor,
    CodeFormatter,
    CustomCommand,
}

/// Base trait for all plugins
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;
    
    /// Get plugin capabilities
    fn capabilities(&self) -> Vec<PluginCapability>;
    
    /// Initialize the plugin
    async fn initialize(&mut self) -> Result<(), PluginError>;
    
    /// Process input before sending to AI
    async fn pre_process(&self, input: &str) -> Result<String, PluginError> {
        Ok(input.to_string())
    }
    
    /// Process AI response
    async fn post_process(&self, response: &str) -> Result<String, PluginError> {
        Ok(response.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin initialization failed: {0}")]
    InitializationError(String),
    
    #[error("Plugin processing error: {0}")]
    ProcessingError(String),
    
    #[error("Plugin not found: {0}")]
    NotFound(String),
}

/// Plugin registry for managing plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
    
    pub async fn register(&mut self, mut plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        plugin.initialize().await?;
        let name = plugin.metadata().name.clone();
        self.plugins.insert(name, plugin);
        Ok(())
    }
    
    pub fn get(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }
    
    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        self.plugins.values()
            .map(|p| p.metadata())
            .collect()
    }
    
    pub async fn pre_process_all(&self, input: &str) -> Result<String, PluginError> {
        let mut result = input.to_string();
        for plugin in self.plugins.values() {
            result = plugin.pre_process(&result).await?;
        }
        Ok(result)
    }
    
    pub async fn post_process_all(&self, response: &str) -> Result<String, PluginError> {
        let mut result = response.to_string();
        for plugin in self.plugins.values() {
            result = plugin.post_process(&result).await?;
        }
        Ok(result)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
