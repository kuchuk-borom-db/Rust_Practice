import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';

const FlowDiagram = ({ data }) => {
    const svgRef = useRef(null);

    useEffect(() => {
        if (!data || !svgRef.current) return;

        // Clear previous diagram
        d3.select(svgRef.current).selectAll("*").remove();

        // Setup SVG
        const svg = d3.select(svgRef.current);
        const margin = { top: 50, right: 50, bottom: 50, left: 50 };
        const width = 1200 - margin.left - margin.right;
        const height = 800 - margin.top - margin.bottom;

        const g = svg
            .attr('width', width + margin.left + margin.right)
            .attr('height', height + margin.top + margin.bottom)
            .append('g')
            .attr('transform', `translate(${margin.left},${margin.top})`);

        // Define blocks layout
        const blocks = Object.entries(data).map(([id, block], index) => ({
            id,
            ...block,
            x: (index % 3) * 400,
            y: Math.floor(index / 3) * 300,
            width: 350,
            height: block.flow.length * 70 + 60,
        }));

        // Add blocks
        const blockGroups = g
            .selectAll('.block')
            .data(blocks)
            .enter()
            .append('g')
            .attr('class', 'block')
            .attr('transform', d => `translate(${d.x},${d.y})`);

        // Block Background
        blockGroups
            .append('rect')
            .attr('width', d => d.width)
            .attr('height', d => d.height)
            .attr('rx', 12)
            .attr('ry', 12)
            .attr('fill', 'url(#gradientBackground)')
            .attr('stroke', '#4299E1')
            .attr('stroke-width', 2)
            .style('box-shadow', '4px 4px 10px rgba(0, 0, 0, 0.3)');

        // Block Titles
        blockGroups
            .append('text')
            .attr('x', 20)
            .attr('y', 40)
            .attr('fill', '#fff')
            .style('font-weight', '600')
            .style('font-size', '18px')
            .text(d => `${d.name} (${d.id})`);

        // Add flow nodes
        blocks.forEach(block => {
            const nodes = block.flow.map((flow, index) => ({
                ...flow,
                x: block.x + 30,
                y: block.y + 70 + index * 70,
                parentBlock: block,
            }));

            // Draw nodes
            const nodeGroups = g
                .selectAll(`.node-${block.id}`)
                .data(nodes)
                .enter()
                .append('g')
                .attr('class', `node-${block.id}`);

            // Node Circles
            nodeGroups
                .append('circle')
                .attr('cx', d => d.x + 10)
                .attr('cy', d => d.y)
                .attr('r', 10)
                .attr('fill', d =>
                    d.flow_type === 'Call'
                        ? '#48BB78'
                        : d.flow_type === 'CallStore'
                            ? '#9F7AEA'
                            : '#4299E1'
                );

            // Node Labels
            nodeGroups
                .append('text')
                .attr('x', d => d.x + 30)
                .attr('y', d => d.y + 5)
                .attr('fill', '#fff')
                .style('font-size', '14px')
                .text(d => `${d.flow_type}${d.value ? ': ' + d.value : ''}`);
        });

        // Add gradient definition
        svg.append('defs')
            .append('linearGradient')
            .attr('id', 'gradientBackground')
            .attr('x1', '0%')
            .attr('y1', '0%')
            .attr('x2', '100%')
            .attr('y2', '100%')
            .html(`
                <stop offset="0%" style="stop-color: #1a202c; stop-opacity: 1" />
                <stop offset="100%" style="stop-color: #2d3748; stop-opacity: 1" />
            `);

        // Add arrows
        svg.append('defs')
            .append('marker')
            .attr('id', 'arrow')
            .attr('viewBox', '0 -5 10 10')
            .attr('refX', 8)
            .attr('refY', 0)
            .attr('markerWidth', 6)
            .attr('markerHeight', 6)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M0,-5L10,0L0,5')
            .attr('fill', '#fff');

    }, [data]);

    return (
        <div className="min-h-screen bg-gradient-to-br from-gray-900 to-gray-800 p-8">
            <div className="text-center mb-12">
                <h1 className="text-4xl font-bold text-white">Flow Diagram Generator</h1>
                <p className="text-gray-400 mt-4">
                    Visualize your system flows with clarity and elegance. Effortlessly map connections and interactions.
                </p>
            </div>
            <div className="overflow-auto rounded-lg bg-gray-900 shadow-lg">
                <svg ref={svgRef} className="w-full h-full" />
            </div>
        </div>
    );
};

export default FlowDiagram;
