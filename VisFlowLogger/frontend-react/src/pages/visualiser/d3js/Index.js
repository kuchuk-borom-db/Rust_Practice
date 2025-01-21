import React, { useState } from 'react';
import { AnimatePresence, motion } from 'framer-motion';
import { ChevronDown, ChevronRight, Database, MessageSquare, Phone, Layout } from 'lucide-react';

const OrientationToggle = ({ orientation, setOrientation }) => {
    return (
        <motion.div className="fixed top-6 right-6 z-50" initial={false}>
            <motion.button
                className="p-4 rounded-full bg-gray-800 border-2 border-gray-600 shadow-lg hover:shadow-xl transition-all duration-300 hover:bg-gray-700 hover:border-gray-500 group"
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                onClick={() => setOrientation(prev => (prev === 'vertical' ? 'horizontal' : 'vertical'))}
            >
                <Layout
                    className={`w-6 h-6 text-gray-300 group-hover:text-white transition-all duration-300 transform ${
                        orientation === 'horizontal' ? 'rotate-90' : 'rotate-0'
                    }`}
                />
            </motion.button>
        </motion.div>
    );
};

const getFlowTypeStyles = (type) => {
    const styles = {
        Log: { icon: <MessageSquare className="w-4 h-4" />, border: 'border-green-500/50', hover: 'hover:bg-green-500/20' },
        CallStore: { icon: <Database className="w-4 h-4" />, border: 'border-purple-500/50', hover: 'hover:bg-purple-500/20' },
        Call: { icon: <Phone className="w-4 h-4" />, border: 'border-blue-500/50', hover: 'hover:bg-blue-500/20' },
    };

    return styles[type] || { icon: null, border: 'border-gray-500/50', hover: 'hover:bg-gray-500/20' };
};

const DownArrow = () => (
    <motion.div className="flex justify-center">
        <ChevronDown className="w-6 h-6 text-blue-500" />
    </motion.div>
);

const RightArrow = () => (
    <motion.div className="flex items-center">
        <ChevronRight className="w-6 h-6 text-blue-500" />
    </motion.div>
);

const Arrow = ({ orientation }) => {
    return orientation === 'vertical' ? <DownArrow /> : <RightArrow />;
};

const FlowNode = ({ type, value, flowPointerId = null, level = 0, orientation }) => {
    const [isExpanded, setIsExpanded] = useState(false);
    const [isHovered, setIsHovered] = useState(false);
    const styles = getFlowTypeStyles(type);

    const isExpandable = (type === 'Call' || type === 'CallStore') && flowPointerId;

    return (
        <motion.div
            className={`relative ${orientation === 'horizontal' ? 'min-w-[280px]' : 'w-full'}`}
            layout
            transition={{ duration: 0.3 }}
            onMouseEnter={() => setIsHovered(true)}
            onMouseLeave={() => setIsHovered(false)}
        >
            <motion.div
                layout
                className={`relative p-4 rounded-lg backdrop-blur-sm border-2 ${styles.border} shadow-lg transition-all duration-300 ${
                    isHovered ? `${styles.hover} shadow-xl bg-gray-800/40` : 'bg-gray-800/30'
                } ${isExpandable ? 'cursor-pointer' : ''}`}
                onClick={() => isExpandable && setIsExpanded(!isExpanded)}
            >
                <motion.div layout className="relative text-gray-300 flex items-center justify-between">
                    <div className="flex items-center space-x-2">
                        {styles.icon}
                        <span className="whitespace-nowrap overflow-hidden text-ellipsis">{value}</span>
                    </div>
                    {isExpandable && (
                        <motion.div animate={{ rotate: isExpanded ? 180 : 0 }} transition={{ duration: 0.3 }}>
                            <ChevronDown className="w-4 h-4 ml-2" />
                        </motion.div>
                    )}
                </motion.div>

                <AnimatePresence>
                    {isExpanded && flowPointerId && global_data[flowPointerId] && (
                        <motion.div
                            initial={{ opacity: 0, height: 0 }}
                            animate={{ opacity: 1, height: 'auto' }}
                            exit={{ opacity: 0, height: 0 }}
                            transition={{ duration: 0.3 }}
                            className="mt-4 overflow-hidden"
                        >
                            <Block
                                id={flowPointerId}
                                name={global_data[flowPointerId]?.name}
                                flows={global_data[flowPointerId]?.flow}
                                level={level + 1}
                                orientation={orientation}
                            />
                        </motion.div>
                    )}
                </AnimatePresence>
            </motion.div>
        </motion.div>
    );
};

const Block = ({ id, name, flows, level = 0, orientation }) => {
    const isStartBlock = id === 'START';

    return (
        <motion.div layout className={`relative w-full ${level === 0 ? 'p-6' : 'p-4'}`}>
            <motion.div
                layout
                className={`backdrop-blur-md border-2 rounded-xl overflow-hidden ${
                    isStartBlock ? 'border-blue-400/50 bg-gray-800/50' : 'border-gray-600/50 bg-gray-800/30'
                } p-6 shadow-lg hover:shadow-xl transition-all duration-300`}
            >
                <motion.div
                    layout
                    className="text-lg font-semibold text-gray-200 mb-4 text-center border-b-2 border-gray-700/50 pb-2"
                >
                    {name} {isStartBlock && '(START)'}
                </motion.div>
                <motion.div
                    layout
                    className={`flex gap-4 w-full ${
                        orientation === 'vertical' ? 'flex-col items-stretch' : 'flex-row items-center justify-center'
                    }`}
                >
                    {flows.map((item, index) => (
                        <React.Fragment key={item.flow_id}>
                            <FlowNode
                                type={item.flow_type}
                                value={item.value}
                                flowPointerId={item.flow_pointer_id}
                                level={level}
                                orientation={orientation}
                            />
                            {index < flows.length - 1 && <Arrow orientation={orientation} />}
                        </React.Fragment>
                    ))}
                </motion.div>
            </motion.div>
        </motion.div>
    );
};

let global_data;
const DS3Diagram = ({ data }) => {
    const [orientation, setOrientation] = useState('vertical');
    global_data = data;

    return (
        <motion.div className="flex min-h-screen w-full bg-gradient-to-br from-gray-900 to-gray-800">
            <OrientationToggle orientation={orientation} setOrientation={setOrientation} />

            <motion.div
                className={`flex-1 p-8 transition-all duration-300 overflow-auto flex ${
                    orientation === 'horizontal' ? 'justify-center items-center' : 'flex-col items-center'
                }`}
            >
                <Block
                    id="START"
                    name={data['START']?.name}
                    flows={data['START']?.flow}
                    orientation={orientation}
                />
            </motion.div>
        </motion.div>
    );
};
export default DS3Diagram;
