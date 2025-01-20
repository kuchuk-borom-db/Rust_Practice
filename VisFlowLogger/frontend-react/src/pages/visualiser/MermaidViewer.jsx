import React, { useEffect, useState } from 'react';
import mermaid from 'mermaid';
import "./Mermaid.css"
import {code} from "./mermaid_code";
const MermaidDiagram = () => {
    const [diagramLoaded, setDiagramLoaded] = useState(false);

    useEffect(() => {
        mermaid.initialize({
            startOnLoad: true,
            theme: 'default',
            flowchart: {
                curve: 'basis',
                padding: 20,
                nodeSpacing: 50,
                rankSpacing: 50,
                htmlLabels: true,
            },
            sequence: {
                actorMargin: 50,
                messageMargin: 40,
            },
            animation: {
                diagramSpeed: 'slow',
                curve: 'linear',
            }
        });

        const renderDiagram = async () => {
            try {
                await mermaid.run();
                setDiagramLoaded(true);
                setupInteractivity();
            } catch (error) {
                console.error('Error rendering diagram:', error);
            }
        };

        renderDiagram();
    }, []);

    const setupInteractivity = () => {
        const subgraphs = document.querySelectorAll('.cluster');

        subgraphs.forEach(subgraph => {
            // Add hover animation class
            subgraph.classList.add('transform', 'transition-all', 'duration-300', 'hover:scale-105');

            // Add click handler for collapse/expand
            subgraph.addEventListener('click', function() {
                const rect = this.querySelector('rect');
                const contents = Array.from(this.querySelectorAll('.node, .edgePath'));

                if (this.dataset.collapsed === 'true') {
                    // Expand with animation
                    contents.forEach(el => {
                        el.style.display = '';
                        el.classList.add('animate-fadeIn');
                    });
                    rect.setAttribute('height', this.dataset.originalHeight);
                    this.dataset.collapsed = 'false';
                } else {
                    // Store original height
                    if (!this.dataset.originalHeight) {
                        this.dataset.originalHeight = rect.getAttribute('height');
                    }
                    // Collapse with animation
                    contents.forEach(el => {
                        el.classList.add('animate-fadeOut');
                        setTimeout(() => {
                            el.style.display = 'none';
                        }, 300);
                    });
                    rect.setAttribute('height', '50');
                    this.dataset.collapsed = 'true';
                }
            });
        });
    };

    return (
        <div className="w-full max-w-6xl mx-auto p-6">
            <div className={`
        bg-white rounded-lg shadow-xl p-8
        transform transition-all duration-500
        ${diagramLoaded ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-4'}
      `}>
                <h2 className="text-2xl font-bold mb-6 text-gray-800">Investment Flow Diagram</h2>
                <div className="mermaid-wrapper overflow-x-auto">
          <pre className="mermaid">
            {code}
          </pre>
                </div>
                <div className="mt-4 text-sm text-gray-600">
                    <p>Click on any subgraph to expand/collapse. Hover to highlight.</p>
                </div>
            </div>
        </div>
    );
};

export default MermaidDiagram;