# 🔗 AvilaDB Integration Guide

Complete guide for integrating Personal Controller with AvilaDB.

## 🎯 Overview

AvilaDB is a NoSQL database optimized for:
- **Vector Search** - For LLM embeddings and RAG
- **Columnar Storage** - Fast analytics
- **Real-time Ingestion** - Streaming data
- **Brazilian Infrastructure** - Optimized for LATAM

## 🚀 Quick Start

### 1. Install AvilaDB

```bash
# Docker
docker pull avilaops/aviladb:latest
docker run -d -p 8000:8000 avilaops/aviladb

# Or build from source
cd arxis/aviladb
cargo build --release
./target/release/aviladb --port 8000
```

### 2. Configure Personal Controller

```toml
# pc-db/Cargo.toml
[dependencies]
aviladb = { path = "../../arxis/aviladb" }
```

### 3. Update pc-db Connection

```rust
// pc-db/src/lib.rs
use aviladb::{AvilaDB, Config};

pub struct PersonalControllerDb {
    db: AvilaDB,
}

impl PersonalControllerDb {
    pub async fn connect(url: &str) -> Result<Self> {
        let config = Config {
            host: "localhost".to_string(),
            port: 8000,
            auth: None,
        };

        let db = AvilaDB::connect(config).await?;

        Ok(Self { db })
    }

    pub async fn insert_company(&self, company: &Company) -> Result<()> {
        self.db
            .collection("companies")
            .insert(company)
            .await?;
        Ok(())
    }

    pub async fn query_companies(&self, query: Query) -> Result<Vec<Company>> {
        self.db
            .collection("companies")
            .find(query)
            .await
    }
}
```

## 🔍 Vector Search for RAG

### Creating Embeddings Collection

```rust
use aviladb::{Collection, IndexType};

pub async fn setup_embeddings_collection(db: &AvilaDB) -> Result<()> {
    let collection = db.create_collection("embeddings").await?;

    // Create vector index for similarity search
    collection
        .create_index(
            "embedding_vector",
            IndexType::Vector {
                dimensions: 384,
                metric: VectorMetric::Cosine,
            },
        )
        .await?;

    Ok(())
}
```

### Storing Embeddings

```rust
pub async fn store_embedding(
    db: &AvilaDB,
    document_id: Uuid,
    text: &str,
    embedding: Vec<f32>,
) -> Result<()> {
    let record = EmbeddingRecord {
        id: Uuid::new_v4(),
        document_id,
        text: text.to_string(),
        embedding,
        created_at: Utc::now(),
    };

    db.collection("embeddings")
        .insert(&record)
        .await?;

    Ok(())
}
```

### Vector Search

```rust
pub async fn search_similar(
    db: &AvilaDB,
    query_embedding: &[f32],
    top_k: usize,
) -> Result<Vec<Document>> {
    let results = db
        .collection("embeddings")
        .vector_search(
            "embedding_vector",
            query_embedding,
            top_k,
        )
        .await?;

    // Extract documents
    let document_ids: Vec<Uuid> = results
        .iter()
        .map(|r| r.document_id)
        .collect();

    db.collection("documents")
        .find(Query::ids(document_ids))
        .await
}
```

## 📊 Using avila-arrow with AvilaDB

```rust
use avila_arrow::{RecordBatch, Schema, Field, DataType};

pub async fn store_batch(
    db: &AvilaDB,
    batch: RecordBatch,
) -> Result<()> {
    // AvilaDB natively understands avila-arrow format
    db.collection("freight_orders")
        .insert_batch(batch)
        .await?;

    Ok(())
}

pub async fn query_with_arrow(
    db: &AvilaDB,
    query: &str,
) -> Result<RecordBatch> {
    // Returns data in avila-arrow format
    let batch = db
        .collection("freight_orders")
        .query_arrow(query)
        .await?;

    Ok(batch)
}
```

## 🔄 Streaming Ingestion

```rust
use futures::StreamExt;

pub async fn stream_timesheets(
    db: &AvilaDB,
    timesheets: impl Stream<Item = Timesheet>,
) -> Result<()> {
    let mut stream = Box::pin(timesheets);
    let collection = db.collection("timesheets");

    while let Some(timesheet) = stream.next().await {
        collection.insert(&timesheet).await?;
    }

    Ok(())
}
```

## 🔐 Authentication

```rust
use aviladb::auth::{Auth, Credentials};

pub async fn connect_with_auth() -> Result<AvilaDB> {
    let creds = Credentials {
        username: "avila_admin".to_string(),
        password: env::var("AVILADB_PASSWORD")?,
    };

    let config = Config {
        host: "localhost".to_string(),
        port: 8000,
        auth: Some(Auth::Basic(creds)),
    };

    AvilaDB::connect(config).await
}
```

## 📈 Monitoring and Telemetry

```rust
use avx_telemetry::{Metrics, Span};

pub async fn query_with_telemetry(
    db: &AvilaDB,
    query: Query,
) -> Result<Vec<Company>> {
    let _span = Span::new("aviladb.query");

    let start = Instant::now();
    let results = db.collection("companies").find(query).await?;
    let duration = start.elapsed();

    Metrics::record("aviladb.query_duration_ms", duration.as_millis());
    Metrics::increment("aviladb.queries_total");

    Ok(results)
}
```

## 🧪 Testing with AvilaDB

```rust
#[tokio::test]
async fn test_aviladb_integration() {
    // Setup test database
    let db = AvilaDB::connect_test().await.unwrap();

    // Insert test data
    let company = Company {
        nome: "Test Company".to_string(),
        ..Default::default()
    };
    db.collection("companies").insert(&company).await.unwrap();

    // Query
    let results = db
        .collection("companies")
        .find(Query::eq("nome", "Test Company"))
        .await
        .unwrap();

    assert_eq!(results.len(), 1);

    // Cleanup
    db.drop_collection("companies").await.unwrap();
}
```

## 🚀 Production Configuration

```toml
# config/production.toml
[database]
host = "aviladb.avila.cloud"
port = 8000
connection_pool_size = 50
max_connections = 100

[database.replication]
enabled = true
replicas = 3
regions = ["us-east-1", "br-saopaulo-1", "eu-west-1"]

[database.backup]
enabled = true
interval = "1h"
retention_days = 30
```

## 📚 Full Integration Example

```rust
// Complete integration in pc-db

use aviladb::{AvilaDB, Query, Sort};
use pc_models::*;

pub struct PersonalControllerDb {
    db: AvilaDB,
}

impl PersonalControllerDb {
    pub async fn new(config: DbConfig) -> Result<Self> {
        let db = AvilaDB::connect(config.into()).await?;
        Self::setup_collections(&db).await?;
        Ok(Self { db })
    }

    async fn setup_collections(db: &AvilaDB) -> Result<()> {
        // Create collections with indexes
        db.create_collection("companies").await?;
        db.create_collection("freight_orders").await?;
        db.create_collection("timesheets").await?;
        db.create_collection("embeddings").await?;

        // Create indexes
        db.collection("companies")
            .create_index("cnpj", IndexType::Unique)
            .await?;

        db.collection("embeddings")
            .create_index("embedding_vector", IndexType::Vector {
                dimensions: 384,
                metric: VectorMetric::Cosine,
            })
            .await?;

        Ok(())
    }

    // Companies
    pub async fn insert_company(&self, company: &Company) -> Result<Uuid> {
        let id = self.db
            .collection("companies")
            .insert(company)
            .await?;
        Ok(id)
    }

    pub async fn list_companies(&self, page: usize, per_page: usize) -> Result<Vec<Company>> {
        self.db
            .collection("companies")
            .find(Query::all())
            .limit(per_page)
            .skip(page * per_page)
            .sort(Sort::by("nome").asc())
            .await
    }

    // Vector search for RAG
    pub async fn semantic_search(
        &self,
        query_embedding: &[f32],
        top_k: usize,
    ) -> Result<Vec<Document>> {
        self.db
            .collection("embeddings")
            .vector_search("embedding_vector", query_embedding, top_k)
            .await
    }
}
```

---

**Powered by AvilaDB 🚀**
