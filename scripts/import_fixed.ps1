# Fixed CSV Import - Skip header line
param([switch]$Verbose)

Write-Host "Fixed CSV Import Starting..." -ForegroundColor Cyan

$outputDir = "d:\Personal-Controller\data\imported"
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

# Freight orders - SKIP FIRST LINE
$freightFiles = @("01-04.csv", "03-04.csv", "07-04.csv", "10-04.csv", "16-04.csv")
$allOrders = @()

Write-Host "Importing freight orders..." -ForegroundColor Yellow

foreach ($file in $freightFiles) {
    $path = Join-Path "d:\Arquivos" $file
    if (Test-Path $path) {
        Write-Host "  Processing $file..." -ForegroundColor Gray

        try {
            # Skip first line and import
            $content = Get-Content -Path $path -Encoding UTF8 | Select-Object -Skip 1
            $csv = $content | ConvertFrom-Csv -Delimiter ';'

            $count = 0
            foreach ($row in $csv) {
                # Try both encodings
                $numero = $null
                if ($row.PSObject.Properties['Número']) {
                    $numero = $row.'Número'
                } elseif ($row.PSObject.Properties['N�mero']) {
                    $numero = $row.'N�mero'
                }

                # Trim and check
                if ($numero) {
                    $numero = $numero.Trim()
                }

                if ($numero -and $numero -match '^\d+$') {
                    $order = [PSCustomObject]@{
                        numero = $numero
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
                        motorista_coleta = $row.'Motorista que Coleta'
                        motorista_entrega = $row.'Motorista'
                        filial_coleta = $row.'Filial que Coleta'
                        filial_entrega = $row.'Filial que entrega'
                        source_file = $file
                    }
                    $allOrders += $order
                    $count++
                }
            }

            Write-Host "    SUCCESS: $count orders" -ForegroundColor Green
        } catch {
            Write-Host "    ERROR: $_" -ForegroundColor Red
        }
    }
}

# Save
$ordersJson = $allOrders | ConvertTo-Json -Depth 10
$ordersJson | Out-File -FilePath (Join-Path $outputDir "freight_orders.json") -Encoding UTF8
Write-Host "Saved $($allOrders.Count) freight orders to JSON" -ForegroundColor Green

# Timesheets
Write-Host "`nImporting timesheets..." -ForegroundColor Yellow
$allTimesheets = @()

$horasPath = "d:\Arquivos\Horas.csv"
if (Test-Path $horasPath) {
    $content = Get-Content -Path $horasPath -Encoding UTF8 | Select-Object -Skip 1
    $csv = $content | ConvertFrom-Csv -Delimiter ';'
    $allTimesheets += $csv
    Write-Host "  Horas.csv: $($csv.Count) entries" -ForegroundColor Green
}

$horasAbrPath = "d:\Arquivos\Horas abr.csv"
if (Test-Path $horasAbrPath) {
    $content = Get-Content -Path $horasAbrPath -Encoding UTF8 | Select-Object -Skip 1
    $csv = $content | ConvertFrom-Csv -Delimiter ';'
    $allTimesheets += $csv
    Write-Host "  Horas abr.csv: $($csv.Count) entries" -ForegroundColor Green
}

if ($allTimesheets.Count -gt 0) {
    $timesheetsJson = $allTimesheets | ConvertTo-Json -Depth 10
    $timesheetsJson | Out-File -FilePath (Join-Path $outputDir "timesheets.json") -Encoding UTF8
}

# Routes
Write-Host "`nImporting routes..." -ForegroundColor Yellow
$routesPath = "d:\Arquivos\Rotas.csv"

if (Test-Path $routesPath) {
    $content = Get-Content -Path $routesPath -Encoding UTF8 | Select-Object -Skip 1
    $routes = $content | ConvertFrom-Csv -Delimiter ';'
    $routesJson = $routes | ConvertTo-Json -Depth 10
    $routesJson | Out-File -FilePath (Join-Path $outputDir "routes.json") -Encoding UTF8
    Write-Host "  Routes: $($routes.Count)" -ForegroundColor Green
}

# Summary
Write-Host "`n========================================" -ForegroundColor Green
Write-Host "IMPORT COMPLETE" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "Output: $outputDir" -ForegroundColor Cyan
Write-Host "  Freight Orders: $($allOrders.Count)" -ForegroundColor White
Write-Host "  Timesheets: $($allTimesheets.Count)" -ForegroundColor White
Write-Host ""

# Analysis
if ($allOrders.Count -gt 0) {
    $uniqueCompanies = $allOrders | Select-Object -ExpandProperty pagador_nome -Unique | Where-Object { $_ -and $_.Trim() }
    Write-Host "Companies: $($uniqueCompanies.Count)" -ForegroundColor Cyan

    if ($Verbose) {
        $topCompanies = $allOrders | Group-Object pagador_nome |
            Where-Object { $_.Name } |
            Sort-Object Count -Descending |
            Select-Object -First 10

        Write-Host "`nTop 10 Companies:" -ForegroundColor Cyan
        foreach ($company in $topCompanies) {
            Write-Host "  $($company.Name): $($company.Count) orders" -ForegroundColor Gray
        }
    }
}

Write-Host "`nDone!" -ForegroundColor Green
