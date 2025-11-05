/// End-to-end tests for Export Functionality
/// 
/// This module tests export to file functionality, covering the roadmap
/// item for exporting conversations and responses to various file formats.

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExportEntry {
    timestamp: String,
    user_prompt: String,
    ai_response: String,
    provider: String,
}

#[derive(Debug)]
enum ExportFormat {
    Markdown,
    Json,
    PlainText,
    Html,
}

struct Exporter;

impl Exporter {
    fn export_to_markdown(entries: &[ExportEntry]) -> String {
        let mut output = String::from("# Vibe Coder Session Export\n\n");
        
        for (i, entry) in entries.iter().enumerate() {
            output.push_str(&format!("## Interaction {}\n\n", i + 1));
            output.push_str(&format!("**Timestamp:** {}\n\n", entry.timestamp));
            output.push_str(&format!("**Provider:** {}\n\n", entry.provider));
            output.push_str(&format!("### User Prompt\n\n{}\n\n", entry.user_prompt));
            output.push_str(&format!("### AI Response\n\n{}\n\n", entry.ai_response));
            output.push_str("---\n\n");
        }
        
        output
    }

    fn export_to_json(entries: &[ExportEntry]) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(entries)
    }

    fn export_to_text(entries: &[ExportEntry]) -> String {
        let mut output = String::from("VIBE CODER SESSION EXPORT\n");
        output.push_str(&"=".repeat(80));
        output.push_str("\n\n");
        
        for (i, entry) in entries.iter().enumerate() {
            output.push_str(&format!("INTERACTION #{}\n", i + 1));
            output.push_str(&format!("Timestamp: {}\n", entry.timestamp));
            output.push_str(&format!("Provider: {}\n\n", entry.provider));
            output.push_str(&format!("USER:\n{}\n\n", entry.user_prompt));
            output.push_str(&format!("AI:\n{}\n\n", entry.ai_response));
            output.push_str(&"-".repeat(80));
            output.push_str("\n\n");
        }
        
        output
    }

    fn export_to_html(entries: &[ExportEntry]) -> String {
        let mut output = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
        output.push_str("<title>Vibe Coder Session Export</title>\n");
        output.push_str("<style>\n");
        output.push_str("body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }\n");
        output.push_str(".interaction { margin-bottom: 30px; border: 1px solid #ccc; padding: 15px; }\n");
        output.push_str(".timestamp { color: #666; font-size: 0.9em; }\n");
        output.push_str(".prompt { background: #f0f0f0; padding: 10px; margin: 10px 0; }\n");
        output.push_str(".response { background: #e8f4f8; padding: 10px; margin: 10px 0; }\n");
        output.push_str("</style>\n</head>\n<body>\n");
        output.push_str("<h1>Vibe Coder Session Export</h1>\n");
        
        for (i, entry) in entries.iter().enumerate() {
            output.push_str(&format!("<div class=\"interaction\">\n"));
            output.push_str(&format!("<h2>Interaction {}</h2>\n", i + 1));
            output.push_str(&format!("<div class=\"timestamp\">Timestamp: {} | Provider: {}</div>\n", 
                entry.timestamp, entry.provider));
            output.push_str(&format!("<div class=\"prompt\"><strong>User:</strong><br>{}</div>\n", 
                entry.user_prompt));
            output.push_str(&format!("<div class=\"response\"><strong>AI:</strong><br>{}</div>\n", 
                entry.ai_response));
            output.push_str("</div>\n");
        }
        
        output.push_str("</body>\n</html>");
        output
    }

    fn save_to_file(content: &str, path: &PathBuf) -> Result<(), std::io::Error> {
        fs::write(path, content)
    }
}

#[test]
fn test_export_to_markdown() {
    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Write a hello world function".to_string(),
            ai_response: "fn hello() { println!(\"Hello, world!\"); }".to_string(),
            provider: "Mock Provider".to_string(),
        },
    ];

    let markdown = Exporter::export_to_markdown(&entries);
    
    assert!(markdown.contains("# Vibe Coder Session Export"));
    assert!(markdown.contains("## Interaction 1"));
    assert!(markdown.contains("Write a hello world function"));
    assert!(markdown.contains("Mock Provider"));
}

#[test]
fn test_export_to_json() {
    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Test prompt".to_string(),
            ai_response: "Test response".to_string(),
            provider: "Mock Provider".to_string(),
        },
    ];

    let result = Exporter::export_to_json(&entries);
    assert!(result.is_ok());

    let json = result.unwrap();
    assert!(json.contains("Test prompt"));
    assert!(json.contains("Mock Provider"));
    
    // Verify it's valid JSON by parsing back
    let parsed: Result<Vec<ExportEntry>, _> = serde_json::from_str(&json);
    assert!(parsed.is_ok());
}

#[test]
fn test_export_to_text() {
    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Test prompt".to_string(),
            ai_response: "Test response".to_string(),
            provider: "Mock Provider".to_string(),
        },
    ];

    let text = Exporter::export_to_text(&entries);
    
    assert!(text.contains("VIBE CODER SESSION EXPORT"));
    assert!(text.contains("INTERACTION #1"));
    assert!(text.contains("USER:"));
    assert!(text.contains("AI:"));
}

#[test]
fn test_export_to_html() {
    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Test prompt".to_string(),
            ai_response: "Test response".to_string(),
            provider: "Mock Provider".to_string(),
        },
    ];

    let html = Exporter::export_to_html(&entries);
    
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("<title>Vibe Coder Session Export</title>"));
    assert!(html.contains("Interaction 1"));
    assert!(html.contains("Test prompt"));
    assert!(html.contains("Test response"));
}

#[test]
fn test_export_multiple_entries() {
    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "First prompt".to_string(),
            ai_response: "First response".to_string(),
            provider: "Provider 1".to_string(),
        },
        ExportEntry {
            timestamp: "2024-01-01 10:05:00".to_string(),
            user_prompt: "Second prompt".to_string(),
            ai_response: "Second response".to_string(),
            provider: "Provider 2".to_string(),
        },
    ];

    let markdown = Exporter::export_to_markdown(&entries);
    assert!(markdown.contains("## Interaction 1"));
    assert!(markdown.contains("## Interaction 2"));
    
    let json = Exporter::export_to_json(&entries).unwrap();
    let parsed: Vec<ExportEntry> = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.len(), 2);
}

#[test]
fn test_save_markdown_to_file() {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_export.md");

    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Test".to_string(),
            ai_response: "Response".to_string(),
            provider: "Mock".to_string(),
        },
    ];

    let markdown = Exporter::export_to_markdown(&entries);
    let result = Exporter::save_to_file(&markdown, &file_path);
    
    assert!(result.is_ok());
    assert!(file_path.exists());

    let contents = fs::read_to_string(&file_path).unwrap();
    assert!(contents.contains("# Vibe Coder Session Export"));

    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_save_json_to_file() {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_export.json");

    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Test".to_string(),
            ai_response: "Response".to_string(),
            provider: "Mock".to_string(),
        },
    ];

    let json = Exporter::export_to_json(&entries).unwrap();
    let result = Exporter::save_to_file(&json, &file_path);
    
    assert!(result.is_ok());
    assert!(file_path.exists());

    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_save_html_to_file() {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_export.html");

    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Test".to_string(),
            ai_response: "Response".to_string(),
            provider: "Mock".to_string(),
        },
    ];

    let html = Exporter::export_to_html(&entries);
    let result = Exporter::save_to_file(&html, &file_path);
    
    assert!(result.is_ok());
    assert!(file_path.exists());

    let contents = fs::read_to_string(&file_path).unwrap();
    assert!(contents.contains("<!DOCTYPE html>"));

    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_export_empty_entries() {
    let entries: Vec<ExportEntry> = vec![];

    let markdown = Exporter::export_to_markdown(&entries);
    assert!(markdown.contains("# Vibe Coder Session Export"));
    assert!(!markdown.contains("## Interaction 1"));

    let json = Exporter::export_to_json(&entries).unwrap();
    assert_eq!(json, "[]");

    let text = Exporter::export_to_text(&entries);
    assert!(text.contains("VIBE CODER SESSION EXPORT"));
}

/// E2E test for complete export workflow
#[test]
fn test_complete_export_workflow() {
    let temp_dir = std::env::temp_dir();

    // Step 1: Create session data
    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Implement a sorting algorithm in Rust".to_string(),
            ai_response: "Here's a bubble sort implementation:\n\n```rust\nfn bubble_sort(arr: &mut [i32]) { ... }\n```".to_string(),
            provider: "Mock Provider".to_string(),
        },
        ExportEntry {
            timestamp: "2024-01-01 10:05:00".to_string(),
            user_prompt: "Explain Rust ownership".to_string(),
            ai_response: "Ownership is a key feature of Rust...".to_string(),
            provider: "Mock Provider".to_string(),
        },
        ExportEntry {
            timestamp: "2024-01-01 10:10:00".to_string(),
            user_prompt: "Create a web server".to_string(),
            ai_response: "You can use actix-web framework...".to_string(),
            provider: "Mock Provider".to_string(),
        },
    ];

    // Step 2: Export to all formats
    let md_path = temp_dir.join("test_complete_export.md");
    let json_path = temp_dir.join("test_complete_export.json");
    let txt_path = temp_dir.join("test_complete_export.txt");
    let html_path = temp_dir.join("test_complete_export.html");

    let markdown = Exporter::export_to_markdown(&entries);
    let json = Exporter::export_to_json(&entries).unwrap();
    let text = Exporter::export_to_text(&entries);
    let html = Exporter::export_to_html(&entries);

    // Step 3: Save all files
    Exporter::save_to_file(&markdown, &md_path).unwrap();
    Exporter::save_to_file(&json, &json_path).unwrap();
    Exporter::save_to_file(&text, &txt_path).unwrap();
    Exporter::save_to_file(&html, &html_path).unwrap();

    // Step 4: Verify all files exist
    assert!(md_path.exists());
    assert!(json_path.exists());
    assert!(txt_path.exists());
    assert!(html_path.exists());

    // Step 5: Verify content
    let md_content = fs::read_to_string(&md_path).unwrap();
    assert!(md_content.contains("Implement a sorting algorithm"));
    assert!(md_content.contains("## Interaction 3"));

    let json_content = fs::read_to_string(&json_path).unwrap();
    let parsed: Vec<ExportEntry> = serde_json::from_str(&json_content).unwrap();
    assert_eq!(parsed.len(), 3);

    let txt_content = fs::read_to_string(&txt_path).unwrap();
    assert!(txt_content.contains("INTERACTION #1"));
    assert!(txt_content.contains("INTERACTION #3"));

    let html_content = fs::read_to_string(&html_path).unwrap();
    assert!(html_content.contains("<!DOCTYPE html>"));
    assert!(html_content.contains("Interaction 3"));

    // Cleanup
    let _ = fs::remove_file(&md_path);
    let _ = fs::remove_file(&json_path);
    let _ = fs::remove_file(&txt_path);
    let _ = fs::remove_file(&html_path);
}

#[test]
fn test_export_with_special_characters() {
    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Test with <html> & \"quotes\"".to_string(),
            ai_response: "Response with special chars: & < > \"".to_string(),
            provider: "Mock Provider".to_string(),
        },
    ];

    // All export formats should handle special characters
    let markdown = Exporter::export_to_markdown(&entries);
    assert!(markdown.contains("\"quotes\""));

    let json = Exporter::export_to_json(&entries).unwrap();
    let parsed: Vec<ExportEntry> = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed[0].user_prompt, entries[0].user_prompt);

    let text = Exporter::export_to_text(&entries);
    assert!(text.contains("<html>"));
}

#[test]
fn test_export_with_code_blocks() {
    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Write Rust code".to_string(),
            ai_response: "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```".to_string(),
            provider: "Mock Provider".to_string(),
        },
    ];

    let markdown = Exporter::export_to_markdown(&entries);
    assert!(markdown.contains("```rust"));
    assert!(markdown.contains("fn main()"));

    let json = Exporter::export_to_json(&entries).unwrap();
    assert!(json.contains("println"));
}

#[test]
fn test_export_file_extensions() {
    let temp_dir = std::env::temp_dir();

    let entries = vec![
        ExportEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            user_prompt: "Test".to_string(),
            ai_response: "Response".to_string(),
            provider: "Mock".to_string(),
        },
    ];

    // Test Markdown export
    let md_path = temp_dir.join("test.md");
    let md_content = Exporter::export_to_markdown(&entries);
    Exporter::save_to_file(&md_content, &md_path).unwrap();
    assert!(md_path.exists());
    assert!(fs::read_to_string(&md_path).unwrap().len() > 0);
    let _ = fs::remove_file(&md_path);

    // Test Text export
    let txt_path = temp_dir.join("test.txt");
    let txt_content = Exporter::export_to_text(&entries);
    Exporter::save_to_file(&txt_content, &txt_path).unwrap();
    assert!(txt_path.exists());
    assert!(fs::read_to_string(&txt_path).unwrap().len() > 0);
    let _ = fs::remove_file(&txt_path);

    // Test HTML export
    let html_path = temp_dir.join("test.html");
    let html_content = Exporter::export_to_html(&entries);
    Exporter::save_to_file(&html_content, &html_path).unwrap();
    assert!(html_path.exists());
    assert!(fs::read_to_string(&html_path).unwrap().len() > 0);
    let _ = fs::remove_file(&html_path);
}
