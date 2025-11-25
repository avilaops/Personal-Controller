'use client';

import { useEffect, useState } from 'react';
import { Package, DollarSign, Weight, Calendar, TrendingUp, MapPin } from 'lucide-react';

interface FreightSummary {
  timestamp: string;
  total_records: number;
  metrics: {
    total_value: number;
    total_weight: number;
    total_volume: number;
    avg_value: number;
    avg_weight: number;
    avg_volume: number;
  };
  date_range: {
    start: string;
    end: string;
    days: number;
  };
  top_clients: Array<{
    name: string;
    order_count: number;
    total_value: number;
    avg_value: number;
  }>;
  top_routes: Array<{
    route: string;
    frequency: number;
    total_value: number;
    avg_value: number;
  }>;
  payment_methods: Array<{
    method: string;
    count: number;
    percentage: number;
  }>;
}

export default function Home() {
  const [summary, setSummary] = useState<FreightSummary | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/data/freight_summary.json')
      .then(res => res.json())
      .then(data => {
        setSummary(data);
        setLoading(false);
      })
      .catch(err => {
        console.error('Error loading summary:', err);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center">
        <div className="text-xl text-gray-700">Carregando dados...</div>
      </div>
    );
  }

  if (!summary) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center">
        <div className="text-xl text-red-600">Erro ao carregar dados</div>
      </div>
    );
  }

  const formatCurrency = (value: number) => 
    new Intl.NumberFormat('pt-BR', { style: 'currency', currency: 'BRL' }).format(value);

  const formatNumber = (value: number) => 
    new Intl.NumberFormat('pt-BR').format(value);

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 py-6 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">Personal Controller</h1>
              <p className="text-sm text-gray-600 mt-1">Plataforma unificada de gestão empresarial</p>
            </div>
            <div className="text-right">
              <p className="text-sm text-gray-500">Última atualização</p>
              <p className="text-sm font-medium text-gray-900">{new Date(summary.timestamp).toLocaleString('pt-BR')}</p>
            </div>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 py-8 sm:px-6 lg:px-8">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <KPICard 
            title="Total de Pedidos"
            value={summary.total_records.toString()}
            icon={Package}
            color="blue"
          />
          <KPICard 
            title="Valor Total"
            value={formatCurrency(summary.metrics.total_value)}
            icon={DollarSign}
            color="green"
          />
          <KPICard 
            title="Peso Total"
            value={`${formatNumber(summary.metrics.total_weight)} kg`}
            icon={Weight}
            color="purple"
          />
          <KPICard 
            title="Período"
            value={`${summary.date_range.days} dias`}
            icon={Calendar}
            color="orange"
            subtitle={`${summary.date_range.start} a ${summary.date_range.end}`}
          />
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow-md p-6">
            <div className="flex items-center mb-4">
              <TrendingUp className="h-6 w-6 text-blue-600 mr-2" />
              <h2 className="text-xl font-semibold text-gray-900">Top Clientes</h2>
            </div>
            <div className="space-y-3">
              {summary.top_clients.slice(0, 5).map((client, index) => (
                <div key={index} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                  <div className="flex-1">
                    <p className="text-sm font-medium text-gray-900 truncate">{client.name}</p>
                    <p className="text-xs text-gray-500">{client.order_count} pedidos</p>
                  </div>
                  <div className="text-right">
                    <p className="text-sm font-bold text-blue-600">{formatCurrency(client.total_value)}</p>
                    <p className="text-xs text-gray-500">Média: {formatCurrency(client.avg_value)}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>

          <div className="bg-white rounded-lg shadow-md p-6">
            <div className="flex items-center mb-4">
              <DollarSign className="h-6 w-6 text-green-600 mr-2" />
              <h2 className="text-xl font-semibold text-gray-900">Formas de Pagamento</h2>
            </div>
            <div className="space-y-3">
              {summary.payment_methods.map((method, index) => (
                <div key={index} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                  <div className="flex-1">
                    <p className="text-sm font-medium text-gray-900">{method.method}</p>
                    <div className="mt-1 w-full bg-gray-200 rounded-full h-2">
                      <div 
                        className="bg-green-600 h-2 rounded-full" 
                        style={{ width: `${method.percentage}%` }}
                      />
                    </div>
                  </div>
                  <div className="ml-4 text-right">
                    <p className="text-sm font-bold text-green-600">{method.count}</p>
                    <p className="text-xs text-gray-500">{method.percentage.toFixed(1)}%</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {summary.top_routes.length > 0 && (
          <div className="bg-white rounded-lg shadow-md p-6 mb-8">
            <div className="flex items-center mb-4">
              <MapPin className="h-6 w-6 text-purple-600 mr-2" />
              <h2 className="text-xl font-semibold text-gray-900">Rotas Principais</h2>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {summary.top_routes.slice(0, 6).map((route, index) => (
                <div key={index} className="p-4 bg-gradient-to-br from-purple-50 to-pink-50 rounded-lg border border-purple-200">
                  <p className="text-sm font-medium text-gray-900">{route.route}</p>
                  <div className="mt-2 flex items-center justify-between">
                    <span className="text-xs text-gray-600">{route.frequency} viagens</span>
                    <span className="text-xs font-bold text-purple-600">{formatCurrency(route.avg_value)}/viagem</span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        <div className="bg-white rounded-lg shadow-md p-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-4">Médias Gerais</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="text-center p-4 bg-blue-50 rounded-lg">
              <p className="text-sm text-gray-600 mb-1">Valor Médio por Pedido</p>
              <p className="text-2xl font-bold text-blue-600">{formatCurrency(summary.metrics.avg_value)}</p>
            </div>
            <div className="text-center p-4 bg-purple-50 rounded-lg">
              <p className="text-sm text-gray-600 mb-1">Peso Médio por Pedido</p>
              <p className="text-2xl font-bold text-purple-600">{formatNumber(summary.metrics.avg_weight)} kg</p>
            </div>
            <div className="text-center p-4 bg-green-50 rounded-lg">
              <p className="text-sm text-gray-600 mb-1">Volume Médio por Pedido</p>
              <p className="text-2xl font-bold text-green-600">{formatNumber(summary.metrics.avg_volume)}</p>
            </div>
          </div>
        </div>
      </main>

      <footer className="bg-white border-t border-gray-200 mt-12">
        <div className="max-w-7xl mx-auto px-4 py-6 sm:px-6 lg:px-8">
          <p className="text-center text-sm text-gray-500">
            Personal Controller v0.1.0 • {summary.total_records} pedidos analisados • Desenvolvido por Nícolas Ávila
          </p>
        </div>
      </footer>
    </div>
  );
}

interface KPICardProps {
  title: string;
  value: string;
  icon: React.ElementType;
  color: 'blue' | 'green' | 'purple' | 'orange';
  subtitle?: string;
}

function KPICard({ title, value, icon: Icon, color, subtitle }: KPICardProps) {
  const colorClasses = {
    blue: 'bg-blue-50 text-blue-600 border-blue-200',
    green: 'bg-green-50 text-green-600 border-green-200',
    purple: 'bg-purple-50 text-purple-600 border-purple-200',
    orange: 'bg-orange-50 text-orange-600 border-orange-200',
  };

  return (
    <div className="bg-white rounded-lg shadow-md p-6 border-l-4 border-gray-200 hover:shadow-lg transition-shadow">
      <div className="flex items-center justify-between mb-2">
        <p className="text-sm font-medium text-gray-600">{title}</p>
        <div className={`p-2 rounded-lg ${colorClasses[color]}`}>
          <Icon className="h-5 w-5" />
        </div>
      </div>
      <p className="text-2xl font-bold text-gray-900">{value}</p>
      {subtitle && <p className="text-xs text-gray-500 mt-1">{subtitle}</p>}
    </div>
  );
}
