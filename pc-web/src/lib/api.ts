import axios from 'axios'

const client = axios.create({
    baseURL: 'http://localhost:3000/api/v1',
    headers: {
        'Content-Type': 'application/json',
    },
})

export const api = {
    // Stats
    getStats: async () => {
        const { data } = await client.get('/stats')
        return data
    },

    // Chat
    sendChatMessage: async (query: string) => {
        const { data } = await client.post('/chat', { query })
        return data
    },

    getChatHistory: async () => {
        const { data } = await client.get('/chat/history')
        return data
    },

    clearChatHistory: async () => {
        const { data } = await client.post('/chat/clear')
        return data
    },

    // Companies
    getCompanies: async (page = 1, perPage = 10) => {
        const { data } = await client.get('/companies', {
            params: { page, per_page: perPage },
        })
        return data
    },

    getCompany: async (id: string) => {
        const { data } = await client.get(`/companies/${id}`)
        return data
    },

    createCompany: async (company: any) => {
        const { data } = await client.post('/companies', company)
        return data
    },

    updateCompany: async (id: string, company: any) => {
        const { data } = await client.put(`/companies/${id}`, company)
        return data
    },

    deleteCompany: async (id: string) => {
        await client.delete(`/companies/${id}`)
    },

    // Freight Orders
    getFreightOrders: async (page = 1, perPage = 10) => {
        const { data } = await client.get('/freight-orders', {
            params: { page, per_page: perPage },
        })
        return data
    },

    getFreightOrder: async (id: string) => {
        const { data } = await client.get(`/freight-orders/${id}`)
        return data
    },

    createFreightOrder: async (order: any) => {
        const { data } = await client.post('/freight-orders', order)
        return data
    },

    updateFreightOrder: async (id: string, order: any) => {
        const { data } = await client.put(`/freight-orders/${id}`, order)
        return data
    },

    deleteFreightOrder: async (id: string) => {
        await client.delete(`/freight-orders/${id}`)
    },
}
