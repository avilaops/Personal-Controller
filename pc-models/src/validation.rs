//! Validation module for data models

use crate::*;
use anyhow::{anyhow, Result};

pub trait Validate {
    fn validate(&self) -> Result<()>;
}

impl Validate for Company {
    fn validate(&self) -> Result<()> {
        // Nome obrigatório e não vazio
        if self.nome.trim().is_empty() {
            return Err(anyhow!("Company name cannot be empty"));
        }

        // CNPJ validation (basic)
        if !self.cnpj.is_empty() {
            let cnpj_digits: String = self.cnpj.chars().filter(|c| c.is_numeric()).collect();
            if cnpj_digits.len() != 14 {
                return Err(anyhow!("CNPJ must have 14 digits"));
            }
        }

        // Email validation (basic)
        if let Some(email) = &self.email {
            if !email.is_empty() && !email.contains('@') {
                return Err(anyhow!("Invalid email format"));
            }
        }

        // Telefone validation
        if let Some(tel) = &self.telefone {
            if !tel.is_empty() {
                let tel_digits: String = tel.chars().filter(|c| c.is_numeric()).collect();
                if tel_digits.len() < 10 || tel_digits.len() > 11 {
                    return Err(anyhow!("Phone number must have 10 or 11 digits"));
                }
            }
        }

        Ok(())
    }
}

impl Validate for FreightOrder {
    fn validate(&self) -> Result<()> {
        // Número do pedido obrigatório
        if self.numero_pedido.trim().is_empty() {
            return Err(anyhow!("Order number cannot be empty"));
        }

        // Valor do frete não pode ser negativo
        if self.valor_frete < 0.0 {
            return Err(anyhow!("Freight value cannot be negative"));
        }

        // Peso não pode ser negativo
        if self.peso < 0.0 {
            return Err(anyhow!("Weight cannot be negative"));
        }

        // Datas: data de entrega deve ser posterior à data de coleta
        if let (Some(coleta), Some(entrega)) = (self.data_coleta, self.data_entrega) {
            if entrega < coleta {
                return Err(anyhow!("Delivery date cannot be before pickup date"));
            }
        }

        // Status deve ser válido
        match self.status.as_str() {
            "pendente" | "em_transito" | "entregue" | "cancelado" => Ok(()),
            _ => Err(anyhow!("Invalid status: {}", self.status)),
        }
    }
}

impl Validate for Timesheet {
    fn validate(&self) -> Result<()> {
        // Data obrigatória
        if self.data.is_none() {
            return Err(anyhow!("Date is required"));
        }

        // Horas não podem ser negativas
        if self.horas_trabalhadas < 0.0 {
            return Err(anyhow!("Hours worked cannot be negative"));
        }

        // Horas não podem exceder 24h por dia
        if self.horas_trabalhadas > 24.0 {
            return Err(anyhow!("Hours worked cannot exceed 24 hours per day"));
        }

        // Valor hora não pode ser negativo
        if self.valor_hora < 0.0 {
            return Err(anyhow!("Hourly rate cannot be negative"));
        }

        Ok(())
    }
}

impl Validate for Route {
    fn validate(&self) -> Result<()> {
        // Origem e destino obrigatórios
        if self.origem.trim().is_empty() {
            return Err(anyhow!("Origin cannot be empty"));
        }

        if self.destino.trim().is_empty() {
            return Err(anyhow!("Destination cannot be empty"));
        }

        // Origem e destino não podem ser iguais
        if self.origem.trim().eq_ignore_ascii_case(self.destino.trim()) {
            return Err(anyhow!("Origin and destination cannot be the same"));
        }

        // Distância não pode ser negativa
        if self.distancia_km < 0.0 {
            return Err(anyhow!("Distance cannot be negative"));
        }

        // Tempo estimado não pode ser negativo
        if self.tempo_estimado_horas < 0.0 {
            return Err(anyhow!("Estimated time cannot be negative"));
        }

        Ok(())
    }
}

// Helper functions for common validations

pub fn validate_cpf(cpf: &str) -> Result<()> {
    let cpf_digits: String = cpf.chars().filter(|c| c.is_numeric()).collect();

    if cpf_digits.len() != 11 {
        return Err(anyhow!("CPF must have 11 digits"));
    }

    // Check for known invalid CPFs (all same digit)
    if cpf_digits.chars().all(|c| c == cpf_digits.chars().next().unwrap()) {
        return Err(anyhow!("Invalid CPF"));
    }

    Ok(())
}

pub fn validate_cnpj(cnpj: &str) -> Result<()> {
    let cnpj_digits: String = cnpj.chars().filter(|c| c.is_numeric()).collect();

    if cnpj_digits.len() != 14 {
        return Err(anyhow!("CNPJ must have 14 digits"));
    }

    // Check for known invalid CNPJs (all same digit)
    if cnpj_digits.chars().all(|c| c == cnpj_digits.chars().next().unwrap()) {
        return Err(anyhow!("Invalid CNPJ"));
    }

    Ok(())
}

pub fn validate_cep(cep: &str) -> Result<()> {
    let cep_digits: String = cep.chars().filter(|c| c.is_numeric()).collect();

    if cep_digits.len() != 8 {
        return Err(anyhow!("CEP must have 8 digits"));
    }

    Ok(())
}

pub fn validate_phone(phone: &str) -> Result<()> {
    let phone_digits: String = phone.chars().filter(|c| c.is_numeric()).collect();

    if phone_digits.len() < 10 || phone_digits.len() > 11 {
        return Err(anyhow!("Phone must have 10 or 11 digits"));
    }

    Ok(())
}

pub fn validate_email(email: &str) -> Result<()> {
    if !email.contains('@') || !email.contains('.') {
        return Err(anyhow!("Invalid email format"));
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(anyhow!("Invalid email format"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_validation() {
        let mut company = Company {
            nome: "".to_string(),
            cnpj: "12345678000199".to_string(),
            ..Default::default()
        };

        // Empty name should fail
        assert!(company.validate().is_err());

        company.nome = "Test Company".to_string();
        assert!(company.validate().is_ok());

        // Invalid CNPJ
        company.cnpj = "123".to_string();
        assert!(company.validate().is_err());
    }

    #[test]
    fn test_freight_order_validation() {
        let mut order = FreightOrder {
            numero_pedido: "12345".to_string(),
            valor_frete: -100.0,
            ..Default::default()
        };

        // Negative value should fail
        assert!(order.validate().is_err());

        order.valor_frete = 100.0;
        assert!(order.validate().is_ok());

        // Invalid status
        order.status = "invalid_status".to_string();
        assert!(order.validate().is_err());
    }

    #[test]
    fn test_timesheet_validation() {
        let mut timesheet = Timesheet {
            horas_trabalhadas: 25.0, // More than 24h
            ..Default::default()
        };

        assert!(timesheet.validate().is_err());

        timesheet.horas_trabalhadas = 8.0;
        assert!(timesheet.validate().is_ok());
    }

    #[test]
    fn test_cpf_validation() {
        assert!(validate_cpf("12345678901").is_ok());
        assert!(validate_cpf("123").is_err());
        assert!(validate_cpf("11111111111").is_err()); // All same digit
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("@example.com").is_err());
    }
}
