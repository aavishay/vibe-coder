/// Parser for AI responses - extracts titles, code blocks, and formatted text
use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentBlock {
    Title { level: u8, text: String },
    Paragraph { text: String },
    CodeBlock { language: Option<String>, code: String },
    List { items: Vec<String> },
    Quote { text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedResponse {
    pub blocks: Vec<ContentBlock>,
}

impl ParsedResponse {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
    
    pub fn add_block(&mut self, block: ContentBlock) {
        self.blocks.push(block);
    }
    
    /// Get all code blocks from the response
    pub fn get_code_blocks(&self) -> Vec<(Option<String>, String)> {
        self.blocks
            .iter()
            .filter_map(|block| {
                if let ContentBlock::CodeBlock { language, code } = block {
                    Some((language.clone(), code.clone()))
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Get all titles from the response
    pub fn get_titles(&self) -> Vec<(u8, String)> {
        self.blocks
            .iter()
            .filter_map(|block| {
                if let ContentBlock::Title { level, text } = block {
                    Some((*level, text.clone()))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for ParsedResponse {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse markdown-formatted AI response into structured blocks
pub fn parse_response(markdown: &str) -> ParsedResponse {
    let mut parsed = ParsedResponse::new();
    let parser = Parser::new(markdown);
    
    let mut current_text = String::new();
    let mut current_code = String::new();
    let mut current_code_lang: Option<String> = None;
    let mut list_items: Vec<String> = Vec::new();
    let mut in_code_block = false;
    let mut in_list = false;
    let mut in_heading = false;
    let mut in_paragraph = false;
    let mut in_quote = false;
    let mut in_list_item = false;
    let mut current_heading_level = 0;
    
    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading(level, _, _) => {
                    in_heading = true;
                    current_heading_level = level as u8;
                    current_text.clear();
                }
                Tag::Paragraph => {
                    in_paragraph = true;
                    current_text.clear();
                }
                Tag::CodeBlock(kind) => {
                    in_code_block = true;
                    current_code.clear();
                    current_code_lang = match kind {
                        CodeBlockKind::Fenced(lang) => {
                            if lang.is_empty() {
                                None
                            } else {
                                Some(lang.to_string())
                            }
                        }
                        CodeBlockKind::Indented => None,
                    };
                }
                Tag::List(_) => {
                    in_list = true;
                    list_items.clear();
                }
                Tag::Item => {
                    in_list_item = true;
                    current_text.clear();
                }
                Tag::BlockQuote => {
                    in_quote = true;
                    current_text.clear();
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                Tag::Heading(_, _, _) => {
                    if in_heading && !current_text.is_empty() {
                        parsed.add_block(ContentBlock::Title {
                            level: current_heading_level,
                            text: current_text.trim().to_string(),
                        });
                        current_text.clear();
                    }
                    in_heading = false;
                }
                Tag::Paragraph => {
                    if in_paragraph && !in_list_item && !current_text.is_empty() {
                        parsed.add_block(ContentBlock::Paragraph {
                            text: current_text.trim().to_string(),
                        });
                        current_text.clear();
                    }
                    in_paragraph = false;
                }
                Tag::CodeBlock(_) => {
                    if in_code_block {
                        parsed.add_block(ContentBlock::CodeBlock {
                            language: current_code_lang.clone(),
                            code: current_code.trim_end().to_string(),
                        });
                        in_code_block = false;
                        current_code.clear();
                        current_code_lang = None;
                    }
                }
                Tag::List(_) => {
                    if in_list && !list_items.is_empty() {
                        parsed.add_block(ContentBlock::List {
                            items: list_items.clone(),
                        });
                        list_items.clear();
                    }
                    in_list = false;
                }
                Tag::Item => {
                    if in_list_item && !current_text.is_empty() {
                        list_items.push(current_text.trim().to_string());
                        current_text.clear();
                    }
                    in_list_item = false;
                }
                Tag::BlockQuote => {
                    if in_quote && !current_text.is_empty() {
                        parsed.add_block(ContentBlock::Quote {
                            text: current_text.trim().to_string(),
                        });
                        current_text.clear();
                    }
                    in_quote = false;
                }
                _ => {}
            },
            Event::Text(text) => {
                if in_code_block {
                    current_code.push_str(&text);
                } else {
                    current_text.push_str(&text);
                }
            }
            Event::Code(code) => {
                current_text.push('`');
                current_text.push_str(&code);
                current_text.push('`');
            }
            Event::SoftBreak | Event::HardBreak => {
                if in_code_block {
                    current_code.push('\n');
                } else if !current_text.is_empty() {
                    current_text.push(' ');
                }
            }
            _ => {}
        }
    }
    
    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_response() {
        let markdown = "# Title\n\nSome text\n\n```rust\nfn main() {}\n```";
        let parsed = parse_response(markdown);
        
        assert_eq!(parsed.blocks.len(), 3);
        assert!(matches!(parsed.blocks[0], ContentBlock::Title { .. }));
        assert!(matches!(parsed.blocks[1], ContentBlock::Paragraph { .. }));
        assert!(matches!(parsed.blocks[2], ContentBlock::CodeBlock { .. }));
    }

    #[test]
    fn test_get_code_blocks() {
        let markdown = "```rust\nfn test() {}\n```\n\n```python\ndef test(): pass\n```";
        let parsed = parse_response(markdown);
        let code_blocks = parsed.get_code_blocks();
        
        assert_eq!(code_blocks.len(), 2);
        assert_eq!(code_blocks[0].0, Some("rust".to_string()));
        assert_eq!(code_blocks[1].0, Some("python".to_string()));
    }

    #[test]
    fn test_get_titles() {
        let markdown = "# Main Title\n\n## Subtitle\n\n### Subsubtitle";
        let parsed = parse_response(markdown);
        let titles = parsed.get_titles();
        
        assert_eq!(titles.len(), 3);
        assert_eq!(titles[0].0, 1);
        assert_eq!(titles[1].0, 2);
        assert_eq!(titles[2].0, 3);
    }
}
