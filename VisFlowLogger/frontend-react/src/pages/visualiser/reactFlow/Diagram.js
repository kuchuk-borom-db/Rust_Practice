import React, {useMemo} from 'react';
import {ReactFlow} from '@xyflow/react';

import '@xyflow/react/dist/style.css';
import {blockNode} from "./components/block";

const initialNodes = [
    {id: '1', type: 'custom', position: {x: 0, y: 0}, data: {wtf: '1'}},
    {id: '2', position: {x: 0, y: 100}, data: {label: '2'}},
];
const initialEdges = [{id: 'e1-2', source: '1', target: '2'}];


export default function App() {
    const nodeTypes = useMemo(() => ({custom: blockNode}), [])

    return (
        <div style={{width: '100vw', height: '100vh'}}>
            <ReactFlow nodeTypes={nodeTypes} nodes={initialNodes} edges={initialEdges}/>
        </div>
    );
}