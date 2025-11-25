# Complete data import scripto 
# Imports all CSV files and photos to AvilaDB

Write-Host "🚀 Personal Controller - Complete Data Import" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

# Check if AvilaDB is running
Write-Host "📊 Checking AvilaDB..." -ForegroundColor Yellow
$aviladbRunning = $false
try {
    $response = Invoke-WebRequest -Uri "http://localhost:8000/health" -Method GET -TimeoutSec 2 -ErrorAction Stop
    if ($response.StatusCode -eq 200) {
        $aviladbRunning = $true
        Write-Host "✅ AvilaDB is running" -ForegroundColor Green
    }
} catch {
    Write-Host "❌ AvilaDB is not running" -ForegroundColor Red
    Write-Host "   Please start AvilaDB first:" -ForegroundColor Yellow
    Write-Host "   cd d:\arxis\aviladb" -ForegroundColor Gray
    Write-Host "   cargo run --release" -ForegroundColor Gray
    exit 1
}

Write-Host ""

# Check if API is running (optional)
Write-Host "🔌 Checking API server..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:3000/health" -Method GET -TimeoutSec 2 -ErrorAction Stop
    if ($response.StatusCode -eq 200) {
        Write-Host "✅ API server is running" -ForegroundColor Green
    }
} catch {
    Write-Host "⚠️  API server is not running (optional)" -ForegroundColor Yellow
}

Write-Host ""

# Build the importer
Write-Host "🔨 Building importer..." -ForegroundColor Yellow
Set-Location "d:\Personal-Controller"

$buildResult = cargo build --example complete_import --release 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed" -ForegroundColor Red
    Write-Host $buildResult
    exit 1
}

Write-Host "✅ Build successful" -ForegroundColor Green
Write-Host ""

# Run the import
Write-Host "📥 Starting import process..." -ForegroundColor Cyan
Write-Host ""

$importResult = cargo run --example complete_import --release 2>&1
$importOutput = $importResult | Out-String

if ($LASTEXITCODE -eq 0) {
    Write-Host $importOutput
    Write-Host ""
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
    Write-Host "✅ IMPORT COMPLETED SUCCESSFULLY!" -ForegroundColor Green
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
    Write-Host ""
    Write-Host "You can now:" -ForegroundColor Cyan
    Write-Host "  1. Access the API at http://localhost:3000" -ForegroundColor Gray
    Write-Host "  2. Access the web interface at http://localhost:3001" -ForegroundColor Gray
    Write-Host "  3. Query data using the CLI: cargo run --bin pc-cli" -ForegroundColor Gray
    Write-Host ""
} else {
    Write-Host "❌ Import failed" -ForegroundColor Red
    Write-Host $importOutput
    exit 1
}

# Show statistics
Write-Host "📊 Database Statistics:" -ForegroundColor Cyan
Write-Host ""

# Query statistics using the API if available
try {
    $stats = Invoke-RestMethod -Uri "http://localhost:3000/api/v1/stats" -Method GET -TimeoutSec 5 -ErrorAction Stop

    Write-Host "  Companies: $($stats.companies)" -ForegroundColor White
    Write-Host "  Freight Orders: $($stats.freight_orders)" -ForegroundColor White
    Write-Host "  Timesheets: $($stats.timesheets)" -ForegroundColor White
    Write-Host "  Routes: $($stats.routes)" -ForegroundColor White
    Write-Host "  Documents: $($stats.documents)" -ForegroundColor White
    Write-Host ""
} catch {
    Write-Host "  (API not available for statistics)" -ForegroundColor Gray
    Write-Host ""
}

Write-Host "✅ All done!" -ForegroundColor Green
