# Direct CSV Import - No database needed
# Parses CSVs and creates structured JSON

Write-Host "🚀 Direct CSV Import to JSON" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

$outputDir = "d:\Personal-Controller\data\imported"
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

# Import freight orders from d:\Arquivos
$freightFiles = @("01-04.csv", "03-04.csv", "07-04.csv", "10-04.csv", "16-04.csv")
$allOrders = @()

Write-Host "📦 Importing freight orders..." -ForegroundColor Yellow

foreach ($file in $freightFiles) {
    $path = Join-Path "d:\Arquivos" $file
    if (Test-Path $path) {
        Write-Host "  Processing $file..." -ForegroundColor Gray

        try {
            $csv = Import-Csv -Path $path -Delimiter ';' -Encoding UTF8 -ErrorAction SilentlyContinue

            foreach ($row in $csv) {
                if ($row.'Número' -and $row.'Número' -match '^\d+$') {
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
                        frete_tabelado = $row.'Frete tabelado'
                        motorista_coleta = $row.'Motorista que Coleta'
                        motorista_entrega = $row.'Motorista'
                        filial_coleta = $row.'Filial que Coleta'
                        filial_entrega = $row.'Filial que entrega'
                        source_file = $file
                    }
                    $allOrders += $order
                }
            }

            Write-Host "    Imported $($csv.Count) orders" -ForegroundColor Green
        } catch {
            Write-Host "    Error: $_" -ForegroundColor Red
        }
    }
}

# Save to JSON
$ordersJson = $allOrders | ConvertTo-Json -Depth 10
$ordersJson | Out-File -FilePath (Join-Path $outputDir "freight_orders.json") -Encoding UTF8
Write-Host "  Saved $($allOrders.Count) orders to freight_orders.json" -ForegroundColor Green

# Import timesheets
Write-Host "`n⏰ Importing timesheets..." -ForegroundColor Yellow
$timesheetFiles = @("Horas.csv", "Horas abr.csv")
$allTimesheets = @()

foreach ($file in $timesheetFiles) {
    $path = Join-Path "d:\Arquivos" $file
    if (Test-Path $path) {
        Write-Host "  Processing $file..." -ForegroundColor Gray

        try {
            $csv = Import-Csv -Path $path -Delimiter ';' -Encoding UTF8 -ErrorAction SilentlyContinue
            $allTimesheets += $csv
            Write-Host "    Imported $($csv.Count) entries" -ForegroundColor Green
        } catch {
            Write-Host "    Error: $_" -ForegroundColor Red
        }
    }
}

if ($allTimesheets.Count -gt 0) {
    $timesheetsJson = $allTimesheets | ConvertTo-Json -Depth 10
    $timesheetsJson | Out-File -FilePath (Join-Path $outputDir "timesheets.json") -Encoding UTF8
    Write-Host "  Saved $($allTimesheets.Count) timesheet entries" -ForegroundColor Green
}

# Import routes
Write-Host "`n🛣️  Importing routes..." -ForegroundColor Yellow
$routesPath = "d:\Arquivos\Rotas.csv"

if (Test-Path $routesPath) {
    try {
        $routes = Import-Csv -Path $routesPath -Delimiter ';' -Encoding UTF8 -ErrorAction SilentlyContinue
        $routesJson = $routes | ConvertTo-Json -Depth 10
        $routesJson | Out-File -FilePath (Join-Path $outputDir "routes.json") -Encoding UTF8
        Write-Host "  Saved $($routes.Count) routes" -ForegroundColor Green
    } catch {
        Write-Host "  Error: $_" -ForegroundColor Red
    }
}

# Create summary
Write-Host ""
Write-Host "IMPORT COMPLETE" -ForegroundColor Green
Write-Host ""
Write-Host "Data saved to: $outputDir" -ForegroundColor Cyan
Write-Host "  Freight Orders: $($allOrders.Count)" -ForegroundColor White
Write-Host "  Timesheets: $($allTimesheets.Count)" -ForegroundColor White
Write-Host ""

# Analyze companies
$uniqueCompanies = $allOrders | Select-Object -ExpandProperty pagador_nome -Unique | Where-Object { $_ }
Write-Host "Analysis:" -ForegroundColor Cyan
Write-Host "  Unique Companies: $($uniqueCompanies.Count)" -ForegroundColor White

$topCompanies = $allOrders | Group-Object pagador_nome | Sort-Object Count -Descending | Select-Object -First 10
Write-Host ""
Write-Host "  Top 10 Companies by Orders:" -ForegroundColor Cyan
foreach ($company in $topCompanies) {
    if ($company.Name) {
        Write-Host "    $($company.Name): $($company.Count) orders" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "Data ready for use" -ForegroundColor Green
