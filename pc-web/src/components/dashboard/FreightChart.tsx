'use client'

import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts'

const data = [
    { month: 'Jan', fretes: 45 },
    { month: 'Fev', fretes: 52 },
    { month: 'Mar', fretes: 48 },
    { month: 'Abr', fretes: 61 },
    { month: 'Mai', fretes: 55 },
    { month: 'Jun', fretes: 67 },
]

export default function FreightChart() {
    return (
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">
                Ordens de Frete por Mês
            </h3>
            <ResponsiveContainer width="100%" height={300}>
                <BarChart data={data}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="month" />
                    <YAxis />
                    <Tooltip />
                    <Bar dataKey="fretes" fill="#3b82f6" />
                </BarChart>
            </ResponsiveContainer>
        </div>
    )
}
