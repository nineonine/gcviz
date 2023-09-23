import React, { useEffect, useRef } from 'react';
import './EventStream.css';
import { logSrcToColor } from './logUtils';
import { EventLogDetails, InstrResult } from './types';

interface EventStreamProps {
    logs: EventLogDetails[];
    highlightCells: (cells: number[]) => void;
    clearHighlightedCells: () => void;
}

const EventStream: React.FC<EventStreamProps> = ({ logs, highlightCells, clearHighlightedCells }) => {
    const endOfMessagesRef = useRef<null | HTMLDivElement>(null);

    useEffect(() => {
        if (endOfMessagesRef.current) {
            endOfMessagesRef.current.scrollIntoView({ behavior: "smooth" });
        }
    }, [logs]);

    return (
        <div className="event-stream">
            {logs.map(([log, ir], index) => (
                <div className='event-log-entry' key={index}
                    onMouseEnter={() => {
                        console.info(ir)
                        const cellsToHighlight: number[] = computeCells(ir) || 0;
                        highlightCells(cellsToHighlight);
                    }}
                    onMouseLeave={clearHighlightedCells}
                >
                    [
                    <span
                        className="log-source"
                        style={{ color: logSrcToColor(log.source) }}>
                        <b>{log.source}</b>
                    </span>
                    ]: {log.msg} {log.frame_id !== null && <span>(Frame ID: <b>{log.frame_id}</b>)
                    </span>}
                </div>
            ))}
            <div ref={endOfMessagesRef}></div>
        </div>
    );
}

export default EventStream;

// Compute the cells to highlight based on log.instruction
const computeCells = (ir: InstrResult | undefined): number[] => {
    if (!ir) return [];
    switch (ir._type) {
        case 'Allocate':
            return Array.from({ length: ir.object.fields.length }, (_, i) => ir.addr + i);
        case 'Write':
        case 'Read':
            return [ir.addr];
        case 'GC':
            return [];
    }
};
