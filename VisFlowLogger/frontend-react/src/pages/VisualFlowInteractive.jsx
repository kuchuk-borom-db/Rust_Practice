import React, { useCallback } from 'react';
import ReactFlow, {
    MiniMap,
    Controls,
    Background,
    useNodesState,
    useEdgesState,
    MarkerType,
} from 'reactflow';
import 'reactflow/dist/style.css';

const CustomNode = ({ data }) => (
    <div className={`px-4 py-2 shadow-md ${data.isOutput ? 'rounded-tr-3xl rounded-bl-3xl' : 'rounded-lg'}`}
         style={{ background: data.background || 'white', border: '1px solid #ddd' }}>
        {data.label}
    </div>
);

const nodeTypes = {
    custom: CustomNode,
};

const initialNodes = [
    // calculateInvestment (START)
    {
        id: 'start',
        type: 'group',
        draggable: false,
        data: { label: 'calculateInvestment' },
        position: { x: 300, y: 0 },
        style: { backgroundColor: '#885B23', padding: 20, borderRadius: 8 },
    },
    {
        id: 'calc-inv-1',
        type: 'custom',
        draggable: false,
        parentNode: 'start',
        data: { label: 'Calculating investment for 1000' },
        position: { x: 20, y: 40 },
    },
    {
        id: 'calc-inv-2',
        type: 'custom',
        draggable: false,
        parentNode: 'start',
        data: { label: 'final Amount = 1210', isOutput: true },
        position: { x: 20, y: 120 },
    },
    {
        id: 'calc-inv-3',
        type: 'custom',
        draggable: false,
        parentNode: 'start',
        data: { label: 'logTransaction' },
        position: { x: 20, y: 200 },
    },

    // addMoney (1)
    {
        id: 'add-money-1',
        type: 'group',
        draggable: false,
        data: { label: 'addMoney' },
        position: { x: 0, y: 200 },
        style: { backgroundColor: '#8D3EC1', padding: 20, borderRadius: 8 },
    },
    {
        id: 'add-1-1',
        type: 'custom',
        draggable: false,
        parentNode: 'add-money-1',
        data: { label: 'Adding money (100) to balance 1000' },
        position: { x: 20, y: 40 },
    },
    {
        id: 'add-1-2',
        type: 'custom',
        draggable: false,
        parentNode: 'add-money-1',
        data: { label: 'Balance = 1100' },
        position: { x: 20, y: 120 },
    },

    // calculateInterest (1)
    {
        id: 'calc-int-1',
        type: 'group',
        draggable: false,
        data: { label: 'calculateInterest' },
        position: { x: 300, y: 200 },
        style: { backgroundColor: '#CF4723', padding: 20, borderRadius: 8 },
    },
    {
        id: 'ci-1-1',
        type: 'custom',
        draggable: false,
        parentNode: 'calc-int-1',
        data: { label: 'Calculating interest for 1100' },
        position: { x: 20, y: 40 },
    },
    {
        id: 'ci-1-2',
        type: 'custom',
        draggable: false,
        parentNode: 'calc-int-1',
        data: { label: 'Interest = 110' },
        position: { x: 20, y: 120 },
    },

    // getCompoundInterest (1)
    {
        id: 'get-ci-1',
        type: 'group',
        draggable: false,
        data: { label: 'getCompoundInterest' },
        position: { x: 600, y: 200 },
        style: { backgroundColor: '#CF99E7', padding: 20, borderRadius: 8 },
    },
    {
        id: 'gci-1-1',
        type: 'custom',
        draggable: false,
        parentNode: 'get-ci-1',
        data: { label: 'Getting CI for 1100, 1' },
        position: { x: 20, y: 40 },
    },
    {
        id: 'gci-1-2',
        type: 'custom',
        draggable: false,
        parentNode: 'get-ci-1',
        data: { label: 'interest = 110', isOutput: true },
        position: { x: 20, y: 120 },
    },
    {
        id: 'gci-1-3',
        type: 'custom',
        draggable: false,
        parentNode: 'get-ci-1',
        data: { label: 'Updated Balance = 1210', isOutput: true },
        position: { x: 20, y: 200 },
    },
    {
        id: 'gci-1-4',
        type: 'custom',
        draggable: false,
        parentNode: 'get-ci-1',
        data: { label: 'CI = 1210', isOutput: true },
        position: { x: 20, y: 280 },
    },

    // addMoney (2)
    {
        id: 'add-money-2',
        type: 'group',
        draggable: false,
        data: { label: 'addMoney' },
        position: { x: 0, y: 400 },
        style: { backgroundColor: '#931798', padding: 20, borderRadius: 8 },
    },
    {
        id: 'add-2-1',
        type: 'custom',
        draggable: false,
        parentNode: 'add-money-2',
        data: { label: 'Adding money (110) to balance 1100' },
        position: { x: 20, y: 40 },
    },
    {
        id: 'add-2-2',
        type: 'custom',
        draggable: false,
        parentNode: 'add-money-2',
        data: { label: 'Balance = 1210' },
        position: { x: 20, y: 120 },
    },

    // logTransaction
    {
        id: 'log-trans',
        type: 'group',
        draggable: false,
        data: { label: 'logTransaction' },
        position: { x: 300, y: 400 },
        style: { backgroundColor: '#89DD3B', padding: 20, borderRadius: 8 },
    },
    {
        id: 'log-1',
        type: 'custom',
        draggable: false,
        parentNode: 'log-trans',
        data: { label: 'Logging transaction 2025-01-15T04:04:35.958Z: Investment matured: $1210' },
        position: { x: 20, y: 40 },
    },

    // getCompoundInterest (Final)
    {
        id: 'get-ci-final',
        type: 'group',
        draggable: false,
        data: { label: 'getCompoundInterest' },
        position: { x: 600, y: 400 },
        style: { backgroundColor: '#203318', padding: 20, borderRadius: 8 },
    },
    {
        id: 'gci-f-1',
        type: 'custom',
        draggable: false,
        parentNode: 'get-ci-final',
        data: { label: 'Getting CI for 1210, 0' },
        position: { x: 20, y: 40 },
    },
    {
        id: 'gci-f-2',
        type: 'custom',
        draggable: false,
        parentNode: 'get-ci-final',
        data: { label: 'Year == 0. Returning 1210' },
        position: { x: 20, y: 120 },
    },
];

const initialEdges = [
    // Main flow edges within calculateInvestment
    {
        id: 'e1',
        source: 'calc-inv-1',
        target: 'calc-inv-2',
        animated: true,
        style: { stroke: '#333' },
        markerEnd: { type: MarkerType.ArrowClosed },
    },
    {
        id: 'e2',
        source: 'calc-inv-2',
        target: 'calc-inv-3',
        animated: true,
        style: { stroke: '#333' },
        markerEnd: { type: MarkerType.ArrowClosed },
    },

    // Implementation edges
    {
        id: 'i1',
        source: 'calc-inv-2',
        target: 'gci-1-1',
        style: { stroke: '#666', strokeDasharray: '5,5' },
        markerEnd: { type: MarkerType.Dot },
    },
    {
        id: 'i2',
        source: 'gci-1-2',
        target: 'ci-1-1',
        style: { stroke: '#666', strokeDasharray: '5,5' },
        markerEnd: { type: MarkerType.Dot },
    },
    {
        id: 'i3',
        source: 'gci-1-3',
        target: 'add-2-1',
        style: { stroke: '#666', strokeDasharray: '5,5' },
        markerEnd: { type: MarkerType.Dot },
    },
    {
        id: 'i4',
        source: 'gci-1-4',
        target: 'gci-f-1',
        style: { stroke: '#666', strokeDasharray: '5,5' },
        markerEnd: { type: MarkerType.Dot },
    },

    // Return value edges
    {
        id: 'r1',
        source: 'ci-1-2',
        target: 'gci-1-2',
        style: { stroke: '#666' },
        markerEnd: { type: MarkerType.ArrowClosed },
    },
    {
        id: 'r2',
        source: 'add-2-2',
        target: 'gci-1-3',
        style: { stroke: '#666' },
        markerEnd: { type: MarkerType.ArrowClosed },
    },
    {
        id: 'r3',
        source: 'gci-f-2',
        target: 'gci-1-4',
        style: { stroke: '#666' },
        markerEnd: { type: MarkerType.ArrowClosed },
    },

    // Log transaction connection
    {
        id: 'l1',
        source: 'calc-inv-3',
        target: 'log-1',
        style: { stroke: '#666', strokeDasharray: '5,5' },
        markerEnd: { type: MarkerType.Dot },
    },
];

const FlowDiagram = () => {
    const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

    const toggleGroup = useCallback((groupId) => {
        setNodes((nds) =>
            nds.map((node) => {
                if (node.id === groupId) {
                    return {
                        ...node,
                        data: {
                            ...node.data,
                            collapsed: !node.data.collapsed,
                        },
                    };
                } else if (node.parentNode === groupId) {
                    return {
                        ...node,
                        hidden: !node.hidden,
                    };
                }
                return node;
            })
        );
    }, [setNodes]);

    const onNodeClick = useCallback((event, node) => {
        if (node.type === 'group') {
            toggleGroup(node.id);
        }
    }, [toggleGroup]);

    return (
        <div className="w-full h-[800px]">
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                onNodeClick={onNodeClick}
                nodeTypes={nodeTypes}
                fitView
                minZoom={0.1}
                maxZoom={1.5}
                nodesDraggable={false}
                nodesConnectable={false}
                elementsSelectable={false}
            >
                <Controls />
                <MiniMap />
                <Background variant="dots" gap={12} size={1} />
            </ReactFlow>
        </div>
    );
};

export default FlowDiagram;