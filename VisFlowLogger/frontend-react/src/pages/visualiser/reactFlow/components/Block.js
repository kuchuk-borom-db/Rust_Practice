import React from 'react';

const BlockGroupNode = ({ data }) => {
    return (
        <div
            className="relative bg-gray-100 border border-gray-300 rounded-lg shadow-lg"
            style={{ width: '100%', height: '100%', zIndex: -1 }} // Ensure group node is behind edges
        >
            {/* Group Title */}
            <div className="absolute -top-4 left-1/2 transform -translate-x-1/2 bg-gray-800 text-white text-sm font-semibold px-3 py-1 rounded-full">
                {data.name || 'Block Group'}
            </div>
        </div>
    );
};

export default React.memo(BlockGroupNode);
