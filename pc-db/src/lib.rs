//! Personal Controller - Database Layer
//!
//! Camada de acesso ao AvilaDB para o Personal Controller

use pc_core::{Entity, Result};
use async_trait::async_trait;
use uuid::Uuid;

// Mock AvilaDB for development until real aviladb is available
#[cfg(not(feature = "real-aviladb"))]
pub mod aviladb_mock;
#[cfg(not(feature = "real-aviladb"))]
pub use aviladb_mock::AvilaDB;

#[cfg(feature = "real-aviladb")]
pub use aviladb::AvilaDB;

pub mod repository;
pub mod client;

pub use repository::Repository;
pub use client::PersonalControllerDb;

/// Trait para repositórios genéricos
#[async_trait]
pub trait GenericRepository<T: Entity + Send + Sync>: Send + Sync {
    /// Busca por ID
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<T>>;

    /// Busca todos
    async fn find_all(&self) -> Result<Vec<T>>;

    /// Salva uma entidade
    async fn save(&self, entity: &T) -> Result<()>;

    /// Atualiza uma entidade
    async fn update(&self, entity: &T) -> Result<()>;

    /// Deleta uma entidade
    async fn delete(&self, id: &Uuid) -> Result<()>;

    /// Busca paginada
    async fn find_paginated(&self, page: usize, per_page: usize) -> Result<Vec<T>>;

    /// Busca por campo
    async fn find_by_field(&self, field: &str, value: &str) -> Result<Vec<T>>;
}
