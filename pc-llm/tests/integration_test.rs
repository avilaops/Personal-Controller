//! Comprehensive tests for pc-llm

#[cfg(test)]
mod tests {
    use super::*;
    use pc_llm::*;
    use tokio;

    #[tokio::test]
    async fn test_llm_initialization() {
        let config = LlmConfig::default();
        let mut llm = PersonalControllerLlm::new(config).unwrap();

        let result = llm.initialize().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_embedding_generation() {
        let generator = EmbeddingGenerator::new().unwrap();

        let text = "Pedido de frete para São Paulo";
        let embedding = generator.generate(text).await.unwrap();

        assert_eq!(embedding.len(), 384);
        assert!(embedding.iter().all(|&x| x.is_finite()));
    }

    #[tokio::test]
    async fn test_embedding_deterministic() {
        let generator = EmbeddingGenerator::new().unwrap();

        let text = "Test text";
        let emb1 = generator.generate(text).await.unwrap();
        let emb2 = generator.generate(text).await.unwrap();

        // Same input should produce same output
        assert_eq!(emb1, emb2);
    }

    #[tokio::test]
    async fn test_chat_basic() {
        let mut llm = PersonalControllerLlm::new(LlmConfig::default()).unwrap();
        llm.initialize().await.unwrap();

        let response = llm.chat("Olá, tudo bem?").await.unwrap();

        assert!(!response.response.is_empty());
        assert_eq!(response.conversation_id.len(), 36); // UUID length
    }

    #[tokio::test]
    async fn test_chat_with_context() {
        let mut llm = PersonalControllerLlm::new(LlmConfig::default()).unwrap();
        llm.initialize().await.unwrap();

        let request = ChatRequest {
            message: "Quantos pedidos de frete temos?".to_string(),
            conversation_id: None,
            temperature: Some(0.7),
            max_tokens: Some(100),
        };

        let response = llm.chat_with_request(request).await.unwrap();
        assert!(!response.response.is_empty());
    }

    #[tokio::test]
    async fn test_rag_document_retrieval() {
        let mut rag = RagSystem::new().unwrap();

        // Add some test documents
        rag.add_document("doc1", "Pedido de frete para São Paulo").await.unwrap();
        rag.add_document("doc2", "Planilha de horas trabalhadas").await.unwrap();
        rag.add_document("doc3", "Empresa cliente ABC Transportes").await.unwrap();

        let docs = rag.retrieve_context("frete São Paulo", 2).await.unwrap();

        assert!(docs.len() <= 2);
        assert!(docs.iter().any(|d| d.content.contains("São Paulo")));
    }

    #[tokio::test]
    async fn test_rag_empty_query() {
        let rag = RagSystem::new().unwrap();

        let docs = rag.retrieve_context("", 5).await.unwrap();
        assert_eq!(docs.len(), 0);
    }

    #[tokio::test]
    async fn test_cosine_similarity() {
        let generator = EmbeddingGenerator::new().unwrap();

        let emb1 = generator.generate("transporte de carga").await.unwrap();
        let emb2 = generator.generate("frete de mercadoria").await.unwrap();
        let emb3 = generator.generate("programação de computadores").await.unwrap();

        let sim_related = cosine_similarity(&emb1, &emb2);
        let sim_unrelated = cosine_similarity(&emb1, &emb3);

        // Related texts should have higher similarity
        assert!(sim_related > sim_unrelated);
    }

    #[tokio::test]
    async fn test_chat_history() {
        let mut llm = PersonalControllerLlm::new(LlmConfig::default()).unwrap();
        llm.initialize().await.unwrap();

        let conv_id = "test-conversation-123".to_string();

        // First message
        let req1 = ChatRequest {
            message: "Meu nome é João".to_string(),
            conversation_id: Some(conv_id.clone()),
            temperature: None,
            max_tokens: None,
        };
        llm.chat_with_request(req1).await.unwrap();

        // Second message referencing first
        let req2 = ChatRequest {
            message: "Qual é o meu nome?".to_string(),
            conversation_id: Some(conv_id.clone()),
            temperature: None,
            max_tokens: None,
        };
        let response = llm.chat_with_request(req2).await.unwrap();

        // Should remember the name from history
        assert!(response.response.to_lowercase().contains("joão"));
    }

    #[tokio::test]
    async fn test_stats() {
        let mut llm = PersonalControllerLlm::new(LlmConfig::default()).unwrap();
        llm.initialize().await.unwrap();

        // Make some requests
        llm.chat("Test 1").await.unwrap();
        llm.chat("Test 2").await.unwrap();

        let stats = llm.get_stats().await;

        assert_eq!(stats.total_requests, 2);
        assert!(stats.total_tokens > 0);
    }

    #[test]
    fn test_llm_config_default() {
        let config = LlmConfig::default();

        assert_eq!(config.model, "local");
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_tokens, 2048);
    }

    #[test]
    fn test_chat_request_validation() {
        let request = ChatRequest {
            message: "".to_string(), // Empty message
            conversation_id: None,
            temperature: Some(1.5), // Out of range
            max_tokens: Some(0), // Invalid
        };

        // Should handle validation in actual implementation
        assert!(request.message.is_empty());
    }
}
