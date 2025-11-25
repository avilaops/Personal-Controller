import { LucideIcon } from 'lucide-react'
import { cn } from '@/lib/utils'

interface StatCardProps {
    title: string
    value: string | number
    icon: LucideIcon
    trend?: string
    trendUp?: boolean
}

export default function StatCard({ title, value, icon: Icon, trend, trendUp }: StatCardProps) {
    return (
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <div className="flex items-center justify-between mb-4">
                <h3 className="text-sm font-medium text-gray-600">{title}</h3>
                <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center">
                    <Icon className="w-5 h-5 text-blue-600" />
                </div>
            </div>
            <div className="flex items-end justify-between">
                <p className="text-2xl font-bold text-gray-900">{value}</p>
                {trend && (
                    <span
                        className={cn(
                            'text-sm font-medium',
                            trendUp ? 'text-green-600' : 'text-red-600'
                        )}
                    >
                        {trend}
                    </span>
                )}
            </div>
        </div>
    )
}
