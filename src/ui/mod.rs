/// UI module for the Vibe Coder console
use iced::{
    widget::{button, column, container, scrollable, text, text_input, Column},
    Application, Command, Element, Length, Settings, Theme,
};

use crate::ai_providers::{AIProvider, AIProviderManager, AIRequest, MockAIProvider, ProviderConfig};
use crate::parser::{parse_response, ContentBlock, ParsedResponse};
use crate::plugins::PluginRegistry;

pub struct VibeCoder {
    input_line1: String,
    input_line2: String,
    response_blocks: Vec<ContentBlock>,
    ai_manager: AIProviderManager,
    plugin_registry: PluginRegistry,
    is_processing: bool,
    status_message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputLine1Changed(String),
    InputLine2Changed(String),
    SendRequest,
    ResponseReceived(Result<ParsedResponse, String>),
}

impl Application for VibeCoder {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = Self {
            input_line1: String::new(),
            input_line2: String::new(),
            response_blocks: Vec::new(),
            ai_manager: AIProviderManager::new(),
            plugin_registry: PluginRegistry::new(),
            is_processing: false,
            status_message: "Ready".to_string(),
        };

        // Initialize with a mock provider
        let init_command = Command::perform(
            async {
                let mut provider = MockAIProvider::new();
                let config = ProviderConfig {
                    name: "Mock Provider".to_string(),
                    api_key: None,
                    api_endpoint: None,
                    model: "mock-model-v1".to_string(),
                };
                provider.configure(config).await.ok();
                provider
            },
            |_provider| {
                // This is a setup command, no message needed
                Message::InputLine1Changed(String::new())
            },
        );

        (app, init_command)
    }

    fn title(&self) -> String {
        String::from("Vibe Coder - AI Coding Console")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputLine1Changed(value) => {
                self.input_line1 = value;
                Command::none()
            }
            Message::InputLine2Changed(value) => {
                self.input_line2 = value;
                Command::none()
            }
            Message::SendRequest => {
                if self.is_processing {
                    return Command::none();
                }

                let prompt = format!("{}\n{}", self.input_line1, self.input_line2);
                if prompt.trim().is_empty() {
                    self.status_message = "Please enter a prompt".to_string();
                    return Command::none();
                }

                self.is_processing = true;
                self.status_message = "Processing...".to_string();

                // Create a mock provider for the command
                Command::perform(
                    async move {
                        let mut provider = MockAIProvider::new();
                        let config = ProviderConfig {
                            name: "Mock Provider".to_string(),
                            api_key: None,
                            api_endpoint: None,
                            model: "mock-model-v1".to_string(),
                        };
                        provider.configure(config).await.ok();

                        let request = AIRequest {
                            prompt: prompt.clone(),
                            context: None,
                            temperature: 0.7,
                            max_tokens: Some(2000),
                        };

                        provider.send_request(request).await
                    },
                    |result| {
                        Message::ResponseReceived(result.map(|resp| parse_response(&resp.content))
                            .map_err(|e| e.to_string()))
                    },
                )
            }
            Message::ResponseReceived(result) => {
                self.is_processing = false;
                match result {
                    Ok(parsed) => {
                        self.response_blocks = parsed.blocks;
                        self.status_message = "Response received".to_string();
                    }
                    Err(e) => {
                        self.status_message = format!("Error: {}", e);
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input_section = column![
            text("Vibe Coder - AI Coding Console")
                .size(24)
                .width(Length::Fill),
            text(&self.status_message).size(14),
            text("Input Line 1:").size(16),
            text_input("Enter your first input line...", &self.input_line1)
                .on_input(Message::InputLine1Changed)
                .padding(10)
                .size(16),
            text("Input Line 2:").size(16),
            text_input("Enter your second input line...", &self.input_line2)
                .on_input(Message::InputLine2Changed)
                .on_submit(Message::SendRequest)
                .padding(10)
                .size(16),
            button(
                text(if self.is_processing {
                    "Processing..."
                } else {
                    "Send Request"
                })
                .size(16)
            )
            .on_press(Message::SendRequest)
            .padding(10),
        ]
        .spacing(10)
        .padding(20);

        let response_section = if self.response_blocks.is_empty() {
            column![text("Response will appear here...").size(14)]
                .spacing(10)
                .padding(20)
        } else {
            let mut response_column = Column::new().spacing(15).padding(20);

            for block in &self.response_blocks {
                match block {
                    ContentBlock::Title { level, text: title_text } => {
                        let size = match level {
                            1 => 28,
                            2 => 24,
                            3 => 20,
                            _ => 16,
                        };
                        response_column = response_column.push(
                            text(title_text).size(size)
                        );
                    }
                    ContentBlock::Paragraph { text: para_text } => {
                        response_column = response_column.push(
                            text(para_text).size(14)
                        );
                    }
                    ContentBlock::CodeBlock { language, code } => {
                        let lang_label = language
                            .as_ref()
                            .map(|l| format!("Language: {}", l))
                            .unwrap_or_else(|| "Code:".to_string());
                        
                        response_column = response_column.push(
                            column![
                                text(&lang_label).size(12),
                                container(
                                    text(code).size(13)
                                )
                                .padding(10)
                            ]
                            .spacing(5)
                        );
                    }
                    ContentBlock::List { items } => {
                        let mut list_column = Column::new().spacing(5);
                        for item in items {
                            list_column = list_column.push(
                                text(format!("• {}", item)).size(14)
                            );
                        }
                        response_column = response_column.push(list_column);
                    }
                    ContentBlock::Quote { text: quote_text } => {
                        response_column = response_column.push(
                            container(
                                text(format!("❝ {}", quote_text)).size(14)
                            )
                            .padding(10)
                        );
                    }
                }
            }

            response_column
        };

        let content = column![
            input_section,
            container(
                scrollable(response_section)
                    .height(Length::Fill)
            )
            .height(Length::Fill)
        ]
        .spacing(0)
        .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

pub fn run() -> iced::Result {
    VibeCoder::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(800.0, 600.0),
            ..Default::default()
        },
        ..Default::default()
    })
}
