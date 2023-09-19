import React, { useEffect, useState } from 'react';
import './Visualization.css';

import InfoBlock from './InfoBlock';
import EventStream from './EventStream';
import HeapGrid from './HeapGrid';
import ControlPanel from './ControlPanel';
import { CellStatus, MemoryCell } from './types';

const INTERVAL_RATE = 500; // 1 second

const Visualization: React.FC = () => {
    const intervalRate = INTERVAL_RATE;
    const [ws, setWs] = useState<WebSocket | null>(null);
    const [memory, setMemory] = useState<Array<MemoryCell>>(new Array(0).fill({ status: CellStatus.Free }));
    const [isRunning, setIsRunning] = useState(false);

    const toggleExecution = () => {
        setIsRunning(!isRunning);
    };

    const handleRestart = () => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send("RESET");
        }
    };

    useEffect(() => {
        // Initialize WebSocket connection only once when component mounts
        const wsConnection = new WebSocket("ws://127.0.0.1:9002");
        setWs(wsConnection);

        wsConnection.onerror = (error) => {
            console.error("WebSocket Error", error);
        };

        wsConnection.onmessage = (event) => {
            const newMemory = JSON.parse(event.data);
            setMemory(newMemory);
        };

        return () => {
            wsConnection.close();
        };
    }, [intervalRate]);

    useEffect(() => {
        let intervalId: any = null;

        if (isRunning && ws?.readyState === WebSocket.OPEN) {
            intervalId = setInterval(() => {
                ws.send("Message to send every interval");
            }, intervalRate);
        }

        return () => {
            intervalId && clearInterval(intervalId);
        };
    }, [isRunning, ws, intervalRate]);

    return (
        <div className="visualization">
            <div className="top-section">
                <div className="left-panel">
                    <InfoBlock />
                    <EventStream />
                </div>
                <HeapGrid memory={memory} />
            </div>
            <ControlPanel isRunning={isRunning} toggleExecution={toggleExecution} onRestart={handleRestart} />
        </div>
    );
}

export default Visualization;
