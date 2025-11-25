# Quick Data Extraction Script
# Extracts metadata from all files without full import

Write-Host "🔍 Personal Controller - Quick Data Extraction" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

$drives = @("d:\Arquivos", "e:\Backup acer", "e:\OneDrive - Avila DevOps", "e:\BACKUP DELL - ARQUIVOS D")
$results = @{}

Write-Host "📊 Scanning drives..." -ForegroundColor Yellow
Write-Host ""

foreach ($drive in $drives) {
    if (Test-Path $drive) {
        Write-Host "  🔍 Scanning $drive..." -ForegroundColor White

        $csvFiles = @(Get-ChildItem -Path $drive -Filter "*.csv" -Recurse -ErrorAction SilentlyContinue)
        $xlsxFiles = @(Get-ChildItem -Path $drive -Filter "*.xlsx" -Recurse -ErrorAction SilentlyContinue)
        $xlsFiles = @(Get-ChildItem -Path $drive -Filter "*.xls" -Recurse -ErrorAction SilentlyContinue)
        $jpgFiles = @(Get-ChildItem -Path $drive -Filter "*.jpg" -Recurse -ErrorAction SilentlyContinue)
        $pngFiles = @(Get-ChildItem -Path $drive -Filter "*.png" -Recurse -ErrorAction SilentlyContinue)
        $pdfFiles = @(Get-ChildItem -Path $drive -Filter "*.pdf" -Recurse -ErrorAction SilentlyContinue)

        $results[$drive] = @{
            CSV = $csvFiles.Count
            XLSX = $xlsxFiles.Count
            XLS = $xlsFiles.Count
            JPG = $jpgFiles.Count
            PNG = $pngFiles.Count
            PDF = $pdfFiles.Count
            TotalSize = (($csvFiles + $xlsxFiles + $xlsFiles + $jpgFiles + $pngFiles + $pdfFiles) | Measure-Object -Property Length -Sum).Sum
        }

        Write-Host "    CSV: $($csvFiles.Count) | Excel: $($xlsxFiles.Count + $xlsFiles.Count) | Images: $($jpgFiles.Count + $pngFiles.Count) | PDF: $($pdfFiles.Count)" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host "📊 EXTRACTION COMPLETE" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host ""

$totalCSV = ($results.Values | ForEach-Object { $_.CSV } | Measure-Object -Sum).Sum
$totalXLSX = ($results.Values | ForEach-Object { $_.XLSX } | Measure-Object -Sum).Sum
$totalXLS = ($results.Values | ForEach-Object { $_.XLS } | Measure-Object -Sum).Sum
$totalJPG = ($results.Values | ForEach-Object { $_.JPG } | Measure-Object -Sum).Sum
$totalPNG = ($results.Values | ForEach-Object { $_.PNG } | Measure-Object -Sum).Sum
$totalPDF = ($results.Values | ForEach-Object { $_.PDF } | Measure-Object -Sum).Sum
$totalSize = ($results.Values | ForEach-Object { $_.TotalSize } | Measure-Object -Sum).Sum

Write-Host "Total Files Found:" -ForegroundColor Cyan
Write-Host "  CSV Files:    $totalCSV" -ForegroundColor White
Write-Host "  XLSX Files:   $totalXLSX" -ForegroundColor White
Write-Host "  XLS Files:    $totalXLS" -ForegroundColor White
Write-Host "  JPG Images:   $totalJPG" -ForegroundColor White
Write-Host "  PNG Images:   $totalPNG" -ForegroundColor White
Write-Host "  PDF Files:    $totalPDF" -ForegroundColor White
Write-Host "  Total Size:   $([math]::Round($totalSize / 1GB, 2)) GB" -ForegroundColor White
Write-Host ""

# Export specific freight CSVs from d:\Arquivos
Write-Host "📦 Extracting freight order CSVs..." -ForegroundColor Yellow
$freightFiles = @("01-04.csv", "03-04.csv", "07-04.csv", "10-04.csv", "16-04.csv", "Planilha Geral2 certo.csv")
$freightData = @()

foreach ($file in $freightFiles) {
    $path = Join-Path "d:\Arquivos" $file
    if (Test-Path $path) {
        $content = Get-Content $path -Encoding UTF8 -TotalCount 5
        $lineCount = (Get-Content $path -Encoding UTF8 | Measure-Object -Line).Lines
        Write-Host "  ✓ $file - $lineCount lines" -ForegroundColor Green

        $freightData += [PSCustomObject]@{
            File = $file
            Path = $path
            Lines = $lineCount
            Size = (Get-Item $path).Length
        }
    }
}

# Save extraction summary
$summary = @"
# Data Extraction Summary
Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Total Files
- CSV Files: $totalCSV
- XLSX Files: $totalXLSX
- XLS Files: $totalXLS
- JPG Images: $totalJPG
- PNG Images: $totalPNG
- PDF Files: $totalPDF
- Total Size: $([math]::Round($totalSize / 1GB, 2)) GB

## Breakdown by Drive
$($results.Keys | ForEach-Object { "### $_`n- CSV: $($results[$_].CSV)`n- Excel: $($results[$_].XLSX + $results[$_].XLS)`n- Images: $($results[$_].JPG + $results[$_].PNG)`n- PDFs: $($results[$_].PDF)`n- Size: $([math]::Round($results[$_].TotalSize / 1MB, 2)) MB`n" })

## Freight Order Files (d:\Arquivos)
$($freightData | ForEach-Object { "- $($_.File): $($_.Lines) lines, $([math]::Round($_.Size / 1KB, 2)) KB" } | Out-String)
"@

$summary | Out-File -FilePath "d:\Personal-Controller\DATA_EXTRACTION_SUMMARY.md" -Encoding UTF8

Write-Host "✅ Summary saved to DATA_EXTRACTION_SUMMARY.md" -ForegroundColor Green
Write-Host ""

# Sample first CSV for structure
Write-Host "📄 Sample data from 01-04.csv:" -ForegroundColor Cyan
$samplePath = "d:\Arquivos\01-04.csv"
if (Test-Path $samplePath) {
    Get-Content $samplePath -Encoding UTF8 -TotalCount 3 | ForEach-Object {
        Write-Host "  $_" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "✅ Extraction complete! Ready for import." -ForegroundColor Green
