//! Example: Basic LLM chat usage

use pc_llm::{PersonalControllerLlm, LlmConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🤖 Personal Controller LLM - Basic Chat Example\n");

    // Create LLM with default config
    let config = LlmConfig::default();
    let mut llm = PersonalControllerLlm::new(config)?;

    // Initialize
    llm.initialize().await?;
    println!("✓ LLM initialized\n");

    // Chat examples
    let queries = vec![
        "Qual o status das ordens de frete?",
        "Quantas empresas estão cadastradas?",
        "Como estão as horas trabalhadas este mês?",
    ];

    for query in queries {
        println!("👤 User: {}", query);

        let response = llm.chat(query).await?;

        println!("🤖 Assistant: {}", response.response);
        println!("   Confidence: {:.2}%", response.confidence * 100.0);
        println!("   Tokens: {}", response.tokens_used);
        if !response.sources.is_empty() {
            println!("   Sources: {:?}", response.sources);
        }
        println!();
    }

    // Show conversation stats
    let stats = llm.get_stats().await;
    println!("📊 Conversation Statistics:");
    println!("   User messages: {}", stats.user_messages);
    println!("   Assistant messages: {}", stats.assistant_messages);
    println!("   Total tokens: {}", stats.total_tokens);

    Ok(())
}
