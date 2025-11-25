import { Clock } from 'lucide-react'

const activities = [
    { id: 1, action: 'Nova ordem de frete criada', time: '5 min atrás' },
    { id: 2, action: 'Empresa atualizada', time: '15 min atrás' },
    { id: 3, action: 'Rota adicionada', time: '1 hora atrás' },
    { id: 4, action: 'Ponto registrado', time: '2 horas atrás' },
]

export default function RecentActivity() {
    return (
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">
                Atividades Recentes
            </h3>
            <div className="space-y-4">
                {activities.map((activity) => (
                    <div key={activity.id} className="flex items-start gap-3">
                        <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
                            <Clock className="w-4 h-4 text-blue-600" />
                        </div>
                        <div className="flex-1 min-w-0">
                            <p className="text-sm font-medium text-gray-900">
                                {activity.action}
                            </p>
                            <p className="text-xs text-gray-500 mt-1">{activity.time}</p>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    )
}
