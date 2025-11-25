//! RAG (Retrieval Augmented Generation) implementation

use pc_core::Result;
use pc_models::{FreightOrder, Company, Timesheet};
use crate::embeddings::{EmbeddingGenerator, cosine_similarity};
use std::sync::Arc;

/// RAG system for retrieving relevant context
pub struct RagSystem {
    embedding_generator: Arc<EmbeddingGenerator>,
    // TODO: pc-db connection for vector search
}

impl RagSystem {
    /// Creates a new RAG system
    pub fn new() -> Result<Self> {
        Ok(Self {
            embedding_generator: Arc::new(EmbeddingGenerator::new()?),
        })
    }

    /// Creates with custom embedding generator
    pub fn with_embeddings(generator: Arc<EmbeddingGenerator>) -> Self {
        Self {
            embedding_generator: generator,
        }
    }

    /// Retrieves relevant documents for a query
    pub async fn retrieve_context(&self, query: &str, top_k: usize) -> Result<Vec<Document>> {
        tracing::debug!("Retrieving context for: {} (top_k={})", query, top_k);

        // 1. Generate query embedding
        let query_embedding = self.embedding_generator.generate(query).await?;

        // 2. Search in AvilaDB with vector search
        // TODO: Implement real AvilaDB vector search
        let documents = self.search_documents(&query_embedding, top_k).await?;

        // 3. Return top-k documents
        Ok(documents)
    }

    /// Searches documents using vector similarity
    async fn search_documents(&self, query_embedding: &[f32], top_k: usize) -> Result<Vec<Document>> {
        // TODO: Replace with real AvilaDB query
        // For now, return empty
        Ok(Vec::new())
    }

    /// Builds prompt with context for LLM
    pub fn build_prompt(&self, query: &str, documents: &[Document]) -> String {
        let mut prompt = String::from(
            "Você é a Personal-Controller-LLM, uma IA especializada da Ávila Transportes.\n\n"
        );

        if !documents.is_empty() {
            prompt.push_str("# Contexto Relevante\n\n");
            for (i, doc) in documents.iter().enumerate() {
                prompt.push_str(&format!(
                    "Documento {} (relevância: {:.2}%):\n{}\n\n",
                    i + 1,
                    doc.score * 100.0,
                    doc.content
                ));
            }
        }

        prompt.push_str(&format!("# Pergunta do Usuário\n{}\n\n", query));
        prompt.push_str("# Instruções\n");
        prompt.push_str("- Responda em português claro e objetivo\n");
        prompt.push_str("- Use os documentos fornecidos como contexto\n");
        prompt.push_str("- Se não tiver certeza, diga que não sabe\n");
        prompt.push_str("- Cite as fontes quando relevante\n\n");
        prompt.push_str("Resposta:");

        prompt
    }

    /// Builds prompt without RAG context
    pub fn build_simple_prompt(&self, query: &str) -> String {
        format!(
            "Você é a Personal-Controller-LLM da Ávila Transportes.\n\nPergunta: {}\n\nResposta:",
            query
        )
    }

    /// Chunks a large text into smaller pieces for embedding
    pub fn chunk_text(&self, text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut chunks = Vec::new();

        let mut start = 0;
        while start < words.len() {
            let end = (start + chunk_size).min(words.len());
            let chunk = words[start..end].join(" ");
            chunks.push(chunk);

            if end >= words.len() {
                break;
            }

            start += chunk_size - overlap;
        }

        chunks
    }
}

impl Default for RagSystem {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// Document retrieved from RAG
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub score: f32,
    pub source: DocumentSource,
    pub metadata: Option<DocumentMetadata>,
}

#[derive(Debug, Clone)]
pub enum DocumentSource {
    FreightOrder(String),
    Company(String),
    Timesheet(String),
    Route(String),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub author: Option<String>,
    pub tags: Vec<String>,
}

impl Document {
    /// Creates document from freight order
    pub fn from_freight_order(order: &FreightOrder, score: f32) -> Self {
        Self {
            id: order.id.to_string(),
            content: format!(
                "Ordem de Frete #{}\n\
                 Número: {}\n\
                 Remetente: {} - {}\n\
                 Destinatário: {} - {}\n\
                 Valor do frete: R$ {:.2}\n\
                 Status: {:?}",
                order.id,
                order.numero,
                order.remetente_cidade,
                order.remetente_estado,
                order.destinatario_cidade,
                order.destinatario_estado,
                order.valor_frete,
                order.status
            ),
            score,
            source: DocumentSource::FreightOrder(order.numero.clone()),
            metadata: Some(DocumentMetadata {
                created_at: order.created_at,
                updated_at: order.updated_at,
                author: None,
                tags: vec!["freight".to_string(), "order".to_string()],
            }),
        }
    }

    /// Creates document from company
    pub fn from_company(company: &Company, score: f32) -> Self {
        Self {
            id: company.id.to_string(),
            content: format!(
                "Empresa: {}\n\
                 Tipo: {:?}\n\
                 CNPJ: {}\n\
                 Cidade: {} - {}\n\
                 Telefone: {}\n\
                 Email: {}",
                company.nome,
                company.tipo,
                company.cnpj,
                company.cidade,
                company.estado,
                company.telefone.as_deref().unwrap_or("N/A"),
                company.email.as_deref().unwrap_or("N/A")
            ),
            score,
            source: DocumentSource::Company(company.nome.clone()),
            metadata: Some(DocumentMetadata {
                created_at: company.created_at,
                updated_at: company.updated_at,
                author: None,
                tags: vec!["company".to_string(), format!("{:?}", company.tipo).to_lowercase()],
            }),
        }
    }

    /// Creates document from timesheet
    pub fn from_timesheet(timesheet: &Timesheet, score: f32) -> Self {
        Self {
            id: timesheet.id.to_string(),
            content: format!(
                "Registro de Ponto\n\
                 Funcionário ID: {}\n\
                 Data: {}\n\
                 Entrada: {:?}\n\
                 Saída: {:?}\n\
                 Horas trabalhadas: {:.2}",
                timesheet.funcionario_id,
                timesheet.data,
                timesheet.hora_entrada,
                timesheet.hora_saida,
                timesheet.horas_trabalhadas
            ),
            score,
            source: DocumentSource::Timesheet(format!("{}-{}", timesheet.funcionario_id, timesheet.data)),
            metadata: Some(DocumentMetadata {
                created_at: timesheet.created_at,
                updated_at: timesheet.updated_at,
                author: None,
                tags: vec!["timesheet".to_string(), "hours".to_string()],
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rag_creation() {
        let rag = RagSystem::new().unwrap();
        assert!(true);
    }

    #[tokio::test]
    async fn test_retrieve_context() {
        let rag = RagSystem::new().unwrap();
        let docs = rag.retrieve_context("test query", 5).await.unwrap();
        assert_eq!(docs.len(), 0); // Empty until AvilaDB integration
    }

    #[test]
    fn test_build_prompt() {
        let rag = RagSystem::new().unwrap();
        let prompt = rag.build_simple_prompt("What is the meaning of life?");
        assert!(prompt.contains("Personal-Controller-LLM"));
        assert!(prompt.contains("What is the meaning of life?"));
    }

    #[test]
    fn test_chunk_text() {
        let rag = RagSystem::new().unwrap();
        let text = "one two three four five six seven eight nine ten";
        let chunks = rag.chunk_text(text, 3, 1);

        assert!(!chunks.is_empty());
        assert!(chunks[0].contains("one"));
    }
}
