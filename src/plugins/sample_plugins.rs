/// Sample plugin demonstrating the plugin system
use async_trait::async_trait;

use super::{Plugin, PluginCapability, PluginError, PluginMetadata};

/// A sample plugin that converts input to uppercase
pub struct UppercasePlugin {
    metadata: PluginMetadata,
    enabled: bool,
}

impl UppercasePlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "Uppercase Converter".to_string(),
                version: "0.1.0".to_string(),
                description: "Converts input text to uppercase".to_string(),
                author: "Vibe Coder Team".to_string(),
            },
            enabled: false,
        }
    }
}

impl Default for UppercasePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for UppercasePlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![PluginCapability::PreProcessor]
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        self.enabled = true;
        Ok(())
    }

    async fn pre_process(&self, input: &str) -> Result<String, PluginError> {
        if !self.enabled {
            return Err(PluginError::ProcessingError(
                "Plugin not initialized".to_string(),
            ));
        }
        Ok(input.to_uppercase())
    }
}

/// A sample plugin that adds markdown formatting to code blocks in responses
pub struct CodeFormatterPlugin {
    metadata: PluginMetadata,
    enabled: bool,
}

impl CodeFormatterPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "Code Formatter".to_string(),
                version: "0.1.0".to_string(),
                description: "Adds syntax highlighting hints to code blocks".to_string(),
                author: "Vibe Coder Team".to_string(),
            },
            enabled: false,
        }
    }
}

impl Default for CodeFormatterPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for CodeFormatterPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![PluginCapability::PostProcessor, PluginCapability::CodeFormatter]
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        self.enabled = true;
        Ok(())
    }

    async fn post_process(&self, response: &str) -> Result<String, PluginError> {
        if !self.enabled {
            return Err(PluginError::ProcessingError(
                "Plugin not initialized".to_string(),
            ));
        }
        
        // Add formatting markers (this is a simple example)
        Ok(response.replace("```", "```\n// Formatted by Code Formatter Plugin\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_uppercase_plugin() {
        let mut plugin = UppercasePlugin::new();
        plugin.initialize().await.unwrap();
        
        let result = plugin.pre_process("hello world").await.unwrap();
        assert_eq!(result, "HELLO WORLD");
    }

    #[tokio::test]
    async fn test_code_formatter_plugin() {
        let mut plugin = CodeFormatterPlugin::new();
        plugin.initialize().await.unwrap();
        
        let input = "Some text\n```rust\ncode\n```\nMore text";
        let result = plugin.post_process(input).await.unwrap();
        assert!(result.contains("// Formatted by Code Formatter Plugin"));
    }
}
