import React from 'react';
import { motion } from 'framer-motion';
import { GitBranch, Globe, Share2 } from 'lucide-react';

interface Feature {
    icon: React.ReactNode;
    title: string;
    description: string;
}

interface AnimationProps {
    opacity: number;
    y: number;
}

const LandingPage: React.FC = () => {
    const features: Feature[] = [
        {
            icon: <GitBranch className="w-6 h-6" />,
            title: "Visual Flow Analytics",
            description: "Transform complex logs into intuitive visual flows, making debugging and system analysis effortless. Trace issues and understand system behavior at a glance."
        },
        {
            icon: <Globe className="w-6 h-6" />,
            title: "Universal Integration",
            description: "Connect from any programming language using our comprehensive API libraries and drivers. No official SDK? No problem - integrate directly via REST API calls."
        },
        {
            icon: <Share2 className="w-6 h-6" />,
            title: "Flexible Visualization",
            description: "Choose from multiple diagram types including Mermaid, React Flow, and more. Customize your log visualization to match your team's needs and preferences."
        }
    ];

    const initialAnimation: AnimationProps = {
        opacity: 0,
        y: 20
    };

    const animateProps: AnimationProps = {
        opacity: 1,
        y: 0
    };

    return (
        <div className="min-h-screen bg-gradient-to-br from-gray-900 to-gray-800 text-white">
            <div className="container mx-auto px-6 py-16">
                <motion.div
                    initial={initialAnimation}
                    animate={animateProps}
                    transition={{ duration: 0.8 }}
                    className="text-center"
                >
                    <div className="mx-auto w-20 h-20 mb-8 bg-blue-500 rounded-lg flex items-center justify-center">
                        <span className="text-2xl font-bold">VF</span>
                    </div>

                    <h1 className="text-5xl font-bold mb-4">
                        VisFlow Logger
                    </h1>
                    <p className="text-xl text-gray-300 mb-8 max-w-2xl mx-auto">
                        Revolutionize your debugging experience with visual log flows.
                        Transform complex system logs into clear, actionable visualizations
                        that make sense at first glance.
                    </p>

                    <motion.button
                        whileHover={{ scale: 1.05 }}
                        whileTap={{ scale: 0.95 }}
                        className="bg-blue-500 text-white px-8 py-3 rounded-lg font-medium
                       hover:bg-blue-600 transition-colors duration-200"
                    >
                        Get Started
                    </motion.button>
                </motion.div>

                <div className="grid md:grid-cols-3 gap-8 mt-20">
                    {features.map((feature, index) => (
                        <motion.div
                            key={index}
                            initial={initialAnimation}
                            animate={animateProps}
                            transition={{ duration: 0.5, delay: index * 0.2 }}
                            className="bg-gray-800 p-6 rounded-lg hover:bg-gray-700
                         transition-colors duration-200"
                        >
                            <div className="bg-blue-500/10 w-12 h-12 rounded-lg flex
                            items-center justify-center mb-4">
                                {feature.icon}
                            </div>
                            <h3 className="text-xl font-semibold mb-2">{feature.title}</h3>
                            <p className="text-gray-400">{feature.description}</p>
                        </motion.div>
                    ))}
                </div>

                <motion.div
                    animate={{
                        y: [0, -10, 0],
                        opacity: [0.3, 0.5, 0.3]
                    }}
                    transition={{
                        duration: 5,
                        repeat: Infinity,
                        ease: "easeInOut"
                    }}
                    className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t
                     from-blue-500/10 to-transparent"
                />
            </div>
        </div>
    );
};

export default LandingPage;