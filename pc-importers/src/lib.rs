//! Personal Controller - Data Importers
//!
//! Importadores de CSV e outros formatos para o Personal Controller

pub mod freight_importer;
pub mod timesheet_importer;
pub mod route_importer;
pub mod photo_importer;
pub mod excel_importer;
pub mod pdf_importer;use pc_core::Result;
use std::path::Path;

/// Trait para importadores genéricos
pub trait Importer {
    type Output;

    /// Importa dados de um arquivo
    fn import_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Self::Output>>;

    /// Valida se o arquivo é compatível com este importador
    fn can_import<P: AsRef<Path>>(&self, path: P) -> bool;
}

/// Detecta automaticamente o tipo de arquivo e retorna o importador apropriado
pub fn auto_detect_importer<P: AsRef<Path>>(path: P) -> Result<Box<dyn std::any::Any>> {
    let path = path.as_ref();
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| pc_core::Error::Import("Nome de arquivo inválido".into()))?;

    // Detecta por nome do arquivo
    if filename.contains("Horas") || filename.contains("Ponto") {
        Ok(Box::new(timesheet_importer::TimesheetImporter::new()))
    } else if filename.contains("Rotas") {
        Ok(Box::new(route_importer::RouteImporter::new()))
    } else if filename.contains("-04") || filename.contains("Planilha") {
        Ok(Box::new(freight_importer::FreightOrderImporter::new()))
    } else {
        Err(pc_core::Error::Import(format!(
            "Não foi possível detectar o tipo do arquivo: {}",
            filename
        )))
    }
}
