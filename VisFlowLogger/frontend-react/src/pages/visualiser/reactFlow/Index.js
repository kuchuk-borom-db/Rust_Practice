import React, { useCallback } from 'react';
import ReactFlow, { Controls, ReactFlowProvider, useNodesState, useEdgesState } from 'reactflow';
import dagre from 'dagre';
import 'reactflow/dist/style.css';
import FlowNode from './components/FlowNode';
import Block from './components/Block';
const nodeTypes = {
    flowNode: FlowNode,
    block: Block,
};

const NODE_WIDTH = 200;
const NODE_HEIGHT = 60;
const BLOCK_PADDING = 40;
const VERTICAL_SPACING = 80;
const HORIZONTAL_SPACING = 250;

const getLayoutedElements = (nodes, edges) => {
    // Find implementation relationships
    const implEdges = edges.filter(edge =>
        edge.sourceHandle === 'impl' || edge.targetHandle === 'hasImpl'
    );

    // Create a map of implementation relationships
    const implMap = new Map();
    implEdges.forEach(edge => {
        implMap.set(edge.source, edge.target);
    });

    const g = new dagre.graphlib.Graph({
        compound: true
    });

    g.setGraph({
        rankdir: 'TB',
        nodesep: VERTICAL_SPACING,
        ranksep: HORIZONTAL_SPACING,
        marginx: 50,
        marginy: 50,
        ranker: 'tight-tree'
    });

    g.setDefaultEdgeLabel(() => ({}));

    // Group nodes by their parent blocks
    const blockGroups = {};
    const blocks = nodes.filter(node => node.type === 'block');

    nodes.forEach(node => {
        if (node.type === 'flowNode') {
            if (!blockGroups[node.parentId]) {
                blockGroups[node.parentId] = [];
            }
            blockGroups[node.parentId].push(node);
        }
    });

    // Add blocks to graph
    blocks.forEach(block => {
        const childNodes = blockGroups[block.id] || [];
        // Calculate block dimensions based on contained nodes and their implementations
        let maxNodesInRow = 0;
        let totalRows = 0;
        let currentRow = [];

        childNodes.forEach(node => {
            if (implMap.has(node.id)) {
                currentRow.push(node);
                maxNodesInRow = Math.max(maxNodesInRow, currentRow.length);
                currentRow = [];
                totalRows++;
            } else {
                currentRow.push(node);
            }
        });
        if (currentRow.length > 0) {
            maxNodesInRow = Math.max(maxNodesInRow, currentRow.length);
            totalRows++;
        }

        const blockWidth = (maxNodesInRow * (NODE_WIDTH + BLOCK_PADDING)) + BLOCK_PADDING;
        const blockHeight = (totalRows * (NODE_HEIGHT + BLOCK_PADDING)) + BLOCK_PADDING;

        g.setNode(block.id, {
            width: blockWidth,
            height: blockHeight,
            label: block.id
        });
    });

    // Add all nodes with their parent relationships
    nodes.forEach(node => {
        if (node.type === 'flowNode') {
            g.setNode(node.id, {
                width: NODE_WIDTH,
                height: NODE_HEIGHT,
                label: node.id
            });
            if (node.parentId) {
                g.setParent(node.id, node.parentId);
            }
        }
    });

    // Add edges
    edges.forEach(edge => {
        if (edge.sourceHandle === 'impl' || edge.targetHandle === 'hasImpl') {
            // For implementation edges, enforce horizontal alignment
            g.setEdge(edge.source, edge.target, {
                weight: 2,
                minlen: 1,
                rankdir: 'LR'
            });
        } else {
            g.setEdge(edge.source, edge.target, {
                weight: 1
            });
        }
    });

    // Apply layout
    dagre.layout(g);

    // Position the nodes
    const layoutedNodes = nodes.map(node => {
        const nodeWithPosition = g.node(node.id);

        if (node.type === 'block') {
            return {
                ...node,
                position: {
                    x: nodeWithPosition.x - nodeWithPosition.width / 2,
                    y: nodeWithPosition.y - nodeWithPosition.height / 2
                },
                style: {
                    ...node.style,
                    width: nodeWithPosition.width,
                    height: nodeWithPosition.height,
                    zIndex: -1
                }
            };
        }

        // For implementation nodes, adjust position relative to their parent
        const isImpl = Array.from(implMap.values()).includes(node.id);
        if (isImpl) {
            const parentNode = nodes.find(n => implMap.get(n.id) === node.id);
            const parentPos = g.node(parentNode.id);
            return {
                ...node,
                position: {
                    x: parentPos.x + NODE_WIDTH + BLOCK_PADDING,
                    y: parentPos.y
                }
            };
        }

        return {
            ...node,
            position: {
                x: nodeWithPosition.x - NODE_WIDTH / 2,
                y: nodeWithPosition.y - NODE_HEIGHT / 2
            }
        };
    });

    return layoutedNodes;
};

const FlowWithGroup = () => {
    const initialNodes = [
        //Blocks
        //Main
        {
            id: "main",
            type: 'block',
            data: {
                name: "main"
            },
            style: {
                width: 400,
                height: 400,
                zIndex: -1,
            },
            position: {
                x: 0, y: 0
            }
        },
        //Sum1
        {
            id: "sum1",
            type: 'block',
            data: {
                name: "sum"
            },
            style: {
                width: 400,
                height: 300,
                zIndex: -1
            },
            position: {
                x: 400, y: 100
            }
        },
        //Sum1
        {
            id: "sum2",
            type: 'block',
            data: {
                name: "sum"
            },
            style: {
                width: 400,
                height: 200,
                zIndex: -1
            },
            position: {
                x: 800, y: 200
            }
        },
        //Foo
        {
            id: "foo",
            type: 'block',
            data: {
                name: "foo"
            },
            style: {
                width: 400,
                height: 200,
                zIndex: -1
            },
            position: {
                x: 400, y: 400
            }
        },
        //FlowNodes
        //Main
        {
            id: 'main-1',
            type: 'flowNode',
            data: {
                value: "Adding 2 and 1",
                hasInput: false,
                hasOutput: true,
                hasImpl: false,
                isImpl: false,
            },
            position: {
                x: 100,
                y: 100
            },
            parentId: 'main',
            extent: "parent",

        },
        {
            id: 'main-2',
            type: 'flowNode',
            data: {
                value: "Sum = 3",
                hasInput: true,
                hasOutput: true,
                hasImpl: true,
                isImpl: false,
                isStore: true,
            },
            position: {
                x: 100,
                y: 200
            },
            parentId: 'main',
            extent: "parent"
        },
        {
            id: 'main-3',
            type: 'flowNode',
            data: {
                value: "foo",
                hasInput: true,
                hasOutput: false,
                hasImpl: true,
                isImpl: false
            },
            position: {
                x: 100,
                y: 300
            },
            parentId: 'main',
            extent: "parent"
        },
        //Sum1
        {
            id: 'sum1-1',
            type: 'flowNode',
            data: {
                value: "2+1 = 3",
                hasInput: false,
                hasOutput: true,
                hasImpl: false,
                isImpl: true,
            },
            position: {
                x: 100,
                y: 100
            },
            parentId: 'sum1',
            extent: "parent"
        },
        {
            id: 'sum1-2',
            type: 'flowNode',
            data: {
                value: "Recursive sum",
                hasInput: true,
                hasOutput: true,
                hasImpl: true,
                isImpl: false,
            },
            position: {
                x: 100,
                y: 200
            },
            parentId: 'sum1',
            extent: "parent"
        },
        {
            id: 'sum2-1',
            type: 'flowNode',
            data: {
                value: "1 + 1 = 2",
                hasInput: false,
                hasOutput: false,
                hasImpl: false,
                isImpl: true,
            },
            position: {
                x: 100,
                y: 100
            },
            parentId: 'sum2',
            extent: "parent"
        },
        //Foo node
        {
            id: 'foo-1',
            type: 'flowNode',
            data: {
                value: "dummy foo function called",
                hasInput: false,
                hasOutput: false,
                hasImpl: false,
                isImpl: true,
            },
            position: {
                x: 100,
                y: 100
            },
            parentId: 'foo',
            extent: "parent"
        },
    ]
    const initialEdges = [
        //block edges
        {
            id: 'main-1-2',
            source: 'main-1',
            target: 'main-2',
        },
        {
            id: 'main-2-3',
            source: 'main-2',
            target: 'main-3',
        },
        {
            id: 'sum-1-2',
            source: 'sum1-1',
            target: 'sum1-2'
        },
        {
            id: 'sum-2-3',
            source: 'sum1-2',
            target: 'sum1-3'
        },
        //Implementation
        //Sum1
        {
            id: 'main-2-sum1',
            source: 'main-2',
            target: 'sum1-1',
            sourceHandle: 'impl',
            targetHandle: 'hasImpl'
        },
        {
            id: 'main-2-sum1-store',
            source: 'sum1-2',
            target: 'main-2',
            targetHandle: "store"
        },
        {
            id: 'sum1-2-sum2-1',
            source: 'sum1-2',
            sourceHandle: "impl",
            target: 'sum2-1',
            targetHandle: "hasImpl",
        },
        {
            id: 'main-3-foo-1',
            source: 'main-3',
            sourceHandle: "impl",
            target: 'foo-1',
            targetHandle: "hasImpl",
        },
    ]


    const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

    const onLayout = useCallback(() => {
        const layoutedNodes = getLayoutedElements(nodes, edges);
        setNodes([...layoutedNodes]);
    }, [nodes, edges, setNodes]);

    React.useEffect(() => {
        onLayout();
    }, []);

    return (
        <div className="h-screen">
            <ReactFlowProvider>
                <ReactFlow
                    nodes={nodes}
                    edges={edges}
                    nodeTypes={nodeTypes}
                    onNodesChange={onNodesChange}
                    onEdgesChange={onEdgesChange}
                    fitView
                    fitViewOptions={{ padding: 0.2 }}
                    minZoom={0.1}
                    maxZoom={4}
                    defaultViewport={{ x: 0, y: 0, zoom: 1 }}
                >
                    <Controls />
                </ReactFlow>
            </ReactFlowProvider>
        </div>
    );
};

export default FlowWithGroup;