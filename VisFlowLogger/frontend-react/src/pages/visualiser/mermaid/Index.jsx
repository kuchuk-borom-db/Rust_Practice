import React, { useEffect, useRef, useState } from "react";
import { Code, ZoomIn, ZoomOut, Maximize2, Move } from "lucide-react";
import mermaid from "mermaid";

const MermaidViewer = ({ code }) => {
    const mermaidRef = useRef(null);
    const containerRef = useRef(null);
    const [transform, setTransform] = useState({ scale: 1, x: 0, y: 0 });
    const [isDragging, setIsDragging] = useState(false);
    const [dragStart, setDragStart] = useState({ x: 0, y: 0 });

    const fitDiagramToView = () => {
        if (!mermaidRef.current || !containerRef.current) return;

        const container = containerRef.current.getBoundingClientRect();
        const diagram = mermaidRef.current.getBoundingClientRect();

        // Calculate scale needed to fit diagram with some padding
        const scaleX = (container.width - 80) / diagram.width;
        const scaleY = (container.height - 80) / diagram.height;
        const scale = Math.min(scaleX, scaleY);

        // Start with a minimum scale of 0.8 to avoid too much zoom out
        const finalScale = Math.max(0.8, scale);

        // Calculate position to center
        const x = (container.width - diagram.width * finalScale) / 2;
        const y = (container.height - diagram.height * finalScale) / 2;

        setTransform({ scale: finalScale, x, y });
    };

    useEffect(() => {
        if (mermaidRef.current && code) {
            mermaid.initialize({
                startOnLoad: true,
                theme: 'dark',
                securityLevel: 'strict',
                themeVariables: {
                    fontSize: '16px',
                    primaryColor: '#3b82f6',
                    primaryTextColor: '#fff',
                    primaryBorderColor: '#60a5fa',
                    lineColor: '#022a58',
                    secondaryColor: '#91ef02',
                    tertiaryColor: '#059669'
                },
                wrap  : true,
                htmlLabels : false,
                markdownAutoWrap : true,
                flowchart :{
                    curve :"cardinal",
                    padding : 50,
                    nodeSpacing : 100,
                    rankSpacing : 50,
                    defaultRenderer :"dagre-wrapper"
                }
            });

            mermaidRef.current.innerHTML = '';
            const diagramId = `mermaid-${Math.random().toString(36).substr(2, 9)}`;

            try {
                mermaid.render(diagramId, code).then(({ svg }) => {
                    mermaidRef.current.innerHTML = svg;
                    setTimeout(fitDiagramToView, 100);
                });
            } catch (error) {
                console.error("Mermaid rendering error:", error);
                mermaidRef.current.innerHTML = `<div class="text-red-500">Error rendering diagram: ${error.message}</div>`;
            }
        }
    }, [code]);

    const handleZoom = (delta, clientX = null, clientY = null) => {
        setTransform(prev => {
            const newScale = Math.max(0.5, Math.min(100.0, prev.scale + delta));

            if (clientX !== null && clientY !== null) {
                const container = containerRef.current.getBoundingClientRect();
                const mouseX = clientX - container.left;
                const mouseY = clientY - container.top;

                const scaleDiff = newScale - prev.scale;
                const newX = prev.x - (mouseX - prev.x) * (scaleDiff / prev.scale);
                const newY = prev.y - (mouseY - prev.y) * (scaleDiff / prev.scale);

                return { scale: newScale, x: newX, y: newY };
            }

            return { ...prev, scale: newScale };
        });
    };

    const handleWheel = (e) => {
        e.preventDefault();
        // Increased zoom sensitivity
        const delta = -e.deltaY * 0.002;
        handleZoom(delta, e.clientX, e.clientY);
    };

    // ... (keeping the mouse and touch handlers the same)
    const handleMouseDown = (e) => {
        if (e.button === 0) {
            setIsDragging(true);
            setDragStart({
                x: e.clientX - transform.x,
                y: e.clientY - transform.y
            });
        }
    };

    const handleMouseMove = (e) => {
        if (isDragging) {
            setTransform(prev => ({
                ...prev,
                x: e.clientX - dragStart.x,
                y: e.clientY - dragStart.y
            }));
        }
    };

    const handleMouseUp = () => {
        setIsDragging(false);
    };

    const handleTouchStart = (e) => {
        if (e.touches.length === 1) {
            const touch = e.touches[0];
            setIsDragging(true);
            setDragStart({
                x: touch.clientX - transform.x,
                y: touch.clientY - transform.y
            });
        }
    };

    const handleTouchMove = (e) => {
        if (isDragging && e.touches.length === 1) {
            const touch = e.touches[0];
            setTransform(prev => ({
                ...prev,
                x: touch.clientX - dragStart.x,
                y: touch.clientY - dragStart.y
            }));
        }
    };

    const handleTouchEnd = () => {
        setIsDragging(false);
    };

    useEffect(() => {
        window.addEventListener('mouseup', handleMouseUp);
        window.addEventListener('mousemove', handleMouseMove);
        window.addEventListener('touchend', handleTouchEnd);
        window.addEventListener('touchmove', handleTouchMove);

        return () => {
            window.removeEventListener('mouseup', handleMouseUp);
            window.removeEventListener('mousemove', handleMouseMove);
            window.removeEventListener('touchend', handleTouchEnd);
            window.removeEventListener('touchmove', handleTouchMove);
        };
    }, [isDragging, dragStart]);

    return (
        <div className="min-h-screen bg-gradient-to-b from-gray-900 to-gray-800 text-white">
            <div className="container mx-auto px-6 py-16">
                <div className="text-center mb-12">


                    <h1 className="text-2xl font-bold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-violet-400">
                        Mermaid Visual Flow Diagram
                    </h1>
                </div>

                <div className="bg-gray-800/50 backdrop-blur-sm p-8 rounded-2xl shadow-xl border border-gray-700/50">
                    <div className="flex items-center gap-3 mb-6">
                        <button
                            onClick={() => handleZoom(0.2)}
                            className="p-2.5 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors duration-200 hover:scale-105 transform"
                            title="Zoom In"
                        >
                            <ZoomIn className="w-5 h-5" />
                        </button>
                        <button
                            onClick={() => handleZoom(-0.2)}
                            className="p-2.5 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors duration-200 hover:scale-105 transform"
                            title="Zoom Out"
                        >
                            <ZoomOut className="w-5 h-5" />
                        </button>
                        <button
                            onClick={fitDiagramToView}
                            className="p-2.5 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors duration-200 hover:scale-105 transform"
                            title="Fit to View"
                        >
                            <Maximize2 className="w-5 h-5" />
                        </button>
                        <div className="text-sm text-gray-300 ml-2 bg-gray-700/50 px-3 py-1.5 rounded-lg">
                            Zoom: {Math.round(transform.scale * 100)}%
                        </div>
                        <div className="flex items-center gap-2 ml-4 bg-gray-700/30 px-4 py-1.5 rounded-lg">
                            <Move className="w-5 h-5 text-gray-400" />
                            <span className="text-sm text-gray-300">Click and drag to pan • Scroll to zoom</span>
                        </div>
                    </div>
                    <div
                        ref={containerRef}
                        className="relative overflow-hidden rounded-xl bg-gray-900/50 backdrop-blur-sm"
                        style={{ height: '600px' }}
                        onWheel={handleWheel}
                    >
                        <div
                            ref={mermaidRef}
                            className="absolute cursor-move transition-transform duration-150 ease-out"
                            style={{
                                transform: `translate(${transform.x}px, ${transform.y}px) scale(${transform.scale})`,
                                transformOrigin: 'center center'
                            }}
                            onMouseDown={handleMouseDown}
                            onTouchStart={handleTouchStart}
                        />
                    </div>
                </div>
            </div>
        </div>
    );
};

export default MermaidViewer;