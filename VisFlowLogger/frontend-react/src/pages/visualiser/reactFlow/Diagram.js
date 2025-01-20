import React, { useState, useMemo } from "react";
import ReactFlow, { Background, Controls, Handle, Position } from "reactflow";
import "reactflow/dist/style.css";
import styled from "styled-components";


// Custom Node for Collapsible Blocks
const CollapsibleNode = ({ data }) => {
    const [collapsed, setCollapsed] = useState(false);

    return (
        <NodeContainer>
            <NodeHeader onClick={() => setCollapsed(!collapsed)}>
                {data.name} {collapsed ? "+" : "-"}
            </NodeHeader>
            {!collapsed && (
                <NodeBody>
                    {data.flow.map((flow, index) => (
                        <div key={index}>
                            <strong>{flow.flow_type}:</strong> {flow.value || "N/A"}
                        </div>
                    ))}
                </NodeBody>
            )}
            <Handle type="source" position={Position.Bottom} />
        </NodeContainer>
    );
};

// Styled Components for Custom Node
const NodeContainer = styled.div`
  border: 1px solid #ddd;
  background-color: white;
  padding: 10px;
  border-radius: 5px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
`;

const NodeHeader = styled.div`
  font-weight: bold;
  cursor: pointer;
`;

const NodeBody = styled.div`
  margin-top: 10px;
`;

// ReactFlow Node Types
const nodeTypes = {
    collapsible: CollapsibleNode,
};

// Process Data Structure into ReactFlow Nodes and Edges
const processData = (data) => {
    const nodes = [];
    const edges = [];

    Object.keys(data).forEach((key) => {
        const block = data[key];
        nodes.push({
            id: key,
            type: "collapsible",
            position: { x: Math.random() * 400, y: Math.random() * 400 },
            data: { ...block },
        });

        block.flow.forEach((flow) => {
            const edgeId = `${key}-${flow.flow_id}`;
            edges.push({
                id: edgeId,
                source: key,
                target: flow.flow_pointer_id || `${key}-${flow.flow_id}`,
                type:
                    flow.flow_type === "CallStore"
                        ? "smoothstep"
                        : flow.flow_type === "Call"
                            ? "step"
                            : "default",
                animated: true,
                label: flow.flow_type,
            });

            // Add return edge for CallStore
            if (flow.flow_type === "CallStore") {
                const returnEdgeId = `${flow.flow_pointer_id}-return-${key}`;
                edges.push({
                    id: returnEdgeId,
                    source: flow.flow_pointer_id,
                    target: key,
                    type: "step",
                    animated: true,
                    label: "Return",
                    style: { stroke: "green" },
                });
            }
        });
    });

    return { nodes, edges };
};

// Diagram Component
const Diagram = ({ data }) => {
    const { nodes, edges } = useMemo(() => processData(data), [data]);

    return (
        <div style={{ width: "100%", height: "100vh" }}>
            <ReactFlow
                nodes={nodes}
                edges={edges}
                nodeTypes={nodeTypes}
                fitView
                defaultZoom={1.5}
            >
                <Background color="#aaa" gap={16} />
                <Controls />
            </ReactFlow>
        </div>
    );
};

export default Diagram;
