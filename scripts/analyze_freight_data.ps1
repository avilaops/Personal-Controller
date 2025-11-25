# Analyze cleaned freight orders and generate summary statistics

param(
    [string]$InputFile = "d:\Personal-Controller\data\cleaned\freight_orders_cleaned.json",
    [string]$OutputFile = "d:\Personal-Controller\data\cleaned\freight_summary.json"
)

$ErrorActionPreference = "Stop"

Write-Host "=== ANÁLISE DE PEDIDOS DE FRETE ===" -ForegroundColor Cyan

# Load data
Write-Host "`nCarregando dados..." -ForegroundColor Yellow
$orders = Get-Content $InputFile -Raw | ConvertFrom-Json

Write-Host "Total de registros: $($orders.Count)" -ForegroundColor Green

# Helper function to parse currency
function Parse-Currency {
    param($value)
    if ([string]::IsNullOrWhiteSpace($value)) { return 0 }
    $cleaned = $value -replace '[R$\s\.]', '' -replace ',', '.'
    try {
        return [double]$cleaned
    } catch {
        return 0
    }
}

# Helper function to parse number
function Parse-Number {
    param($value)
    if ([string]::IsNullOrWhiteSpace($value)) { return 0 }
    try {
        return [double]$value
    } catch {
        return 0
    }
}

# Calculate aggregated metrics
Write-Host "`nCalculando métricas..." -ForegroundColor Yellow

# 1. Total values
$totalValue = 0
$totalWeight = 0
$totalVolume = 0
$validDates = @()

foreach ($order in $orders) {
    $totalValue += Parse-Currency $order.'Valor do Frete'
    $totalWeight += Parse-Number $order.'Soma dos Pesos'
    $totalVolume += Parse-Number $order.'Soma dos Volumes'

    if (![string]::IsNullOrWhiteSpace($order.'Data de Agendamento')) {
        try {
            $validDates += [DateTime]::Parse($order.'Data de Agendamento')
        } catch {}
    }
}

# 2. Top clients
$clientStats = $orders |
    Where-Object { ![string]::IsNullOrWhiteSpace($_.'Pagador do Frete - Nome') } |
    Group-Object -Property 'Pagador do Frete - Nome' |
    ForEach-Object {
        $clientOrders = $_.Group
        $clientValue = ($clientOrders | ForEach-Object { Parse-Currency $_.'Valor do Frete' } | Measure-Object -Sum).Sum
        [PSCustomObject]@{
            name = $_.Name
            order_count = $_.Count
            total_value = $clientValue
            avg_value = if ($_.Count -gt 0) { $clientValue / $_.Count } else { 0 }
        }
    } |
    Sort-Object -Property order_count -Descending |
    Select-Object -First 10

# 3. Top routes
$routeStats = $orders |
    Where-Object {
        ![string]::IsNullOrWhiteSpace($_.'Remetente - Cidade') -and
        ![string]::IsNullOrWhiteSpace($_.'Destinatário - Cidade ')
    } |
    Group-Object -Property { "$($_.'Remetente - Cidade') → $($_.'Destinatário - Cidade ')" } |
    ForEach-Object {
        $routeOrders = $_.Group
        $routeValue = ($routeOrders | ForEach-Object { Parse-Currency $_.'Valor do Frete' } | Measure-Object -Sum).Sum
        $routeWeight = ($routeOrders | ForEach-Object { Parse-Number $_.'Soma dos Pesos' } | Measure-Object -Sum).Sum

        [PSCustomObject]@{
            route = $_.Name
            frequency = $_.Count
            total_value = $routeValue
            avg_value = if ($_.Count -gt 0) { $routeValue / $_.Count } else { 0 }
            total_weight = $routeWeight
            avg_weight = if ($_.Count -gt 0) { $routeWeight / $_.Count } else { 0 }
        }
    } |
    Sort-Object -Property frequency -Descending |
    Select-Object -First 10

# 4. Payment methods distribution
$paymentMethods = $orders |
    Where-Object { ![string]::IsNullOrWhiteSpace($_.'Forma de Pagamento') } |
    Group-Object -Property 'Forma de Pagamento' |
    ForEach-Object {
        [PSCustomObject]@{
            method = $_.Name
            count = $_.Count
            percentage = [math]::Round(($_.Count / $orders.Count) * 100, 2)
        }
    } |
    Sort-Object -Property count -Descending

# 5. Driver stats
$driverStats = $orders |
    Where-Object { ![string]::IsNullOrWhiteSpace($_.'Motorista que entrega ') } |
    Group-Object -Property 'Motorista que entrega ' |
    ForEach-Object {
        [PSCustomObject]@{
            driver = $_.Name
            deliveries = $_.Count
        }
    } |
    Sort-Object -Property deliveries -Descending |
    Select-Object -First 10

# Build summary object
$summary = @{
    timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    total_records = $orders.Count
    metrics = @{
        total_value = [math]::Round($totalValue, 2)
        total_weight = [math]::Round($totalWeight, 2)
        total_volume = [math]::Round($totalVolume, 2)
        avg_value = if ($orders.Count -gt 0) { [math]::Round($totalValue / $orders.Count, 2) } else { 0 }
        avg_weight = if ($orders.Count -gt 0) { [math]::Round($totalWeight / $orders.Count, 2) } else { 0 }
        avg_volume = if ($orders.Count -gt 0) { [math]::Round($totalVolume / $orders.Count, 2) } else { 0 }
    }
    date_range = if ($validDates.Count -gt 0) {
        @{
            start = ($validDates | Measure-Object -Minimum).Minimum.ToString("yyyy-MM-dd")
            end = ($validDates | Measure-Object -Maximum).Maximum.ToString("yyyy-MM-dd")
            days = (($validDates | Measure-Object -Maximum).Maximum - ($validDates | Measure-Object -Minimum).Minimum).Days
        }
    } else {
        @{
            start = "N/A"
            end = "N/A"
            days = 0
        }
    }
    top_clients = $clientStats
    top_routes = $routeStats
    payment_methods = $paymentMethods
    top_drivers = $driverStats
}

# Save summary
$summary | ConvertTo-Json -Depth 10 | Out-File $OutputFile -Encoding UTF8

Write-Host "`n=== RESUMO DAS ANÁLISES ===" -ForegroundColor Cyan
Write-Host "Total de Pedidos: $($orders.Count)" -ForegroundColor Green
Write-Host "Valor Total: R$ $($summary.metrics.total_value)" -ForegroundColor Green
Write-Host "Peso Total: $($summary.metrics.total_weight) kg" -ForegroundColor Green
Write-Host "Volume Total: $($summary.metrics.total_volume)" -ForegroundColor Green

if ($validDates.Count -gt 0) {
    Write-Host "`nPeríodo:" -ForegroundColor Yellow
    Write-Host "  Início: $($summary.date_range.start)" -ForegroundColor Gray
    Write-Host "  Fim: $($summary.date_range.end)" -ForegroundColor Gray
    Write-Host "  Duração: $($summary.date_range.days) dias" -ForegroundColor Gray
}

Write-Host "`nTop 5 Clientes:" -ForegroundColor Yellow
$clientStats | Select-Object -First 5 | ForEach-Object {
    Write-Host "  $($_.name): $($_.order_count) pedidos (R$ $($_.total_value))" -ForegroundColor Gray
}

Write-Host "`nTop 5 Rotas:" -ForegroundColor Yellow
$routeStats | Select-Object -First 5 | ForEach-Object {
    Write-Host "  $($_.route): $($_.frequency) vezes" -ForegroundColor Gray
}

Write-Host "`nFormas de Pagamento:" -ForegroundColor Yellow
$paymentMethods | ForEach-Object {
    Write-Host "  $($_.method): $($_.count) ($($_.percentage)%)" -ForegroundColor Gray
}

Write-Host "`n✅ Análise salva em: $OutputFile" -ForegroundColor Green
