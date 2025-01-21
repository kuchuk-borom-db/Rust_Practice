import React, { useState, useCallback } from 'react';
import ReactFlow, {
    MiniMap,
    Controls,
    Background,
    useNodesState,
    useEdgesState,
    MarkerType,
    Panel,
} from 'reactflow';
import { AnimatePresence, motion } from 'framer-motion';
import { ChevronDown, Database, MessageSquare, Phone, Layout } from 'lucide-react';
import 'reactflow/dist/style.css';

const GROUP_PADDING = 50;
const NODE_HEIGHT = 60;
const NODE_WIDTH = 250;
const NODE_SPACING = 30;

// Custom Node Types
const FlowNode = ({ data }) => {
    const getIcon = () => {
        switch (data.type) {
            case 'Log':
                return <MessageSquare className="w-4 h-4 text-green-500" />;
            case 'CallStore':
                return <Database className="w-4 h-4 text-purple-500" />;
            case 'Call':
                return <Phone className="w-4 h-4 text-blue-500" />;
            default:
                return null;
        }
    };

    const getBorderColor = () => {
        switch (data.type) {
            case 'Log':
                return 'border-green-500/50';
            case 'CallStore':
                return 'border-purple-500/50';
            case 'Call':
                return 'border-blue-500/50';
            default:
                return 'border-gray-500/50';
        }
    };

    return (
        <div
            className={`px-4 py-2 rounded-lg backdrop-blur-sm border-2 ${getBorderColor()} 
        bg-gray-800/30 hover:bg-gray-800/40 shadow-lg w-[${NODE_WIDTH}px]`}
        >
            <div className="flex items-center justify-between text-gray-300">
                <div className="flex items-center space-x-2">
                    {getIcon()}
                    <span className="truncate">{data.value}</span>
                </div>
                {(data.type === 'Call' || data.type === 'CallStore') && (
                    <button
                        onClick={data.onToggle}
                        className="p-1 hover:bg-gray-700 rounded-full transition-colors"
                    >
                        <ChevronDown
                            className={`w-4 h-4 transition-transform duration-300 ${
                                data.isExpanded ? 'rotate-180' : ''
                            }`}
                        />
                    </button>
                )}
            </div>
        </div>
    );
};

const GroupNode = ({ data }) => {
    const isStartBlock = data.id === 'START';

    return (
        <div
            className={`rounded-xl backdrop-blur-md border-2 
        ${isStartBlock ? 'border-blue-400/50' : 'border-gray-600/50'}
        bg-gray-800/50 p-4`}
        >
            <div className="text-lg font-semibold text-gray-200 mb-4 text-center border-b-2 border-gray-700/50 pb-2">
                {data.label} {isStartBlock && '(START)'}
            </div>
        </div>
    );
};

const nodeTypes = {
    flowNode: FlowNode,
    group: GroupNode,
};

const DS3Diagram = ({ data }) => {
    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);
    const [expandedNodes, setExpandedNodes] = useState(new Set());

    const toggleNodeExpansion = useCallback((nodeId) => {
        setExpandedNodes(prev => {
            const next = new Set(prev);
            if (next.has(nodeId)) {
                next.delete(nodeId);
            } else {
                next.add(nodeId);
            }
            return next;
        });
    }, []);

    const processBlock = useCallback((
        blockId,
        position,
        visited = new Set(),
        level = 0
    ) => {
        if (visited.has(blockId)) return { nodes: [], edges: [], width: 0, height: 0 };
        visited.add(blockId);

        const block = data[blockId];
        if (!block) return { nodes: [], edges: [], width: 0, height: 0 };

        let currentNodes = [];
        let currentEdges = [];
        let flowNodesHeight = 0;

        // Process flow nodes
        const flowNodes = block.flow.map((flow, index) => {
            const nodeId = `${blockId}-flow-${index}`;
            const yPosition = flowNodesHeight;
            flowNodesHeight += NODE_HEIGHT + NODE_SPACING;

            return {
                id: nodeId,
                type: 'flowNode',
                position: { x: GROUP_PADDING, y: yPosition + GROUP_PADDING },
                data: {
                    type: flow.flow_type,
                    value: flow.value,
                    isExpanded: expandedNodes.has(nodeId),
                    onToggle: () => toggleNodeExpansion(nodeId),
                    flowPointerId: flow.flow_pointer_id,
                },
                parentNode: `group-${blockId}`,
                extent: 'parent',
            };
        });

        // Create group node
        const groupNode = {
            id: `group-${blockId}`,
            type: 'group',
            position,
            data: {
                label: block.name,
                id: blockId,
            },
            style: {
                width: NODE_WIDTH + (GROUP_PADDING * 2),
                height: flowNodesHeight + (GROUP_PADDING * 2),
            },
        };

        currentNodes = [groupNode, ...flowNodes];

        // Process edges between flow nodes
        block.flow.forEach((flow, index) => {
            const sourceId = `${blockId}-flow-${index}`;

            // Edge to next flow node in same block
            if (index < block.flow.length - 1) {
                currentEdges.push({
                    id: `edge-${sourceId}-${blockId}-flow-${index + 1}`,
                    source: sourceId,
                    target: `${blockId}-flow-${index + 1}`,
                    type: 'smoothstep',
                    markerEnd: { type: MarkerType.ArrowClosed },
                });
            }

            // If node is expanded and has a flow pointer, process the linked block
            if (
                expandedNodes.has(sourceId) &&
                flow.flow_pointer_id &&
                (flow.flow_type === 'Call' || flow.flow_type === 'CallStore')
            ) {
                const nextPosition = {
                    x: position.x + NODE_WIDTH + GROUP_PADDING * 4,
                    y: position.y,
                };

                const {
                    nodes: subNodes,
                    edges: subEdges,
                } = processBlock(flow.flow_pointer_id, nextPosition, visited, level + 1);

                if (subNodes.length > 0) {
                    currentNodes = [...currentNodes, ...subNodes];
                    currentEdges = [...currentEdges, ...subEdges];

                    // Edge from flow node to first node of called block
                    currentEdges.push({
                        id: `edge-${sourceId}-${flow.flow_pointer_id}-start`,
                        source: sourceId,
                        target: `${flow.flow_pointer_id}-flow-0`,
                        type: 'smoothstep',
                        markerEnd: { type: MarkerType.ArrowClosed },
                        animated: true,
                    });

                    // If CallStore, add return edge from last node of called block
                    if (flow.flow_type === 'CallStore') {
                        const lastFlowIndex = data[flow.flow_pointer_id].flow.length - 1;
                        currentEdges.push({
                            id: `edge-return-${flow.flow_pointer_id}-${sourceId}`,
                            source: `${flow.flow_pointer_id}-flow-${lastFlowIndex}`,
                            target: sourceId,
                            type: 'smoothstep',
                            markerEnd: { type: MarkerType.ArrowClosed },
                            animated: true,
                            style: { stroke: '#9333ea' },
                        });
                    }
                }
            }
        });

        return { nodes: currentNodes, edges: currentEdges };
    }, [data, expandedNodes, toggleNodeExpansion]);

    // Initialize diagram
    React.useEffect(() => {
        const { nodes: initialNodes, edges: initialEdges } = processBlock('START', { x: 0, y: 0 });
        setNodes(initialNodes);
        setEdges(initialEdges);
    }, [data, processBlock, setNodes, setEdges, expandedNodes]);

    const [orientation, setOrientation] = useState('TB');

    return (
        <div className="w-full h-screen bg-gradient-to-br from-gray-900 to-gray-800">
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                nodeTypes={nodeTypes}
                fitView
                className="bg-transparent"
                direction={orientation}
            >
                <Controls className="bg-gray-800/50 border-gray-700" />
                <MiniMap className="bg-gray-800/50" />
                <Background color="#4a5568" gap={16} />
                <Panel position="top-right">
                    <button
                        onClick={() => setOrientation(prev => prev === 'TB' ? 'LR' : 'TB')}
                        className="p-4 rounded-full bg-gray-800 border-2 border-gray-600 shadow-lg hover:shadow-xl transition-all duration-300 hover:bg-gray-700 hover:border-gray-500 group"
                    >
                        <Layout
                            className={`w-6 h-6 text-gray-300 group-hover:text-white transition-all duration-300 transform ${
                                orientation === 'LR' ? 'rotate-90' : 'rotate-0'
                            }`}
                        />
                    </button>
                </Panel>
            </ReactFlow>
        </div>
    );
};

export default DS3Diagram;