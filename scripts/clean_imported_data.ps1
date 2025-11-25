# Clean and validate imported JSON data
# Removes empty records and fixes encoding issues

param(
    [string]$InputDir = "d:\Personal-Controller\data\imported",
    [string]$OutputDir = "d:\Personal-Controller\data\cleaned"
)

$ErrorActionPreference = "Stop"

# Create output directory
New-Item -Path $OutputDir -ItemType Directory -Force | Out-Null

Write-Host "=== LIMPANDO DADOS IMPORTADOS ===" -ForegroundColor Cyan

# Function to check if a record is empty/invalid
function Test-ValidRecord {
    param($Record)

    # Check if essential fields are filled
    $hasNumber = ![string]::IsNullOrWhiteSpace($Record.'Número')
    $hasDate = ![string]::IsNullOrWhiteSpace($Record.'Data de Agendamento') -or
               ![string]::IsNullOrWhiteSpace($Record.'Data de Emissão')
    $hasCompany = ![string]::IsNullOrWhiteSpace($Record.'Pagador do Frete - Nome') -or
                  ![string]::IsNullOrWhiteSpace($Record.'Remetente - Nome')

    return ($hasNumber -or $hasDate -or $hasCompany)
}

# Clean freight orders
Write-Host "`n[1/3] Limpando freight_orders..." -ForegroundColor Yellow
$rawOrders = Get-Content "$InputDir\freight_orders_raw.json" -Raw | ConvertFrom-Json
$validOrders = $rawOrders | Where-Object { Test-ValidRecord $_ }

Write-Host "  Total registros: $($rawOrders.Count)" -ForegroundColor Gray
Write-Host "  Registros válidos: $($validOrders.Count)" -ForegroundColor Green
Write-Host "  Removidos: $($rawOrders.Count - $validOrders.Count)" -ForegroundColor Red

$validOrders | ConvertTo-Json -Depth 10 | Out-File "$OutputDir\freight_orders_cleaned.json" -Encoding UTF8

# Clean timesheets
Write-Host "`n[2/3] Limpando timesheets..." -ForegroundColor Yellow
$rawTimesheets = Get-Content "$InputDir\timesheets_raw.json" -Raw | ConvertFrom-Json
$validTimesheets = $rawTimesheets | Where-Object {
    ![string]::IsNullOrWhiteSpace($_.'Data') -or
    ![string]::IsNullOrWhiteSpace($_.'Motorista')
}

Write-Host "  Total registros: $($rawTimesheets.Count)" -ForegroundColor Gray
Write-Host "  Registros válidos: $($validTimesheets.Count)" -ForegroundColor Green
Write-Host "  Removidos: $($rawTimesheets.Count - $validTimesheets.Count)" -ForegroundColor Red

$validTimesheets | ConvertTo-Json -Depth 10 | Out-File "$OutputDir\timesheets_cleaned.json" -Encoding UTF8

# Clean routes
Write-Host "`n[3/3] Limpando routes..." -ForegroundColor Yellow
$rawRoutes = Get-Content "$InputDir\routes_raw.json" -Raw | ConvertFrom-Json
$validRoutes = $rawRoutes | Where-Object {
    ![string]::IsNullOrWhiteSpace($_.'Origem') -and
    ![string]::IsNullOrWhiteSpace($_.'Destino')
}

Write-Host "  Total registros: $($rawRoutes.Count)" -ForegroundColor Gray
Write-Host "  Registros válidos: $($validRoutes.Count)" -ForegroundColor Green
Write-Host "  Removidos: $($rawRoutes.Count - $validRoutes.Count)" -ForegroundColor Red

$validRoutes | ConvertTo-Json -Depth 10 | Out-File "$OutputDir\routes_cleaned.json" -Encoding UTF8

# Generate summary report
Write-Host "`n=== RESUMO DA LIMPEZA ===" -ForegroundColor Cyan
$summary = @{
    timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    freight_orders = @{
        raw = $rawOrders.Count
        cleaned = $validOrders.Count
        removed = $rawOrders.Count - $validOrders.Count
    }
    timesheets = @{
        raw = $rawTimesheets.Count
        cleaned = $validTimesheets.Count
        removed = $rawTimesheets.Count - $validTimesheets.Count
    }
    routes = @{
        raw = $rawRoutes.Count
        cleaned = $validRoutes.Count
        removed = $rawRoutes.Count - $validRoutes.Count
    }
}

$summary | ConvertTo-Json -Depth 5 | Out-File "$OutputDir\cleaning_summary.json" -Encoding UTF8
Write-Host "`nSalvo em: $OutputDir" -ForegroundColor Green
$summary | ConvertTo-Json -Depth 5
