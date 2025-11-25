#!/bin/bash
# Personal Controller - Startup Script
# Linux/Mac

set -e

echo "🚀 Starting Personal Controller..."
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust not found. Install from https://rustup.rs/${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Rust installed${NC}"

# Check Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found. Install from https://nodejs.org/${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Node.js installed${NC}"

# Check if .env exists
if [ ! -f ".env" ]; then
    echo -e "${YELLOW}⚠️  .env not found, creating from example...${NC}"
    cp .env.example .env
    echo -e "${YELLOW}📝 Please edit .env with your configuration${NC}"
fi

# Install frontend dependencies if needed
if [ ! -d "pc-web/node_modules" ]; then
    echo ""
    echo -e "${YELLOW}📦 Installing frontend dependencies...${NC}"
    cd pc-web
    npm install
    cd ..
    echo -e "${GREEN}✅ Frontend dependencies installed${NC}"
fi

echo ""
echo -e "${YELLOW}🏗️  Building backend...${NC}"
cd pc-api
cargo build --release
cd ..
echo -e "${GREEN}✅ Backend built successfully${NC}"

echo ""
echo -e "${CYAN}🎉 Starting services...${NC}"
echo ""

# Cleanup function
cleanup() {
    echo ""
    echo -e "${YELLOW}🛑 Stopping services...${NC}"
    kill $API_PID 2>/dev/null || true
    echo ""
    echo -e "${CYAN}👋 Personal Controller stopped${NC}"
    exit 0
}

trap cleanup INT TERM

# Start API in background
echo -e "${YELLOW}🔧 Starting API server...${NC}"
cd pc-api
cargo run --release &
API_PID=$!
cd ..

# Wait a bit for API to start
sleep 3

# Check if API is running
if curl -s http://localhost:3000/health > /dev/null; then
    echo -e "${GREEN}✅ API server running on http://localhost:3000${NC}"
else
    echo -e "${YELLOW}⚠️  API might still be starting...${NC}"
fi

echo ""
echo -e "${YELLOW}🌐 Starting frontend...${NC}"
cd pc-web

# Open browser after a delay (background)
(sleep 5 && xdg-open http://localhost:3001 2>/dev/null || open http://localhost:3001 2>/dev/null || true) &

# Run frontend (blocks here)
npm run dev

# Cleanup
cleanup
