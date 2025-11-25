//! Timesheet importer

use pc_core::Result;
use pc_models::Timesheet;
use crate::Importer;
use chrono::{NaiveDate, NaiveTime};
use encoding_rs::WINDOWS_1252;
use std::path::Path;
use regex::Regex;

pub struct TimesheetImporter {
    date_regex: Regex,
    time_regex: Regex,
}

impl TimesheetImporter {
    pub fn new() -> Self {
        Self {
            date_regex: Regex::new(r"(\d{2})/(\d{2})/(\d{4})").unwrap(),
            time_regex: Regex::new(r"(\d{1,2}):(\d{2})").unwrap(),
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
    
    fn parse_time(&self, time_str: &str) -> Option<NaiveTime> {
        if let Some(caps) = self.time_regex.captures(time_str) {
            let hour: u32 = caps.get(1)?.as_str().parse().ok()?;
            let minute: u32 = caps.get(2)?.as_str().parse().ok()?;
            NaiveTime::from_hms_opt(hour, minute, 0)
        } else {
            None
        }
    }
}

impl Default for TimesheetImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Importer for TimesheetImporter {
    type Output = Timesheet;
    
    fn import_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Self::Output>> {
        let path = path.as_ref();
        tracing::info!("Importando timesheets de: {:?}", path);
        
        let file_bytes = std::fs::read(path)
            .map_err(|e| pc_core::Error::Import(format!("Erro ao ler arquivo: {}", e)))?;
        
        let (content, _, _) = WINDOWS_1252.decode(&file_bytes);
        
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .flexible(true)
            .from_reader(content.as_bytes());
        
        let mut timesheets = Vec::new();
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
            
            if record.len() < 5 {
                continue;
            }
            
            let funcionario = record.get(0).unwrap_or("").trim();
            if funcionario.is_empty() {
                continue;
            }
            
            let mes = record.get(1).unwrap_or("").trim().to_string();
            let data = record.get(2).and_then(|s| self.parse_date(s));
            let entrada = record.get(3).and_then(|s| self.parse_time(s));
            let saida = record.get(4).and_then(|s| self.parse_time(s));
            
            if let (Some(data), Some(entrada), Some(saida)) = (data, entrada, saida) {
                let mut timesheet = Timesheet::new(
                    funcionario.to_string(),
                    mes,
                    data,
                    entrada,
                    saida,
                );
                timesheet.fonte_arquivo = source_file.clone();
                timesheets.push(timesheet);
            }
        }
        
        tracing::info!("Importados {} registros de ponto", timesheets.len());
        Ok(timesheets)
    }
    
    fn can_import<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            filename.ends_with(".csv") && filename.contains("Horas")
        } else {
            false
        }
    }
}
