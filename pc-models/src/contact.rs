//! Contact model

use pc_core::{AuditMetadata, Entity, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Contato de empresa ou pessoa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub nome: String,
    pub cargo: Option<String>,
    pub empresa_id: Uuid,

    pub telefone: Option<String>,
    pub celular: Option<String>,
    pub email: Option<String>,
    pub whatsapp: Option<String>,

    pub principal: bool,
    pub ativo: bool,
    pub observacoes: Option<String>,

    pub metadata: AuditMetadata,
}

impl Contact {
    pub fn new(nome: String, empresa_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            nome,
            cargo: None,
            empresa_id,
            telefone: None,
            celular: None,
            email: None,
            whatsapp: None,
            principal: false,
            ativo: true,
            observacoes: None,
            metadata: AuditMetadata::default(),
        }
    }
}

impl Entity for Contact {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn entity_type(&self) -> &str {
        "contact"
    }

    fn validate(&self) -> Result<()> {
        if self.nome.trim().is_empty() {
            return Err(pc_core::Error::Validation("Nome não pode ser vazio".into()));
        }
        Ok(())
    }
}
