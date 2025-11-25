//! Example: Embeddings generation

use pc_llm::{PersonalControllerLlm, LlmConfig};
use pc_llm::embeddings::cosine_similarity;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🧬 Personal Controller LLM - Embeddings Example\n");

    let config = LlmConfig::default();
    let llm = PersonalControllerLlm::new(config)?;

    // Generate embeddings for different texts
    let texts = vec![
        "Ordem de frete para São Paulo",
        "Transporte de carga para SP",
        "Registro de ponto do funcionário",
        "Horário de trabalho",
    ];

    println!("Generating embeddings for {} texts...\n", texts.len());

    let mut embeddings = Vec::new();
    for text in &texts {
        let embedding = llm.embed(text).await?;
        embeddings.push(embedding);
        println!("✓ '{}' -> embedding[{}]", text, embeddings.last().unwrap().len());
    }

    println!("\n📊 Similarity Matrix:\n");
    println!("     | Text 1 | Text 2 | Text 3 | Text 4 |");
    println!("-----|--------|--------|--------|--------|");

    for (i, emb_i) in embeddings.iter().enumerate() {
        print!("Text {} |", i + 1);
        for emb_j in &embeddings {
            let sim = cosine_similarity(emb_i, emb_j);
            print!(" {:.3}  |", sim);
        }
        println!();
    }

    println!("\n✓ Observations:");
    println!("  - Text 1 and 2 should have high similarity (both about freight)");
    println!("  - Text 3 and 4 should have high similarity (both about work hours)");
    println!("  - Cross-topic similarity should be lower");

    // Batch generation
    println!("\n🚀 Batch Generation:");
    let batch_texts: Vec<String> = (0..10)
        .map(|i| format!("Test document number {}", i))
        .collect();

    let batch_embeddings = llm.embed_batch(&batch_texts).await?;
    println!("✓ Generated {} embeddings in batch", batch_embeddings.len());

    Ok(())
}
