import React from 'react';
import { CellStatus, MemoryCell } from './types';
import './HeapGrid.css';
import { AnimatedCell } from './useHeapAnimation';

interface HeapGridProps {
    memory: MemoryCell[];
    highlightedCells: number[];
    animatedCells: AnimatedCell[];
}

const cellStyleMap: Record<CellStatus, string> = {
    [CellStatus.Free]: 'white',
    [CellStatus.Allocated]: '#7CCD7C',
    [CellStatus.Marked]: 'yellow',
    [CellStatus.Used]: '#228B22',
};

const HeapGrid: React.FC<HeapGridProps> = ({ memory, highlightedCells, animatedCells }) => {
    const memoryLen = memory.length;
    const numCols = Math.ceil(Math.sqrt(memoryLen));

    const isFirstHighlighted = (index: number) => highlightedCells[0] === index;

    const getAnimationStyle = (index: number): React.CSSProperties => {
        const cellAnimation: AnimatedCell | undefined = animatedCells.find(anim => anim.cellIndex === index);
        if (!cellAnimation) return {};

        let baseStyle: React.CSSProperties = {
        };

        switch (cellAnimation.animation.type) {
            case 'flashing':
                return {
                    ...baseStyle,
                    animation: `flash ${cellAnimation.animation.duration}ms`,
                    opacity: cellAnimation.animation.opacity.toString()
                };
            case 'flickering':
                return {
                    ...baseStyle,
                    animation: `flicker ${cellAnimation.animation.duration}ms`,
                    opacity: cellAnimation.animation.opacity.toString()
                };
            default:
                return baseStyle;
        }
    }

    return (
        <div
            className="heap-grid"
            style={{ gridTemplateColumns: `repeat(${numCols}, 1fr)` }}
        >
            {memory.map((cell, index) => {
                const isHighlighted = highlightedCells.includes(index);
                const animationStyle = getAnimationStyle(index);

                return (
                    <div
                        key={index}
                        className={`cell ${isHighlighted ? 'highlighted' : ''}`}
                        style={{
                            ...animationStyle,
                            backgroundColor: cellStyleMap[cell.status]
                        }}
                        data-address={isFirstHighlighted(index) ? `0x${index.toString(16).toUpperCase()}` : undefined}
                    />
                );
            })}
        </div>
    );
};

export default HeapGrid;
