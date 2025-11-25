# 📚 Personal Controller API Documentation

## Overview

Personal Controller API is a REST API for managing Ávila Transportes operations with built-in AI capabilities.

**Base URL:** `http://localhost:3000`
**API Version:** v1
**Authentication:** Coming soon (JWT)

---

## 🚀 Quick Start

### Starting the Server

```bash
cd pc-api
cargo run
```

Server will start on `http://localhost:3000`

### Health Check

```bash
curl http://localhost:3000/health
```

---

## 📋 Endpoints

### Root

#### GET /

Returns API information and available endpoints.

**Response:**
```json
{
  "name": "Personal Controller API",
  "version": "0.1.0",
  "description": "REST API for Ávila Transportes Personal Controller",
  "status": "online",
  "endpoints": {
    "companies": "/api/v1/companies",
    "freight_orders": "/api/v1/freight-orders",
    "timesheets": "/api/v1/timesheets",
    "routes": "/api/v1/routes",
    "chat": "/api/v1/chat",
    "stats": "/api/v1/stats"
  }
}
```

---

## 🏢 Companies API

### List Companies

**GET** `/api/v1/companies`

Query Parameters:
- `page` (number, default: 1) - Page number
- `per_page` (number, default: 10) - Items per page

**Example:**
```bash
curl "http://localhost:3000/api/v1/companies?page=1&per_page=10"
```

**Response:**
```json
{
  "data": [
    {
      "id": "company-123",
      "nome": "Empresa Exemplo LTDA",
      "cnpj": "12345678000199",
      "tipo": "cliente",
      "cidade": "São Paulo",
      "estado": "SP"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 10,
    "total": 42,
    "total_pages": 5
  }
}
```

### Create Company

**POST** `/api/v1/companies`

**Request Body:**
```json
{
  "nome": "Nova Empresa LTDA",
  "cnpj": "12345678000199",
  "tipo": "cliente",
  "cidade": "São Paulo",
  "estado": "SP",
  "telefone": "(11) 98765-4321",
  "email": "contato@empresa.com"
}
```

**Response:** `201 Created`
```json
{
  "id": "new-company-id",
  "message": "Company created successfully"
}
```

### Get Company

**GET** `/api/v1/companies/:id`

**Example:**
```bash
curl http://localhost:3000/api/v1/companies/company-123
```

**Response:**
```json
{
  "id": "company-123",
  "nome": "Empresa Exemplo LTDA",
  "cnpj": "12345678000199",
  "tipo": "cliente",
  "cidade": "São Paulo",
  "estado": "SP"
}
```

### Update Company

**PUT** `/api/v1/companies/:id`

**Request Body:**
```json
{
  "nome": "Empresa Atualizada LTDA",
  "telefone": "(11) 91234-5678"
}
```

**Response:**
```json
{
  "id": "company-123",
  "message": "Company updated successfully"
}
```

### Delete Company

**DELETE** `/api/v1/companies/:id`

**Response:** `204 No Content`

---

## 🚚 Freight Orders API

### List Freight Orders

**GET** `/api/v1/freight-orders`

Query Parameters:
- `page` (number, default: 1)
- `per_page` (number, default: 10)

**Response:**
```json
{
  "data": [
    {
      "id": "freight-456",
      "numero": "288415",
      "remetente_cidade": "São Paulo",
      "destinatario_cidade": "Rio de Janeiro",
      "valor_frete": 1500.00,
      "status": "in_transit"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 10,
    "total": 156,
    "total_pages": 16
  }
}
```

### Create Freight Order

**POST** `/api/v1/freight-orders`

**Request Body:**
```json
{
  "numero": "288415",
  "remetente_cidade": "São Paulo",
  "remetente_estado": "SP",
  "destinatario_cidade": "Rio de Janeiro",
  "destinatario_estado": "RJ",
  "valor_frete": 1500.00,
  "status": "pending"
}
```

**Response:** `201 Created`

### Get Freight Order

**GET** `/api/v1/freight-orders/:id`

### Update Freight Order

**PUT** `/api/v1/freight-orders/:id`

### Delete Freight Order

**DELETE** `/api/v1/freight-orders/:id`

---

## ⏰ Timesheets API

### List Timesheets

**GET** `/api/v1/timesheets`

### Create Timesheet

**POST** `/api/v1/timesheets`

**Request Body:**
```json
{
  "funcionario_id": "emp-123",
  "data": "2025-11-24",
  "hora_entrada": "08:00:00",
  "hora_saida": "17:30:00",
  "horas_trabalhadas": 8.5
}
```

### Get Timesheet

**GET** `/api/v1/timesheets/:id`

### Update Timesheet

**PUT** `/api/v1/timesheets/:id`

### Delete Timesheet

**DELETE** `/api/v1/timesheets/:id`

---

## 🗺️ Routes API

### List Routes

**GET** `/api/v1/routes`

### Create Route

**POST** `/api/v1/routes`

**Request Body:**
```json
{
  "origem": "São Paulo - SP",
  "destino": "Rio de Janeiro - RJ",
  "distancia_km": 430,
  "tempo_estimado_horas": 6.5
}
```

### Get Route

**GET** `/api/v1/routes/:id`

### Update Route

**PUT** `/api/v1/routes/:id`

### Delete Route

**DELETE** `/api/v1/routes/:id`

---

## 🤖 LLM Chat API

### Chat with AI

**POST** `/api/v1/chat`

Interact with the Personal Controller LLM for intelligent queries about your data.

**Request Body:**
```json
{
  "query": "Quantas ordens de frete estão pendentes?"
}
```

**Response:**
```json
{
  "response": "Existem 15 ordens de frete pendentes no momento.",
  "sources": [
    "FreightOrder(288415)",
    "FreightOrder(288416)"
  ],
  "confidence": 0.92,
  "tokens_used": 245
}
```

### Get Chat History

**GET** `/api/v1/chat/history`

**Response:**
```json
{
  "messages": [
    {
      "role": "User",
      "content": "Quantas empresas temos?",
      "timestamp": "2025-11-24T10:30:00Z"
    },
    {
      "role": "Assistant",
      "content": "Temos 42 empresas cadastradas.",
      "timestamp": "2025-11-24T10:30:02Z"
    }
  ],
  "count": 10
}
```

### Clear Chat History

**POST** `/api/v1/chat/clear`

**Response:**
```json
{
  "message": "Chat history cleared"
}
```

---

## 📊 Statistics API

### General Statistics

**GET** `/api/v1/stats`

**Response:**
```json
{
  "companies": 42,
  "freight_orders": 156,
  "timesheets": 890,
  "routes": 25,
  "total_revenue": 125000.00
}
```

### Company Statistics

**GET** `/api/v1/stats/companies`

**Response:**
```json
{
  "total": 42,
  "by_type": {
    "cliente": 30,
    "fornecedor": 8,
    "transportadora": 4
  }
}
```

### Freight Statistics

**GET** `/api/v1/stats/freight`

**Response:**
```json
{
  "total": 156,
  "by_status": {
    "pending": 15,
    "in_transit": 8,
    "delivered": 128,
    "cancelled": 5
  },
  "total_value": 234500.00
}
```

### Timesheet Statistics

**GET** `/api/v1/stats/timesheets`

**Response:**
```json
{
  "total_hours": 7120.5,
  "total_employees": 25,
  "average_hours_per_day": 8.2
}
```

---

## 🔒 Error Responses

All endpoints may return the following error responses:

### 400 Bad Request
```json
{
  "error": "Invalid request",
  "message": "Field 'nome' is required"
}
```

### 404 Not Found
```json
{
  "error": "Not found",
  "message": "Company with id 'xyz' not found"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal server error",
  "message": "An unexpected error occurred"
}
```

---

## 🌐 CORS

CORS is enabled for all origins in development mode.

---

## 📝 Rate Limiting

Coming soon: Rate limiting of 100 requests per minute per IP.

---

## 🔐 Authentication

Coming soon: JWT-based authentication.

**Example:**
```bash
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  http://localhost:3000/api/v1/companies
```

---

## 🧪 Testing

### Using curl

```bash
# List companies
curl http://localhost:3000/api/v1/companies

# Create company
curl -X POST http://localhost:3000/api/v1/companies \
  -H "Content-Type: application/json" \
  -d '{"nome": "Test Company", "cnpj": "12345678000199", "tipo": "cliente"}'

# Chat with AI
curl -X POST http://localhost:3000/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"query": "Quantas ordens de frete temos?"}'
```

### Using the Example Client

```bash
cd Personal-Controller
cargo run --example api_client
```

---

## 📚 Additional Resources

- [Personal Controller README](./README.md)
- [Architecture Documentation](./ARCHITECTURE.md)
- [LLM Documentation](./pc-llm/README.md)

---

**Built with ❤️ by Ávila Transportes**
