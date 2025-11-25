// Mock implementation of AvilaDB for development
// TODO: Replace with real aviladb when ../../arxis/aviladb is available

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::{Result, Error};

/// Mock vector index
#[derive(Clone)]
pub struct VectorIndex {
    name: String,
    dimension: usize,
    data: HashMap<String, Vec<f32>>,
}

impl VectorIndex {
    pub fn new(name: impl Into<String>, dimension: usize) -> Self {
        Self {
            name: name.into(),
            dimension,
            data: HashMap::new(),
        }
    }

    pub async fn insert(&mut self, id: impl Into<String>, vector: Vec<f32>) -> Result<()> {
        if vector.len() != self.dimension {
            return Err(Error::msg(format!(
                "Vector dimension {} doesn't match index dimension {}",
                vector.len(),
                self.dimension
            )));
        }
        self.data.insert(id.into(), vector);
        Ok(())
    }

    pub async fn search(&self, query: &[f32], limit: usize) -> Result<Vec<(String, f32)>> {
        let mut results: Vec<(String, f32)> = self
            .data
            .iter()
            .map(|(id, vec)| {
                let similarity = cosine_similarity(query, vec);
                (id.clone(), similarity)
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results.truncate(limit);
        Ok(results)
    }
}

/// Mock document store
#[derive(Clone)]
pub struct DocumentStore {
    name: String,
    documents: HashMap<String, serde_json::Value>,
}

impl DocumentStore {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            documents: HashMap::new(),
        }
    }

    pub async fn insert(&mut self, id: impl Into<String>, doc: serde_json::Value) -> Result<()> {
        self.documents.insert(id.into(), doc);
        Ok(())
    }

    pub async fn get(&self, id: &str) -> Result<Option<serde_json::Value>> {
        Ok(self.documents.get(id).cloned())
    }

    pub async fn query(&self, _filter: serde_json::Value) -> Result<Vec<serde_json::Value>> {
        // Simple mock: return all documents
        Ok(self.documents.values().cloned().collect())
    }

    pub async fn delete(&mut self, id: &str) -> Result<bool> {
        Ok(self.documents.remove(id).is_some())
    }
}

/// Mock AvilaDB client
#[derive(Clone)]
pub struct AvilaDB {
    vector_indices: HashMap<String, VectorIndex>,
    document_stores: HashMap<String, DocumentStore>,
}

impl AvilaDB {
    pub fn new() -> Self {
        Self {
            vector_indices: HashMap::new(),
            document_stores: HashMap::new(),
        }
    }

    pub async fn connect(_config: &str) -> Result<Self> {
        Ok(Self::new())
    }

    pub async fn create_vector_index(&mut self, name: impl Into<String>, dimension: usize) -> Result<()> {
        let name = name.into();
        self.vector_indices.insert(name.clone(), VectorIndex::new(name, dimension));
        Ok(())
    }

    pub async fn get_vector_index(&mut self, name: &str) -> Result<&mut VectorIndex> {
        self.vector_indices
            .get_mut(name)
            .ok_or_else(|| Error::msg(format!("Vector index '{}' not found", name)))
    }

    pub async fn create_document_store(&mut self, name: impl Into<String>) -> Result<()> {
        let name = name.into();
        self.document_stores.insert(name.clone(), DocumentStore::new(name));
        Ok(())
    }

    pub async fn get_document_store(&mut self, name: &str) -> Result<&mut DocumentStore> {
        self.document_stores
            .get_mut(name)
            .ok_or_else(|| Error::msg(format!("Document store '{}' not found", name)))
    }
}

impl Default for AvilaDB {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function: cosine similarity
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vector_index() {
        let mut index = VectorIndex::new("test", 3);
        index.insert("vec1", vec![1.0, 0.0, 0.0]).await.unwrap();
        index.insert("vec2", vec![0.0, 1.0, 0.0]).await.unwrap();

        let results = index.search(&[1.0, 0.0, 0.0], 5).await.unwrap();
        assert_eq!(results[0].0, "vec1");
        assert!(results[0].1 > 0.99);
    }

    #[tokio::test]
    async fn test_document_store() {
        let mut store = DocumentStore::new("test");
        let doc = serde_json::json!({"name": "Test", "value": 42});
        store.insert("doc1", doc.clone()).await.unwrap();

        let retrieved = store.get("doc1").await.unwrap();
        assert_eq!(retrieved, Some(doc));
    }
}
