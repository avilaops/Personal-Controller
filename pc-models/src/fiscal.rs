//! Fiscal models (invoices, CT-e, etc)

use chrono::NaiveDate;
use pc_core::{AuditMetadata, Entity, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Nota Fiscal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub numero: String,
    pub serie: Option<String>,
    pub chave_acesso: Option<String>,
    pub data_emissao: NaiveDate,

    pub emitente_id: Uuid,
    pub emitente_nome: String,

    pub destinatario_id: Option<Uuid>,
    pub destinatario_nome: String,

    pub valor_total: f64,
    pub valor_produtos: f64,
    pub valor_frete: Option<f64>,
    pub valor_icms: Option<f64>,

    pub natureza_operacao: Option<String>,
    pub cfop: Option<String>,

    pub xml_path: Option<String>,
    pub pdf_path: Option<String>,

    pub metadata: AuditMetadata,
}

impl Entity for Invoice {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn entity_type(&self) -> &str {
        "invoice"
    }

    fn validate(&self) -> Result<()> {
        if self.numero.trim().is_empty() {
            return Err(pc_core::Error::Validation("Número não pode ser vazio".into()));
        }

        if self.valor_total < 0.0 {
            return Err(pc_core::Error::Validation("Valor não pode ser negativo".into()));
        }

        Ok(())
    }
}

/// CT-e (Conhecimento de Transporte Eletrônico)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cte {
    pub id: Uuid,
    pub numero: String,
    pub serie: Option<String>,
    pub chave_acesso: String,
    pub data_emissao: NaiveDate,

    pub emitente_id: Uuid,
    pub remetente_id: Uuid,
    pub destinatario_id: Uuid,
    pub expedidor_id: Option<Uuid>,
    pub recebedor_id: Option<Uuid>,

    pub valor_total: f64,
    pub valor_receber: f64,

    pub modal: String, // Rodoviário, Aéreo, etc
    pub tipo_servico: String,

    pub notas_fiscais: Vec<String>,

    pub xml_path: Option<String>,
    pub pdf_path: Option<String>,

    pub metadata: AuditMetadata,
}

impl Entity for Cte {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn entity_type(&self) -> &str {
        "cte"
    }

    fn validate(&self) -> Result<()> {
        if self.numero.trim().is_empty() {
            return Err(pc_core::Error::Validation("Número não pode ser vazio".into()));
        }

        if self.chave_acesso.len() != 44 {
            return Err(pc_core::Error::Validation("Chave de acesso inválida".into()));
        }

        Ok(())
    }
}
