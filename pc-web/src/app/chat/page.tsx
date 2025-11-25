'use client'

import { useState } from 'react'
import { useMutation, useQuery } from '@tanstack/react-query'
import { Send, Bot, User } from 'lucide-react'
import { api } from '@/lib/api'

export default function ChatPage() {
    const [input, setInput] = useState('')

    const { data: history } = useQuery({
        queryKey: ['chatHistory'],
        queryFn: () => api.getChatHistory(),
    })

    const chatMutation = useMutation({
        mutationFn: (query: string) => api.sendChatMessage(query),
    })

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        if (!input.trim()) return

        await chatMutation.mutateAsync(input)
        setInput('')
    }

    return (
        <div className="flex flex-col h-full">
            {/* Header */}
            <div className="border-b bg-white px-8 py-4">
                <h1 className="text-2xl font-bold text-gray-900">Chat com IA</h1>
                <p className="text-gray-600 text-sm mt-1">
                    Converse com a Personal-Controller-LLM
                </p>
            </div>

            {/* Chat Messages */}
            <div className="flex-1 overflow-y-auto p-8 space-y-4">
                {history?.messages?.map((msg: any, idx: number) => (
                    <div
                        key={idx}
                        className={`flex ${msg.role === 'User' ? 'justify-end' : 'justify-start'}`}
                    >
                        <div
                            className={`flex gap-3 max-w-2xl ${msg.role === 'User' ? 'flex-row-reverse' : 'flex-row'
                                }`}
                        >
                            <div
                                className={`w-10 h-10 rounded-full flex items-center justify-center ${msg.role === 'User' ? 'bg-blue-500' : 'bg-purple-500'
                                    }`}
                            >
                                {msg.role === 'User' ? (
                                    <User className="w-6 h-6 text-white" />
                                ) : (
                                    <Bot className="w-6 h-6 text-white" />
                                )}
                            </div>
                            <div
                                className={`px-4 py-3 rounded-lg ${msg.role === 'User'
                                        ? 'bg-blue-500 text-white'
                                        : 'bg-white border border-gray-200 text-gray-900'
                                    }`}
                            >
                                <p className="text-sm">{msg.content}</p>
                                {msg.metadata && (
                                    <div className="mt-2 text-xs opacity-75">
                                        Confiança: {(msg.metadata.confidence * 100).toFixed(0)}%
                                    </div>
                                )}
                            </div>
                        </div>
                    </div>
                ))}

                {chatMutation.isPending && (
                    <div className="flex justify-start">
                        <div className="flex gap-3 max-w-2xl">
                            <div className="w-10 h-10 rounded-full flex items-center justify-center bg-purple-500">
                                <Bot className="w-6 h-6 text-white" />
                            </div>
                            <div className="px-4 py-3 rounded-lg bg-white border border-gray-200">
                                <div className="flex gap-1">
                                    <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" />
                                    <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce delay-100" />
                                    <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce delay-200" />
                                </div>
                            </div>
                        </div>
                    </div>
                )}
            </div>

            {/* Input */}
            <div className="border-t bg-white px-8 py-4">
                <form onSubmit={handleSubmit} className="flex gap-4">
                    <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        placeholder="Digite sua pergunta..."
                        className="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />
                    <button
                        type="submit"
                        disabled={!input.trim() || chatMutation.isPending}
                        className="px-6 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                    >
                        <Send className="w-5 h-5" />
                        Enviar
                    </button>
                </form>
            </div>
        </div>
    )
}
