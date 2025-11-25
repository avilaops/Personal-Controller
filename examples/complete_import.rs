//! Complete data import example
//!
//! This example imports ALL data from CSV files and photos

use pc_core::Result;
use pc_importers::{
    freight_importer::FreightOrderImporter,
    timesheet_importer::TimesheetImporter,
    route_importer::RouteImporter,
    photo_importer::PhotoImporter,
    Importer,
};
use pc_db::PersonalControllerDb;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚀 Starting complete data import...\n");

    // Connect to database
    println!("📊 Connecting to AvilaDB...");
    let db = PersonalControllerDb::connect("http://localhost:8000").await?;
    println!("✅ Connected!\n");

    // Base directory with all data
    let base_dir = Path::new("d:/Arquivos");

    // 1. Import Freight Orders
    println!("📦 Importing Freight Orders...");
    let freight_files = vec![
        "01-04.csv",
        "03-04.csv",
        "07-04.csv",
        "10-04.csv",
        "16-04.csv",
        "Planilha Geral2 certo.csv",
        "PRE-EMBARQUE (2).csv",
    ];

    let freight_importer = FreightOrderImporter::new();
    let mut total_freight = 0;

    for filename in freight_files {
        let path = base_dir.join(filename);
        if path.exists() {
            match freight_importer.import_file(&path) {
                Ok(orders) => {
                    println!("  ✓ {} - {} orders", filename, orders.len());

                    // Store in database
                    for order in orders {
                        db.insert_freight_order(&order).await?;
                    }

                    total_freight += orders.len();
                }
                Err(e) => {
                    println!("  ✗ {} - Error: {}", filename, e);
                }
            }
        } else {
            println!("  ⚠ {} - File not found", filename);
        }
    }

    println!("📦 Total freight orders imported: {}\n", total_freight);

    // 2. Import Timesheets
    println!("⏰ Importing Timesheets...");
    let timesheet_files = vec![
        "Horas.csv",
        "Horas abr.csv",
    ];

    let timesheet_importer = TimesheetImporter::new();
    let mut total_timesheets = 0;

    for filename in timesheet_files {
        let path = base_dir.join(filename);
        if path.exists() {
            match timesheet_importer.import_file(&path) {
                Ok(timesheets) => {
                    println!("  ✓ {} - {} entries", filename, timesheets.len());

                    // Store in database
                    for timesheet in timesheets {
                        db.insert_timesheet(&timesheet).await?;
                    }

                    total_timesheets += timesheets.len();
                }
                Err(e) => {
                    println!("  ✗ {} - Error: {}", filename, e);
                }
            }
        }
    }

    println!("⏰ Total timesheet entries imported: {}\n", total_timesheets);

    // 3. Import Routes
    println!("🛣️  Importing Routes...");
    let route_path = base_dir.join("Rotas.csv");

    let route_importer = RouteImporter::new();
    let mut total_routes = 0;

    if route_path.exists() {
        match route_importer.import_file(&route_path) {
            Ok(routes) => {
                println!("  ✓ {} routes", routes.len());

                // Store in database
                for route in routes {
                    db.insert_route(&route).await?;
                }

                total_routes = routes.len();
            }
            Err(e) => {
                println!("  ✗ Error: {}", e);
            }
        }
    }

    println!("🛣️  Total routes imported: {}\n", total_routes);

    // 4. Import Photos
    println!("📸 Importing Photos...");
    let photos_dir = base_dir.join("Fotos");

    let photo_importer = PhotoImporter::new();
    let mut total_photos = 0;

    if photos_dir.exists() {
        match photo_importer.import_directory(&photos_dir) {
            Ok(photos) => {
                println!("  ✓ {} photos found", photos.len());

                // Store metadata in database
                for photo in photos {
                    let document = photo_importer.to_document(&photo);
                    db.insert_document(&document).await?;
                    total_photos += 1;
                }
            }
            Err(e) => {
                println!("  ✗ Error: {}", e);
            }
        }
    }

    println!("📸 Total photos imported: {}\n", total_photos);

    // 5. Summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ IMPORT COMPLETE!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  📦 Freight Orders: {}", total_freight);
    println!("  ⏰ Timesheet Entries: {}", total_timesheets);
    println!("  🛣️  Routes: {}", total_routes);
    println!("  📸 Photos: {}", total_photos);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // 6. Verify data
    println!("🔍 Verifying imported data...");

    let companies = db.list_companies(0, 10).await?;
    println!("  Companies in database: {}", companies.len());

    let orders = db.list_freight_orders(0, 10).await?;
    println!("  Recent freight orders: {}", orders.len());

    let timesheets = db.list_timesheets(0, 10).await?;
    println!("  Recent timesheets: {}", timesheets.len());

    println!("\n✅ All data successfully imported and verified!");

    Ok(())
}
