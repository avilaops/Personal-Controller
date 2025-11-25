# Script de chat interativo com a Personal-Controller-LLM
# Personal Controller - Ávila Transportes

Write-Host "💬 Personal-Controller-LLM - Chat Interativo" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""
Write-Host "Digite suas perguntas sobre a Ávila Transportes" -ForegroundColor Green
Write-Host "Digite 'sair' para encerrar" -ForegroundColor Gray
Write-Host ""

while ($true) {
    Write-Host "Você: " -ForegroundColor Yellow -NoNewline
    $query = Read-Host
    
    if ($query -eq "sair" -or $query -eq "exit") {
        Write-Host ""
        Write-Host "👋 Até logo!" -ForegroundColor Cyan
        break
    }
    
    if ([string]::IsNullOrWhiteSpace($query)) {
        continue
    }
    
    Write-Host ""
    cargo run --release --bin pc -- chat "$query"
    Write-Host ""
}
