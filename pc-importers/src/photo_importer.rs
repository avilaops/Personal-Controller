//! Photo/Image importer with OCR capabilities

use pc_core::Result;
use pc_models::Document;
use std::path::{Path, PathBuf};
use chrono::Utc;
use uuid::Uuid;

pub struct PhotoImporter {
    supported_formats: Vec<String>,
}

impl PhotoImporter {
    pub fn new() -> Self {
        Self {
            supported_formats: vec![
                "png".to_string(),
                "jpg".to_string(),
                "jpeg".to_string(),
                "bmp".to_string(),
                "gif".to_string(),
                "webp".to_string(),
            ],
        }
    }

    /// Check if file is a supported image format
    pub fn is_image(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return self.supported_formats.contains(&ext_str.to_lowercase());
            }
        }
        false
    }

    /// Import single photo and extract metadata
    pub fn import_photo(&self, path: &Path) -> Result<PhotoMetadata> {
        tracing::info!("Importing photo: {:?}", path);

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

        // Extract date from filename if present (e.g., "2025-05-07")
        let date_from_filename = self.extract_date_from_filename(&filename);

        Ok(PhotoMetadata {
            id: Uuid::new_v4(),
            path: path.to_path_buf(),
            filename,
            file_size,
            captured_at: date_from_filename,
            imported_at: Utc::now(),
            description: None,
            tags: Vec::new(),
        })
    }

    /// Import all photos from a directory
    pub fn import_directory(&self, dir: &Path) -> Result<Vec<PhotoMetadata>> {
        tracing::info!("Importing photos from directory: {:?}", dir);

        if !dir.is_dir() {
            return Err(pc_core::Error::Import(format!("Not a directory: {:?}", dir)));
        }

        let mut photos = Vec::new();

        for entry in std::fs::read_dir(dir)
            .map_err(|e| pc_core::Error::Import(format!("Failed to read directory: {}", e)))?
        {
            let entry = entry.map_err(|e| pc_core::Error::Import(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.is_file() && self.is_image(&path) {
                match self.import_photo(&path) {
                    Ok(metadata) => photos.push(metadata),
                    Err(e) => tracing::warn!("Failed to import {:?}: {}", path, e),
                }
            }
        }

        tracing::info!("Imported {} photos", photos.len());
        Ok(photos)
    }

    /// Extract date from filename (e.g., "Captura de tela 2025-05-07 154629.png")
    fn extract_date_from_filename(&self, filename: &str) -> Option<chrono::NaiveDate> {
        use regex::Regex;
        let date_regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").ok()?;

        if let Some(caps) = date_regex.captures(filename) {
            let year: i32 = caps.get(1)?.as_str().parse().ok()?;
            let month: u32 = caps.get(2)?.as_str().parse().ok()?;
            let day: u32 = caps.get(3)?.as_str().parse().ok()?;
            chrono::NaiveDate::from_ymd_opt(year, month, day)
        } else {
            None
        }
    }

    /// Convert photo metadata to Document for storage
    pub fn to_document(&self, metadata: &PhotoMetadata) -> Document {
        Document {
            id: metadata.id,
            title: metadata.filename.clone(),
            content: format!(
                "Photo: {}\nSize: {} bytes\nCaptured: {:?}",
                metadata.filename,
                metadata.file_size,
                metadata.captured_at
            ),
            document_type: "photo".to_string(),
            source: Some(metadata.path.to_string_lossy().to_string()),
            created_at: metadata.imported_at,
            metadata: serde_json::json!({
                "filename": metadata.filename,
                "file_size": metadata.file_size,
                "captured_at": metadata.captured_at,
                "tags": metadata.tags,
            }),
        }
    }
}

impl Default for PhotoImporter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct PhotoMetadata {
    pub id: Uuid,
    pub path: PathBuf,
    pub filename: String,
    pub file_size: u64,
    pub captured_at: Option<chrono::NaiveDate>,
    pub imported_at: chrono::DateTime<Utc>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_date_from_filename() {
        let importer = PhotoImporter::new();

        let date = importer.extract_date_from_filename("Captura de tela 2025-05-07 154629.png");
        assert!(date.is_some());
        assert_eq!(date.unwrap().to_string(), "2025-05-07");

        let no_date = importer.extract_date_from_filename("random_photo.png");
        assert!(no_date.is_none());
    }

    #[test]
    fn test_is_image() {
        let importer = PhotoImporter::new();

        assert!(importer.is_image(Path::new("test.png")));
        assert!(importer.is_image(Path::new("test.jpg")));
        assert!(importer.is_image(Path::new("test.JPEG")));
        assert!(!importer.is_image(Path::new("test.txt")));
        assert!(!importer.is_image(Path::new("test.csv")));
    }
}
