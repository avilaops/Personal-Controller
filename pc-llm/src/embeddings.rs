//! Embedding generation with avila-ml integration

use pc_core::Result;
use ndarray::{Array1, Array2};

/// Gerador de embeddings para textos usando avila-ml
pub struct EmbeddingGenerator {
    model_name: String,
    dimension: usize,
}

impl EmbeddingGenerator {
    /// Creates a new embedding generator
    pub fn new() -> Result<Self> {
        Ok(Self {
            model_name: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
            dimension: 384, // MiniLM produces 384-dim embeddings
        })
    }

    /// Creates with custom model
    pub fn with_model(model_name: String, dimension: usize) -> Result<Self> {
        Ok(Self {
            model_name,
            dimension,
        })
    }

    /// Generates embedding for a text
    pub async fn generate(&self, text: &str) -> Result<Vec<f32>> {
        tracing::debug!("Generating embedding for text: {}", text);

        // TODO: Integrate with avila-ml for real embeddings
        // For now, generate a simple hash-based embedding for development
        let embedding = self.generate_simple_embedding(text);

        Ok(embedding)
    }

    /// Generates embeddings in batch (more efficient)
    pub async fn generate_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        tracing::debug!("Generating {} embeddings in batch", texts.len());

        let mut embeddings = Vec::with_capacity(texts.len());
        for text in texts {
            embeddings.push(self.generate(text).await?);
        }
        Ok(embeddings)
    }

    /// Get embedding dimension
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Simple deterministic embedding for development
    /// In production, this should use avila-ml with real sentence transformers
    fn generate_simple_embedding(&self, text: &str) -> Vec<f32> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut embedding = vec![0.0; self.dimension];

        // Create deterministic pseudo-embedding based on text content
        let words: Vec<&str> = text.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            let mut hasher = DefaultHasher::new();
            word.hash(&mut hasher);
            let hash = hasher.finish();

            let idx = (hash as usize) % self.dimension;
            embedding[idx] += 1.0 / (i + 1) as f32;
        }

        // Normalize
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in &mut embedding {
                *x /= norm;
            }
        }

        embedding
    }

    /// Prepare text for embedding (tokenization, normalization)
    pub fn preprocess(&self, text: &str) -> String {
        // Basic preprocessing: lowercase, trim
        text.trim().to_lowercase()
    }
}

impl Default for EmbeddingGenerator {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// Calculates cosine similarity between two vectors
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same length");

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

/// Calculates euclidean distance between two vectors
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same length");

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

/// Calculates dot product
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same length");
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embedding_generation() {
        let generator = EmbeddingGenerator::new().unwrap();
        let embedding = generator.generate("test text").await.unwrap();
        assert_eq!(embedding.len(), 384);
    }

    #[tokio::test]
    async fn test_batch_generation() {
        let generator = EmbeddingGenerator::new().unwrap();
        let texts = vec!["text 1".to_string(), "text 2".to_string()];
        let embeddings = generator.generate_batch(&texts).await.unwrap();
        assert_eq!(embeddings.len(), 2);
        assert_eq!(embeddings[0].len(), 384);
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![0.0, 1.0, 0.0];
        assert!(cosine_similarity(&a, &c).abs() < 0.001);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((euclidean_distance(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![3.0, 4.0, 0.0];
        assert!((euclidean_distance(&a, &c) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_deterministic_embedding() {
        let generator = EmbeddingGenerator::new().unwrap();
        let emb1 = generator.generate_simple_embedding("hello world");
        let emb2 = generator.generate_simple_embedding("hello world");

        // Should be exactly the same (deterministic)
        assert_eq!(emb1, emb2);
    }
}
