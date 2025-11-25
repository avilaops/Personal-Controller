//! Route importer

use pc_core::Result;
use pc_models::Route;
use crate::Importer;
use encoding_rs::WINDOWS_1252;
use std::path::Path;

pub struct RouteImporter;

impl RouteImporter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RouteImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Importer for RouteImporter {
    type Output = Route;
    
    fn import_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Self::Output>> {
        let path = path.as_ref();
        tracing::info!("Importando rotas de: {:?}", path);
        
        let file_bytes = std::fs::read(path)
            .map_err(|e| pc_core::Error::Import(format!("Erro ao ler arquivo: {}", e)))?;
        
        let (content, _, _) = WINDOWS_1252.decode(&file_bytes);
        
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .flexible(true)
            .from_reader(content.as_bytes());
        
        let mut routes = Vec::new();
        
        for result in reader.records() {
            let record = match result {
                Ok(r) => r,
                Err(_) => continue,
            };
            
            // Processa as cidades das rotas
            for (col_idx, field) in record.iter().enumerate() {
                let cidade = field.trim();
                if cidade.is_empty() || cidade.starts_with('�') {
                    continue;
                }
                
                // Extrai região baseada na coluna
                let regiao = match col_idx {
                    0..=7 => "Ribeirão Preto (RA15)",
                    8..=15 => "Franca",
                    16..=23 => "Central (RA14 - Araraquara)",
                    _ => "Araras",
                };
                
                let nome = format!("{} - {}", regiao, cidade);
                let mut route = Route::new(nome, regiao.to_string());
                route.add_cidade(cidade.to_string());
                routes.push(route);
            }
        }
        
        tracing::info!("Importadas {} rotas", routes.len());
        Ok(routes)
    }
    
    fn can_import<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            filename.ends_with(".csv") && filename.contains("Rotas")
        } else {
            false
        }
    }
}
