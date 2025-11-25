# 🌐 Personal Controller Web

Frontend web para o Personal Controller da Ávila Transportes.

## 🚀 Tecnologias

- **Next.js 15** - Framework React
- **TypeScript** - Type safety
- **Tailwind CSS** - Styling
- **React Query** - Data fetching
- **Recharts** - Data visualization
- **Axios** - HTTP client

## 📦 Instalação

```bash
cd pc-web
npm install
```

## 🏃 Development

```bash
npm run dev
```

Abra [http://localhost:3001](http://localhost:3001) no navegador.

## 🏗️ Build

```bash
npm run build
npm start
```

## 📁 Estrutura

```
pc-web/
├── src/
│   ├── app/              # Pages (Next.js App Router)
│   │   ├── page.tsx      # Dashboard
│   │   ├── chat/         # Chat com IA
│   │   ├── companies/    # Empresas
│   │   └── layout.tsx    # Layout principal
│   ├── components/       # Componentes React
│   │   ├── layout/       # Layout (Sidebar, Header)
│   │   ├── dashboard/    # Dashboard components
│   │   └── providers/    # Context providers
│   └── lib/              # Utilities
│       ├── api.ts        # API client
│       └── utils.ts      # Helper functions
├── public/               # Static assets
└── package.json
```

## 🎨 Features

- ✅ Dashboard com estatísticas
- ✅ Chat com IA (Personal-Controller-LLM)
- ✅ Gerenciamento de empresas
- ✅ Ordens de frete
- ✅ Registros de ponto
- ✅ Rotas de transporte
- ✅ Gráficos e visualizações

## 🔌 API

O frontend se conecta com a API REST em `http://localhost:3000`.

Certifique-se de que o servidor está rodando:

```bash
cd ../pc-api
cargo run
```

## 🎯 Próximos Passos

- [ ] Implementar páginas de Companies, Freight, Timesheets, Routes
- [ ] Adicionar autenticação JWT
- [ ] Implementar formulários de criação/edição
- [ ] Adicionar filtros e busca
- [ ] Melhorar UI/UX com animações
- [ ] Adicionar testes (Jest, React Testing Library)

---

**Built with ❤️ by Ávila Transportes**
