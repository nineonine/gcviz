import { useState } from 'react';

export interface TimedAnimation {
    duration: number;
    type: AnimationType;
    opacity: number;
    newColor?: string;
}

type AnimationType = 'flashing' | 'flickering' | 'highlighted-margins';

export interface AnimatedCell {
    cellIndex: number;
    animation: TimedAnimation;
}

export const createTimedAnimation = (
    duration: number,
    type: AnimationType,
    opacity: number,
    newColor?:string
): TimedAnimation => ({
    duration,
    type,
    opacity,
    newColor,
});

const useHeapAnimation = () => {
    const [highlightedCells, setHighlightedCells] = useState<number[]>([]);
    const [animatedCells, setAnimatedCells] = useState<AnimatedCell[]>([]);
    const [animationTimeouts, setAnimationTimeouts] = useState<number[]>([]);

    const highlightCells = (cellIndices: number[]) => {
        setHighlightedCells(cellIndices);
    };

    const clearHighlightedCells = () => {
        setHighlightedCells([]);
    };

    const enqueueAnimation = (cellIndex: number, animation: TimedAnimation) => {
        // Remove any existing animation for this cell
        setAnimatedCells(prevState => prevState.filter(cell => cell.cellIndex !== cellIndex));
        setAnimatedCells(prevState => [...prevState, { cellIndex, animation }]);

        // Set timeout to clear the animation after its duration
        const timeout = window.setTimeout(() => {
            setAnimatedCells(prevState => prevState.filter(cell => cell.cellIndex !== cellIndex));
        }, animation.duration);

        setAnimationTimeouts(prevState => [...prevState, timeout]);
    };

    const clearAnimations = () => {
        // Clear all animated cells
        setAnimatedCells([]);

        // Clear any pending animation timeouts
        animationTimeouts.forEach(timeout => clearTimeout(timeout));
        setAnimationTimeouts([]);
    };

    return {
        highlightedCells,
        highlightCells,
        clearHighlightedCells,
        animatedCells,
        enqueueAnimation,
        clearAnimations
    };
};

export default useHeapAnimation;
