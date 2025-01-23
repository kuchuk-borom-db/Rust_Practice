import React, {useState} from 'react';
import {ArrowRightLeft, ArrowUpDown} from 'lucide-react';
import {type BlockData, type BlockFlow, BlockFlowType} from '../models/BlockData';

const Flow: React.FC<{
    blockID: string,
    flow: BlockFlow,
    blocks: Record<string, BlockData>,
    isHorizontal: boolean
}> = ({flow, blocks, isHorizontal}) => {
    const [isExpanded, setIsExpanded] = useState(false);

    const getFlowTypeLabel = (flowType: BlockFlowType) => {
        const labels: Record<BlockFlowType, string> = {
            [BlockFlowType.Log]: 'Log',
            [BlockFlowType.CallStore]: 'Call Store',
            [BlockFlowType.Call]: 'Call',
            [BlockFlowType.ExternCall]: 'Extern Call',
            [BlockFlowType.ExternCallStore]: 'Extern Call Store',
        };
        return labels[flowType] || 'Unknown';
    };

    const getFlowTypeColor = (flowType: BlockFlowType) => {
        const colors: Record<BlockFlowType, string> = {
            [BlockFlowType.Log]: 'bg-blue-500/30 border-blue-500/50',
            [BlockFlowType.CallStore]: 'bg-red-400/30 border-red-400/50',
            [BlockFlowType.Call]: 'bg-pink-600/30 border-pink-600/50',
            [BlockFlowType.ExternCall]: 'bg-yellow-600/30 border-yellow-600/50',
            [BlockFlowType.ExternCallStore]: 'bg-red-600/30 border-red-600/50',
        };
        return colors[flowType] || 'bg-gray-600/30 border-gray-600/50';
    };

    return (
        <div
            onClick={(e) => {
                e.stopPropagation();
                setIsExpanded(!isExpanded);
            }}
            className={`
                flow-container 
                ${getFlowTypeColor(flow.flowType)} 
                p-4 
                rounded-lg 
                text-white 
                cursor-pointer 
                relative 
                transition-colors 
                duration-200 
                ${isHorizontal ? 'mr-4 h-auto' : 'mb-4'}
                w-max
                flex flex-col
                justify-start
            `}
        >
            <div className="flex items-center justify-between mb-2">
                <h3 className="text-lg font-semibold">{getFlowTypeLabel(flow.flowType)}</h3>
                <span className="text-sm bg-black/20 px-2 py-1 rounded">{flow.flowId}</span>
            </div>
            <p className="text-sm mb-2">Pointer ID: {flow.flowPointerId}</p>
            {flow.value && (
                <p className="text-sm">Value: {flow.value}</p>
            )}

            {isExpanded && flow.flowPointerId && (
                <div
                    className="relative mt-4 pl-5 sub-block-container"
                    onClick={(e) => e.stopPropagation()}
                >
                    <div
                        className="absolute left-0 top-0 bottom-0 w-0.5 bg-white/30"
                        style={{left: '-20px'}}
                    />
                    {flow.flowType === BlockFlowType.CallStore && (
                        <div className="bg-green-700/50 p-2 rounded-t-lg mb-2">
                            <p className="text-sm font-semibold">Storing result: <span className="font-mono">{flow.value}</span></p>
                        </div>
                    )}
                    <Block
                        blockID={flow.flowPointerId}
                        blockData={blocks[flow.flowPointerId]}
                        blocks={blocks}
                    />
                </div>
            )}
        </div>
    );
};

const Block: React.FC<{
    blockID: string;
    blockData: BlockData;
    blocks: Record<string, BlockData>;
}> = ({blockID, blockData, blocks}) => {
    const [isHorizontal, setIsHorizontal] = useState(false);

    return (
        <div
            className={`
                block-container 
                bg-[#1a202c]
                rounded-lg 
                p-6 
                text-white 
                relative 
                w-max
                transition-all 
                duration-300
                ${isHorizontal ? 'mr-4' : 'mb-4'}
            `}
        >
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-semibold">Block ID: {blockID}</h2>
                <button
                    onClick={(e) => {
                        e.stopPropagation();
                        setIsHorizontal(!isHorizontal);
                    }}
                    className="
                        p-2
                        bg-blue-600
                        hover:bg-blue-700
                        rounded-lg
                        text-white
                        transition-colors
                        flex
                        items-center
                        justify-center
                    "
                >
                    {isHorizontal ? <ArrowRightLeft size={20}/> : <ArrowUpDown size={20}/>}
                </button>
            </div>
            <p className="text-sm text-gray-300 mb-1">Caller: {blockData.caller}</p>
            <p className="text-sm text-gray-300 mb-4">Name: {blockData.name}</p>

            <div
                className={`
                    flows-container 
                    ${isHorizontal ? 'flex flex-row items-start' : 'flex flex-col'} 
                    gap-4
                    w-max
                `}
            >
                {blockData.flow.map((flow, index) => (
                    <Flow
                        key={index}
                        blockID={blockID}
                        flow={flow}
                        blocks={blocks}
                        isHorizontal={isHorizontal}
                    />
                ))}
            </div>
        </div>
    );
};

export default Block;