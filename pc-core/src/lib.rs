//! Personal Controller - Core types and traits
//!
//! Este módulo define os tipos fundamentais e traits usados em todo o sistema.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Result type padrão do Personal Controller
pub type Result<T> = std::result::Result<T, Error>;

/// Erros do Personal Controller
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Import error: {0}")]
    Import(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Trait para entidades que podem ser armazenadas no banco
pub trait Entity: Send + Sync {
    /// Retorna o ID único da entidade
    fn id(&self) -> &Uuid;

    /// Retorna o tipo da entidade
    fn entity_type(&self) -> &str;

    /// Valida a entidade
    fn validate(&self) -> Result<()>;
}

/// Trait para entidades que podem ser importadas de CSV
pub trait Importable: Sized {
    /// Importa de um registro CSV
    fn from_csv_record(record: &csv::StringRecord) -> Result<Self>;

    /// Retorna os headers esperados do CSV
    fn csv_headers() -> Vec<&'static str>;
}

/// Trait para entidades que podem ser convertidas para embeddings
pub trait Embeddable {
    /// Converte a entidade em texto para embedding
    fn to_embedding_text(&self) -> String;

    /// Retorna os campos mais importantes para busca semântica
    fn searchable_fields(&self) -> Vec<String>;
}

/// Status geral de processamento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pendente"),
            Status::Processing => write!(f, "Processando"),
            Status::Completed => write!(f, "Concluído"),
            Status::Failed => write!(f, "Falhou"),
            Status::Cancelled => write!(f, "Cancelado"),
        }
    }
}

/// Metadados de auditoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl Default for AuditMetadata {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}

/// Paginação de resultados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
}

impl Pagination {
    pub fn new(page: usize, per_page: usize, total: usize) -> Self {
        Self { page, per_page, total }
    }

    pub fn total_pages(&self) -> usize {
        (self.total + self.per_page - 1) / self.per_page
    }

    pub fn offset(&self) -> usize {
        (self.page - 1) * self.per_page
    }
}

/// Resposta paginada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new(2, 10, 45);
        assert_eq!(pagination.total_pages(), 5);
        assert_eq!(pagination.offset(), 10);
    }

    #[test]
    fn test_status_display() {
        assert_eq!(Status::Pending.to_string(), "Pendente");
        assert_eq!(Status::Completed.to_string(), "Concluído");
    }
}
