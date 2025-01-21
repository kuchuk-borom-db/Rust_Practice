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

const getFlowTypeStyles = (type: string) => {
    switch (type) {
        case 'Log':
            return {
                icon: <MessageSquare className="w-4 h-4 mr-2"/>,
                gradient: 'from-green-500/20 to-green-600/20',
                border: 'border-green-500/30',
                hover: 'group-hover:bg-green-500/20'
            };
        case 'CallStore':
            return {
                icon: <Database className="w-4 h-4 mr-2"/>,
                gradient: 'from-purple-500/20 to-purple-600/20',
                border: 'border-purple-500/30',
                hover: 'group-hover:bg-purple-500/20'
            };
        case 'Call':
            return {
                icon: <Phone className="w-4 h-4 mr-2"/>,
                gradient: 'from-blue-500/20 to-blue-600/20',
                border: 'border-blue-500/30',
                hover: 'group-hover:bg-blue-500/20'
            };
        default:
            return {
                icon: null,
                gradient: 'from-gray-500/20 to-gray-600/20',
                border: 'border-gray-500/30',
                hover: 'group-hover:bg-gray-500/20'
            };
    }
};

const ExpandableBlock = ({blockData}: { blockData: BlockData }) => {
    return (
        <div className="w-full mt-4 pl-4 border-l-2 border-gray-700/40">
            <div className="text-sm font-semibold text-gray-400 mb-2 pl-2">
                {blockData.name}
            </div>
            <div className="flex flex-col gap-4">
                {blockData.flow.map((item, index) => (
                    <FlowNode
                        key={item.flow_id}
                        type={item.flow_type}
                        value={item.value}
                        flowPointerId={item.flow_pointer_id}
                        isNested={true}
                    />
                ))}
            </div>
        </div>
    );
};

const FlowNode = ({
                      type,
                      value,
                      flowPointerId = null,
                      isNested = false,

                  }: {
    type: string;
    value: string | null;
    flowPointerId?: string | null;
    isNested?: boolean;
}) => {
    const [isExpanded, setIsExpanded] = useState(false);
    const styles = getFlowTypeStyles(type);
    const isExpandable = (type === 'Call' || type === 'CallStore') && flowPointerId;

    return (
        <div className="relative group w-full">
            <div
                className={`absolute inset-0 rounded-lg bg-gradient-to-r ${styles.gradient} transform translate-x-1 translate-y-1`}/>
            <div className="absolute inset-0 rounded-lg bg-gray-800/40 transform translate-x-0.5 translate-y-0.5"/>
            <motion.div
                className={`relative p-4 rounded-lg backdrop-blur-sm bg-gray-800/30 ${styles.border} border 
                    shadow-lg transition-all duration-300 
                    group-hover:translate-x-0.5 group-hover:translate-y-0.5 ${styles.hover}
                    ${isExpandable ? 'cursor-pointer' : ''}`}
                onClick={() => isExpandable && setIsExpanded(!isExpanded)}
            >
                <div className="text-gray-300 flex items-center justify-between">
                    <div className="flex items-center">
                        {styles.icon}
                        <span>{value || type}</span>
                    </div>
                    {isExpandable && (
                        <motion.div
                            animate={{rotate: isExpanded ? 180 : 0}}
                            transition={{duration: 0.3}}
                        >
                            <ChevronDown className="w-4 h-4"/>
                        </motion.div>
                    )}
                </div>

                <AnimatePresence>
                    {isExpanded && flowPointerId && !isNested && (
                        <motion.div
                            initial={{opacity: 0, height: 0}}
                            animate={{opacity: 1, height: 'auto'}}
                            exit={{opacity: 0, height: 0}}
                            transition={{duration: 0.3}}
                        >
                            <ExpandableBlock blockData={global_data[flowPointerId]}/>
                        </motion.div>
                    )}
                </AnimatePresence>
            </motion.div>
        </div>
    );
};

const Arrow = () => {
    return (
        <div className="h-8 relative w-full flex items-center justify-center">
            <motion.div
                className="h-px w-full bg-blue-500/50 relative"
                initial={{scale: 0}}
                animate={{scale: 1}}
                transition={{duration: 0.3}}
            >
                <motion.div
                    animate={{
                        x: [0, 20, 0]
                    }}
                    transition={{
                        duration: 2,
                        repeat: Infinity,
                        ease: "linear"
                    }}
                    className="absolute right-0 top-1/2 -translate-y-1/2"
                >
                    <ArrowRight className="w-4 h-4 text-blue-500/50"/>
                </motion.div>
            </motion.div>
        </div>
    );
};

const Block = ({id, name, flows}: { id: string; name: string; flows: FlowItem[] }) => {
    const isStartBlock = id === "START";
    return (
        <motion.div
            initial={{opacity: 0, y: 20}}
            animate={{opacity: 1, y: 0}}
            transition={{duration: 0.5}}
            className={`relative p-6 rounded-lg backdrop-blur-md border border-gray-700/40 
                 shadow-lg hover:shadow-xl transition-all duration-300 mb-8 min-w-[280px]
                 overflow-hidden ${isStartBlock ? 'bg-transparent' : 'bg-gray-800/30 hover:bg-gray-700/40'}`}
        >
            {isStartBlock && (
                <div className="absolute inset-0 -z-10">
                    <div
                        className="absolute inset-0 bg-gradient-to-r from-blue-500/20 via-purple-500/20 to-pink-500/20 animate-gradient-x"/>
                    <div
                        className="absolute inset-0 bg-gradient-to-b from-cyan-500/20 via-blue-500/20 to-purple-500/20 animate-gradient-y"/>
                    <motion.div
                        animate={{
                            background: [
                                'radial-gradient(circle at 0% 0%, rgba(56, 189, 248, 0.15) 0%, transparent 50%)',
                                'radial-gradient(circle at 100% 100%, rgba(56, 189, 248, 0.15) 0%, transparent 50%)',
                                'radial-gradient(circle at 0% 0%, rgba(56, 189, 248, 0.15) 0%, transparent 50%)',
                            ],
                        }}
                        transition={{
                            duration: 8,
                            repeat: Infinity,
                            ease: "linear"
                        }}
                        className="absolute inset-0"
                    />
                </div>
            )}

            <div className="text-lg font-semibold text-gray-200 mb-4 text-center border-b border-gray-700/40 pb-2">
                {name} ({id})
            </div>
            <div className="flex flex-col items-center gap-4 w-full relative z-10">
                {flows.map((item, index) => (
                    <React.Fragment key={item.flow_id}>
                        <FlowNode
                            type={item.flow_type}
                            value={item.value}
                            flowPointerId={item.flow_pointer_id}
                        />
                        {index < flows.length - 1 && <Arrow/>}
                    </React.Fragment>
                ))}
            </div>
        </motion.div>
    );
};
let global_data;
const DS3Diagram = ({data}) => {
    global_data = data;
    return (
        <div className="flex flex-col items-center p-8 min-h-screen bg-gradient-to-br from-gray-900 to-gray-800">
            <div className="relative grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
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
            </div>
        </div>
    );
};

export default DS3Diagram;