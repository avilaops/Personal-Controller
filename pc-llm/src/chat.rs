//! Chat interface with conversation history

use pc_core::Result;
use serde::{Deserialize, Serialize};

/// Chat interface for user interactions
pub struct ChatInterface {
    history: Vec<ChatMessage>,
    max_history: usize,
}

/// Single chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<MessageMetadata>,
}

/// Message role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// User message
    User,
    /// AI assistant response
    Assistant,
    /// System message (instructions)
    System,
}

/// Additional message metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub tokens_used: Option<usize>,
    pub model: Option<String>,
    pub confidence: Option<f32>,
    pub sources: Vec<String>,
}

impl ChatInterface {
    /// Creates a new chat interface
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            max_history: 100,
        }
    }

    /// Creates with custom max history
    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            history: Vec::new(),
            max_history,
        }
    }

    /// Adds a message to history
    pub fn add_message(&mut self, role: Role, content: String) {
        self.add_message_with_metadata(role, content, None);
    }

    /// Adds a message with metadata
    pub fn add_message_with_metadata(
        &mut self,
        role: Role,
        content: String,
        metadata: Option<MessageMetadata>,
    ) {
        let message = ChatMessage {
            role,
            content,
            timestamp: chrono::Utc::now(),
            metadata,
        };

        self.history.push(message);

        // Trim history if too long
        if self.history.len() > self.max_history {
            self.history.drain(0..(self.history.len() - self.max_history));
        }
    }

    /// Gets recent conversation context
    pub fn get_context(&self, max_messages: usize) -> Vec<&ChatMessage> {
        self.history
            .iter()
            .rev()
            .take(max_messages)
            .rev()
            .collect()
    }

    /// Gets all conversation history
    pub fn get_history(&self) -> &[ChatMessage] {
        &self.history
    }

    /// Clears conversation history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Gets the number of messages in history
    pub fn message_count(&self) -> usize {
        self.history.len()
    }

    /// Formats conversation history as prompt string
    pub fn format_as_prompt(&self, max_messages: Option<usize>) -> String {
        let messages = if let Some(max) = max_messages {
            self.get_context(max)
        } else {
            self.history.iter().collect()
        };

        let mut prompt = String::new();
        for msg in messages {
            let role_str = match msg.role {
                Role::User => "Usuário",
                Role::Assistant => "Assistente",
                Role::System => "Sistema",
            };
            prompt.push_str(&format!("{}: {}\n\n", role_str, msg.content));
        }

        prompt
    }

    /// Exports conversation to JSON
    pub fn export_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.history)?)
    }

    /// Imports conversation from JSON
    pub fn import_json(&mut self, json: &str) -> Result<()> {
        let messages: Vec<ChatMessage> = serde_json::from_str(json)?;
        self.history = messages;
        Ok(())
    }

    /// Gets conversation statistics
    pub fn get_stats(&self) -> ConversationStats {
        let mut stats = ConversationStats::default();

        for msg in &self.history {
            match msg.role {
                Role::User => stats.user_messages += 1,
                Role::Assistant => stats.assistant_messages += 1,
                Role::System => stats.system_messages += 1,
            }
            stats.total_chars += msg.content.len();

            if let Some(metadata) = &msg.metadata {
                if let Some(tokens) = metadata.tokens_used {
                    stats.total_tokens += tokens;
                }
            }
        }

        stats
    }
}

impl Default for ChatInterface {
    fn default() -> Self {
        Self::new()
    }
}

/// Conversation statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversationStats {
    pub user_messages: usize,
    pub assistant_messages: usize,
    pub system_messages: usize,
    pub total_chars: usize,
    pub total_tokens: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_interface() {
        let mut chat = ChatInterface::new();
        chat.add_message(Role::User, "Hello".to_string());
        chat.add_message(Role::Assistant, "Hi there!".to_string());

        assert_eq!(chat.message_count(), 2);
    }

    #[test]
    fn test_chat_context() {
        let mut chat = ChatInterface::new();
        for i in 0..10 {
            chat.add_message(Role::User, format!("Message {}", i));
        }

        let context = chat.get_context(5);
        assert_eq!(context.len(), 5);
        assert!(context[0].content.contains("Message 5"));
    }

    #[test]
    fn test_max_history() {
        let mut chat = ChatInterface::with_max_history(5);
        for i in 0..10 {
            chat.add_message(Role::User, format!("Message {}", i));
        }

        assert_eq!(chat.message_count(), 5);
        assert!(chat.get_history()[0].content.contains("Message 5"));
    }

    #[test]
    fn test_export_import() {
        let mut chat = ChatInterface::new();
        chat.add_message(Role::User, "Test".to_string());

        let json = chat.export_json().unwrap();

        let mut chat2 = ChatInterface::new();
        chat2.import_json(&json).unwrap();

        assert_eq!(chat2.message_count(), 1);
        assert_eq!(chat2.get_history()[0].content, "Test");
    }

    #[test]
    fn test_stats() {
        let mut chat = ChatInterface::new();
        chat.add_message(Role::User, "Hello".to_string());
        chat.add_message(Role::Assistant, "Hi".to_string());

        let stats = chat.get_stats();
        assert_eq!(stats.user_messages, 1);
        assert_eq!(stats.assistant_messages, 1);
        assert_eq!(stats.total_chars, 7); // "Hello" + "Hi"
    }
}
