import {Handle, Position} from "@xyflow/react";

export const blockNode = ({data}) => {
    return (
        <>
            <Handle type="target" position={Position.Top}/>
            <div>
                <label>{data.wtf}</label>
            </div>
            <Handle type="source" position={Position.Bottom} id="a" />
        </>
    );
}