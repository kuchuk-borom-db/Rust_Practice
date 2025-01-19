import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { Search, Filter, Clock } from 'lucide-react';

const OperationsView = () => {
    const [operations] = useState([
        {
            id: "op-1",
            name: "Upload Profile Picture",
            created: "2024-01-19T14:30:00Z",
            updated: "2024-01-19T14:30:05Z",
        },
        {
            id: "op-2",
            name: "Process Video Content",
            created: "2024-01-19T14:25:00Z",
            updated: "2024-01-19T14:27:30Z",
        },
        {
            id: "op-3",
            name: "Database Backup",
            created: "2024-01-19T14:20:00Z",
            updated: "2024-01-19T14:20:45Z",
        }
    ]);

    // Function to format datetime
    const formatDateTime = (dateString) => {
        const date = new Date(dateString);
        return date.toLocaleString();
    };

    // Function to calculate duration between created and updated
    const calculateDuration = (created, updated) => {
        const start = new Date(created);
        const end = new Date(updated);
        const durationMs = end - start;
        const seconds = Math.floor(durationMs / 1000);

        if (seconds < 60) return `${seconds}s`;
        const minutes = Math.floor(seconds / 60);
        const remainingSeconds = seconds % 60;
        return `${minutes}m ${remainingSeconds}s`;
    };

    return (
        <div className="min-h-screen bg-gradient-to-br from-gray-900 to-gray-800 text-white">
            <div className="container mx-auto px-6 py-16">
                {/* Header Section */}
                <motion.div
                    initial={{ opacity: 0, y: -20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.8 }}
                    className="mb-12"
                >
                    <h1 className="text-4xl font-bold mb-4">Operations</h1>
                    <p className="text-gray-300">Monitor and track your system operations in real-time</p>
                </motion.div>

                {/* Search and Filter Section */}
                <motion.div
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    transition={{ duration: 0.8, delay: 0.2 }}
                    className="mb-8 flex flex-col md:flex-row gap-4"
                >
                    <div className="relative flex-grow">
                        <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
                        <input
                            type="text"
                            placeholder="Search operations..."
                            className="w-full bg-gray-800 border border-gray-700 rounded-lg py-2 px-10 focus:outline-none focus:border-blue-500 transition-colors duration-200"
                        />
                    </div>
                    <button className="flex items-center justify-center gap-2 bg-gray-800 px-4 py-2 rounded-lg hover:bg-gray-700 transition-colors duration-200">
                        <Filter className="w-5 h-5" />
                        <span>Filter</span>
                    </button>
                </motion.div>

                {/* Operations List */}
                <div className="space-y-4">
                    {operations.map((operation, index) => (
                        <motion.div
                            key={operation.id}
                            initial={{ opacity: 0, x: -20 }}
                            animate={{ opacity: 1, x: 0 }}
                            transition={{ duration: 0.5, delay: index * 0.1 }}
                            className="bg-gray-800 rounded-lg p-6 hover:bg-gray-700 transition-all duration-200"
                        >
                            <div className="flex items-center justify-between">
                                <div className="flex items-center gap-4">
                                    <div className="bg-blue-500/10 p-3 rounded-lg">
                                        <Clock className="w-5 h-5" />
                                    </div>
                                    <div>
                                        <h3 className="text-lg font-semibold">{operation.name}</h3>
                                        <p className="text-gray-400 text-sm">ID: {operation.id}</p>
                                    </div>
                                </div>
                                <div className="flex items-center gap-6">
                                    <div className="text-right mr-6">
                                        <p className="text-sm text-gray-400">Created: {formatDateTime(operation.created)}</p>
                                        <p className="text-sm text-gray-400">Updated: {formatDateTime(operation.updated)}</p>
                                        <p className="text-sm text-gray-400">
                                            Duration: {calculateDuration(operation.created, operation.updated)}
                                        </p>
                                    </div>
                                    <div className="flex gap-3">
                                        <button
                                            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors duration-200"
                                            onClick={() => console.log('Visual Flow Logs clicked for:', operation.id)}
                                        >
                                            Visual Flow Logs
                                        </button>
                                        <button
                                            className="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded-lg transition-colors duration-200"
                                            onClick={() => console.log('Logs clicked for:', operation.id)}
                                        >
                                            Logs
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </motion.div>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default OperationsView;