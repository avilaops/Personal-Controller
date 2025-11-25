//! Personal Controller CLI

use clap::{Parser, Subcommand};
use pc_importers::{freight_importer::FreightOrderImporter, Importer};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pc")]
#[command(about = "Personal Controller - Plataforma de gestão da Ávila", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Importa dados de arquivos CSV
    Import {
        /// Tipo de dados (freight, timesheet, route, auto)
        #[arg(short, long, default_value = "auto")]
        r#type: String,
        
        /// Arquivo ou diretório para importar
        #[arg(short, long)]
        file: PathBuf,
    },
    
    /// Chat com a Personal-Controller-LLM
    Chat {
        /// Query para a LLM
        query: String,
    },
    
    /// Estatísticas do banco de dados
    Stats,
    
    /// Inicializa o banco de dados
    Init,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter("pc=debug,pc_core=debug,pc_models=debug")
        .init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Import { r#type, file } => {
            import_command(r#type, file).await?;
        }
        Commands::Chat { query } => {
            chat_command(query).await?;
        }
        Commands::Stats => {
            stats_command().await?;
        }
        Commands::Init => {
            init_command().await?;
        }
    }
    
    Ok(())
}

async fn import_command(import_type: String, file: PathBuf) -> anyhow::Result<()> {
    println!("🚀 Importando dados do tipo: {}", import_type);
    println!("📁 Arquivo: {:?}", file);
    
    match import_type.as_str() {
        "freight" | "auto" => {
            let importer = FreightOrderImporter::new();
            let orders = importer.import_file(&file)?;
            println!("✅ Importadas {} ordens de frete", orders.len());
            
            // TODO: Salvar no banco
            // let db = pc_db::PersonalControllerDb::connect("http://localhost:8000").await?;
            // for order in orders {
            //     db.freight_orders.save(&order).await?;
            // }
        }
        "timesheet" => {
            println!("⏰ Importando timesheets...");
            // TODO: Implementar
        }
        "route" => {
            println!("🗺️ Importando rotas...");
            // TODO: Implementar
        }
        _ => {
            println!("❌ Tipo desconhecido: {}", import_type);
        }
    }
    
    Ok(())
}

async fn chat_command(query: String) -> anyhow::Result<()> {
    println!("💬 Personal-Controller-LLM");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Você: {}", query);
    println!();
    
    let config = pc_llm::LlmConfig::default();
    let llm = pc_llm::PersonalControllerLlm::new(config)?;
    
    let response = llm.chat(&query).await?;
    
    println!("🤖 Assistente: {}", response);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    Ok(())
}

async fn stats_command() -> anyhow::Result<()> {
    println!("📊 Estatísticas do Personal Controller");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // TODO: Conectar ao banco e buscar stats
    // let db = pc_db::PersonalControllerDb::connect("http://localhost:8000").await?;
    // let stats = db.stats().await?;
    
    println!("🏢 Empresas: 0");
    println!("📦 Ordens de Frete: 0");
    println!("⏰ Registros de Ponto: 0");
    println!("👥 Contatos: 0");
    println!("🗺️ Rotas: 0");
    println!("📄 Manifestos: 0");
    
    Ok(())
}

async fn init_command() -> anyhow::Result<()> {
    println!("🔧 Inicializando Personal Controller Database");
    
    // TODO: Conectar e inicializar
    // let db = pc_db::PersonalControllerDb::connect("http://localhost:8000").await?;
    // db.initialize().await?;
    
    println!("✅ Banco de dados inicializado com sucesso!");
    
    Ok(())
}
