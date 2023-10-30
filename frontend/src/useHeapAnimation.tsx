import { useState } from 'react';

export interface TimedAnimation {
    duration: number;
    type: AnimationType;
    opacity: number;
    newColor?: string;
}

type AnimationType = 'flashing' | 'flickering';

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

    const enqueueAnimation = (cellIndexes: number[], animation: TimedAnimation) => {
        // Remove any existing animations for these cells
        setAnimatedCells(prevState => prevState.filter(cell => !cellIndexes.includes(cell.cellIndex)));

        // Add new animations for these cells
        const newAnimations: AnimatedCell[] = cellIndexes.map(cellIndex => ({ cellIndex, animation }));
        setAnimatedCells(prevState => [...prevState, ...newAnimations]);

        // Set timeouts to clear the animations after their duration
        const timeouts = cellIndexes.map(cellIndex => {
            return window.setTimeout(() => {
                setAnimatedCells(prevState => prevState.filter(cell => cell.cellIndex !== cellIndex));
            }, animation.duration);
        });

        setAnimationTimeouts(prevState => [...prevState, ...timeouts]);
    }

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
