import {AnimatePresence, motion} from 'framer-motion';
import {BarChart2, Download, GitBranch, Layers} from 'lucide-react';

const VisualFlowLoggerLanding = () => {
    const desc = `
    Visual Flow Logger is a developer tool that transforms your code logs into intuitive visual diagrams. Whether you're tracing operations in a single application or across a distributed system, it helps you understand, debug, and optimize your code flow with ease. Flexible, powerful, and designed for clarity—turn your logs into actionable insights.
    `
    const features = [{
        icon: <Layers/>,
        title: "Universal Integration",
        description: "Language-agnostic logging solution with comprehensive API support across programming ecosystems."
    }, {
        icon: <GitBranch/>,
        title: "Comprehensive Flow Tracing",
        description: "Granular log tracing across processes with visualization control."
    }, {
        icon: <BarChart2/>,
        title: "Dynamic Visualizations",
        description: "Transform logs into intelligent diagrams using Mermaid, stack charts, and advanced rendering techniques."
    }, {
        icon: <Download/>,
        title: "Seamless Exportability",
        description: "Export and share logging insights across multiple formats for collaborative analysis."
    }];

    return (<motion.div
        initial={{opacity: 0}}
        animate={{opacity: 1}}
        transition={{duration: 0.5}}
        className="min-h-screen bg-[#0A1128] text-gray-100 overflow-hidden relative"
    >
        <div className="container mx-auto px-6 py-12 relative z-10">
            <motion.header
                initial={{y: -50, opacity: 0}}
                animate={{y: 0, opacity: 1}}
                transition={{duration: 0.6}}
                className="text-center mb-16"
            >
                <h1 className="text-5xl font-bold mb-4 text-indigo-300">
                    Visual Flow Logger
                </h1>
                <p className="text-xl max-w-2xl mx-auto text-gray-400">
                    {desc}
                </p>
            </motion.header>

            <motion.section
                initial={{scale: 0.9, opacity: 0}}
                animate={{scale: 1, opacity: 1}}
                transition={{duration: 0.7}}
                className="grid md:grid-cols-2 gap-12 items-center"
            >


                <motion.div
                    initial={{x: 50, opacity: 0}}
                    animate={{x: 0, opacity: 1}}
                    transition={{duration: 0.6}}
                    className="bg-[#1F2041]/50 rounded-xl p-8 border border-indigo-900/30"
                >
                    <h3 className="text-2xl font-semibold mb-4 text-indigo-300">
                        Code Flow Visualization
                    </h3>
                    <div className="w-full h-64 bg-[#1F2041]/30 rounded-lg flex items-center justify-center">
                        <p className="text-gray-500">Visualization Preview</p>
                    </div>
                </motion.div>
            </motion.section>

            <motion.section
                initial={{opacity: 0}}
                whileInView={{opacity: 1}}
                transition={{duration: 0.7}}
                className="mt-24"
            >
                <h2 className="text-4xl font-bold text-center mb-12 text-indigo-300">
                    Developer-Centric Features
                </h2>
                <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <AnimatePresence>
                        {features.map((feature, index) => (<motion.div
                            key={index}
                            initial={{opacity: 0, y: 20}}
                            animate={{opacity: 1, y: 0}}
                            exit={{opacity: 0, y: -20}}
                            whileHover={{scale: 1.05}}
                            transition={{duration: 0.3}}
                            className="bg-[#1F2041]/50 rounded-xl p-6 border border-indigo-900/30"
                        >
                            <div className="text-indigo-400 mb-4">{feature.icon}</div>
                            <h3 className="text-xl font-semibold mb-3 text-indigo-300">
                                {feature.title}
                            </h3>
                            <p className="text-gray-400">{feature.description}</p>
                        </motion.div>))}
                    </AnimatePresence>
                </div>
            </motion.section>
        </div>
    </motion.div>);
};

export default VisualFlowLoggerLanding;