import React, { useState, useRef } from 'react';
import { AnimatePresence, motion } from 'framer-motion';
import { ArrowRightLeft, ArrowUpDown, Database, ExternalLink, MessageSquare, Phone, Save, Download } from 'lucide-react';
import { type BlockData, type BlockFlow, BlockFlowType } from '../models/BlockData';
import html2canvas from 'html2canvas';
import jsPDF from 'jspdf';

const getFlowTypeStyles = (flowType: BlockFlowType) => {
    const styles: Record<BlockFlowType, {
        bgColor: string;
        border: string;
        icon: React.ReactNode;
        textColor: string;
    }> = {
        [BlockFlowType.Log]: {
            bgColor: 'bg-blue-900/30',
            border: 'border-blue-500',
            icon: <MessageSquare className="text-blue-400"/>,
            textColor: 'text-blue-300'
        },
        [BlockFlowType.CallStore]: {
            bgColor: 'bg-purple-900/30',
            border: 'border-purple-500',
            icon: <Save className="text-purple-400"/>,
            textColor: 'text-purple-300'
        },
        [BlockFlowType.Call]: {
            bgColor: 'bg-green-900/30',
            border: 'border-green-500',
            icon: <Phone className="text-green-400"/>,
            textColor: 'text-green-300'
        },
        [BlockFlowType.ExternCall]: {
            bgColor: 'bg-yellow-900/30',
            border: 'border-yellow-500',
            icon: <ExternalLink className="text-yellow-400"/>,
            textColor: 'text-yellow-300'
        },
        [BlockFlowType.ExternCallStore]: {
            bgColor: 'bg-red-900/30',
            border: 'border-red-500',
            icon: <Database className="text-red-400"/>,
            textColor: 'text-red-300'
        }
    };
    return styles[flowType] || {
        bgColor: 'bg-gray-900/30',
        border: 'border-gray-500',
        icon: null,
        textColor: 'text-gray-300'
    };
};

const Flow: React.FC<{
    blockID: string,
    flow: BlockFlow,
    blocks: Record<string, BlockData>,
    isHorizontal: boolean
}> = ({flow, blocks, isHorizontal}) => {
    const [isExpanded, setIsExpanded] = useState(false);
    const {bgColor, border, icon, textColor} = getFlowTypeStyles(flow.flowType);

    const getDisplayValue = () => {
        if (flow.value && flow.value.trim() !== '') {
            const lines = flow.value.split('\n');
            const uniqueLines = [...new Set(lines)];

            if (uniqueLines.length === 1 && lines.length > 1) {
                return `${uniqueLines[0]} (${lines.length} times)`;
            }

            return flow.value;
        }

        if (flow.flowPointerId) {
            const referencedBlock = blocks[flow.flowPointerId];
            return referencedBlock ? referencedBlock.name : 'Unnamed Block';
        }

        return 'Unnamed Flow';
    };

    return (
        <motion.div
            layout
            initial={{opacity: 0, scale: 0.95}}
            animate={{opacity: 1, scale: 1}}
            exit={{opacity: 0, scale: 0.95}}
            transition={{duration: 0.3}}
            onClick={(e) => {
                e.stopPropagation();
                setIsExpanded(!isExpanded);
            }}
            className={`
                flow-container 
                ${bgColor}
                ${border}
                p-4 
                rounded-lg 
                relative 
                cursor-pointer 
                transition-colors 
                duration-200 
                ${isHorizontal ? 'mr-4 h-auto' : 'mb-4'}
                w-full
                flex flex-col
                justify-start
                border-l-4
                hover:shadow-lg
                max-w-full
            `}
        >
            {flow.flowType !== BlockFlowType.Call || !isExpanded ? (
                <div className="flex items-center space-x-3 mb-2">
                    {icon}
                    <div className="flex-grow overflow-hidden">
                        <p className={`
                            text-lg 
                            font-semibold 
                            ${textColor} 
                            whitespace-nowrap 
                            overflow-hidden 
                            text-ellipsis
                            max-w-full
                        `}>
                            {getDisplayValue()}
                        </p>
                    </div>
                </div>
            ) : null}

            <AnimatePresence>
                {isExpanded && flow.flowPointerId && (
                    <motion.div
                        initial={{opacity: 0, height: 0}}
                        animate={{opacity: 1, height: 'auto'}}
                        exit={{opacity: 0, height: 0}}
                        transition={{duration: 0.3}}
                        className="relative mt-4 pl-5 sub-block-container"
                        onClick={(e) => e.stopPropagation()}
                    >
                        <div
                            className="absolute left-0 top-0 bottom-0 w-0.5 bg-white/30"
                            style={{left: '-20px'}}
                        />
                        <Block
                            blockID={flow.flowPointerId}
                            blockData={blocks[flow.flowPointerId]}
                            blocks={blocks}
                        />
                    </motion.div>
                )}
            </AnimatePresence>
        </motion.div>
    );
};

const Block: React.FC<{
    blockID: string;
    blockData: BlockData;
    blocks: Record<string, BlockData>;
}> = ({ blockID, blockData, blocks }) => {
    const [isHorizontal, setIsHorizontal] = useState(false);
    const blockRef = useRef<HTMLDivElement>(null);

    const handleExport = async (format: 'png' | 'pdf') => {
        if (blockRef.current) {
            const canvas = await html2canvas(blockRef.current);
            const imgData = canvas.toDataURL('image/png');

            if (format === 'png') {
                const link = document.createElement('a');
                link.href = imgData;
                link.download = `${blockData.name}.png`;
                link.click();
            } else if (format === 'pdf') {
                const pdf = new jsPDF('p', 'mm', 'a4');
                const imgWidth = 210; // A4 width in mm
                const imgHeight = (canvas.height * imgWidth) / canvas.width;
                pdf.addImage(imgData, 'PNG', 0, 0, imgWidth, imgHeight);
                pdf.save(`${blockData.name}.pdf`);
            }
        }
    };

    return (
        <motion.div
            ref={blockRef}
            layout
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.3 }}
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
                hover:shadow-xl
            `}
        >
            <div className="flex items-center justify-between mb-4">
                <div className="flex items-center space-x-3">
                    <motion.h2
                        layout
                        className="text-xl font-semibold"
                    >
                        {blockData.name}
                    </motion.h2>
                    <motion.button
                        whileHover={{ scale: 1.1 }}
                        whileTap={{ scale: 0.9 }}
                        onClick={(e) => {
                            e.stopPropagation();
                            handleExport('png'); // Change to 'pdf' for PDF export
                        }}
                        className="
                            p-1
                            bg-green-600
                            hover:bg-green-700
                            rounded-lg
                            text-white
                            transition-colors
                            flex
                            items-center
                            justify-center
                        "
                    >
                        <Download size={16} />
                    </motion.button>
                </div>
                <motion.button
                    whileHover={{ scale: 1.1 }}
                    whileTap={{ scale: 0.9 }}
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
                    {isHorizontal ? <ArrowRightLeft size={20} /> : <ArrowUpDown size={20} />}
                </motion.button>
            </div>
            {
                blockData.caller && <p className="text-sm text-gray-300 mb-1">Caller: {blockData.caller}</p>
            }
            <p className="text-sm text-gray-300 mb-4">Name: {blockData.name}</p>

            <motion.div
                layout
                className={`
                    flows-container 
                    ${isHorizontal ? 'flex flex-row items-start' : 'flex flex-col'} 
                    gap-4
                    w-max
                `}
            >
                <AnimatePresence>
                    {blockData.flow.map((flow, index) => (
                        <Flow
                            key={index}
                            blockID={blockID}
                            flow={flow}
                            blocks={blocks}
                            isHorizontal={isHorizontal}
                        />
                    ))}
                </AnimatePresence>
            </motion.div>
        </motion.div>
    );
};

export default Block;