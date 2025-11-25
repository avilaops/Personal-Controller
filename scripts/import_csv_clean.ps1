# Direct CSV to JSON Import
# Simple script without Unicode issues

param([switch]$Verbose)

Write-Host "CSV to JSON Import Starting..." -ForegroundColor Cyan

$outputDir = "d:\Personal-Controller\data\imported"
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

# Freight orders
$freightFiles = @("01-04.csv", "03-04.csv", "07-04.csv", "10-04.csv", "16-04.csv")
$allOrders = @()

Write-Host "Importing freight orders..." -ForegroundColor Yellow

foreach ($file in $freightFiles) {
    $path = Join-Path "d:\Arquivos" $file
    if (Test-Path $path) {
        Write-Host "  Processing $file..." -ForegroundColor Gray

        try {
            $csv = Import-Csv -Path $path -Delimiter ';' -Encoding UTF8 -ErrorAction SilentlyContinue

            foreach ($row in $csv) {
                if ($row.PSObject.Properties['Número'] -and $row.'Número' -match '^\d+$') {
                    $order = [PSCustomObject]@{
                        numero = $row.'Número'
                        data_agendamento = $row.'Data de Agendamento'
                        data_emissao = $row.'Data de Emissão'
                        notas_fiscais = $row.'Notas Fiscais'
                        pagador_nome = $row.'Pagador do Frete - Nome'
                        pagador_fone = $row.'Pagador do Frete - Fone'
                        remetente_nome = $row.'Remetente - Nome'
                        remetente_cidade = $row.'Remetente - Cidade'
                        destinatario_nome = $row.'Destinatário - Nome'
                        destinatario_cidade = $row.'Destinatário - Cidade '
                        volumes = $row.'Soma dos Volumes'
                        peso = $row.'Soma dos Pesos'
                        valor_notas = $row.'Soma das Notas'
                        valor_frete = $row.'Valor do Frete'
                        source_file = $file
                    }
                    $allOrders += $order
                }
            }

            Write-Host "    Imported: $($csv.Count) rows" -ForegroundColor Green
        } catch {
            Write-Host "    Error: $_" -ForegroundColor Red
        }
    } else {
        Write-Host "  Not found: $file" -ForegroundColor Yellow
    }
}

# Save JSON
$ordersJson = $allOrders | ConvertTo-Json -Depth 10
$ordersJson | Out-File -FilePath (Join-Path $outputDir "freight_orders.json") -Encoding UTF8
Write-Host "Saved $($allOrders.Count) freight orders" -ForegroundColor Green

# Timesheets
Write-Host "`nImporting timesheets..." -ForegroundColor Yellow
$timesheetFiles = @("Horas.csv", "Horas abr.csv")
$allTimesheets = @()

foreach ($file in $timesheetFiles) {
    $path = Join-Path "d:\Arquivos" $file
    if (Test-Path $path) {
        try {
            $csv = Import-Csv -Path $path -Delimiter ';' -Encoding UTF8 -ErrorAction SilentlyContinue
            $allTimesheets += $csv
            Write-Host "  Imported: $($csv.Count) entries from $file" -ForegroundColor Green
        } catch {
            Write-Host "  Error in $file" -ForegroundColor Red
        }
    }
}

if ($allTimesheets.Count -gt 0) {
    $timesheetsJson = $allTimesheets | ConvertTo-Json -Depth 10
    $timesheetsJson | Out-File -FilePath (Join-Path $outputDir "timesheets.json") -Encoding UTF8
    Write-Host "Saved $($allTimesheets.Count) timesheet entries" -ForegroundColor Green
}

# Routes
Write-Host "`nImporting routes..." -ForegroundColor Yellow
$routesPath = "d:\Arquivos\Rotas.csv"

if (Test-Path $routesPath) {
    try {
        $routes = Import-Csv -Path $routesPath -Delimiter ';' -Encoding UTF8
        $routesJson = $routes | ConvertTo-Json -Depth 10
        $routesJson | Out-File -FilePath (Join-Path $outputDir "routes.json") -Encoding UTF8
        Write-Host "Saved $($routes.Count) routes" -ForegroundColor Green
    } catch {
        Write-Host "Error importing routes" -ForegroundColor Red
    }
}

# Summary
Write-Host "`nIMPORT COMPLETE" -ForegroundColor Green
Write-Host "Output: $outputDir" -ForegroundColor Cyan
Write-Host "  Freight Orders: $($allOrders.Count)" -ForegroundColor White
Write-Host "  Timesheets: $($allTimesheets.Count)" -ForegroundColor White

# Analysis
if ($Verbose) {
    $uniqueCompanies = $allOrders | Select-Object -ExpandProperty pagador_nome -Unique | Where-Object { $_ }
    Write-Host "`nUnique Companies: $($uniqueCompanies.Count)" -ForegroundColor Cyan

    $topCompanies = $allOrders | Group-Object pagador_nome | Sort-Object Count -Descending | Select-Object -First 10
    Write-Host "Top 10 Companies:" -ForegroundColor Cyan
    foreach ($company in $topCompanies) {
        if ($company.Name) {
            Write-Host "  $($company.Name): $($company.Count)" -ForegroundColor Gray
        }
    }
}

Write-Host "`nDone!" -ForegroundColor Green
