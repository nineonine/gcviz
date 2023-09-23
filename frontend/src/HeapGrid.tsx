import React from 'react';
import { CellStatus, MemoryCell } from './types';
import './HeapGrid.css';

interface HeapGridProps {
    memory: MemoryCell[];
    highlightedCells: number[];
}

const cellStyleMap: Record<CellStatus, string> = {
    [CellStatus.Free]: 'white',
    [CellStatus.ToBeFree]: 'magenta',
    [CellStatus.Allocated]: '#7CCD7C',
    [CellStatus.Marked]: 'yellow',
    [CellStatus.Used]: '#228B22',
};

const HeapGrid: React.FC<HeapGridProps> = ({ memory, highlightedCells }) => {
    const memoryLen = memory.length;
    const numCols = Math.ceil(Math.sqrt(memoryLen));

    const isFirstHighlighted = (index: number) => highlightedCells[0] === index;

    return (
        <div
            className="heap-grid"
            style={{ gridTemplateColumns: `repeat(${numCols}, 1fr)` }}
        >
            {memory.map((cell, index) => {
                return (
                    <div
                        key={index}
                        className={`cell ${highlightedCells.includes(index) ? 'highlighted' : ''}`}
                        style={{ backgroundColor: cellStyleMap[cell.status] }}
                        data-address={isFirstHighlighted(index) ? `0x${index.toString(16).toUpperCase()}` : undefined}
                    />
                );
            })}
        </div>
    );
};

export default HeapGrid;
