# Script de importação automática de todos os CSVs
# Personal Controller - Ávila Transportes

Write-Host "🚀 Personal Controller - Importação Automática" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan

$arquivosPath = "d:\Arquivos"
$csvFiles = Get-ChildItem -Path $arquivosPath -Filter "*.csv"

Write-Host ""
Write-Host "📁 Encontrados $($csvFiles.Count) arquivos CSV" -ForegroundColor Green
Write-Host ""

foreach ($file in $csvFiles) {
    Write-Host "📄 Importando: $($file.Name)" -ForegroundColor Yellow
    
    # Detecta tipo automaticamente
    if ($file.Name -like "*Horas*") {
        Write-Host "   Tipo: Timesheet" -ForegroundColor Gray
        cargo run --bin pc -- import --type timesheet --file $file.FullName
    }
    elseif ($file.Name -like "*Rotas*") {
        Write-Host "   Tipo: Rotas" -ForegroundColor Gray
        cargo run --bin pc -- import --type route --file $file.FullName
    }
    elseif ($file.Name -like "*-04.csv" -or $file.Name -like "*Planilha*") {
        Write-Host "   Tipo: Freight Orders" -ForegroundColor Gray
        cargo run --bin pc -- import --type freight --file $file.FullName
    }
    else {
        Write-Host "   Tipo: Auto-detect" -ForegroundColor Gray
        cargo run --bin pc -- import --type auto --file $file.FullName
    }
    
    Write-Host ""
}

Write-Host "✅ Importação concluída!" -ForegroundColor Green
Write-Host ""
Write-Host "📊 Verificando estatísticas..." -ForegroundColor Cyan
cargo run --bin pc -- stats
