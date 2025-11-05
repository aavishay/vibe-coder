/// End-to-end tests for Configuration System
/// 
/// This module tests configuration file support, covering the roadmap
/// item for configuration file functionality.

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    #[serde(default)]
    ai_providers: Vec<ProviderSettings>,
    #[serde(default)]
    plugins: Vec<PluginSettings>,
    #[serde(default)]
    ui: UiSettings,
    #[serde(default)]
    general: GeneralSettings,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProviderSettings {
    name: String,
    enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_endpoint: Option<String>,
    model: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PluginSettings {
    name: String,
    enabled: bool,
}

fn default_theme() -> String {
    "light".to_string()
}

fn default_font_size() -> u32 {
    14
}

fn default_auto_save() -> bool {
    true
}

fn default_max_history() -> usize {
    100
}

#[derive(Debug, Serialize, Deserialize)]
struct UiSettings {
    #[serde(default = "default_theme")]
    theme: String,
    #[serde(default = "default_font_size")]
    font_size: u32,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            font_size: default_font_size(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GeneralSettings {
    #[serde(default = "default_auto_save")]
    auto_save: bool,
    #[serde(default = "default_max_history")]
    max_history: usize,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            auto_save: default_auto_save(),
            max_history: default_max_history(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ai_providers: vec![],
            plugins: vec![],
            ui: UiSettings {
                theme: "light".to_string(),
                font_size: 14,
            },
            general: GeneralSettings {
                auto_save: true,
                max_history: 100,
            },
        }
    }
}

#[test]
fn test_config_serialization() {
    let config = AppConfig::default();
    let toml_str = toml::to_string(&config);
    assert!(toml_str.is_ok(), "Config should serialize to TOML");
}

#[test]
fn test_config_deserialization() {
    let toml_str = r#"
        [ui]
        theme = "dark"
        font_size = 16

        [general]
        auto_save = false
        max_history = 50
    "#;

    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_ok(), "Should parse valid TOML");

    let cfg = config.unwrap();
    assert_eq!(cfg.ui.theme, "dark");
    assert_eq!(cfg.ui.font_size, 16);
    assert_eq!(cfg.general.auto_save, false);
    assert_eq!(cfg.general.max_history, 50);
}

#[test]
fn test_config_with_providers() {
    let toml_str = r#"
        [[ai_providers]]
        name = "OpenAI"
        enabled = true
        api_key = "sk-test-123"
        model = "gpt-4"

        [[ai_providers]]
        name = "Anthropic"
        enabled = false
        model = "claude-3"
    "#;

    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_ok(), "Should parse providers config");

    let cfg = config.unwrap();
    assert_eq!(cfg.ai_providers.len(), 2);
    assert_eq!(cfg.ai_providers[0].name, "OpenAI");
    assert_eq!(cfg.ai_providers[0].enabled, true);
    assert_eq!(cfg.ai_providers[1].name, "Anthropic");
    assert_eq!(cfg.ai_providers[1].enabled, false);
}

#[test]
fn test_config_with_plugins() {
    let toml_str = r#"
        [[plugins]]
        name = "Uppercase Plugin"
        enabled = true

        [[plugins]]
        name = "Code Formatter Plugin"
        enabled = false
    "#;

    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_ok(), "Should parse plugins config");

    let cfg = config.unwrap();
    assert_eq!(cfg.plugins.len(), 2);
    assert_eq!(cfg.plugins[0].name, "Uppercase Plugin");
    assert_eq!(cfg.plugins[0].enabled, true);
}

#[test]
fn test_complete_config() {
    let toml_str = r#"
        [[ai_providers]]
        name = "OpenAI"
        enabled = true
        api_key = "sk-test-key"
        api_endpoint = "https://api.openai.com/v1"
        model = "gpt-4"

        [[plugins]]
        name = "Uppercase Plugin"
        enabled = true

        [ui]
        theme = "dark"
        font_size = 18

        [general]
        auto_save = true
        max_history = 200
    "#;

    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_ok(), "Should parse complete config");

    let cfg = config.unwrap();
    assert_eq!(cfg.ai_providers.len(), 1);
    assert_eq!(cfg.plugins.len(), 1);
    assert_eq!(cfg.ui.theme, "dark");
    assert_eq!(cfg.general.max_history, 200);
}

#[test]
fn test_config_file_write_and_read() {
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_vibe_coder_config.toml");

    // Create config
    let mut config = AppConfig::default();
    config.ai_providers.push(ProviderSettings {
        name: "TestProvider".to_string(),
        enabled: true,
        api_key: Some("test-key".to_string()),
        api_endpoint: None,
        model: "test-model".to_string(),
    });

    // Write to file
    let toml_str = toml::to_string(&config).unwrap();
    let write_result = fs::write(&config_path, toml_str);
    assert!(write_result.is_ok(), "Should write config file");

    // Read from file
    let read_result = fs::read_to_string(&config_path);
    assert!(read_result.is_ok(), "Should read config file");

    let read_config: Result<AppConfig, _> = toml::from_str(&read_result.unwrap());
    assert!(read_config.is_ok(), "Should parse read config");

    let cfg = read_config.unwrap();
    assert_eq!(cfg.ai_providers.len(), 1);
    assert_eq!(cfg.ai_providers[0].name, "TestProvider");

    // Cleanup
    let _ = fs::remove_file(&config_path);
}

#[test]
fn test_config_default_values() {
    let config = AppConfig::default();
    
    assert_eq!(config.ai_providers.len(), 0);
    assert_eq!(config.plugins.len(), 0);
    assert_eq!(config.ui.theme, "light");
    assert_eq!(config.ui.font_size, 14);
    assert_eq!(config.general.auto_save, true);
    assert_eq!(config.general.max_history, 100);
}

#[test]
fn test_config_partial_defaults() {
    let toml_str = r#"
        [[ai_providers]]
        name = "OpenAI"
        enabled = true
        model = "gpt-4"
    "#;

    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_ok());

    let cfg = config.unwrap();
    // UI and general should use defaults
    assert_eq!(cfg.ui.theme, "light");
    assert_eq!(cfg.general.auto_save, true);
}

#[test]
fn test_config_empty_file() {
    let toml_str = "";
    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_ok(), "Empty config should use defaults");

    let cfg = config.unwrap();
    assert_eq!(cfg.ai_providers.len(), 0);
    assert_eq!(cfg.plugins.len(), 0);
}

#[test]
fn test_config_invalid_toml() {
    let toml_str = "this is not valid toml [[[";
    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_err(), "Invalid TOML should fail parsing");
}

#[test]
fn test_provider_settings_serialization() {
    let provider = ProviderSettings {
        name: "OpenAI".to_string(),
        enabled: true,
        api_key: Some("sk-123".to_string()),
        api_endpoint: Some("https://api.openai.com".to_string()),
        model: "gpt-4".to_string(),
    };

    let toml_str = toml::to_string(&provider);
    assert!(toml_str.is_ok());
}

#[test]
fn test_config_with_environment_variable_placeholders() {
    // Note: This test demonstrates the TOML structure for environment variable placeholders.
    // Actual environment variable expansion would be implemented in the application layer.
    let toml_str = r#"
        [[ai_providers]]
        name = "OpenAI"
        enabled = true
        api_key = "${OPENAI_API_KEY}"
        model = "gpt-4"
    "#;

    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_ok());

    let cfg = config.unwrap();
    assert!(cfg.ai_providers[0].api_key.is_some());
    // The literal placeholder value is stored; expansion would happen at runtime
    assert_eq!(cfg.ai_providers[0].api_key.as_ref().unwrap(), "${OPENAI_API_KEY}");
}

/// E2E test for complete configuration workflow
#[test]
fn test_complete_config_workflow() {
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_workflow_config.toml");

    // Step 1: Create default config
    let mut config = AppConfig::default();

    // Step 2: Add providers
    config.ai_providers.push(ProviderSettings {
        name: "OpenAI".to_string(),
        enabled: true,
        api_key: Some("test-key".to_string()),
        api_endpoint: Some("https://api.openai.com/v1".to_string()),
        model: "gpt-4".to_string(),
    });

    config.ai_providers.push(ProviderSettings {
        name: "Anthropic".to_string(),
        enabled: false,
        api_key: None,
        api_endpoint: None,
        model: "claude-3".to_string(),
    });

    // Step 3: Add plugins
    config.plugins.push(PluginSettings {
        name: "Uppercase Plugin".to_string(),
        enabled: true,
    });

    // Step 4: Customize UI
    config.ui.theme = "dark".to_string();
    config.ui.font_size = 16;

    // Step 5: Write to file
    let toml_str = toml::to_string(&config).unwrap();
    fs::write(&config_path, toml_str).unwrap();

    // Step 6: Read back and verify
    let contents = fs::read_to_string(&config_path).unwrap();
    let loaded_config: AppConfig = toml::from_str(&contents).unwrap();

    assert_eq!(loaded_config.ai_providers.len(), 2);
    assert_eq!(loaded_config.plugins.len(), 1);
    assert_eq!(loaded_config.ui.theme, "dark");
    assert_eq!(loaded_config.ui.font_size, 16);

    // Step 7: Verify provider details
    let openai = &loaded_config.ai_providers[0];
    assert_eq!(openai.name, "OpenAI");
    assert_eq!(openai.enabled, true);
    assert_eq!(openai.model, "gpt-4");

    // Cleanup
    let _ = fs::remove_file(&config_path);
}

#[test]
fn test_config_ui_settings() {
    let toml_str = r#"
        [ui]
        theme = "solarized"
        font_size = 20
    "#;

    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_ok());

    let cfg = config.unwrap();
    assert_eq!(cfg.ui.theme, "solarized");
    assert_eq!(cfg.ui.font_size, 20);
}

#[test]
fn test_config_general_settings() {
    let toml_str = r#"
        [general]
        auto_save = false
        max_history = 500
    "#;

    let config: Result<AppConfig, _> = toml::from_str(toml_str);
    assert!(config.is_ok());

    let cfg = config.unwrap();
    assert_eq!(cfg.general.auto_save, false);
    assert_eq!(cfg.general.max_history, 500);
}
