//! Timesheet model

use chrono::{NaiveDate, NaiveTime, Duration};
use pc_core::{AuditMetadata, Entity, Embeddable, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Registro de ponto de funcionário
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timesheet {
    pub id: Uuid,
    pub funcionario: String,
    pub funcionario_id: Option<Uuid>,
    pub mes: String,
    pub data: NaiveDate,
    pub entrada: NaiveTime,
    pub saida: NaiveTime,
    pub total_minutos: i32,
    pub saldo_minutos: i32,
    pub observacoes: Option<String>,
    pub fonte_arquivo: Option<String>,
    pub metadata: AuditMetadata,
}

impl Timesheet {
    pub fn new(
        funcionario: String,
        mes: String,
        data: NaiveDate,
        entrada: NaiveTime,
        saida: NaiveTime,
    ) -> Self {
        let total_minutos = Self::calculate_minutes(entrada, saida);

        Self {
            id: Uuid::new_v4(),
            funcionario,
            funcionario_id: None,
            mes,
            data,
            entrada,
            saida,
            total_minutos,
            saldo_minutos: 0,
            observacoes: None,
            fonte_arquivo: None,
            metadata: AuditMetadata::default(),
        }
    }

    fn calculate_minutes(entrada: NaiveTime, saida: NaiveTime) -> i32 {
        let entrada_mins = entrada.hour() as i32 * 60 + entrada.minute() as i32;
        let saida_mins = saida.hour() as i32 * 60 + saida.minute() as i32;

        if saida_mins >= entrada_mins {
            saida_mins - entrada_mins
        } else {
            // Passou da meia-noite
            (24 * 60 - entrada_mins) + saida_mins
        }
    }

    pub fn total_hours(&self) -> f64 {
        self.total_minutos as f64 / 60.0
    }

    pub fn saldo_hours(&self) -> f64 {
        self.saldo_minutos as f64 / 60.0
    }
}

impl Entity for Timesheet {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn entity_type(&self) -> &str {
        "timesheet"
    }

    fn validate(&self) -> Result<()> {
        if self.funcionario.trim().is_empty() {
            return Err(pc_core::Error::Validation("Funcionário não pode ser vazio".into()));
        }

        if self.saida < self.entrada {
            // Pode ser válido se passou da meia-noite, mas vamos alertar
            // return Err(pc_core::Error::Validation("Saída anterior à entrada".into()));
        }

        Ok(())
    }
}

impl Embeddable for Timesheet {
    fn to_embedding_text(&self) -> String {
        format!(
            "Ponto {} em {} entrada {} saida {} total {}h mes {}",
            self.funcionario,
            self.data,
            self.entrada,
            self.saida,
            self.total_hours(),
            self.mes
        )
    }

    fn searchable_fields(&self) -> Vec<String> {
        vec![
            self.funcionario.clone(),
            self.mes.clone(),
            self.data.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_timesheet_creation() {
        let timesheet = Timesheet::new(
            "João Silva".into(),
            "Janeiro/2025".into(),
            NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        );

        assert_eq!(timesheet.total_minutos, 540); // 9 horas
        assert_eq!(timesheet.total_hours(), 9.0);
    }
}
