//! Freight order model

use chrono::NaiveDate;
use pc_core::{AuditMetadata, Entity, Embeddable, Result, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Ordem de frete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreightOrder {
    pub id: Uuid,
    pub numero: String,
    pub data_emissao: NaiveDate,
    pub data_agendamento: Option<NaiveDate>,
    pub data_entrega: Option<NaiveDate>,

    // Documentos
    pub notas_fiscais: Vec<String>,
    pub cte_numero: Option<String>,
    pub cte_chave: Option<String>,

    // Partes envolvidas (IDs de Company)
    pub pagador_id: Uuid,
    pub pagador_nome: String,
    pub pagador_telefone: Option<String>,

    pub remetente_id: Option<Uuid>,
    pub remetente_nome: String,
    pub remetente_cidade: String,

    pub destinatario_id: Option<Uuid>,
    pub destinatario_nome: String,
    pub destinatario_cidade: String,

    // Carga
    pub volumes: i32,
    pub peso: f64,
    pub valor_notas: f64,
    pub valor_frete: f64,
    pub frete_tabelado: Option<f64>,

    // Operação
    pub filial_coleta: Option<String>,
    pub motorista_coleta: Option<String>,
    pub filial_entrega: Option<String>,
    pub motorista_entrega: Option<String>,

    pub forma_pagamento: Option<String>,
    pub status: Status,
    pub observacoes: Option<String>,

    // Origem dos dados
    pub fonte_arquivo: Option<String>,

    // Auditoria
    pub metadata: AuditMetadata,
}

impl FreightOrder {
    pub fn new(
        numero: String,
        data_emissao: NaiveDate,
        pagador_nome: String,
        remetente_nome: String,
        remetente_cidade: String,
        destinatario_nome: String,
        destinatario_cidade: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            numero,
            data_emissao,
            data_agendamento: None,
            data_entrega: None,
            notas_fiscais: Vec::new(),
            cte_numero: None,
            cte_chave: None,
            pagador_id: Uuid::new_v4(), // Temporary - will be resolved
            pagador_nome,
            pagador_telefone: None,
            remetente_id: None,
            remetente_nome,
            remetente_cidade,
            destinatario_id: None,
            destinatario_nome,
            destinatario_cidade,
            volumes: 0,
            peso: 0.0,
            valor_notas: 0.0,
            valor_frete: 0.0,
            frete_tabelado: None,
            filial_coleta: None,
            motorista_coleta: None,
            filial_entrega: None,
            motorista_entrega: None,
            forma_pagamento: None,
            status: Status::Pending,
            observacoes: None,
            fonte_arquivo: None,
            metadata: AuditMetadata::default(),
        }
    }
}

impl Entity for FreightOrder {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn entity_type(&self) -> &str {
        "freight_order"
    }

    fn validate(&self) -> Result<()> {
        if self.numero.trim().is_empty() {
            return Err(pc_core::Error::Validation("Número não pode ser vazio".into()));
        }

        if self.pagador_nome.trim().is_empty() {
            return Err(pc_core::Error::Validation("Pagador não pode ser vazio".into()));
        }

        if self.volumes < 0 {
            return Err(pc_core::Error::Validation("Volumes não pode ser negativo".into()));
        }

        if self.peso < 0.0 {
            return Err(pc_core::Error::Validation("Peso não pode ser negativo".into()));
        }

        if self.valor_frete < 0.0 {
            return Err(pc_core::Error::Validation("Valor do frete não pode ser negativo".into()));
        }

        Ok(())
    }
}

impl Embeddable for FreightOrder {
    fn to_embedding_text(&self) -> String {
        format!(
            "Ordem {} de {} para {} pagador {} notas {} valor {} motorista {}",
            self.numero,
            self.remetente_cidade,
            self.destinatario_cidade,
            self.pagador_nome,
            self.notas_fiscais.join(","),
            self.valor_frete,
            self.motorista_entrega.as_deref().unwrap_or("")
        )
    }

    fn searchable_fields(&self) -> Vec<String> {
        let mut fields = vec![
            self.numero.clone(),
            self.pagador_nome.clone(),
            self.remetente_nome.clone(),
            self.remetente_cidade.clone(),
            self.destinatario_nome.clone(),
            self.destinatario_cidade.clone(),
        ];

        fields.extend(self.notas_fiscais.clone());

        if let Some(motorista) = &self.motorista_coleta {
            fields.push(motorista.clone());
        }
        if let Some(motorista) = &self.motorista_entrega {
            fields.push(motorista.clone());
        }

        fields
    }
}

/// Manifesto de transporte
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub id: Uuid,
    pub tipo: String, // Minuta, CT-e, etc
    pub numero: String,
    pub data_emissao: NaiveDate,
    pub data_entrega: Option<NaiveDate>,
    pub data_agendamento: Option<NaiveDate>,

    pub notas_fiscais: Vec<String>,
    pub chave_acesso: Option<String>,

    pub remetente_nome: String,
    pub destinatario_nome: String,
    pub destinatario_cidade: String,

    pub volumes: i32,
    pub peso: f64,
    pub valor_notas: f64,
    pub valor_frete: f64,

    pub status: String, // Finalizada, Pendente, Em Atraso
    pub observacoes: Option<String>,

    pub metadata: AuditMetadata,
}

impl Entity for Manifest {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn entity_type(&self) -> &str {
        "manifest"
    }

    fn validate(&self) -> Result<()> {
        if self.numero.trim().is_empty() {
            return Err(pc_core::Error::Validation("Número não pode ser vazio".into()));
        }
        Ok(())
    }
}

impl Embeddable for Manifest {
    fn to_embedding_text(&self) -> String {
        format!(
            "Manifesto {} tipo {} de {} para {} notas {} status {}",
            self.numero,
            self.tipo,
            self.remetente_nome,
            self.destinatario_cidade,
            self.notas_fiscais.join(","),
            self.status
        )
    }

    fn searchable_fields(&self) -> Vec<String> {
        let mut fields = vec![
            self.numero.clone(),
            self.tipo.clone(),
            self.remetente_nome.clone(),
            self.destinatario_nome.clone(),
            self.destinatario_cidade.clone(),
            self.status.clone(),
        ];
        fields.extend(self.notas_fiscais.clone());
        fields
    }
}
