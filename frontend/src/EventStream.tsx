import React, { useEffect, useRef } from 'react';
import './EventStream.css';
import { LogEntry, logSrcToColor } from './logEntry';

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
                <div className='event-log-entry' key={index}>
                    [
                    <span
                        className="log-source"
                        style={{ color: logSrcToColor(log.source) }}>
                        <b>{log.source}</b>
                    </span>
                    ]: {log.msg} {log.frame_id !== null && <span>(Frame ID: <b>{log.frame_id}</b>)</span>}
                </div>
            ))}
            <div ref={endOfMessagesRef}></div>
        </div>
    );
}

export default EventStream;
