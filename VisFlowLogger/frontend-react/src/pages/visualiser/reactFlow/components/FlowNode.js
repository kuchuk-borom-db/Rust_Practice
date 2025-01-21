import React from 'react';
import {Handle} from 'reactflow';

const CustomNode = ({data}) => {
    return (<div className="relative bg-white p-4 border rounded shadow-md w-52">
        <p className="text-gray-800 font-medium text-center">{data.value}</p>

        {/* Top Input Handle */}
        {data.hasInput && (<Handle
            type="target"
            position="top"
            id="input"
            className="w-3 h-3 bg-blue-500 -translate-x-1/2 left-1/2"
        />)}

        {/* Bottom Output Handle */}
        {data.hasOutput && (<Handle
            type="source"
            position="bottom"
            id="output"
            className="w-3 h-3 bg-blue-500 -translate-x-1/2 left-1/2"
        />)}
        {/* Right Implementation Handle */}
        {data.hasImpl && (<Handle
            type="source"
            position="right"
            id="impl"
            className="w-3 h-3 bg-green-500"
            style={{top: '20%'}} // Adjust position
        />)}

        {/* Right Store Handle */}
        {data.isStore && (
            <Handle
                type="target"
                position="right"
                id="store"
                className="w-3 h-3 bg-blue-500"
                style={{top: '80%'}} // Adjust position
            />)}


        {/* Left Implementation Handle */}
        {data.isImpl && (<Handle
            type="target"
            position="left"
            id="isImpl"
            className="w-3 h-3 bg-red-500 translate-y-1/2 top-1/2"
        />)}
    </div>);
};

export default React.memo(CustomNode);
