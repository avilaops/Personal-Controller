# Advanced Bulk Import Script
# Scans E: drive and imports ALL data (CSVs, Excel, Photos, PDFs)

Write-Host "🚀 Personal Controller - ADVANCED BULK IMPORT" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

# Scan summary first
Write-Host "📁 SCANNING ALL DRIVES..." -ForegroundColor Yellow
Write-Host ""

$drives = @("d:\Arquivos", "e:\Backup acer", "e:\OneDrive - Avila DevOps", "e:\BACKUP DELL - ARQUIVOS D")
$totalFiles = @{}

foreach ($drive in $drives) {
    if (Test-Path $drive) {
        Write-Host "  🔍 Scanning $drive..." -ForegroundColor Gray

        # Count CSVs
        $csvCount = (Get-ChildItem -Path $drive -Filter "*.csv" -Recurse -ErrorAction SilentlyContinue | Measure-Object).Count
        $totalFiles["CSV"] = ($totalFiles["CSV"] ?? 0) + $csvCount

        # Count Excel
        $xlsxCount = (Get-ChildItem -Path $drive -Filter "*.xlsx" -Recurse -ErrorAction SilentlyContinue | Measure-Object).Count
        $xlsCount = (Get-ChildItem -Path $drive -Filter "*.xls" -Recurse -ErrorAction SilentlyContinue | Measure-Object).Count
        $totalFiles["Excel"] = ($totalFiles["Excel"] ?? 0) + $xlsxCount + $xlsCount

        # Count Images
        $jpgCount = (Get-ChildItem -Path $drive -Filter "*.jpg" -Recurse -ErrorAction SilentlyContinue | Measure-Object).Count
        $pngCount = (Get-ChildItem -Path $drive -Filter "*.png" -Recurse -ErrorAction SilentlyContinue | Measure-Object).Count
        $totalFiles["Images"] = ($totalFiles["Images"] ?? 0) + $jpgCount + $pngCount

        # Count PDFs
        $pdfCount = (Get-ChildItem -Path $drive -Filter "*.pdf" -Recurse -ErrorAction SilentlyContinue | Measure-Object).Count
        $totalFiles["PDFs"] = ($totalFiles["PDFs"] ?? 0) + $pdfCount

        Write-Host "    CSV: $csvCount | Excel: $($xlsxCount + $xlsCount) | Images: $($jpgCount + $pngCount) | PDFs: $pdfCount" -ForegroundColor White
    } else {
        Write-Host "  ⚠️  $drive not found" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "📊 TOTAL FILES FOUND:" -ForegroundColor Cyan
Write-Host "  CSV Files:    $($totalFiles['CSV'])" -ForegroundColor White
Write-Host "  Excel Files:  $($totalFiles['Excel'])" -ForegroundColor White
Write-Host "  Images:       $($totalFiles['Images'])" -ForegroundColor White
Write-Host "  PDFs:         $($totalFiles['PDFs'])" -ForegroundColor White
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

# Ask for confirmation
$confirm = Read-Host "Do you want to proceed with import? (y/n)"
if ($confirm -ne "y") {
    Write-Host "❌ Import cancelled" -ForegroundColor Red
    exit 0
}

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

# Add walkdir dependency check
Write-Host "📦 Checking dependencies..." -ForegroundColor Yellow
$cargoToml = Get-Content "d:\Personal-Controller\Cargo.toml" -Raw
if ($cargoToml -notlike "*walkdir*") {
    Write-Host "  Adding walkdir dependency..." -ForegroundColor Gray
    Add-Content -Path "d:\Personal-Controller\Cargo.toml" -Value "`nwalkdir = `"2.4`""
}

# Build the advanced importer
Write-Host "🔨 Building advanced importer..." -ForegroundColor Yellow
Set-Location "d:\Personal-Controller"

$buildResult = cargo build --example advanced_bulk_import --release 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed" -ForegroundColor Red
    Write-Host $buildResult
    exit 1
}

Write-Host "✅ Build successful" -ForegroundColor Green
Write-Host ""

# Run the import
Write-Host "📥 Starting advanced import process..." -ForegroundColor Cyan
Write-Host "   This may take several minutes..." -ForegroundColor Gray
Write-Host ""

$importResult = cargo run --example advanced_bulk_import --release 2>&1
$importOutput = $importResult | Out-String

if ($LASTEXITCODE -eq 0) {
    Write-Host $importOutput
    Write-Host ""
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
    Write-Host "✅ ADVANCED IMPORT COMPLETED!" -ForegroundColor Green
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
    Write-Host ""
} else {
    Write-Host "❌ Import failed" -ForegroundColor Red
    Write-Host $importOutput
    exit 1
}

# Create import summary file
$summary = @"
# Import Summary - $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Files Scanned
- CSV Files: $($totalFiles['CSV'])
- Excel Files: $($totalFiles['Excel'])
- Images: $($totalFiles['Images'])
- PDFs: $($totalFiles['PDFs'])

## Drives Scanned
$($drives -join "`n")

## Status
Import completed successfully at $(Get-Date -Format "HH:mm:ss")
"@

$summary | Out-File -FilePath "d:\Personal-Controller\IMPORT_SUMMARY.md" -Encoding UTF8

Write-Host "📄 Summary saved to IMPORT_SUMMARY.md" -ForegroundColor Cyan
Write-Host ""
Write-Host "✅ All done!" -ForegroundColor Green
