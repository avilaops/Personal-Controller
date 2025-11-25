//! Company model

use chrono::{DateTime, Utc};
use pc_core::{AuditMetadata, Entity, Embeddable, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tipo de empresa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyType {
    Cliente,
    Fornecedor,
    Parceiro,
    Transportadora,
    Outros,
}

/// Empresa (cliente, fornecedor, parceiro)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub nome: String,
    pub nome_fantasia: Option<String>,
    pub cnpj: Option<String>,
    pub cpf: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub tipo: CompanyType,

    // Endereço
    pub endereco: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub bairro: Option<String>,
    pub cidade: String,
    pub estado: String,
    pub cep: Option<String>,

    // Contato
    pub telefone: Option<String>,
    pub celular: Option<String>,
    pub email: Option<String>,
    pub site: Option<String>,

    // Informações adicionais
    pub observacoes: Option<String>,
    pub ativo: bool,

    // Auditoria
    pub metadata: AuditMetadata,
}

impl Company {
    pub fn new(nome: String, cidade: String, estado: String, tipo: CompanyType) -> Self {
        Self {
            id: Uuid::new_v4(),
            nome,
            nome_fantasia: None,
            cnpj: None,
            cpf: None,
            inscricao_estadual: None,
            tipo,
            endereco: None,
            numero: None,
            complemento: None,
            bairro: None,
            cidade,
            estado,
            cep: None,
            telefone: None,
            celular: None,
            email: None,
            site: None,
            observacoes: None,
            ativo: true,
            metadata: AuditMetadata::default(),
        }
    }

    pub fn with_cnpj(mut self, cnpj: String) -> Self {
        self.cnpj = Some(cnpj);
        self
    }

    pub fn with_telefone(mut self, telefone: String) -> Self {
        self.telefone = Some(telefone);
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// Valida CNPJ (formato simplificado)
    pub fn validate_cnpj(cnpj: &str) -> bool {
        let digits: String = cnpj.chars().filter(|c| c.is_numeric()).collect();
        digits.len() == 14
    }

    /// Valida CPF (formato simplificado)
    pub fn validate_cpf(cpf: &str) -> bool {
        let digits: String = cpf.chars().filter(|c| c.is_numeric()).collect();
        digits.len() == 11
    }
}

impl Entity for Company {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn entity_type(&self) -> &str {
        "company"
    }

    fn validate(&self) -> Result<()> {
        if self.nome.trim().is_empty() {
            return Err(pc_core::Error::Validation("Nome não pode ser vazio".into()));
        }

        if self.cidade.trim().is_empty() {
            return Err(pc_core::Error::Validation("Cidade não pode ser vazia".into()));
        }

        if self.estado.trim().is_empty() {
            return Err(pc_core::Error::Validation("Estado não pode ser vazio".into()));
        }

        if let Some(cnpj) = &self.cnpj {
            if !Self::validate_cnpj(cnpj) {
                return Err(pc_core::Error::Validation("CNPJ inválido".into()));
            }
        }

        if let Some(cpf) = &self.cpf {
            if !Self::validate_cpf(cpf) {
                return Err(pc_core::Error::Validation("CPF inválido".into()));
            }
        }

        Ok(())
    }
}

impl Embeddable for Company {
    fn to_embedding_text(&self) -> String {
        format!(
            "{} {} {} {} {} {} {}",
            self.nome,
            self.nome_fantasia.as_deref().unwrap_or(""),
            self.cnpj.as_deref().unwrap_or(""),
            self.cidade,
            self.estado,
            self.telefone.as_deref().unwrap_or(""),
            self.email.as_deref().unwrap_or("")
        )
    }

    fn searchable_fields(&self) -> Vec<String> {
        let mut fields = vec![
            self.nome.clone(),
            self.cidade.clone(),
            self.estado.clone(),
        ];

        if let Some(fantasia) = &self.nome_fantasia {
            fields.push(fantasia.clone());
        }
        if let Some(cnpj) = &self.cnpj {
            fields.push(cnpj.clone());
        }

        fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_creation() {
        let company = Company::new(
            "Avila Transportes".into(),
            "Ribeirão Preto".into(),
            "SP".into(),
            CompanyType::Transportadora,
        );

        assert_eq!(company.nome, "Avila Transportes");
        assert_eq!(company.cidade, "Ribeirão Preto");
        assert!(company.ativo);
    }

    #[test]
    fn test_cnpj_validation() {
        assert!(Company::validate_cnpj("12.345.678/0001-90"));
        assert!(Company::validate_cnpj("12345678000190"));
        assert!(!Company::validate_cnpj("123456"));
    }
}
