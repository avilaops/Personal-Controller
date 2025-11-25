# 🎯 Personal Controller - Quickstart

## Setup Rápido

### 1. Build do Projeto

```powershell
cd d:\Personal-Controller
cargo build --release
```

### 2. Inicializar Banco de Dados

```powershell
cargo run --release --bin pc -- init
```

### 3. Importar Dados

#### Importação Automática (Recomendado)

```powershell
.\scripts\import-all-data.ps1
```

#### Importação Manual

```powershell
# Ordens de frete
cargo run --bin pc -- import --type freight --file "d:\Arquivos\01-04.csv"

# Timesheets
cargo run --bin pc -- import --type timesheet --file "d:\Arquivos\Horas.csv"

# Rotas
cargo run --bin pc -- import --type route --file "d:\Arquivos\Rotas.csv"
```

### 4. Iniciar API Server

```powershell
.\scripts\start-server.ps1
```

Ou manualmente:

```powershell
cargo run --release --bin pc-server
```

A API estará disponível em: `http://localhost:3000`

### 5. Usar a CLI

```powershell
# Ver estatísticas
cargo run --bin pc -- stats

# Chat com a LLM
cargo run --bin pc -- chat "Quais fretes temos para São José do Rio Preto?"

# Chat interativo
.\scripts\chat.ps1
```

## Endpoints da API

### Health Check
```
GET http://localhost:3000/health
```

### Empresas
```
GET  http://localhost:3000/api/v1/companies
POST http://localhost:3000/api/v1/companies
GET  http://localhost:3000/api/v1/companies/:id
```

### Ordens de Frete
```
GET  http://localhost:3000/api/v1/freight-orders
POST http://localhost:3000/api/v1/freight-orders
GET  http://localhost:3000/api/v1/freight-orders/:id
```

### Chat com LLM
```
POST http://localhost:3000/api/v1/chat
Body: { "query": "sua pergunta aqui" }
```

### Estatísticas
```
GET http://localhost:3000/api/v1/stats
```

## Exemplos de Uso

### Importar e Consultar

```powershell
# Importar ordens de frete
cargo run --bin pc -- import --type freight --file "d:\Arquivos\01-04.csv"

# Ver estatísticas
cargo run --bin pc -- stats
```

### Chat com a LLM

```powershell
# Pergunta simples
cargo run --bin pc -- chat "Quantos fretes temos cadastrados?"

# Pergunta complexa
cargo run --bin pc -- chat "Me mostre os fretes mais caros de abril"
```

### API REST

```powershell
# Listar empresas
curl http://localhost:3000/api/v1/companies

# Criar empresa
curl -X POST http://localhost:3000/api/v1/companies `
  -H "Content-Type: application/json" `
  -d '{"nome":"Avila Transportes","cidade":"Ribeirão Preto","estado":"SP"}'

# Chat
curl -X POST http://localhost:3000/api/v1/chat `
  -H "Content-Type: application/json" `
  -d '{"query":"Quais são os motoristas mais ativos?"}'
```

## Estrutura do Projeto

```
personal-controller/
├── pc-core/          ✅ Core types and traits
├── pc-models/        ✅ Data models
├── pc-db/            ✅ Database layer (AvilaDB)
├── pc-importers/     ✅ CSV importers
├── pc-llm/           ✅ Personal-Controller-LLM
├── pc-api/           ✅ REST API
├── pc-cli/           ✅ Command-line interface
└── scripts/          ✅ PowerShell scripts
```

## Próximos Passos

1. ✅ Build inicial concluído
2. ⏳ Conectar com AvilaDB real
3. ⏳ Implementar embeddings com avila-ml
4. ⏳ Treinar modelo LLM com dados da Ávila
5. ⏳ Criar frontend web (pc-web)
6. ⏳ Deploy em produção

## Problemas Comuns

### Erro de compilação
```powershell
# Limpar build e recompilar
cargo clean
cargo build --release
```

### Porta 3000 ocupada
```powershell
# Verificar processo usando a porta
Get-NetTCPConnection -LocalPort 3000
# Ou use outra porta editando pc-api/src/main.rs
```

## Suporte

Para dúvidas ou problemas, consulte o README.md completo.

---

**Feito com ❤️ pela Ávila, para a Ávila**
