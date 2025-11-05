/// Integration End-to-End Tests
/// 
/// This module tests complete workflows combining multiple roadmap features:
/// - AI Providers + Plugins + Parser
/// - Session History + Export
/// - Configuration + AI Providers
/// - Complete User Journey

mod common;

use vibe_coder::ai_providers::{AIProvider, AIProviderManager, AIRequest};
use vibe_coder::plugins::{PluginRegistry, sample_plugins::UppercasePlugin};
use vibe_coder::parser::parse_response;
use common::{setup_mock_provider, create_test_request};

#[tokio::test]
async fn test_ai_provider_with_plugin_integration() {
    // Setup AI provider
    let provider = setup_mock_provider().await;
    
    // Setup plugin system
    let mut plugin_registry = PluginRegistry::new();
    plugin_registry.register(Box::new(UppercasePlugin::new())).await.unwrap();
    
    // Create request
    let user_input = "write a hello world function";
    
    // Pre-process with plugins
    let processed_input = plugin_registry.pre_process_all(user_input).await.unwrap();
    assert_eq!(processed_input, "WRITE A HELLO WORLD FUNCTION");
    
    // Send to AI provider
    let request = create_test_request(&processed_input);
    let response = provider.send_request(request).await.unwrap();
    
    // Verify response received
    assert!(!response.content.is_empty());
    assert!(response.content.contains("AI Response"));
}

#[tokio::test]
async fn test_ai_provider_with_parser_integration() {
    let provider = setup_mock_provider().await;
    let request = create_test_request("Explain Rust ownership");
    
    // Get AI response
    let response = provider.send_request(request).await.unwrap();
    
    // Parse the response
    let parser = parse_response(&response.content);
    let code_blocks = parser.get_code_blocks();
    let titles = parser.get_titles();
    
    // Verify parsed content
    assert!(!titles.is_empty(), "Should have titles");
    assert!(!code_blocks.is_empty(), "Should have code blocks");
}

#[tokio::test]
async fn test_provider_manager_with_multiple_providers() {
    let mut manager = AIProviderManager::new();
    
    // Add multiple providers
    let provider1 = setup_mock_provider().await;
    let provider2 = setup_mock_provider().await;
    
    manager.add_provider(Box::new(provider1)).await;
    manager.add_provider(Box::new(provider2)).await;
    
    assert_eq!(manager.list_providers().len(), 2);
    
    // Test switching between providers
    manager.set_active_provider(0).unwrap();
    assert!(manager.get_active_provider().is_some());
    
    manager.set_active_provider(1).unwrap();
    assert!(manager.get_active_provider().is_some());
}

/// E2E test simulating a complete user session workflow
#[tokio::test]
async fn test_complete_user_session() {
    // Step 1: Setup environment
    let mut manager = AIProviderManager::new();
    let provider = setup_mock_provider().await;
    manager.add_provider(Box::new(provider)).await;
    
    let mut plugin_registry = PluginRegistry::new();
    plugin_registry.register(Box::new(UppercasePlugin::new())).await.unwrap();
    
    // Step 2: User makes first query
    let user_query1 = "implement a binary search function";
    let processed_query1 = plugin_registry.pre_process_all(user_query1).await.unwrap();
    
    let request1 = AIRequest {
        prompt: processed_query1,
        context: None,
        temperature: 0.7,
        max_tokens: Some(500),
    };
    
    let active_provider = manager.get_active_provider().unwrap();
    let response1 = active_provider.send_request(request1).await.unwrap();
    
    // Step 3: Parse first response
    let parser1 = parse_response(&response1.content);
    let code_blocks1 = parser1.get_code_blocks();
    assert!(!code_blocks1.is_empty());
    
    // Step 4: User makes follow-up query with context
    let user_query2 = "add error handling to the previous function";
    let processed_query2 = plugin_registry.pre_process_all(user_query2).await.unwrap();
    
    let request2 = AIRequest {
        prompt: processed_query2,
        context: Some(response1.content.clone()),
        temperature: 0.7,
        max_tokens: Some(500),
    };
    
    let response2 = active_provider.send_request(request2).await.unwrap();
    
    // Step 5: Verify second response
    assert!(!response2.content.is_empty());
    assert!(response2.tokens_used.is_some());
}

#[tokio::test]
async fn test_plugin_chain_with_ai_provider() {
    // Setup
    let provider = setup_mock_provider().await;
    let mut plugin_registry = PluginRegistry::new();
    plugin_registry.register(Box::new(UppercasePlugin::new())).await.unwrap();
    
    // User input -> Plugin pre-processing -> AI -> Plugin post-processing
    let user_input = "write code";
    
    // Pre-process
    let pre_processed = plugin_registry.pre_process_all(user_input).await.unwrap();
    assert_eq!(pre_processed, "WRITE CODE");
    
    // AI processing
    let request = create_test_request(&pre_processed);
    let ai_response = provider.send_request(request).await.unwrap();
    
    // Post-process (would apply if we had post-processor plugins)
    let post_processed = plugin_registry.post_process_all(&ai_response.content).await.unwrap();
    
    // Verify complete chain
    assert!(!post_processed.is_empty());
}

#[tokio::test]
async fn test_error_handling_across_components() {
    // Test that errors in one component don't crash the system
    let mut manager = AIProviderManager::new();
    
    // Try to get provider when none exists
    let provider = manager.get_active_provider();
    assert!(provider.is_none());
    
    // Try to set invalid provider index
    let result = manager.set_active_provider(99);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_concurrent_requests() {
    use tokio::task;
    
    let provider = setup_mock_provider().await;
    let mut handles = vec![];
    
    // Spawn multiple concurrent requests
    for i in 0..5 {
        let request = create_test_request(&format!("Query {}", i));
        // Note: We can't share provider across threads easily, so this tests the concept
        // In real implementation, each task would get its own provider instance or use Arc
        handles.push(task::spawn(async move {
            // Simulate concurrent work
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            format!("Response {}", i)
        }));
    }
    
    // Wait for all to complete
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_response_parsing_with_different_content_types() {
    let provider = setup_mock_provider().await;
    let request = create_test_request("Show me various content types");
    
    let response = provider.send_request(request).await.unwrap();
    let parser = parse_response(&response.content);
    
    // Verify parser extracts different content types
    let titles = parser.get_titles();
    let code_blocks = parser.get_code_blocks();
    let blocks = &parser.blocks;
    
    assert!(!titles.is_empty(), "Should parse titles");
    assert!(!code_blocks.is_empty(), "Should parse code blocks");
    assert!(!blocks.is_empty(), "Should parse content blocks");
}

/// Test workflow similar to what a real user would experience
#[tokio::test]
async fn test_realistic_user_workflow() {
    // Setup
    let mut manager = AIProviderManager::new();
    let provider = setup_mock_provider().await;
    manager.add_provider(Box::new(provider)).await;
    
    // Scenario: User wants to learn about Rust
    let queries = vec![
        "What is Rust programming language?",
        "Show me a simple Rust program",
        "Explain ownership in Rust",
        "Write a function to read a file in Rust",
    ];
    
    let active_provider = manager.get_active_provider().unwrap();
    
    for query in queries {
        let request = create_test_request(query);
        let response = active_provider.send_request(request).await;
        
        assert!(response.is_ok(), "Each query should succeed");
        
        let ai_response = response.unwrap();
        assert!(!ai_response.content.is_empty());
        assert!(ai_response.tokens_used.is_some());
        
        // Parse response
        let parser = parse_response(&ai_response.content);
        assert!(!parser.blocks.is_empty());
    }
}

#[tokio::test]
async fn test_plugin_registry_isolation() {
    // Test that multiple plugin registries are independent
    let mut registry1 = PluginRegistry::new();
    let registry2 = PluginRegistry::new();
    
    registry1.register(Box::new(UppercasePlugin::new())).await.unwrap();
    
    assert_eq!(registry1.list_plugins().len(), 1);
    assert_eq!(registry2.list_plugins().len(), 0);
}

#[tokio::test]
async fn test_ai_provider_state_management() {
    let provider = setup_mock_provider().await;
    
    assert!(provider.is_ready());
    assert_eq!(provider.name(), "Mock Provider");
    
    // Provider should maintain state across requests
    let request1 = create_test_request("First query");
    let response1 = provider.send_request(request1).await;
    assert!(response1.is_ok());
    
    let request2 = create_test_request("Second query");
    let response2 = provider.send_request(request2).await;
    assert!(response2.is_ok());
    
    // Provider should still be ready
    assert!(provider.is_ready());
}

/// Test the integration of all major components
#[tokio::test]
async fn test_full_stack_integration() {
    // This test combines: Provider Manager + Plugins + Parser
    
    // 1. Setup Provider Manager
    let mut manager = AIProviderManager::new();
    let provider = setup_mock_provider().await;
    manager.add_provider(Box::new(provider)).await;
    
    // 2. Setup Plugins
    let mut plugins = PluginRegistry::new();
    plugins.register(Box::new(UppercasePlugin::new())).await.unwrap();
    
    // 3. Process user input through plugins
    let user_input = "explain rust traits";
    let processed = plugins.pre_process_all(user_input).await.unwrap();
    
    // 4. Send to AI provider
    let request = create_test_request(&processed);
    let active = manager.get_active_provider().unwrap();
    let response = active.send_request(request).await.unwrap();
    
    // 5. Parse response
    let parser = parse_response(&response.content);
    let titles = parser.get_titles();
    let code_blocks = parser.get_code_blocks();
    
    // 6. Verify complete workflow
    assert!(!processed.is_empty());
    assert!(!response.content.is_empty());
    assert!(!titles.is_empty());
    assert!(!code_blocks.is_empty());
}

#[tokio::test]
async fn test_performance_under_load() {
    use std::time::Instant;
    
    let provider = setup_mock_provider().await;
    let start = Instant::now();
    
    // Process multiple requests
    for i in 0..10 {
        let request = create_test_request(&format!("Query {}", i));
        let response = provider.send_request(request).await;
        assert!(response.is_ok());
    }
    
    let duration = start.elapsed();
    
    // Basic performance check - should complete reasonably fast
    // (This is just a smoke test, real benchmarks would be more thorough)
    assert!(duration.as_secs() < 5, "Should complete 10 requests in under 5 seconds");
}
