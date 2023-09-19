import React, { useEffect, useRef } from 'react';
import './EventStream.css';
import { LogEntry } from './logEntry';

interface EventStreamProps {
    logs: LogEntry[];
}

const EventStream: React.FC<EventStreamProps> = ({ logs }) => {
    const endOfMessagesRef = useRef<null | HTMLDivElement>(null);

    useEffect(() => {
        if (endOfMessagesRef.current) {
            endOfMessagesRef.current.scrollIntoView({ behavior: "smooth" });
        }
    }, [logs]);

    return (
        <div className="event-stream">
            {logs.map((log, index) => (
                <p key={index}>{log.msg} (Source: {log.source}, Frame ID: {log.frame_id || 'N/A'})</p>
            ))}
            <div ref={endOfMessagesRef}></div>
        </div>
    );
}

export default EventStream;
