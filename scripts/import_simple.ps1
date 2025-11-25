# Simple Import - No validation
Write-Host "Simple Import - Importing ALL rows..." -ForegroundColor Cyan

$outputDir = "d:\Personal-Controller\data\imported"
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

# Freight - Import everything
$freightFiles = @("01-04.csv", "03-04.csv", "07-04.csv", "10-04.csv", "16-04.csv")
$allOrders = @()

Write-Host "Freight orders..." -ForegroundColor Yellow

foreach ($file in $freightFiles) {
    $path = Join-Path "d:\Arquivos" $file
    if (Test-Path $path) {
        $content = Get-Content -Path $path -Encoding UTF8 | Select-Object -Skip 1
        $csv = $content | ConvertFrom-Csv -Delimiter ';'
        $allOrders += $csv
        Write-Host "  $file : $($csv.Count) rows" -ForegroundColor Green
    }
}

$ordersJson = $allOrders | ConvertTo-Json -Depth 10
$ordersJson | Out-File -FilePath (Join-Path $outputDir "freight_orders_raw.json") -Encoding UTF8
Write-Host "SAVED: $($allOrders.Count) freight orders" -ForegroundColor Green

# Timesheets
Write-Host "`nTimesheets..." -ForegroundColor Yellow
$allTimesheets = @()

$horasFiles = @("Horas.csv", "Horas abr.csv")
foreach ($file in $horasFiles) {
    $path = Join-Path "d:\Arquivos" $file
    if (Test-Path $path) {
        $content = Get-Content -Path $path -Encoding UTF8 | Select-Object -Skip 1
        $csv = $content | ConvertFrom-Csv -Delimiter ';'
        $allTimesheets += $csv
        Write-Host "  $file : $($csv.Count)" -ForegroundColor Green
    }
}

$timesheetsJson = $allTimesheets | ConvertTo-Json -Depth 10
$timesheetsJson | Out-File -FilePath (Join-Path $outputDir "timesheets_raw.json") -Encoding UTF8
Write-Host "SAVED: $($allTimesheets.Count) timesheets" -ForegroundColor Green

# Routes
Write-Host "`nRoutes..." -ForegroundColor Yellow
$routesPath = "d:\Arquivos\Rotas.csv"
if (Test-Path $routesPath) {
    $content = Get-Content -Path $routesPath -Encoding UTF8 | Select-Object -Skip 1
    $routes = $content | ConvertFrom-Csv -Delimiter ';'
    $routesJson = $routes | ConvertTo-Json -Depth 10
    $routesJson | Out-File -FilePath (Join-Path $outputDir "routes_raw.json") -Encoding UTF8
    Write-Host "SAVED: $($routes.Count) routes" -ForegroundColor Green
}

Write-Host "`n======================================" -ForegroundColor Green
Write-Host "SUCCESS - All data imported!" -ForegroundColor Green
Write-Host "======================================" -ForegroundColor Green
Write-Host "Location: $outputDir" -ForegroundColor Cyan
Write-Host "  freight_orders_raw.json: $($allOrders.Count) records" -ForegroundColor White
Write-Host "  timesheets_raw.json: $($allTimesheets.Count) records" -ForegroundColor White
Write-Host ""
Write-Host "Done!" -ForegroundColor Green
