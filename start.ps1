# Personal Controller - Startup Script
# Windows PowerShell

Write-Host "🚀 Starting Personal Controller..." -ForegroundColor Cyan
Write-Host ""

# Check prerequisites
Write-Host "Checking prerequisites..." -ForegroundColor Yellow

# Check Rust
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust not found. Install from https://rustup.rs/" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Rust installed" -ForegroundColor Green

# Check Node.js
if (!(Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Node.js not found. Install from https://nodejs.org/" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Node.js installed" -ForegroundColor Green

# Check if .env exists
if (!(Test-Path ".env")) {
    Write-Host "⚠️  .env not found, creating from example..." -ForegroundColor Yellow
    Copy-Item ".env.example" ".env"
    Write-Host "📝 Please edit .env with your configuration" -ForegroundColor Yellow
}

# Install frontend dependencies if needed
if (!(Test-Path "pc-web\node_modules")) {
    Write-Host ""
    Write-Host "📦 Installing frontend dependencies..." -ForegroundColor Yellow
    Push-Location pc-web
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to install frontend dependencies" -ForegroundColor Red
        Pop-Location
        exit 1
    }
    Pop-Location
    Write-Host "✅ Frontend dependencies installed" -ForegroundColor Green
}

Write-Host ""
Write-Host "🏗️  Building backend..." -ForegroundColor Yellow
Push-Location pc-api
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to build backend" -ForegroundColor Red
    Pop-Location
    exit 1
}
Pop-Location
Write-Host "✅ Backend built successfully" -ForegroundColor Green

Write-Host ""
Write-Host "🎉 Starting services..." -ForegroundColor Cyan
Write-Host ""

# Start API in background
Write-Host "🔧 Starting API server..." -ForegroundColor Yellow
$apiJob = Start-Job -ScriptBlock {
    Set-Location $using:PWD
    Set-Location pc-api
    cargo run --release
}

# Wait a bit for API to start
Start-Sleep -Seconds 3

# Check if API is running
try {
    $response = Invoke-WebRequest -Uri "http://localhost:3000/health" -UseBasicParsing -TimeoutSec 5
    Write-Host "✅ API server running on http://localhost:3000" -ForegroundColor Green
} catch {
    Write-Host "⚠️  API might still be starting..." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🌐 Starting frontend..." -ForegroundColor Yellow
Push-Location pc-web

# Open browser after a delay
Start-Job -ScriptBlock {
    Start-Sleep -Seconds 5
    Start-Process "http://localhost:3001"
} | Out-Null

# Run frontend (blocks here)
npm run dev

# Cleanup on exit
Write-Host ""
Write-Host "🛑 Stopping services..." -ForegroundColor Yellow
Stop-Job $apiJob
Remove-Job $apiJob

Pop-Location

Write-Host ""
Write-Host "👋 Personal Controller stopped" -ForegroundColor Cyan
