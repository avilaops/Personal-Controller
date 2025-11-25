# 🏗️ Arquitetura do Personal Controller

## Visão Geral

```
┌─────────────────────────────────────────────────────────────┐
│                    Personal Controller                       │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   pc-web     │  │   pc-cli     │  │   pc-api     │     │
│  │  (Frontend)  │  │  (Terminal)  │  │ (REST/GQL)   │     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘     │
│         │                  │                  │              │
│         └──────────────────┴──────────────────┘              │
│                            │                                 │
│                   ┌────────▼────────┐                       │
│                   │    pc-llm       │                       │
│                   │  (AI Assistant) │                       │
│                   │   - RAG         │                       │
│                   │   - Embeddings  │                       │
│                   │   - Chat        │                       │
│                   └────────┬────────┘                       │
│                            │                                 │
│         ┌──────────────────┼──────────────────┐            │
│         │                  │                  │             │
│  ┌──────▼──────┐  ┌───────▼──────┐  ┌───────▼──────┐     │
│  │ pc-models   │  │   pc-db      │  │ pc-importers │     │
│  │   (Data)    │  │  (Database)  │  │  (CSV/Data)  │     │
│  └──────┬──────┘  └──────┬───────┘  └──────┬───────┘     │
│         │                │                  │              │
│         └────────────────┴──────────────────┘              │
│                          │                                  │
│                   ┌──────▼──────┐                          │
│                   │   pc-core   │                          │
│                   │   (Traits)  │                          │
│                   └─────────────┘                          │
└──────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────▼────┐      ┌─────▼─────┐     ┌─────▼─────┐
   │ AvilaDB │      │ avila-ml  │     │  Deriax   │
   │ (NoSQL) │      │   (ML)    │     │ (Security)│
   └─────────┘      └───────────┘     └───────────┘
        │                  │                  │
        └──────────────────┴──────────────────┘
                    Projeto Arxis
```

## Componentes

### pc-core
**Responsabilidade**: Tipos fundamentais e traits
- `Entity` trait para todas as entidades
- `Importable` trait para importadores
- `Embeddable` trait para embeddings LLM
- Tipos de erro padronizados
- Paginação e metadata de auditoria

### pc-models
**Responsabilidade**: Modelos de dados
- `Company` - Empresas (clientes, fornecedores)
- `FreightOrder` - Ordens de frete
- `Timesheet` - Registros de ponto
- `Contact` - Contatos
- `Route` - Rotas de transporte
- `Invoice` / `Cte` - Documentos fiscais

### pc-db
**Responsabilidade**: Camada de persistência
- Integração com AvilaDB
- Repositórios genéricos
- Queries e índices
- Busca vetorial para RAG

### pc-importers
**Responsabilidade**: Importação de dados
- Parser de CSV com encoding Windows-1252
- Auto-detecção de tipo de arquivo
- Validação e transformação de dados
- Suporte para múltiplos formatos

### pc-llm
**Responsabilidade**: Inteligência Artificial
- **RAG** (Retrieval Augmented Generation)
- Geração de embeddings com avila-ml
- Interface de chat conversacional
- Busca semântica nos dados
- Fine-tuning com dados da Ávila

### pc-api
**Responsabilidade**: API REST/GraphQL
- Endpoints REST para todas as entidades
- WebSocket para chat em tempo real
- CORS configurado
- Autenticação JWT (futuro)
- Rate limiting (futuro)

### pc-cli
**Responsabilidade**: Interface de linha de comando
- Importação de dados
- Chat interativo com LLM
- Administração do banco
- Visualização de estatísticas

### pc-web (futuro)
**Responsabilidade**: Interface web
- Dashboard interativo
- Visualizações de dados
- Chat com LLM
- Gestão de documentos

## Fluxo de Dados

### 1. Importação de CSV

```
CSV File → pc-importers → Validation → pc-models → pc-db → AvilaDB
                                                      ↓
                                              Embeddings → Vector Index
```

### 2. Consulta com LLM

```
User Query → pc-llm → Tokenization
                ↓
          Embedding Generation (avila-ml)
                ↓
          Vector Search (AvilaDB)
                ↓
          RAG Context Building
                ↓
          LLM Inference
                ↓
          Response → User
```

### 3. API REST

```
HTTP Request → pc-api → pc-db → AvilaDB
                  ↓
            Validation (pc-core)
                  ↓
         Transformation (pc-models)
                  ↓
            JSON Response
```

## Dependências Externas

### Do Projeto Arxis
- **AvilaDB**: NoSQL database com vector search
- **avila-ml**: Machine learning suite
- **avila-clustering**: Algoritmos de clustering
- **avila-linalg**: Álgebra linear
- **avx-telemetry**: Métricas e logs
- **avx-image**: Processamento de imagens (futuro)

### Do Projeto Deriax
- **Criptografia**: SHA-256, MD5, Base64
- **Validação**: Regex patterns
- **Análise**: Parsing de dados

### Externas (crates.io)
- **Tokio**: Runtime async
- **Axum**: Web framework
- **Serde**: Serialização
- **Clap**: CLI parsing
- **Tokenizers**: Hugging Face tokenizers
- **llm-chain**: LLM orchestration

## Escalabilidade

### Horizontal Scaling
- API pode rodar em múltiplas instâncias
- AvilaDB distribuído com multi-region
- Load balancer (avl-loadbalancer do arxis)

### Vertical Scaling
- Embeddings em GPU (avx-gpu do arxis)
- Batch processing com Rayon
- Cache em memória para queries frequentes

## Segurança

### Camadas de Segurança
1. **Autenticação**: JWT tokens (futuro)
2. **Autorização**: RBAC (futuro)
3. **Criptografia**: TLS/HTTPS
4. **Auditoria**: Logs completos
5. **Validação**: Input sanitization

### Baseado em Deriax
- Hashing seguro de senhas
- Validação de entrada
- Detecção de padrões maliciosos

## Performance

### Otimizações
- **Compilação**: LTO thin, opt-level 3
- **Parallel**: Rayon para processamento
- **Async**: Tokio para I/O
- **Cache**: Em memória e Redis (futuro)
- **Índices**: AvilaDB vector search

### Métricas Esperadas
- Importação: ~1000 registros/s
- API: <100ms p95
- LLM Query: <2s com RAG
- Embedding: ~50 docs/s

## Roadmap Técnico

### Fase 1 (Atual) ✅
- [x] Estrutura base do projeto
- [x] Modelos de dados
- [x] Importadores CSV
- [x] API REST básica
- [x] CLI funcional
- [x] Estrutura LLM/RAG

### Fase 2 (Próxima)
- [ ] Integração real com AvilaDB
- [ ] Embeddings com avila-ml
- [ ] Busca vetorial funcional
- [ ] LLM inference (local ou API)
- [ ] Testes de integração

### Fase 3 (Futuro)
- [ ] Frontend web (pc-web)
- [ ] Autenticação JWT
- [ ] WebSocket para real-time
- [ ] Fine-tuning do LLM
- [ ] Mobile app (React Native)

### Fase 4 (Expansão)
- [ ] Multi-tenant
- [ ] Analytics avançado
- [ ] Relatórios automatizados
- [ ] Integrações (ERP, CRM)
- [ ] Machine Learning preditivo

---

**Arquitetura desenhada para escalar com a Ávila**
