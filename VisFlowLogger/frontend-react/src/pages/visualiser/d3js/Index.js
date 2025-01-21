import React, {useState} from 'react';
import {AnimatePresence, motion} from 'framer-motion';
import {ArrowRight, ChevronDown, Database, MessageSquare, Phone} from 'lucide-react';

interface FlowItem {
    flow_pointer_id: string | null;
    flow_id: string;
    flow_type: string;
    value: string | null;
}

interface BlockData {
    caller: string | null;
    name: string;
    flow: FlowItem[];
}

interface DS3Data {
    [key: string]: BlockData;
}

// Enhanced theme system with nested level variations
const getFlowTypeStyles = (type: string, level: number = 0) => {
    const baseStyles = {
        Log: {
            icon: <MessageSquare className="w-4 h-4 mr-2"/>,
            gradients: [
                'from-green-500/20 to-green-600/20',
                'from-emerald-500/20 to-emerald-600/20',
                'from-teal-500/20 to-teal-600/20'
            ],
            borders: [
                'border-green-500/30',
                'border-emerald-500/30',
                'border-teal-500/30'
            ],
            hovers: [
                'group-hover:bg-green-500/20',
                'group-hover:bg-emerald-500/20',
                'group-hover:bg-teal-500/20'
            ]
        },
        CallStore: {
            icon: <Database className="w-4 h-4 mr-2"/>,
            gradients: [
                'from-purple-500/20 to-purple-600/20',
                'from-fuchsia-500/20 to-fuchsia-600/20',
                'from-pink-500/20 to-pink-600/20'
            ],
            borders: [
                'border-purple-500/30',
                'border-fuchsia-500/30',
                'border-pink-500/30'
            ],
            hovers: [
                'group-hover:bg-purple-500/20',
                'group-hover:bg-fuchsia-500/20',
                'group-hover:bg-pink-500/20'
            ]
        },
        Call: {
            icon: <Phone className="w-4 h-4 mr-2"/>,
            gradients: [
                'from-blue-500/20 to-blue-600/20',
                'from-cyan-500/20 to-cyan-600/20',
                'from-sky-500/20 to-sky-600/20'
            ],
            borders: [
                'border-blue-500/30',
                'border-cyan-500/30',
                'border-sky-500/30'
            ],
            hovers: [
                'group-hover:bg-blue-500/20',
                'group-hover:bg-cyan-500/20',
                'group-hover:bg-sky-500/20'
            ]
        }
    };

    const styleLevel = Math.min(level, 2); // Cap at 3 levels of variation
    const typeStyle = baseStyles[type] || {
        icon: null,
        gradients: ['from-gray-500/20 to-gray-600/20'],
        borders: ['border-gray-500/30'],
        hovers: ['group-hover:bg-gray-500/20']
    };

    return {
        icon: typeStyle.icon,
        gradient: typeStyle.gradients[styleLevel] || typeStyle.gradients[0],
        border: typeStyle.borders[styleLevel] || typeStyle.borders[0],
        hover: typeStyle.hovers[styleLevel] || typeStyle.hovers[0]
    };
};

const ExpandableBlock = ({blockData, level = 0}: { blockData: BlockData; level?: number }) => {
    return (
        <motion.div
            className="w-full mt-4"
            layout
            transition={{
                layout: { duration: 0.3, ease: "easeOut" }
            }}
        >
            <motion.div layout className="text-sm font-semibold text-gray-400 mb-2 pl-2">
                {blockData.name}
            </motion.div>
            <motion.div layout className="flex flex-col gap-4">
                {blockData.flow.map((item, index) => (
                    <React.Fragment key={item.flow_id}>
                        <motion.div layout>
                            <FlowNode
                                type={item.flow_type}
                                value={item.value}
                                flowPointerId={item.flow_pointer_id}
                                isNested={true}
                                level={level}
                            />
                        </motion.div>
                        {index < blockData.flow.length - 1 && <Arrow />}
                    </React.Fragment>
                ))}
            </motion.div>
        </motion.div>
    );
};
const FlowNode = ({
                      type,
                      value,
                      flowPointerId = null,
                      isNested = false,
                      level = 0,
                  }: {
    type: string;
    value: string | null;
    flowPointerId?: string | null;
    isNested?: boolean;
    level?: number;
}) => {
    const [isExpanded, setIsExpanded] = useState(false);
    const [isHovered, setIsHovered] = useState(false);
    const styles = getFlowTypeStyles(type, level);

    const isExpandable = (type === 'Call' || type === 'CallStore') && flowPointerId;

    const handleClick = (e: React.MouseEvent) => {
        e.stopPropagation();
        if (isExpandable) {
            setIsExpanded(!isExpanded);
        }
    };

    const handleMouseEnter = (e: React.MouseEvent) => {
        e.stopPropagation();
        setIsHovered(true);
    };

    const handleMouseLeave = (e: React.MouseEvent) => {
        e.stopPropagation();
        setIsHovered(false);
    };

    return (
        <motion.div
            className="relative w-full"
            layout
            transition={{
                layout: { duration: 0.3, ease: "easeOut" }
            }}
            onMouseEnter={handleMouseEnter}
            onMouseLeave={handleMouseLeave}
        >
            {/* Outer glow effect on hover */}
            <div
                className={`absolute -inset-0.5 bg-gradient-to-r from-blue-500/30 via-purple-500/30 to-pink-500/30 rounded-lg blur-sm
                    transition-opacity duration-300 ${isHovered ? 'opacity-100' : 'opacity-0'}`}
            />

            {/* Main gradient background with enhanced hover effect */}
            <div
                className={`absolute inset-0 rounded-lg bg-gradient-to-r ${styles.gradient} 
                    transform transition-all duration-300 ease-out
                    ${isHovered ? 'translate-x-1 translate-y-1 scale-[1.02] blur-[0.5px]' : 'translate-x-0 translate-y-0 scale-100'}`}
            />

            {/* Middle layer with glass effect */}
            <div
                className={`absolute inset-0 rounded-lg bg-gray-800/40 backdrop-blur-[2px]
                    transform transition-all duration-300 ease-out
                    ${isHovered ? 'translate-x-0.5 translate-y-0.5 bg-gray-800/30' : 'translate-x-0 translate-y-0'}`}
            />

            {/* Main content container */}
            <motion.div
                layout
                className={`relative p-4 rounded-lg backdrop-blur-sm ${styles.border} border 
                    shadow-lg transition-all duration-300 
                    ${isHovered ? `${styles.hover} shadow-2xl shadow-blue-500/10 bg-gray-800/40` : 'bg-gray-800/30'}
                    ${isExpandable ? 'cursor-pointer' : ''}`}
                onClick={handleClick}
            >
                {/* Inner highlight on hover */}
                <div
                    className={`absolute inset-0 rounded-lg bg-gradient-to-br from-white/5 to-transparent
                        transition-opacity duration-300 ${isHovered ? 'opacity-100' : 'opacity-0'}`}
                />

                {/* Node content with enhanced contrast on hover */}
                <motion.div
                    layout
                    className={`relative text-gray-300 flex items-center justify-between
                        transition-colors duration-300 ${isHovered ? 'text-white' : ''}`}
                >
                    <div className="flex items-center">
                        <div className={`transition-transform duration-300 ${isHovered ? 'scale-110' : ''}`}>
                            {styles.icon}
                        </div>
                        <span className="ml-2">{value || type}</span>
                    </div>
                    {isExpandable && (
                        <motion.div
                            animate={{ rotate: isExpanded ? 180 : 0 }}
                            transition={{ duration: 0.3 }}
                            className={`transition-transform duration-300 ${isHovered ? 'scale-110' : ''}`}
                        >
                            <ChevronDown className="w-4 h-4" />
                        </motion.div>
                    )}
                </motion.div>

                {/* Sub-flow expansion */}
                <AnimatePresence>
                    {isExpanded && flowPointerId && (
                        <motion.div
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            exit={{ opacity: 0 }}
                            transition={{ duration: 0.3 }}
                            onClick={(e) => e.stopPropagation()}
                            className="w-full"
                            layout
                        >
                            {global_data[flowPointerId] && (
                                <ExpandableBlock
                                    blockData={global_data[flowPointerId]}
                                    level={level + 1}
                                />
                            )}
                        </motion.div>
                    )}
                </AnimatePresence>
            </motion.div>
        </motion.div>
    );
};

const Arrow = () => {
    return (
        <motion.div
            className="h-full w-8 relative flex items-center justify-center"
            layout
        >
            <motion.div
                className="w-px h-full bg-blue-500/50 relative"
                initial={{ scale: 0 }}
                animate={{ scale: 1 }}
                transition={{ duration: 0.3 }}
            >
                <motion.div
                    animate={{
                        y: [0, 20, 0],
                    }}
                    transition={{
                        duration: 2,
                        repeat: Infinity,
                        ease: "linear",
                    }}
                    className="absolute bottom-0 left-1/2 -translate-x-1/2"
                >
                    <ArrowRight className="w-4 h-4 text-blue-500/50 rotate-90" />
                </motion.div>
            </motion.div>
        </motion.div>
    );
};

const Block = ({id, name, flows}: { id: string; name: string; flows: FlowItem[] }) => {
    const isStartBlock = id === "START";
    return (
        <motion.div
            layout
            initial={{opacity: 0, y: 20}}
            animate={{opacity: 1, y: 0}}
            transition={{
                duration: 0.5,
                layout: { duration: 0.3, ease: "easeOut" }
            }}
            className={`relative p-6 rounded-lg backdrop-blur-md border border-gray-700/40 
                shadow-lg hover:shadow-xl transition-all duration-300 mb-8 min-w-[280px]
                overflow-visible ${isStartBlock ? 'bg-transparent' : 'bg-gray-800/30 hover:bg-gray-700/40'}`}
        >
            {/* ... (rest of Block component remains the same) ... */}
            <motion.div layout className="text-lg font-semibold text-gray-200 mb-4 text-center border-b border-gray-700/40 pb-2">
                {name} ({id})
            </motion.div>
            <motion.div
                layout
                className="flex flex-col items-center gap-4 w-full relative z-10"
            >
                {flows.map((item, index) => (
                    <React.Fragment key={item.flow_id}>
                        <motion.div layout>
                            <FlowNode
                                type={item.flow_type}
                                value={item.value}
                                flowPointerId={item.flow_pointer_id}
                                isNested={false}
                                level={0}
                            />
                        </motion.div>
                        {index < flows.length - 1 && <Arrow/>}
                    </React.Fragment>
                ))}
            </motion.div>
        </motion.div>
    );
};
let global_data;
const DS3Diagram = ({data}) => {
    global_data = data;
    return (
        <motion.div
            layout
            className="flex flex-col items-center p-8 min-h-screen bg-gradient-to-br from-gray-900 to-gray-800"
        >
            <motion.div
                layout
                className="relative grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8"
            >
                <Block
                    key={"START"}
                    id={"START"}
                    name={data["START"].name}
                    flows={data["START"].flow}
                />
                <motion.div
                    animate={{
                        opacity: [0.3, 0.5, 0.3]
                    }}
                    transition={{
                        duration: 5,
                        repeat: Infinity,
                        ease: "easeInOut"
                    }}
                    className="absolute -z-10 inset-0 bg-gradient-to-t from-blue-500/10 to-transparent"
                />
            </motion.div>
        </motion.div>
    );
};

export default DS3Diagram;