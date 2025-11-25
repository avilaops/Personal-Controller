# 🎯 Personal Controller

**A Plataforma de Gestão Empresarial da Ávila, pela Ávila**

Personal Controller é uma plataforma 100% em Rust que centraliza todas as informações empresariais da Ávila Transportes em um único sistema integrado, potencializado pela **Personal-Controller-LLM** - uma IA especializada treinada com dados reais da empresa.

## 🚀 Visão Geral

O Personal Controller integra:

- 📦 **Gestão de Fretes** - Ordens, manifestos, rastreamento
- 🏢 **Cadastro de Empresas** - Clientes, fornecedores, parceiros
- 📊 **Controle Fiscal** - Notas fiscais, CT-e, documentos
- 💰 **Gestão Financeira** - Pagamentos, recebimentos, fluxo de caixa
- 👥 **Recursos Humanos** - Funcionários, ponto, folha de pagamento
- 📍 **Rotas e Logística** - Planejamento de rotas, otimização
- 📧 **Comunicação** - Emails, contatos, histórico
- 🤖 **Personal-Controller-LLM** - IA conversacional com conhecimento da Ávila

## 🏗️ Arquitetura

```
personal-controller/
├── pc-core/          # Core types and traits
├── pc-models/        # Data models (empresas, materiais, fiscal, etc)
├── pc-db/            # Database layer (using AvilaDB)
├── pc-importers/     # CSV and data importers
├── pc-llm/           # Personal-Controller-LLM (RAG + inference)
├── pc-api/           # REST/GraphQL API
├── pc-cli/           # Command-line interface
└── pc-web/           # Web frontend
```

### Tecnologias Base

- **Banco de Dados**: [AvilaDB](../arxis/aviladb) - NoSQL distribuído com busca vetorial
- **Machine Learning**: [avila-ml](../arxis/avila-ml) - Suite completa de ML
- **Tokenização**: Hugging Face Tokenizers + tiktoken-rs
- **LLM Chain**: llm-chain para orquestração de modelos
- **Analytics**: avila-telemetry para métricas e logs
- **Security**: Baseado em Deriax para criptografia e validações

## 🎯 Funcionalidades

### 1. Importação Inteligente de Dados

```bash
pc import --type freight --file "d:/Arquivos/01-04.csv"
pc import --type timesheets --file "d:/Arquivos/Horas.csv"
pc import --auto "d:/Arquivos/*.csv"  # Auto-detect e importa tudo
```

### 2. Consultas Naturais com LLM

```bash
pc chat "Quais foram os fretes para São José do Rio Preto em abril?"
pc chat "Me mostre o saldo de horas do funcionário Lindomar"
pc chat "Qual cliente teve mais entregas este mês?"
```

### 3. API REST/GraphQL

```rust
// REST API
GET /api/v1/freight-orders?pagador=ACME
GET /api/v1/companies?city=Ribeirão+Preto
POST /api/v1/freight-orders

// GraphQL
query {
  freightOrders(filter: { city: "Ribeirão Preto" }) {
    numero
    pagadorNome
    valorFrete
    motorista
  }
}
```

### 4. Dashboard Web

Interface web moderna com:
- Visualizações de dados em tempo real
- Gráficos interativos
- Chat com a Personal-Controller-LLM
- Gestão de documentos
- Relatórios customizados

## 📦 Instalação

### Requisitos

- Rust 1.75+
- AvilaDB instalado (do projeto arxis)
- 4GB+ RAM

### Setup Rápido

```powershell
# Clone o projeto
cd d:\Personal-Controller

# Build completo
cargo build --release

# Importar dados iniciais
.\scripts\import-all-data.ps1

# Iniciar servidor
cargo run --release --bin pc-server

# Iniciar CLI
cargo run --release --bin pc-cli
```

## 🧠 Personal-Controller-LLM

A Personal-Controller-LLM é uma IA especializada que:

1. **Conhece o negócio**: Treinada com dados históricos da Ávila
2. **RAG inteligente**: Busca vetorial no AvilaDB para contexto preciso
3. **Responde em português**: Otimizada para linguagem natural em PT-BR
4. **Aprende continuamente**: Fine-tuning com novos dados

### Arquitetura da LLM

```
┌─────────────────┐
│  User Query     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Tokenizer      │ (tiktoken-rs)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Embeddings     │ (avila-ml)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Vector Search  │ (AvilaDB)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  LLM Chain      │ (llm-chain)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Response       │
└─────────────────┘
```

## 📊 Modelos de Dados

### Freight Order

```rust
pub struct FreightOrder {
    pub numero: String,
    pub data_emissao: NaiveDate,
    pub data_agendamento: Option<NaiveDate>,
    pub notas_fiscais: Vec<String>,
    pub pagador: Company,
    pub remetente: Company,
    pub destinatario: Company,
    pub volumes: i32,
    pub peso: f64,
    pub valor_notas: f64,
    pub valor_frete: f64,
    pub motorista_coleta: Option<String>,
    pub motorista_entrega: Option<String>,
}
```

### Company

```rust
pub struct Company {
    pub id: Uuid,
    pub nome: String,
    pub cnpj: Option<String>,
    pub telefone: Option<String>,
    pub email: Option<String>,
    pub cidade: String,
    pub estado: String,
    pub tipo: CompanyType, // Cliente, Fornecedor, Parceiro
    pub contatos: Vec<Contact>,
}
```

### Timesheet

```rust
pub struct Timesheet {
    pub funcionario: String,
    pub mes: String,
    pub data: NaiveDate,
    pub entrada: NaiveTime,
    pub saida: NaiveTime,
    pub total: Duration,
    pub saldo: Duration,
}
```

## 🔐 Segurança

- Hashing de senhas: SHA-256
- Tokens JWT para autenticação
- Criptografia de dados sensíveis
- Audit log completo
- RBAC (Role-Based Access Control)

## 📈 Roadmap

- [x] Setup inicial do projeto
- [x] Estrutura de workspace
- [ ] Implementar pc-core (types e traits)
- [ ] Implementar pc-models (data models)
- [ ] Implementar pc-db (AvilaDB integration)
- [ ] Implementar pc-importers (CSV parsers)
- [ ] Implementar pc-llm (RAG + LLM chain)
- [ ] Implementar pc-api (REST + GraphQL)
- [ ] Implementar pc-cli (command-line)
- [ ] Implementar pc-web (frontend)
- [ ] Testes de integração
- [ ] Deploy em produção

## 🤝 Contribuindo

Este é um projeto interno da Ávila Transportes. Contribuições são bem-vindas!

## 📄 Licença

MIT OR Apache-2.0

## 🙏 Agradecimentos

- **Projeto Arxis**: Base de ML, telemetria e AvilaDB
- **Projeto Deriax**: Ferramentas de segurança e análise
- **Equipe Ávila**: Dados e feedback essenciais

---

**Feito com ❤️ pela Ávila, para a Ávila**
