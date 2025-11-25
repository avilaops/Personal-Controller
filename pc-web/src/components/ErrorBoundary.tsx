'use client';

import { Component, ReactNode } from 'react';

interface Props {
    children: ReactNode;
    fallback?: ReactNode;
}

interface State {
    hasError: boolean;
    error?: Error;
}

export class ErrorBoundary extends Component<Props, State> {
    constructor(props: Props) {
        super(props);
        this.state = { hasError: false };
    }

    static getDerivedStateFromError(error: Error): State {
        return { hasError: true, error };
    }

    componentDidCatch(error: Error, errorInfo: any) {
        console.error('Error caught by boundary:', error, errorInfo);
    }

    render() {
        if (this.state.hasError) {
            return (
                this.props.fallback || (
                    <div className="min-h-screen flex items-center justify-center bg-gray-50">
                        <div className="max-w-md w-full bg-white shadow-lg rounded-lg p-6">
                            <div className="flex items-center justify-center w-12 h-12 mx-auto bg-red-100 rounded-full">
                                <svg
                                    className="w-6 h-6 text-red-600"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        strokeLinecap="round"
                                        strokeLinejoin="round"
                                        strokeWidth={2}
                                        d="M6 18L18 6M6 6l12 12"
                                    />
                                </svg>
                            </div>
                            <h2 className="mt-4 text-xl font-semibold text-center text-gray-900">
                                Algo deu errado
                            </h2>
                            <p className="mt-2 text-sm text-center text-gray-600">
                                Ocorreu um erro inesperado. Por favor, recarregue a página.
                            </p>
                            {this.state.error && (
                                <details className="mt-4 text-xs text-gray-500">
                                    <summary className="cursor-pointer">Detalhes técnicos</summary>
                                    <pre className="mt-2 p-2 bg-gray-100 rounded overflow-auto">
                                        {this.state.error.message}
                                    </pre>
                                </details>
                            )}
                            <button
                                onClick={() => window.location.reload()}
                                className="mt-6 w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 transition-colors"
                            >
                                Recarregar página
                            </button>
                        </div>
                    </div>
                )
            );
        }

        return this.props.children;
    }
}
