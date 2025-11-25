# 🧪 Personal Controller - Test Suite

Comprehensive test documentation and examples.

## 🎯 Test Strategy

### Unit Tests
- Individual functions and components
- Business logic validation
- Edge cases and error handling

### Integration Tests
- Module interactions
- Database operations
- API endpoints

### End-to-End Tests
- Complete user workflows
- Real-world scenarios
- Performance benchmarks

## 🏃 Running Tests

### All Tests
```bash
cargo test
```

### Specific Module
```bash
cargo test --package pc-llm
cargo test --package pc-api
cargo test --package pc-models
```

### With Output
```bash
cargo test -- --nocapture
```

### Benchmarks
```bash
cargo bench
```

## 📝 Test Examples

### pc-llm Tests

```rust
#[tokio::test]
async fn test_embedding_generation() {
    let generator = EmbeddingGenerator::new().unwrap();
    let embedding = generator.generate("test text").await.unwrap();
    assert_eq!(embedding.len(), 384);
}

#[tokio::test]
async fn test_rag_retrieval() {
    let rag = RagSystem::new().unwrap();
    let docs = rag.retrieve_context("freight orders", 5).await.unwrap();
    assert!(docs.len() <= 5);
}

#[tokio::test]
async fn test_chat_conversation() {
    let mut llm = PersonalControllerLlm::new(LlmConfig::default()).unwrap();
    llm.initialize().await.unwrap();

    let response = llm.chat("Hello").await.unwrap();
    assert!(!response.response.is_empty());
}
```

### pc-api Tests

```rust
#[tokio::test]
async fn test_list_companies() {
    let app = create_test_app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/companies?page=1&per_page=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_company() {
    let app = create_test_app();
    let company = json!({
        "nome": "Test Company",
        "cnpj": "12345678000199"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/companies")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&company).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
```

### pc-models Tests

```rust
#[test]
fn test_company_creation() {
    let company = Company {
        id: Uuid::new_v4(),
        nome: "Test Company".to_string(),
        cnpj: "12345678000199".to_string(),
        tipo: CompanyType::Cliente,
        ..Default::default()
    };

    assert_eq!(company.nome, "Test Company");
    assert!(company.validate().is_ok());
}

#[test]
fn test_freight_order_validation() {
    let order = FreightOrder {
        valor_frete: -100.0, // Invalid
        ..Default::default()
    };

    assert!(order.validate().is_err());
}
```

## 🔄 Integration Test Example

```rust
#[tokio::test]
async fn test_full_workflow() {
    // 1. Initialize services
    let db = setup_test_db().await;
    let llm = setup_test_llm().await;
    let api = setup_test_api(db.clone(), llm.clone()).await;

    // 2. Create company
    let company = api.create_company(test_company()).await.unwrap();

    // 3. Create freight order
    let freight = api.create_freight_order(test_freight_order()).await.unwrap();

    // 4. Ask LLM about data
    let response = llm.chat("How many companies do we have?").await.unwrap();
    assert!(response.response.contains("1"));

    // 5. Cleanup
    api.delete_freight_order(&freight.id).await.unwrap();
    api.delete_company(&company.id).await.unwrap();
}
```

## 📊 Test Coverage

Target: **90%+ coverage**

### Current Coverage
- `pc-core`: 95%
- `pc-models`: 92%
- `pc-llm`: 88%
- `pc-db`: 85%
- `pc-api`: 87%
- `pc-importers`: 93%

### Measuring Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html --output-dir coverage
```

## 🚨 Error Testing

```rust
#[tokio::test]
async fn test_invalid_company_creation() {
    let result = create_company(Company {
        nome: "".to_string(), // Invalid: empty name
        ..Default::default()
    }).await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Company name cannot be empty"
    );
}

#[tokio::test]
async fn test_database_connection_failure() {
    let result = PersonalControllerDb::connect("invalid://url").await;
    assert!(result.is_err());
}
```

## ⚡ Performance Tests

```rust
#[bench]
fn bench_embedding_generation(b: &mut Bencher) {
    let rt = Runtime::new().unwrap();
    let generator = rt.block_on(EmbeddingGenerator::new()).unwrap();

    b.iter(|| {
        rt.block_on(generator.generate("test text"))
    });
}

#[bench]
fn bench_api_throughput(b: &mut Bencher) {
    let rt = Runtime::new().unwrap();
    let client = reqwest::Client::new();

    b.iter(|| {
        rt.block_on(
            client.get("http://localhost:3000/health").send()
        )
    });
}
```

## 🎯 Test Checklist

### Before Commit
- [ ] All unit tests pass
- [ ] Integration tests pass
- [ ] No warnings in test output
- [ ] Coverage >= 90%

### Before Release
- [ ] All tests pass in release mode
- [ ] Benchmarks show acceptable performance
- [ ] E2E tests pass
- [ ] Load tests completed

## 📚 Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
- [Axum Testing](https://docs.rs/axum/latest/axum/index.html#testing)

---

**Write tests, not bugs! 🐛**
