// hooks/useHighlightState.ts
import { useState, useCallback } from 'react';

const useHighlightState = () => {
    const [highlightedCells, setHighlightedCells] = useState<number[]>([]);

    const highlightCells = useCallback((cells: number[]) => {
        setHighlightedCells(cells);
    }, []);

    const clearHighlightedCells = useCallback(() => {
        setHighlightedCells([]);
    }, []);

    return {
        highlightedCells,
        highlightCells,
        clearHighlightedCells,
    };
};

export default useHighlightState;
