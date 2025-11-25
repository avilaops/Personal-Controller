# 🚛 Personal Controller

> **Sistema completo que organiza automaticamente seus fretes, rotas e horas trabalhadas - com inteligência artificial integrada**

[🌐 Ver Landing Page](landing-page.html) | [📖 Guia Rápido](QUICKSTART.md) | [🔧 API Docs](API_DOCUMENTATION.md)

---

## ⚡ O que ele faz por você

### 📦 **Fretes no Automático**
Jogue seus arquivos Excel ou PDF de fretes e pronto - o sistema organiza tudo, valida CNPJs, calcula totais e mostra onde está cada entrega. Sem digitar nada.

### 🗺️ **Rotas Otimizadas**
Coloque de onde sai e onde vai - ele calcula a rota mais econômica, quanto vai gastar por km e quando fazer manutenção do veículo.

### ⏰ **Horas Calculadas**
Registre quando trabalhou e ele calcula sozinho horas normais, extras e adicional noturno. Tudo pronto para folha de pagamento.

### 🤖 **IA que Responde**
Pergunte qualquer coisa: *"Quanto gastei de combustível em setembro?"* ou *"Quais fretes estão atrasados?"* - A IA responde na hora.

### 📊 **Tudo Visual**
Dashboard com gráficos em tempo real mostrando custos, fretes ativos, rotas rentáveis. Fácil de entender.

---

## 🎯 Está pronto para usar?

✅ **SIM!** Principais funcionalidades operacionais:

- ✅ Importação de fretes (Excel/PDF)
- ✅ Gestão de rotas e custos
- ✅ Controle de horas com cálculos automáticos
- ✅ Chatbot com IA para consultas
- ✅ Dashboard web interativo
- ✅ API REST para integrações
- ✅ Scripts de automação prontos

🚧 **Em desenvolvimento:**
- OCR de documentos fiscais
- Análise preditiva de custos

---

## 🚀 Como começar (4 passos)

### **Windows:**
```powershell
# 1. Clone o repositório
git clone https://github.com/avilaops/Personal-Controller.git
cd Personal-Controller

# 2. Execute o script de inicialização
.\start.ps1

# 3. Abra o navegador
# http://localhost:3000

# 4. Comece a importar seus dados!
```

### **Linux/Mac:**
```bash
# 1. Clone o repositório
git clone https://github.com/avilaops/Personal-Controller.git
cd Personal-Controller

# 2. Execute o script de inicialização
chmod +x start.sh
./start.sh

# 3. Abra o navegador
# http://localhost:3000
```

---

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

## 💡 Exemplos de uso

### Importar fretes do Excel
```powershell
# PowerShell
.\scripts\import_all.ps1

# Ou manualmente
cargo run --bin pc-cli import --file "seus-fretes.xlsx"
```

### Consultar via IA
```bash
# Abra o chat no dashboard (http://localhost:3000/chat)
# Ou use a CLI:
cargo run --bin pc-cli chat

# Exemplos de perguntas:
# "Quanto gastei de combustível em outubro?"
# "Quais fretes estão atrasados?"
# "Qual motorista fez mais viagens?"
# "Me mostra o resumo do último mês"
```

### Ver relatórios
```bash
# Dashboard web com gráficos
http://localhost:3000

# Ou via linha de comando
cargo run --bin pc-cli report --type monthly
```

---

## 📁 Estrutura do Projeto

```
personal-controller/
├── pc-api/           # Servidor REST API
├── pc-cli/           # Interface de linha de comando
├── pc-core/          # Tipos e traits principais
├── pc-db/            # Camada de banco de dados
├── pc-importers/     # Importadores Excel/PDF/CSV
├── pc-llm/           # IA e chatbot
├── pc-models/        # Modelos de dados
├── pc-web/           # Frontend Next.js
├── scripts/          # Scripts de automação
├── examples/         # Exemplos de código
└── data/             # Dados importados
```

---

## 🔧 Para desenvolvedores

### Compilar
```bash
cargo build --release
```

### Rodar testes
```bash
cargo test
```

### Rodar API
```bash
cargo run --bin pc-api
# API disponível em http://localhost:8080
```

### Rodar frontend
```bash
cd pc-web
npm install
npm run dev
# Frontend em http://localhost:3000
```

---

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

## 🤝 Contribuir

Contribuições são bem-vindas! 

1. Fork o projeto
2. Crie uma branch: `git checkout -b minha-feature`
3. Commit suas mudanças: `git commit -m 'Adiciona nova feature'`
4. Push: `git push origin minha-feature`
5. Abra um Pull Request

---

## 📄 Licença

MIT License - veja [LICENSE](LICENSE) para detalhes.

---

## 👤 Autor

**Nícolas Ávila**
- GitHub: [@avilaops](https://github.com/avilaops)
- Website: [avilaops.com](https://www.avilaops.com)
- Empresa: [@avilainc](https://github.com/avilainc)

---

## 📚 Documentação Adicional

- [📖 Guia Rápido](QUICKSTART.md) - Comece em 5 minutos
- [🏗️ Arquitetura](ARCHITECTURE.md) - Detalhes técnicos
- [🔧 API](API_DOCUMENTATION.md) - Documentação da API
- [🧪 Testes](TESTING.md) - Como testar
- [🚀 Deploy](DEPLOYMENT.md) - Colocar em produção
- [📊 Análise de Dados](DATA_ANALYSIS_GUIDE.md) - Guia de análise

---

## 🆘 Suporte

Encontrou um problema? 
- Abra uma [issue no GitHub](https://github.com/avilaops/Personal-Controller/issues)
- Veja a [documentação completa](https://github.com/avilaops/Personal-Controller)

---

## ⭐ Gostou?

Se este projeto te ajudou, deixe uma ⭐ no repositório!

---

<div align="center">
  
**Personal Controller** - Sistema inteligente de gestão logística
  
Feito com 💙 por [Nícolas Ávila](https://github.com/avilaops)

</div>

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
