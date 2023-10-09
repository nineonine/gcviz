import React, { useEffect, useRef } from 'react';
import './EventStream.css';
import { GCEvent, LogEntry } from './types';
import { EventOps } from './eventlog';

interface EventStreamProps {
    logs: LogEntry[] | GCEvent[];
    ops: (event: LogEntry | GCEvent) => EventOps;
    highlightCells: (cells: number[]) => void;
    clearHighlightedCells: () => void;
    className?: string;
}

const EventStream: React.FC<EventStreamProps> = ({ className, logs, ops, highlightCells, clearHighlightedCells }) => {
    const endOfMessagesRef = useRef<null | HTMLDivElement>(null);

    useEffect(() => {
        if (endOfMessagesRef.current) {
            endOfMessagesRef.current.scrollIntoView({ behavior: "smooth" });
        }
    }, [logs]);

    return (
        <div className={`event-stream ${className || ''}`}>
            {logs.map((event, index) => {
                const eventOps = ops(event);
                return (
                    <div className='event-log-entry' key={index}
                        onMouseEnter={() => {
                            const cells = eventOps.cellsToHighlight();
                            highlightCells(cells);
                        }}
                        onMouseLeave={clearHighlightedCells}
                    >
                        {eventOps.render()}
                    </div>
                );
            })}
            <div ref={endOfMessagesRef}></div>
        </div>
    );
}

export default EventStream;
