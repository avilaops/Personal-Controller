//! Personal Controller LLM - A IA da Ávila
//!
//! Sistema de LLM com RAG para consultas inteligentes nos dados da Ávila

pub mod chat;
pub mod embeddings;
pub mod rag;

use pc_core::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use chat::{ChatInterface, ChatMessage, Role, MessageMetadata};
pub use embeddings::{EmbeddingGenerator, cosine_similarity};
pub use rag::{RagSystem, Document, DocumentSource};

/// LLM Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Model to use (e.g. "gpt-4", "llama2", "mistral", "local")
    pub model: String,

    /// API URL (for remote models)
    pub api_url: Option<String>,

    /// API key (for remote models)
    pub api_key: Option<String>,

    /// Temperature for generation (0.0 = deterministic, 1.0 = creative)
    pub temperature: f32,

    /// Max tokens in response
    pub max_tokens: usize,

    /// Use RAG (Retrieval Augmented Generation)
    pub use_rag: bool,

    /// Number of documents for RAG context
    pub rag_top_k: usize,

    /// Max conversation history to include
    pub max_history: usize,

    /// Embedding model dimension
    pub embedding_dim: usize,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            model: "local".to_string(),
            api_url: None,
            api_key: None,
            temperature: 0.7,
            max_tokens: 2048,
            use_rag: true,
            rag_top_k: 5,
            max_history: 10,
            embedding_dim: 384,
        }
    }
}

/// Personal Controller LLM client
pub struct PersonalControllerLlm {
    config: LlmConfig,
    chat_interface: Arc<RwLock<ChatInterface>>,
    rag_system: Arc<RagSystem>,
    embedding_generator: Arc<EmbeddingGenerator>,
}

impl PersonalControllerLlm {
    /// Creates a new LLM instance
    pub fn new(config: LlmConfig) -> Result<Self> {
        let embedding_generator = Arc::new(EmbeddingGenerator::new()?);
        let rag_system = Arc::new(RagSystem::with_embeddings(embedding_generator.clone()));

        Ok(Self {
            config,
            chat_interface: Arc::new(RwLock::new(ChatInterface::with_max_history(10))),
            rag_system,
            embedding_generator,
        })
    }

    /// Initializes the LLM (loads models, etc)
    pub async fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing Personal Controller LLM with model: {}", self.config.model);

        // TODO: Load tokenizer if needed
        // TODO: Load embedding model
        // TODO: Connect to API if needed

        // Add system message
        let mut chat = self.chat_interface.write().await;
        chat.add_message(
            Role::System,
            "Você é a Personal-Controller-LLM, uma IA especializada da Ávila Transportes. \
             Responda sempre em português, de forma clara e objetiva.".to_string()
        );

        Ok(())
    }

    /// Processes a user query
    pub async fn chat(&self, query: &str) -> Result<ChatResponse> {
        tracing::info!("Processing query: {}", query);

        // Add user message to history
        {
            let mut chat = self.chat_interface.write().await;
            chat.add_message(Role::User, query.to_string());
        }

        // Retrieve context with RAG if enabled
        let documents = if self.config.use_rag {
            self.rag_system.retrieve_context(query, self.config.rag_top_k).await?
        } else {
            Vec::new()
        };

        // Build prompt with context
        let prompt = if documents.is_empty() {
            self.rag_system.build_simple_prompt(query)
        } else {
            self.rag_system.build_prompt(query, &documents)
        };

        // Generate response
        // TODO: Implement real LLM inference (local or API)
        let response_text = self.generate_response(&prompt).await?;

        // Extract sources from documents
        let sources: Vec<String> = documents
            .iter()
            .map(|doc| format!("{:?}", doc.source))
            .collect();

        // Calculate tokens used (approximation)
        let tokens_used = (prompt.len() + response_text.len()) / 4;

        // Add assistant response to history
        {
            let mut chat = self.chat_interface.write().await;
            chat.add_message_with_metadata(
                Role::Assistant,
                response_text.clone(),
                Some(MessageMetadata {
                    tokens_used: Some(tokens_used),
                    model: Some(self.config.model.clone()),
                    confidence: Some(0.85),
                    sources: sources.clone(),
                }),
            );
        }

        Ok(ChatResponse {
            response: response_text,
            sources,
            confidence: 0.85,
            tokens_used,
        })
    }

    /// Generates response from prompt
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        // TODO: Implement real LLM generation
        // Options:
        // 1. Local model with llm-chain
        // 2. OpenAI API
        // 3. Anthropic Claude API
        // 4. Local Llama/Mistral via llama.cpp bindings

        tracing::debug!("Generating response for prompt (length: {})", prompt.len());

        // Placeholder response
        Ok(format!(
            "Esta é uma resposta gerada pela Personal-Controller-LLM. \
             Em produção, aqui seria usada a inferência real do modelo {}.",
            self.config.model
        ))
    }

    /// Generates embeddings for text
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        self.embedding_generator.generate(text).await
    }

    /// Generates embeddings for multiple texts
    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        self.embedding_generator.generate_batch(texts).await
    }

    /// Gets conversation history
    pub async fn get_history(&self) -> Vec<ChatMessage> {
        let chat = self.chat_interface.read().await;
        chat.get_history().to_vec()
    }

    /// Clears conversation history
    pub async fn clear_history(&self) -> Result<()> {
        let mut chat = self.chat_interface.write().await;
        chat.clear_history();
        Ok(())
    }

    /// Exports conversation to JSON
    pub async fn export_conversation(&self) -> Result<String> {
        let chat = self.chat_interface.read().await;
        chat.export_json()
    }

    /// Gets conversation statistics
    pub async fn get_stats(&self) -> chat::ConversationStats {
        let chat = self.chat_interface.read().await;
        chat.get_stats()
    }
}

/// Chat response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// Generated response text
    pub response: String,

    /// Source documents used (for RAG)
    pub sources: Vec<String>,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,

    /// Number of tokens used
    pub tokens_used: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_llm_creation() {
        let config = LlmConfig::default();
        let llm = PersonalControllerLlm::new(config);
        assert!(llm.is_ok());
    }

    #[tokio::test]
    async fn test_llm_initialization() {
        let config = LlmConfig::default();
        let mut llm = PersonalControllerLlm::new(config).unwrap();
        let result = llm.initialize().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_llm_chat() {
        let config = LlmConfig::default();
        let mut llm = PersonalControllerLlm::new(config).unwrap();
        llm.initialize().await.unwrap();

        let response = llm.chat("Hello, how are you?").await.unwrap();
        assert!(!response.response.is_empty());
        assert!(response.tokens_used > 0);
    }

    #[tokio::test]
    async fn test_llm_embeddings() {
        let config = LlmConfig::default();
        let llm = PersonalControllerLlm::new(config).unwrap();

        let embedding = llm.embed("test text").await.unwrap();
        assert_eq!(embedding.len(), 384);
    }

    #[tokio::test]
    async fn test_conversation_history() {
        let config = LlmConfig::default();
        let mut llm = PersonalControllerLlm::new(config).unwrap();
        llm.initialize().await.unwrap();

        llm.chat("First message").await.unwrap();
        llm.chat("Second message").await.unwrap();

        let history = llm.get_history().await;
        assert!(history.len() >= 4); // System + 2 user + 2 assistant
    }

    #[tokio::test]
    async fn test_clear_history() {
        let config = LlmConfig::default();
        let mut llm = PersonalControllerLlm::new(config).unwrap();
        llm.initialize().await.unwrap();

        llm.chat("Test").await.unwrap();
        llm.clear_history().await.unwrap();

        let history = llm.get_history().await;
        assert_eq!(history.len(), 0);
    }
}
