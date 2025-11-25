# Script para iniciar o Personal Controller
# Personal Controller - Ávila Transportes

Write-Host "🚀 Personal Controller - Startup" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

# Verifica se o build já foi feito
if (!(Test-Path "target\release\pc-server.exe")) {
    Write-Host "🔨 Compilando Personal Controller..." -ForegroundColor Yellow
    cargo build --release
    Write-Host ""
}

Write-Host "🗄️ Inicializando banco de dados..." -ForegroundColor Yellow
cargo run --release --bin pc -- init
Write-Host ""

Write-Host "🌐 Iniciando API Server na porta 3000..." -ForegroundColor Green
Write-Host "   http://localhost:3000" -ForegroundColor Cyan
Write-Host ""
Write-Host "💡 Dica: Use Ctrl+C para parar o servidor" -ForegroundColor Gray
Write-Host ""

cargo run --release --bin pc-server
