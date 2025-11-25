//! Route model

use pc_core::{AuditMetadata, Entity, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Rota de transporte
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: Uuid,
    pub nome: String,
    pub regiao: String,
    pub cidades: Vec<String>,

    pub distancia_km: Option<f64>,
    pub tempo_estimado_horas: Option<f64>,
    pub pedagios: Option<i32>,
    pub custo_pedagio: Option<f64>,

    pub ativo: bool,
    pub observacoes: Option<String>,

    pub metadata: AuditMetadata,
}

impl Route {
    pub fn new(nome: String, regiao: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            nome,
            regiao,
            cidades: Vec::new(),
            distancia_km: None,
            tempo_estimado_horas: None,
            pedagios: None,
            custo_pedagio: None,
            ativo: true,
            observacoes: None,
            metadata: AuditMetadata::default(),
        }
    }

    pub fn add_cidade(&mut self, cidade: String) {
        if !self.cidades.contains(&cidade) {
            self.cidades.push(cidade);
        }
    }
}

impl Entity for Route {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn entity_type(&self) -> &str {
        "route"
    }

    fn validate(&self) -> Result<()> {
        if self.nome.trim().is_empty() {
            return Err(pc_core::Error::Validation("Nome não pode ser vazio".into()));
        }
        Ok(())
    }
}
