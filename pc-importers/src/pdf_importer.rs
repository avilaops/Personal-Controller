//! PDF document importer with text extraction capabilities

use pc_core::Result;
use pc_models::Document;
use std::path::{Path, PathBuf};
use chrono::Utc;
use uuid::Uuid;

pub struct PdfImporter {
    supported_formats: Vec<String>,
}

impl PdfImporter {
    pub fn new() -> Self {
        Self {
            supported_formats: vec!["pdf".to_string()],
        }
    }

    /// Check if file is a PDF
    pub fn is_pdf(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return self.supported_formats.contains(&ext_str.to_lowercase());
            }
        }
        false
    }

    /// Import single PDF and extract metadata
    pub fn import_pdf(&self, path: &Path) -> Result<PdfMetadata> {
        tracing::info!("Importing PDF: {:?}", path);

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

        // Detect document type from filename
        let doc_type = self.detect_document_type(&filename);

        Ok(PdfMetadata {
            id: Uuid::new_v4(),
            path: path.to_path_buf(),
            filename,
            file_size,
            document_type: doc_type,
            imported_at: Utc::now(),
            page_count: None, // TODO: Extract actual page count
            text_content: None, // TODO: Extract text using pdf-extract or similar
        })
    }

    /// Import all PDFs from a directory
    pub fn import_directory(&self, dir: &Path) -> Result<Vec<PdfMetadata>> {
        tracing::info!("Importing PDFs from directory: {:?}", dir);

        if !dir.is_dir() {
            return Err(pc_core::Error::Import(format!("Not a directory: {:?}", dir)));
        }

        let mut pdfs = Vec::new();

        for entry in std::fs::read_dir(dir)
            .map_err(|e| pc_core::Error::Import(format!("Failed to read directory: {}", e)))?
        {
            let entry = entry.map_err(|e| pc_core::Error::Import(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.is_file() && self.is_pdf(&path) {
                match self.import_pdf(&path) {
                    Ok(metadata) => pdfs.push(metadata),
                    Err(e) => tracing::warn!("Failed to import {:?}: {}", path, e),
                }
            }
        }

        tracing::info!("Imported {} PDFs", pdfs.len());
        Ok(pdfs)
    }

    /// Detect document type from filename
    fn detect_document_type(&self, filename: &str) -> String {
        let lower = filename.to_lowercase();

        if lower.contains("cte") || lower.contains("ct-e") {
            "CT-e".to_string()
        } else if lower.contains("nfe") || lower.contains("nf-e") {
            "NF-e".to_string()
        } else if lower.contains("minuta") {
            "Minuta".to_string()
        } else if lower.contains("comprovante") {
            "Comprovante".to_string()
        } else if lower.contains("embarque") {
            "Pre-Embarque".to_string()
        } else if lower.contains("fatura") {
            "Fatura".to_string()
        } else if lower.contains("relatorio") || lower.contains("report") {
            "Relatorio".to_string()
        } else {
            "PDF".to_string()
        }
    }

    /// Convert PDF metadata to Document for storage
    pub fn to_document(&self, metadata: &PdfMetadata) -> Document {
        Document {
            id: metadata.id,
            title: metadata.filename.clone(),
            content: format!(
                "PDF Document: {}\nType: {}\nSize: {} bytes\nPages: {:?}",
                metadata.filename,
                metadata.document_type,
                metadata.file_size,
                metadata.page_count
            ),
            document_type: metadata.document_type.clone(),
            source: Some(metadata.path.to_string_lossy().to_string()),
            created_at: metadata.imported_at,
            metadata: serde_json::json!({
                "filename": metadata.filename,
                "file_size": metadata.file_size,
                "page_count": metadata.page_count,
                "document_type": metadata.document_type,
            }),
        }
    }
}

impl Default for PdfImporter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct PdfMetadata {
    pub id: Uuid,
    pub path: PathBuf,
    pub filename: String,
    pub file_size: u64,
    pub document_type: String,
    pub imported_at: chrono::DateTime<Utc>,
    pub page_count: Option<usize>,
    pub text_content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_document_type() {
        let importer = PdfImporter::new();

        assert_eq!(importer.detect_document_type("CT-e_2024.pdf"), "CT-e");
        assert_eq!(importer.detect_document_type("NF-e_12345.pdf"), "NF-e");
        assert_eq!(importer.detect_document_type("Minuta_001.pdf"), "Minuta");
        assert_eq!(importer.detect_document_type("PRE-EMBARQUE 288650.pdf"), "Pre-Embarque");
        assert_eq!(importer.detect_document_type("Fatura_2024.pdf"), "Fatura");
        assert_eq!(importer.detect_document_type("random_document.pdf"), "PDF");
    }

    #[test]
    fn test_is_pdf() {
        let importer = PdfImporter::new();

        assert!(importer.is_pdf(Path::new("test.pdf")));
        assert!(importer.is_pdf(Path::new("test.PDF")));
        assert!(!importer.is_pdf(Path::new("test.txt")));
        assert!(!importer.is_pdf(Path::new("test.docx")));
    }
}
