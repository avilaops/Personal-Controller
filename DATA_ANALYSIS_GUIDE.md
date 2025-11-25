# 🎯 Análise dos 135 Pedidos de Frete Válidos
**Gerado em**: 2025-11-24 04:36:00
**Fonte**: `d:\Personal-Controller\data\cleaned\freight_orders_cleaned.json`

## 📊 DADOS ESTRUTURADOS

### Campos Disponíveis por Pedido
```json
{
  "Número": "string",
  "Data de Agendamento": "date",
  "Data de Emissão": "date",
  "Notas Fiscais": "string",
  "Pagador do Frete - Nome": "string",
  "Pagador do Frete - Fone": "string",
  "Remetente - Nome": "string",
  "Remetente - Cidade": "string",
  "Destinatário - Nome": "string",
  "Destinatário - Cidade ": "string",
  "Soma dos Volumes": "number",
  "Soma dos Pesos": "number",
  "Soma das Notas": "number",
  "Valor do Frete": "currency",
  "Frete tabelado": "currency",
  "Frete Valor": "currency",
  "Frete Peso": "currency",
  "Frete mínimo": "currency",
  "Frete volumes": "number",
  "Valor Frete Cubado": "currency",
  "Forma de Pagamento": "string",
  "Minuta/Cte": "string",
  "Filial que Coleta": "string",
  "Motorista que Coleta": "string",
  "Filial que entrega": "string",
  "Motorista que entrega": "string"
}
```

## 🔍 ANÁLISES POSSÍVEIS

### 1. Dashboard de Estatísticas
```typescript
// Métricas principais
- Total de pedidos: 135
- Valor total transportado
- Peso total transportado
- Volume total transportado
- Período coberto (Data de Agendamento min/max)
```

### 2. Análise por Empresa
```sql
-- Top 10 pagadores de frete
SELECT
  "Pagador do Frete - Nome",
  COUNT(*) as total_pedidos,
  SUM("Valor do Frete") as valor_total
GROUP BY "Pagador do Frete - Nome"
ORDER BY total_pedidos DESC
LIMIT 10
```

### 3. Análise Geográfica
```typescript
// Rotas mais frequentes
interface RouteAnalysis {
  origin: string; // Remetente - Cidade
  destination: string; // Destinatário - Cidade
  frequency: number;
  avgWeight: number;
  avgValue: number;
}
```

### 4. Análise Temporal
```typescript
// Distribuição por dia/mês
interface TemporalAnalysis {
  date: Date;
  orders: number;
  totalValue: number;
  avgDeliveryTime?: number;
}
```

### 5. Análise de Performance
```typescript
// Motoristas e filiais
interface PerformanceMetrics {
  driver: string;
  branch: string;
  totalDeliveries: number;
  avgValue: number;
  routesCovered: string[];
}
```

## 📋 EXEMPLOS DE QUERIES RAG

### Query 1: Busca Temporal
```
"Mostre todos os pedidos agendados em abril de 2025"
→ Filtrar por "Data de Agendamento" BETWEEN '2025-04-01' AND '2025-04-30'
```

### Query 2: Busca por Cliente
```
"Quais empresas mais utilizaram nosso serviço?"
→ GROUP BY "Pagador do Frete - Nome", COUNT(*), SUM("Valor do Frete")
```

### Query 3: Busca Geográfica
```
"Quantos fretes foram de São Paulo para Rio de Janeiro?"
→ WHERE "Remetente - Cidade" = 'São Paulo'
    AND "Destinatário - Cidade " = 'Rio de Janeiro'
```

### Query 4: Análise de Custo
```
"Qual a diferença média entre frete tabelado e frete cobrado?"
→ AVG("Frete tabelado" - "Valor do Frete")
```

### Query 5: Análise de Motoristas
```
"Quais motoristas fizeram mais entregas?"
→ GROUP BY "Motorista que entrega", COUNT(*)
```

## 🎨 COMPONENTES DE DASHBOARD

### 1. KPI Cards
```typescript
<KPICard title="Total de Pedidos" value={135} icon={Package} />
<KPICard title="Valor Total" value="R$ XXX.XXX,XX" icon={DollarSign} />
<KPICard title="Peso Total" value="XX toneladas" icon={Weight} />
<KPICard title="Período" value="Abril 2025" icon={Calendar} />
```

### 2. Charts
```typescript
// Line Chart - Pedidos por dia
<LineChart data={ordersPerDay} />

// Bar Chart - Top 10 clientes
<BarChart data={topClients} />

// Pie Chart - Formas de pagamento
<PieChart data={paymentMethods} />

// Heatmap - Rotas mais frequentes
<HeatMap data={routeFrequency} />
```

### 3. Data Table
```typescript
<DataTable
  columns={['Número', 'Data', 'Pagador', 'Origem', 'Destino', 'Valor']}
  data={freightOrders}
  sortable
  filterable
  exportable
/>
```

### 4. Map View
```typescript
// Visualização geográfica das rotas
<MapView
  routes={routesData}
  markers={[origins, destinations]}
  clustering
/>
```

## 🔮 EMBEDDINGS PARA RAG

### Estratégia de Vetorização
```python
# Para cada pedido, criar embedding composto:
text_to_embed = f"""
Pedido {numero}
Data: {data_agendamento}
Origem: {remetente_cidade}
Destino: {destinatario_cidade}
Cliente: {pagador_nome}
Valor: {valor_frete}
Peso: {peso_total}
Volumes: {volumes}
"""

# Gerar embedding (384 dimensions)
embedding = embed_model.encode(text_to_embed)

# Inserir no AvilaDB
await aviladb.vector_index("freight_orders").insert(
    id=pedido_id,
    vector=embedding,
    metadata={
        "numero": numero,
        "data": data_agendamento,
        "cliente": pagador_nome,
        # ... outros campos
    }
)
```

### Queries Semânticas
```python
# Usuário pergunta: "Fretes caros para SP"
query_embedding = embed_model.encode("Fretes com alto valor para São Paulo")

# Busca por similaridade
results = await aviladb.vector_index("freight_orders").search(
    query=query_embedding,
    limit=10,
    filter={"destination_city": "São Paulo"}
)
```

## 📊 ESTRUTURA DE AGREGAÇÃO

### Modelo de Dados Agregados
```rust
// pc-models/src/analytics.rs

#[derive(Debug, Serialize, Deserialize)]
pub struct FreightSummary {
    pub total_orders: usize,
    pub total_value: f64,
    pub total_weight: f64,
    pub total_volume: f64,
    pub date_range: (NaiveDate, NaiveDate),
    pub top_clients: Vec<ClientMetrics>,
    pub top_routes: Vec<RouteMetrics>,
    pub payment_distribution: HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMetrics {
    pub name: String,
    pub order_count: usize,
    pub total_value: f64,
    pub avg_value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteMetrics {
    pub origin: String,
    pub destination: String,
    pub frequency: usize,
    pub avg_value: f64,
    pub avg_weight: f64,
}
```

## 🚀 PRÓXIMOS PASSOS

### FASE 1: Análise Básica (Hoje)
- [ ] Criar script PowerShell para gerar `freight_summary.json`
- [ ] Calcular estatísticas agregadas
- [ ] Identificar top 10 clientes, rotas, motoristas

### FASE 2: Dashboard Frontend (Amanhã)
- [ ] Implementar KPI cards
- [ ] Criar gráficos interativos
- [ ] Implementar tabela paginada
- [ ] Adicionar filtros e busca

### FASE 3: RAG Integration (Esta Semana)
- [ ] Gerar embeddings dos 135 pedidos
- [ ] Indexar no AvilaDB (mock)
- [ ] Implementar busca semântica
- [ ] Criar chat interface

### FASE 4: Bulk Processing (Próxima Semana)
- [ ] Processar 55,807 arquivos do E:
- [ ] Correlacionar CT-e com pedidos
- [ ] Indexar documentos fiscais
- [ ] Criar timeline de entregas

---

**Total de Registros**: 135 pedidos válidos
**Qualidade**: Campos essenciais preenchidos
**Ready for**: Dashboard, Analytics, RAG, ML

**Arquivo**: `d:\Personal-Controller\DATA_ANALYSIS_GUIDE.md`
