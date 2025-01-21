import React from 'react';
import ReactFlow, {Controls, ReactFlowProvider} from 'reactflow';
import 'reactflow/dist/style.css';
import FlowNode from './components/FlowNode';
import Block from './components/Block';
import * as edge from "framer-motion/m";

const nodeTypes = {
    flowNode: FlowNode,
    block: Block,
};


// Constants for layout
const BLOCK_PADDING = 40;
const NODE_HEIGHT = 60;
const NODE_WIDTH = 200;
const NODE_VERTICAL_SPACING = 40;
const COLUMN_SPACING = 40;
const MIN_BLOCK_WIDTH = 300;

function calculateLayout(rawNodes, rawEdges) {
    const blockGroups = {};
    const blocks = [];
    const nodes = [...rawNodes];

    // Group nodes by their parent blocks
    nodes.forEach(node => {
        if (node.type === 'block') {
            blocks.push(node);
        } else if (node.type === 'flowNode') {
            if (!blockGroups[node.parentId]) {
                blockGroups[node.parentId] = [];
            }
            blockGroups[node.parentId].push(node);
        }
    });

    // Find implementation relationships
    const implConnections = rawEdges.filter(edge =>
        edge.sourceHandle === 'impl' || edge.targetHandle === 'hasImpl'
    ).map(edge => ({
        sourceId: edge.source,
        targetId: edge.target,
        sourceBlock: nodes.find(n => n.id === edge.source)?.parentId,
        targetBlock: nodes.find(n => n.id === edge.target)?.parentId,
        sourceNode: nodes.find(n => n.id === edge.source)
    }));

    // Create a map to track block positions by column
    const columnBlocks = new Map(); // Maps column index to array of blocks in that column
    const blockPositions = new Map(); // Maps block ID to its column and vertical position

    // Start with main block in column 0
    const mainBlock = blocks.find(b => b.data.name === 'main');
    columnBlocks.set(0, [mainBlock]);

    // Position main block and its nodes
    mainBlock.position = { x: 0, y: 0 };
    let maxY = BLOCK_PADDING;
    blockGroups[mainBlock.id].forEach(node => {
        node.position = {
            x: BLOCK_PADDING,
            y: maxY
        };
        maxY += NODE_HEIGHT + NODE_VERTICAL_SPACING;
    });

    mainBlock.style = {
        width: MIN_BLOCK_WIDTH,
        height: maxY + BLOCK_PADDING,
        zIndex: -1
    };

    blockPositions.set(mainBlock.id, { column: 0, y: 0 });

    // Process implementation relationships to build column structure
    function getSourceColumn(blockId) {
        return blockPositions.get(blockId)?.column || 0;
    }

    // Sort connections based on source node vertical position
    const sortedConnections = [...implConnections].sort((a, b) => {
        const aNode = a.sourceNode;
        const bNode = b.sourceNode;
        const aY = aNode.position.y;
        const bY = bNode.position.y;
        return aY - bY;
    });

    // Process each implementation connection
    sortedConnections.forEach(conn => {
        const sourceColumn = getSourceColumn(conn.sourceBlock);
        const targetColumn = sourceColumn + 1;
        const implBlock = blocks.find(b => b.id === conn.targetBlock);
        const implNodes = blockGroups[conn.targetBlock] || [];

        // Calculate impl block height
        const implHeight = (implNodes.length * (NODE_HEIGHT + NODE_VERTICAL_SPACING)) + BLOCK_PADDING * 2;

        // Get source node's absolute position
        const sourceBlock = blocks.find(b => b.id === conn.sourceBlock);
        const sourceNodeAbsY = sourceBlock.position.y + conn.sourceNode.position.y;

        // Position the implementation block in next column aligned with source node
        const xPos = (targetColumn * (MIN_BLOCK_WIDTH + COLUMN_SPACING));
        const yPos = sourceNodeAbsY - BLOCK_PADDING;

        // Update block position and style
        implBlock.position = {
            x: xPos,
            y: yPos
        };

        implBlock.style = {
            width: MIN_BLOCK_WIDTH,
            height: implHeight,
            zIndex: -1
        };

        // Position nodes within implementation block
        let currentNodeY = BLOCK_PADDING;
        implNodes.forEach(node => {
            node.position = {
                x: BLOCK_PADDING,
                y: currentNodeY
            };
            currentNodeY += NODE_HEIGHT + NODE_VERTICAL_SPACING;
        });

        // Track block position
        if (!columnBlocks.has(targetColumn)) {
            columnBlocks.set(targetColumn, []);
        }
        columnBlocks.get(targetColumn).push(implBlock);
        blockPositions.set(implBlock.id, { column: targetColumn, y: yPos });

        // Adjust other blocks in the same column if there's overlap
        const blocksInColumn = columnBlocks.get(targetColumn);
        blocksInColumn.sort((a, b) => a.position.y - b.position.y);

        for (let i = 1; i < blocksInColumn.length; i++) {
            const currentBlock = blocksInColumn[i];
            const previousBlock = blocksInColumn[i - 1];
            const minY = previousBlock.position.y + previousBlock.style.height + NODE_VERTICAL_SPACING;

            if (currentBlock.position.y < minY) {
                currentBlock.position.y = minY;
                blockPositions.set(currentBlock.id, {
                    column: targetColumn,
                    y: minY
                });
            }
        }
    });

    return nodes;
}

const FlowWithGroup = () => {


    const nodes = [
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
    const edges = [
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

    const final = calculateLayout(nodes, edges);

    return (
        <div className="h-screen">
            <ReactFlowProvider> {/* Add this provider */}
                <ReactFlow
                    nodes={final}
                    edges={edges}
                    nodeTypes={nodeTypes}
                    fitView
                >
                    <Controls/>
                </ReactFlow>
            </ReactFlowProvider>
        </div>
    );
};

export default FlowWithGroup;
