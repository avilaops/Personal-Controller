'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { LayoutDashboard, MessageSquare, Building2, Truck, Clock, Route } from 'lucide-react'
import { cn } from '@/lib/utils'

const navigation = [
    { name: 'Dashboard', href: '/', icon: LayoutDashboard },
    { name: 'Chat IA', href: '/chat', icon: MessageSquare },
    { name: 'Empresas', href: '/companies', icon: Building2 },
    { name: 'Fretes', href: '/freight', icon: Truck },
    { name: 'Horas', href: '/timesheets', icon: Clock },
    { name: 'Rotas', href: '/routes', icon: Route },
]

export default function Sidebar() {
    const pathname = usePathname()

    return (
        <div className="w-64 bg-gray-900 text-white flex flex-col">
            {/* Logo */}
            <div className="p-6 border-b border-gray-800">
                <h1 className="text-xl font-bold">Personal Controller</h1>
                <p className="text-sm text-gray-400 mt-1">Ávila Transportes</p>
            </div>

            {/* Navigation */}
            <nav className="flex-1 p-4 space-y-1">
                {navigation.map((item) => {
                    const isActive = pathname === item.href
                    return (
                        <Link
                            key={item.name}
                            href={item.href}
                            className={cn(
                                'flex items-center gap-3 px-4 py-3 rounded-lg transition-colors',
                                isActive
                                    ? 'bg-blue-600 text-white'
                                    : 'text-gray-300 hover:bg-gray-800'
                            )}
                        >
                            <item.icon className="w-5 h-5" />
                            <span>{item.name}</span>
                        </Link>
                    )
                })}
            </nav>

            {/* Footer */}
            <div className="p-4 border-t border-gray-800 text-sm text-gray-400">
                <p>v0.1.0</p>
                <p className="mt-1">© 2025 Ávila</p>
            </div>
        </div>
    )
}
