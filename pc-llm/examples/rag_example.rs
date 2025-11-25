//! Example: RAG (Retrieval Augmented Generation)

use pc_llm::{PersonalControllerLlm, LlmConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🔍 Personal Controller LLM - RAG Example\n");

    // Create LLM with RAG enabled
    let config = LlmConfig {
        model: "local".to_string(),
        use_rag: true,
        rag_top_k: 5,
        ..Default::default()
    };

    let mut llm = PersonalControllerLlm::new(config)?;
    llm.initialize().await?;

    println!("✓ LLM with RAG initialized\n");

    // Complex queries that benefit from RAG
    let queries = vec![
        "Quais são as principais rotas de transporte da Ávila?",
        "Qual o valor médio dos fretes para São Paulo?",
        "Quantos funcionários trabalharam mais de 40 horas esta semana?",
    ];

    for query in queries {
        println!("👤 Query: {}", query);

        let response = llm.chat(query).await?;

        println!("🤖 Response: {}", response.response);

        if !response.sources.is_empty() {
            println!("📚 Sources retrieved:");
            for (i, source) in response.sources.iter().enumerate() {
                println!("   {}. {}", i + 1, source);
            }
        } else {
            println!("ℹ️  No sources retrieved (RAG not yet connected to AvilaDB)");
        }

        println!();
    }

    Ok(())
}
