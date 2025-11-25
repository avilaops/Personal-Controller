//! Freight order importer

use pc_core::Result;
use pc_models::FreightOrder;
use crate::Importer;
use chrono::NaiveDate;
use encoding_rs::WINDOWS_1252;
use std::path::Path;
use regex::Regex;

pub struct FreightOrderImporter {
    date_regex: Regex,
}

impl FreightOrderImporter {
    pub fn new() -> Self {
        Self {
            date_regex: Regex::new(r"(\d{2})/(\d{2})/(\d{4})").unwrap(),
        }
    }
    
    fn parse_date(&self, date_str: &str) -> Option<NaiveDate> {
        if let Some(caps) = self.date_regex.captures(date_str) {
            let day: u32 = caps.get(1)?.as_str().parse().ok()?;
            let month: u32 = caps.get(2)?.as_str().parse().ok()?;
            let year: i32 = caps.get(3)?.as_str().parse().ok()?;
            NaiveDate::from_ymd_opt(year, month, day)
        } else {
            None
        }
    }
    
    fn parse_float(&self, value: &str) -> f64 {
        value
            .replace("R$", "")
            .replace(".", "")
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0)
    }
    
    fn parse_int(&self, value: &str) -> i32 {
        value.trim().parse().unwrap_or(0)
    }
    
    fn split_notas(&self, notas_str: &str) -> Vec<String> {
        notas_str
            .split(&[',', ';'][..])
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

impl Default for FreightOrderImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Importer for FreightOrderImporter {
    type Output = FreightOrder;
    
    fn import_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Self::Output>> {
        let path = path.as_ref();
        tracing::info!("Importando ordens de frete de: {:?}", path);
        
        // Lê o arquivo em Windows-1252 (encoding comum em CSVs brasileiros)
        let file_bytes = std::fs::read(path)
            .map_err(|e| pc_core::Error::Import(format!("Erro ao ler arquivo: {}", e)))?;
        
        let (content, _, had_errors) = WINDOWS_1252.decode(&file_bytes);
        if had_errors {
            tracing::warn!("Erros de encoding detectados no arquivo");
        }
        
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .flexible(true)
            .from_reader(content.as_bytes());
        
        let mut orders = Vec::new();
        let source_file = path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string());
        
        for (idx, result) in reader.records().enumerate() {
            let record = match result {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!("Erro na linha {}: {}", idx + 1, e);
                    continue;
                }
            };
            
            // Pula linhas vazias ou com dados insuficientes
            if record.len() < 10 {
                continue;
            }
            
            // Coluna 0: Número
            let numero = record.get(0).unwrap_or("").trim();
            if numero.is_empty() {
                continue;
            }
            
            // Colunas principais (baseado no CSV real)
            let data_agendamento = record.get(1).and_then(|s| self.parse_date(s));
            let data_emissao = record.get(2).and_then(|s| self.parse_date(s));
            let notas_fiscais = record.get(3).map(|s| self.split_notas(s)).unwrap_or_default();
            let pagador_nome = record.get(4).unwrap_or("").trim().to_string();
            let pagador_telefone = record.get(5).map(|s| s.trim().to_string());
            let remetente_nome = record.get(6).unwrap_or("").trim().to_string();
            let remetente_cidade = record.get(7).unwrap_or("").trim().to_string();
            let destinatario_nome = record.get(8).unwrap_or("").trim().to_string();
            let destinatario_cidade = record.get(9).unwrap_or("").trim().to_string();
            
            if pagador_nome.is_empty() || remetente_nome.is_empty() || destinatario_nome.is_empty() {
                continue;
            }
            
            let volumes = record.get(10).map(|s| self.parse_int(s)).unwrap_or(0);
            let peso = record.get(11).map(|s| self.parse_float(s)).unwrap_or(0.0);
            let valor_notas = record.get(12).map(|s| self.parse_float(s)).unwrap_or(0.0);
            let valor_frete = record.get(13).map(|s| self.parse_float(s)).unwrap_or(0.0);
            let frete_tabelado = record.get(14).map(|s| self.parse_float(s));
            
            let filial_coleta = record.get(26).map(|s| s.trim().to_string());
            let motorista_coleta = record.get(27).map(|s| s.trim().to_string());
            let filial_entrega = record.get(28).map(|s| s.trim().to_string());
            let motorista_entrega = record.get(29).map(|s| s.trim().to_string());
            
            let Some(data_emissao) = data_emissao else {
                tracing::warn!("Ordem {} sem data de emissão", numero);
                continue;
            };
            
            let mut order = FreightOrder::new(
                numero.to_string(),
                data_emissao,
                pagador_nome,
                remetente_nome,
                remetente_cidade,
                destinatario_nome,
                destinatario_cidade,
            );
            
            order.data_agendamento = data_agendamento;
            order.notas_fiscais = notas_fiscais;
            order.pagador_telefone = pagador_telefone;
            order.volumes = volumes;
            order.peso = peso;
            order.valor_notas = valor_notas;
            order.valor_frete = valor_frete;
            order.frete_tabelado = frete_tabelado;
            order.filial_coleta = filial_coleta;
            order.motorista_coleta = motorista_coleta;
            order.filial_entrega = filial_entrega;
            order.motorista_entrega = motorista_entrega;
            order.fonte_arquivo = source_file.clone();
            
            orders.push(order);
        }
        
        tracing::info!("Importadas {} ordens de frete", orders.len());
        Ok(orders)
    }
    
    fn can_import<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            filename.ends_with(".csv") && 
            (filename.contains("-04") || filename.contains("Planilha"))
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        let importer = FreightOrderImporter::new();
        let date = importer.parse_date("01/04/2025");
        assert!(date.is_some());
        assert_eq!(date.unwrap().to_string(), "2025-04-01");
    }
    
    #[test]
    fn test_parse_float() {
        let importer = FreightOrderImporter::new();
        assert_eq!(importer.parse_float("R$ 1.234,56"), 1234.56);
        assert_eq!(importer.parse_float("90"), 90.0);
    }
}
