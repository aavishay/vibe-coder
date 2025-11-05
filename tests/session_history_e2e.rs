/// End-to-end tests for Session History
/// 
/// This module tests session history functionality, covering the roadmap
/// item for session history and conversation persistence.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionEntry {
    id: String,
    timestamp: u64,
    user_prompt: String,
    ai_response: String,
    provider: String,
    tokens_used: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionHistory {
    entries: VecDeque<SessionEntry>,
    max_size: usize,
}

impl SessionHistory {
    fn new(max_size: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_size,
        }
    }

    fn add_entry(&mut self, entry: SessionEntry) {
        if self.entries.len() >= self.max_size {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    fn get_entries(&self) -> Vec<SessionEntry> {
        self.entries.iter().cloned().collect()
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn get_by_id(&self, id: &str) -> Option<&SessionEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    fn search(&self, query: &str) -> Vec<SessionEntry> {
        self.entries
            .iter()
            .filter(|e| {
                e.user_prompt.contains(query) || e.ai_response.contains(query)
            })
            .cloned()
            .collect()
    }
}

#[test]
fn test_session_history_creation() {
    let history = SessionHistory::new(100);
    assert_eq!(history.len(), 0);
    assert_eq!(history.max_size, 100);
}

#[test]
fn test_add_session_entry() {
    let mut history = SessionHistory::new(100);

    let entry = SessionEntry {
        id: "entry-1".to_string(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        user_prompt: "Write a hello world function".to_string(),
        ai_response: "fn hello() { println!(\"Hello\"); }".to_string(),
        provider: "Mock Provider".to_string(),
        tokens_used: Some(50),
    };

    history.add_entry(entry);
    assert_eq!(history.len(), 1);
}

#[test]
fn test_session_history_max_size() {
    let mut history = SessionHistory::new(3);

    for i in 0..5 {
        let entry = SessionEntry {
            id: format!("entry-{}", i),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_prompt: format!("Prompt {}", i),
            ai_response: format!("Response {}", i),
            provider: "Mock Provider".to_string(),
            tokens_used: Some(50),
        };
        history.add_entry(entry);
    }

    assert_eq!(history.len(), 3, "History should be capped at max_size");

    let entries = history.get_entries();
    assert_eq!(entries[0].user_prompt, "Prompt 2", "Oldest entries should be removed");
    assert_eq!(entries[2].user_prompt, "Prompt 4", "Newest entries should be kept");
}

#[test]
fn test_clear_history() {
    let mut history = SessionHistory::new(100);

    for i in 0..5 {
        let entry = SessionEntry {
            id: format!("entry-{}", i),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_prompt: format!("Prompt {}", i),
            ai_response: format!("Response {}", i),
            provider: "Mock Provider".to_string(),
            tokens_used: Some(50),
        };
        history.add_entry(entry);
    }

    assert_eq!(history.len(), 5);
    history.clear();
    assert_eq!(history.len(), 0);
}

#[test]
fn test_get_entry_by_id() {
    let mut history = SessionHistory::new(100);

    let entry = SessionEntry {
        id: "unique-id-123".to_string(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        user_prompt: "Test prompt".to_string(),
        ai_response: "Test response".to_string(),
        provider: "Mock Provider".to_string(),
        tokens_used: Some(50),
    };

    history.add_entry(entry);

    let found = history.get_by_id("unique-id-123");
    assert!(found.is_some());
    assert_eq!(found.unwrap().user_prompt, "Test prompt");

    let not_found = history.get_by_id("nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn test_search_history() {
    let mut history = SessionHistory::new(100);

    let entries = vec![
        ("1", "Write a Rust function", "fn test() {}"),
        ("2", "Create a Python script", "def test(): pass"),
        ("3", "Rust error handling", "Result<T, E>"),
    ];

    for (id, prompt, response) in entries {
        history.add_entry(SessionEntry {
            id: id.to_string(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_prompt: prompt.to_string(),
            ai_response: response.to_string(),
            provider: "Mock Provider".to_string(),
            tokens_used: Some(50),
        });
    }

    let rust_results = history.search("Rust");
    assert_eq!(rust_results.len(), 2, "Should find 2 Rust-related entries");

    let python_results = history.search("Python");
    assert_eq!(python_results.len(), 1);

    let no_results = history.search("Java");
    assert_eq!(no_results.len(), 0);
}

#[test]
fn test_session_entry_serialization() {
    let entry = SessionEntry {
        id: "test-1".to_string(),
        timestamp: 1234567890,
        user_prompt: "Test".to_string(),
        ai_response: "Response".to_string(),
        provider: "Mock".to_string(),
        tokens_used: Some(100),
    };

    let json = serde_json::to_string(&entry);
    assert!(json.is_ok());

    let deserialized: Result<SessionEntry, _> = serde_json::from_str(&json.unwrap());
    assert!(deserialized.is_ok());
}

#[test]
fn test_session_history_serialization() {
    let mut history = SessionHistory::new(100);

    history.add_entry(SessionEntry {
        id: "1".to_string(),
        timestamp: 1234567890,
        user_prompt: "Test".to_string(),
        ai_response: "Response".to_string(),
        provider: "Mock".to_string(),
        tokens_used: Some(50),
    });

    let json = serde_json::to_string(&history);
    assert!(json.is_ok());

    let deserialized: Result<SessionHistory, _> = serde_json::from_str(&json.unwrap());
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap().len(), 1);
}

#[test]
fn test_session_history_persistence() {
    use std::fs;
    use std::path::PathBuf;

    let temp_dir = std::env::temp_dir();
    let history_path = temp_dir.join("test_session_history.json");

    // Create and populate history
    let mut history = SessionHistory::new(100);
    for i in 0..3 {
        history.add_entry(SessionEntry {
            id: format!("entry-{}", i),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_prompt: format!("Prompt {}", i),
            ai_response: format!("Response {}", i),
            provider: "Mock Provider".to_string(),
            tokens_used: Some(50),
        });
    }

    // Save to file
    let json = serde_json::to_string(&history).unwrap();
    fs::write(&history_path, json).unwrap();

    // Load from file
    let contents = fs::read_to_string(&history_path).unwrap();
    let loaded_history: SessionHistory = serde_json::from_str(&contents).unwrap();

    assert_eq!(loaded_history.len(), 3);
    assert_eq!(loaded_history.get_entries()[0].user_prompt, "Prompt 0");

    // Cleanup
    let _ = fs::remove_file(&history_path);
}

/// E2E test for complete session history workflow
#[test]
fn test_complete_session_workflow() {
    use std::fs;

    let temp_dir = std::env::temp_dir();
    let history_path = temp_dir.join("test_complete_session.json");

    // Step 1: Create new session
    let mut history = SessionHistory::new(50);

    // Step 2: User interactions
    let interactions = vec![
        ("Write a sorting algorithm", "Here's bubble sort: ..."),
        ("Explain Rust ownership", "Ownership is ..."),
        ("Create a web server", "Use actix-web ..."),
    ];

    for (i, (prompt, response)) in interactions.iter().enumerate() {
        history.add_entry(SessionEntry {
            id: format!("session-{}", i),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() + i as u64,
            user_prompt: prompt.to_string(),
            ai_response: response.to_string(),
            provider: "Mock Provider".to_string(),
            tokens_used: Some(150),
        });
    }

    // Step 3: Search history
    let rust_entries = history.search("Rust");
    assert_eq!(rust_entries.len(), 1);

    // Step 4: Save session
    let json = serde_json::to_string(&history).unwrap();
    fs::write(&history_path, json).unwrap();

    // Step 5: Load session in new instance
    let contents = fs::read_to_string(&history_path).unwrap();
    let loaded_history: SessionHistory = serde_json::from_str(&contents).unwrap();

    // Step 6: Verify loaded data
    assert_eq!(loaded_history.len(), 3);
    let entries = loaded_history.get_entries();
    assert_eq!(entries[1].user_prompt, "Explain Rust ownership");

    // Step 7: Continue session with new entries
    let mut continued_history = loaded_history;
    continued_history.add_entry(SessionEntry {
        id: "session-3".to_string(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        user_prompt: "More questions".to_string(),
        ai_response: "More answers".to_string(),
        provider: "Mock Provider".to_string(),
        tokens_used: Some(75),
    });

    assert_eq!(continued_history.len(), 4);

    // Cleanup
    let _ = fs::remove_file(&history_path);
}

#[test]
fn test_session_history_ordering() {
    let mut history = SessionHistory::new(100);

    let mut timestamp = 1000u64;
    for i in 0..5 {
        history.add_entry(SessionEntry {
            id: format!("entry-{}", i),
            timestamp,
            user_prompt: format!("Prompt {}", i),
            ai_response: format!("Response {}", i),
            provider: "Mock Provider".to_string(),
            tokens_used: Some(50),
        });
        timestamp += 100;
    }

    let entries = history.get_entries();
    // Verify chronological order
    for i in 0..4 {
        assert!(entries[i].timestamp < entries[i + 1].timestamp);
    }
}

#[test]
fn test_session_history_token_tracking() {
    let mut history = SessionHistory::new(100);

    for i in 0..5 {
        history.add_entry(SessionEntry {
            id: format!("entry-{}", i),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_prompt: format!("Prompt {}", i),
            ai_response: format!("Response {}", i),
            provider: "Mock Provider".to_string(),
            tokens_used: Some(100 * (i + 1)),
        });
    }

    let entries = history.get_entries();
    let total_tokens: usize = entries.iter().filter_map(|e| e.tokens_used).sum();
    assert_eq!(total_tokens, 100 + 200 + 300 + 400 + 500);
}

#[test]
fn test_empty_session_history() {
    let history = SessionHistory::new(100);
    assert_eq!(history.len(), 0);
    assert_eq!(history.get_entries().len(), 0);
    assert_eq!(history.search("anything").len(), 0);
    assert!(history.get_by_id("any-id").is_none());
}
