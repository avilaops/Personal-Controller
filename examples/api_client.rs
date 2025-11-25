//! Example: Using the Personal Controller API

use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Personal Controller API - Client Example\n");

    let base_url = "http://localhost:3000/api/v1";
    let client = reqwest::Client::new();

    // 1. Get API root
    println!("1️⃣  Getting API info...");
    let info: serde_json::Value = client
        .get("http://localhost:3000/")
        .send()
        .await?
        .json()
        .await?;
    println!("   ✓ API: {} v{}\n", info["name"], info["version"]);

    // 2. Create a company
    println!("2️⃣  Creating a company...");
    let company = json!({
        "nome": "Test Transport LTDA",
        "cnpj": "12345678000199",
        "tipo": "cliente",
        "cidade": "São Paulo",
        "estado": "SP"
    });

    let response: serde_json::Value = client
        .post(&format!("{}/companies", base_url))
        .json(&company)
        .send()
        .await?
        .json()
        .await?;
    println!("   ✓ Created: {}\n", response["message"]);

    // 3. List companies
    println!("3️⃣  Listing companies...");
    let companies: serde_json::Value = client
        .get(&format!("{}/companies?page=1&per_page=10", base_url))
        .send()
        .await?
        .json()
        .await?;
    println!("   ✓ Found {} companies\n", companies["pagination"]["total"]);

    // 4. Create a freight order
    println!("4️⃣  Creating a freight order...");
    let freight = json!({
        "numero": "288415",
        "remetente_cidade": "São Paulo",
        "destinatario_cidade": "Rio de Janeiro",
        "valor_frete": 1500.00,
        "status": "pending"
    });

    let response: serde_json::Value = client
        .post(&format!("{}/freight-orders", base_url))
        .json(&freight)
        .send()
        .await?
        .json()
        .await?;
    println!("   ✓ Created: {}\n", response["message"]);

    // 5. Chat with LLM
    println!("5️⃣  Asking LLM a question...");
    let chat_request = json!({
        "query": "Quantas ordens de frete estão pendentes?"
    });

    let chat_response: serde_json::Value = client
        .post(&format!("{}/chat", base_url))
        .json(&chat_request)
        .send()
        .await?
        .json()
        .await?;
    println!("   💬 Question: Quantas ordens de frete estão pendentes?");
    println!("   🤖 Response: {}", chat_response["response"]);
    println!("   📊 Confidence: {}%\n", chat_response["confidence"].as_f64().unwrap() * 100.0);

    // 6. Get statistics
    println!("6️⃣  Getting statistics...");
    let stats: serde_json::Value = client
        .get(&format!("{}/stats", base_url))
        .send()
        .await?
        .json()
        .await?;
    println!("   📊 Companies: {}", stats["companies"]);
    println!("   📊 Freight Orders: {}", stats["freight_orders"]);
    println!("   📊 Timesheets: {}", stats["timesheets"]);

    println!("\n✅ All API calls completed successfully!");

    Ok(())
}
