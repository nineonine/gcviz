import React from 'react';
import { CellStatus, MemoryCell } from './types';

import './HeapGrid.css';

interface HeapGridProps {
    memory: MemoryCell[];
}

const cellStyleMap: Record<CellStatus, string> = {
    [CellStatus.Free]: 'white',
    [CellStatus.ToBeFree]: 'magenta',
    [CellStatus.Allocated]: 'green',
    [CellStatus.Marked]: 'yellow',
    [CellStatus.Used]: 'black',
};

const HeapGrid: React.FC<HeapGridProps> = ({ memory }) => {
    const memoryLen = memory.length;
    const numCols = Math.ceil(Math.sqrt(memoryLen));

    return (
        <div
            className="heap-grid"
            style={{ gridTemplateColumns: `repeat(${numCols}, 1fr)` }}
        >
            {memory.map((cell, index) => (
                <div
                    key={index}
                    className="cell"
                    style={{ backgroundColor: cellStyleMap[cell.status] }}
                />
            ))}
        </div>
    );
};

export default HeapGrid;
