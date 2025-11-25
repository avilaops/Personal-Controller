//! Advanced bulk data importer for all drives
//! Scans and imports data from D:\Arquivos, E:\Backup acer, E:\OneDrive

use pc_core::Result;
use pc_importers::{
    freight_importer::FreightOrderImporter,
    timesheet_importer::TimesheetImporter,
    route_importer::RouteImporter,
    photo_importer::PhotoImporter,
    Importer,
};
use pc_db::PersonalControllerDb;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚀 Personal Controller - ADVANCED BULK IMPORT");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📁 Scanning multiple drives for data...\n");

    // Connect to database
    println!("📊 Connecting to AvilaDB...");
    let db = PersonalControllerDb::connect("http://localhost:8000").await?;
    println!("✅ Connected!\n");

    // Define scan paths
    let scan_paths = vec![
        PathBuf::from("d:/Arquivos"),
        PathBuf::from("e:/Backup acer"),
        PathBuf::from("e:/OneDrive - Avila DevOps"),
        PathBuf::from("e:/BACKUP DELL - ARQUIVOS D"),
    ];

    let mut stats = ImportStats::default();

    // 1. Scan and import all CSV files
    println!("📦 Phase 1: Scanning for CSV files...");
    for base_path in &scan_paths {
        if !base_path.exists() {
            println!("  ⚠️  Skipping {:?} - not found", base_path);
            continue;
        }

        println!("  🔍 Scanning {:?}...", base_path);
        scan_and_import_csvs(&db, base_path, &mut stats).await?;
    }

    // 2. Scan and import all Excel files
    println!("\n📊 Phase 2: Scanning for Excel files (.xlsx, .xls)...");
    for base_path in &scan_paths {
        if !base_path.exists() {
            continue;
        }

        println!("  🔍 Scanning {:?}...", base_path);
        scan_and_import_excel(&db, base_path, &mut stats).await?;
    }

    // 3. Scan and import all images
    println!("\n📸 Phase 3: Scanning for images (jpg, png, pdf)...");
    for base_path in &scan_paths {
        if !base_path.exists() {
            continue;
        }

        println!("  🔍 Scanning {:?}...", base_path);
        scan_and_import_images(&db, base_path, &mut stats).await?;
    }

    // 4. Print comprehensive statistics
    print_statistics(&stats);

    // 5. Verify and analyze data
    println!("\n🔍 Verifying imported data...");
    verify_data(&db).await?;

    println!("\n✅ ADVANCED IMPORT COMPLETED SUCCESSFULLY!");

    Ok(())
}

#[derive(Default)]
struct ImportStats {
    freight_orders: usize,
    timesheets: usize,
    routes: usize,
    photos: usize,
    pdfs: usize,
    excel_files: usize,
    csv_files: usize,
    errors: Vec<String>,
    file_sources: HashMap<String, usize>,
}

async fn scan_and_import_csvs(
    db: &PersonalControllerDb,
    base_path: &Path,
    stats: &mut ImportStats,
) -> Result<()> {
    let freight_importer = FreightOrderImporter::new();
    let timesheet_importer = TimesheetImporter::new();
    let route_importer = RouteImporter::new();

    // Walk directory recursively
    if let Ok(entries) = walkdir::WalkDir::new(base_path)
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("csv"))
        .collect::<Vec<_>>()
    {
        for entry in entries {
            let path = entry.path();
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            // Try freight orders
            if filename.contains("-04") || filename.contains("Planilha") || filename.contains("PRE-EMBARQUE") {
                match freight_importer.import_file(path) {
                    Ok(orders) => {
                        for order in orders {
                            db.insert_freight_order(&order).await?;
                        }
                        stats.freight_orders += orders.len();
                        stats.csv_files += 1;
                        *stats.file_sources.entry(filename.to_string()).or_insert(0) += 1;
                        println!("    ✓ {} - {} orders", filename, orders.len());
                    }
                    Err(e) => {
                        stats.errors.push(format!("{}: {}", filename, e));
                    }
                }
            }
            // Try timesheets
            else if filename.contains("Horas") || filename.contains("Ponto") {
                match timesheet_importer.import_file(path) {
                    Ok(timesheets) => {
                        for timesheet in &timesheets {
                            db.insert_timesheet(timesheet).await?;
                        }
                        stats.timesheets += timesheets.len();
                        stats.csv_files += 1;
                        println!("    ✓ {} - {} entries", filename, timesheets.len());
                    }
                    Err(e) => {
                        stats.errors.push(format!("{}: {}", filename, e));
                    }
                }
            }
            // Try routes
            else if filename.contains("Rotas") {
                match route_importer.import_file(path) {
                    Ok(routes) => {
                        for route in &routes {
                            db.insert_route(route).await?;
                        }
                        stats.routes += routes.len();
                        stats.csv_files += 1;
                        println!("    ✓ {} - {} routes", filename, routes.len());
                    }
                    Err(e) => {
                        stats.errors.push(format!("{}: {}", filename, e));
                    }
                }
            }
        }
    }

    Ok(())
}

async fn scan_and_import_excel(
    db: &PersonalControllerDb,
    base_path: &Path,
    stats: &mut ImportStats,
) -> Result<()> {
    // Note: For Excel files, we would need a separate Excel importer
    // For now, just count them and note they need conversion

    if let Ok(entries) = walkdir::WalkDir::new(base_path)
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let ext = e.path().extension().and_then(|s| s.to_str());
            ext == Some("xlsx") || ext == Some("xls")
        })
        .collect::<Vec<_>>()
    {
        stats.excel_files += entries.len();
        if entries.len() > 0 {
            println!("    ℹ️  Found {} Excel files (need conversion to CSV)", entries.len());
        }
    }

    Ok(())
}

async fn scan_and_import_images(
    db: &PersonalControllerDb,
    base_path: &Path,
    stats: &mut ImportStats,
) -> Result<()> {
    let photo_importer = PhotoImporter::new();

    if let Ok(entries) = walkdir::WalkDir::new(base_path)
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let ext = e.path().extension().and_then(|s| s.to_str());
            ext == Some("jpg") || ext == Some("jpeg") || ext == Some("png")
        })
        .collect::<Vec<_>>()
    {
        for entry in entries.iter().take(1000) { // Limit to 1000 images
            let path = entry.path();

            match photo_importer.import_photo(path) {
                Ok(metadata) => {
                    let document = photo_importer.to_document(&metadata);
                    db.insert_document(&document).await?;
                    stats.photos += 1;
                }
                Err(e) => {
                    stats.errors.push(format!("{:?}: {}", path, e));
                }
            }
        }

        if entries.len() > 0 {
            println!("    ✓ Imported {} images (limited to 1000)", stats.photos);
        }
    }

    // Count PDFs separately
    if let Ok(entries) = walkdir::WalkDir::new(base_path)
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("pdf"))
        .collect::<Vec<_>>()
    {
        stats.pdfs += entries.len();
        if entries.len() > 0 {
            println!("    ℹ️  Found {} PDF files", entries.len());
        }
    }

    Ok(())
}

fn print_statistics(stats: &ImportStats) {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 IMPORT STATISTICS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  📦 Freight Orders:    {}", stats.freight_orders);
    println!("  ⏰ Timesheet Entries: {}", stats.timesheets);
    println!("  🛣️  Routes:            {}", stats.routes);
    println!("  📸 Photos:            {}", stats.photos);
    println!("  📄 PDFs:              {}", stats.pdfs);
    println!("  📊 Excel Files:       {}", stats.excel_files);
    println!("  📝 CSV Files:         {}", stats.csv_files);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if !stats.errors.is_empty() {
        println!("\n⚠️  ERRORS ({}):", stats.errors.len());
        for (i, error) in stats.errors.iter().take(10).enumerate() {
            println!("  {}. {}", i + 1, error);
        }
        if stats.errors.len() > 10 {
            println!("  ... and {} more", stats.errors.len() - 10);
        }
    }
}

async fn verify_data(db: &PersonalControllerDb) -> Result<()> {
    let companies = db.list_companies(0, 10).await?;
    println!("  ✓ Companies in database: {}", companies.len());

    let orders = db.list_freight_orders(0, 10).await?;
    println!("  ✓ Recent freight orders: {}", orders.len());

    let timesheets = db.list_timesheets(0, 10).await?;
    println!("  ✓ Recent timesheets: {}", timesheets.len());

    Ok(())
}
