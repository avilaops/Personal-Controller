//! Excel file importer (.xlsx, .xls)
//! Converts Excel files to internal format for processing

use pc_core::Result;
use std::path::Path;
use serde_json::Value;

pub struct ExcelImporter {
    // Future: Add calamine or xlsx crate for native Excel reading
}

impl ExcelImporter {
    pub fn new() -> Self {
        Self {}
    }

    /// Check if file is an Excel file
    pub fn is_excel(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return ext_str.eq_ignore_ascii_case("xlsx") || ext_str.eq_ignore_ascii_case("xls");
            }
        }
        false
    }

    /// Import Excel file (placeholder - needs calamine crate)
    pub fn import_excel(&self, path: &Path) -> Result<Vec<ExcelRow>> {
        tracing::info!("Excel import requested for: {:?}", path);
        tracing::warn!("Excel import not yet implemented - please convert to CSV first");

        // TODO: Implement Excel reading using calamine crate
        // For now, return empty result
        Ok(Vec::new())
    }

    /// Get Excel file metadata
    pub fn get_metadata(&self, path: &Path) -> Result<ExcelMetadata> {
        if !path.exists() {
            return Err(pc_core::Error::Import(format!("File not found: {:?}", path)));
        }

        let file_size = std::fs::metadata(path)
            .map_err(|e| pc_core::Error::Import(format!("Failed to read metadata: {}", e)))?
            .len();

        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| pc_core::Error::Import("Invalid filename".into()))?
            .to_string();

        Ok(ExcelMetadata {
            filename,
            file_size,
            sheet_count: 0, // TODO: Read actual sheet count
        })
    }
}

impl Default for ExcelImporter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ExcelRow {
    pub sheet_name: String,
    pub row_number: usize,
    pub data: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct ExcelMetadata {
    pub filename: String,
    pub file_size: u64,
    pub sheet_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_excel() {
        let importer = ExcelImporter::new();

        assert!(importer.is_excel(Path::new("test.xlsx")));
        assert!(importer.is_excel(Path::new("test.xls")));
        assert!(importer.is_excel(Path::new("test.XLSX")));
        assert!(!importer.is_excel(Path::new("test.csv")));
        assert!(!importer.is_excel(Path::new("test.txt")));
    }
}
