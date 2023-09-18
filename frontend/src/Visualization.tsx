import React, { useEffect, useState } from 'react';
import './Visualization.css';

import InfoBlock from './InfoBlock';
import EventStream from './EventStream';
import HeapGrid from './HeapGrid';
import ControlPanel from './ControlPanel';
import { CellStatus, MemoryCell } from './types';

const INTERVAL_RATE = 1000; // 1 second

const Visualization: React.FC = () => {
    const intervalRate = INTERVAL_RATE;
    const [memory, setMemory] = useState<Array<MemoryCell>>(new Array(1024).fill({status: CellStatus.Free}));

    useEffect(() => {
        // Initialize WebSocket connection
        const wsConnection = new WebSocket("ws://127.0.0.1:9002");

        // Handle any errors that occur on the WebSocket connection
        wsConnection.onerror = (error) => {
            console.error("WebSocket Error", error);
        };

        // Periodically send a message over WebSocket
        const intervalId = setInterval(() => {
            if (wsConnection.readyState === WebSocket.OPEN) {
                wsConnection.send("Message to send every interval");
            }
        }, intervalRate);

        wsConnection.onmessage = (event) => {
            // console.log("Received message:", event.data);
            const newMemory = JSON.parse(event.data);
            setMemory(newMemory);
        };

        // Cleanup: close the WebSocket and clear the interval when the component is unmounted
        return () => {
            clearInterval(intervalId);
            wsConnection.close();
        };
    }, [intervalRate]);

    return (
        <div className="visualization">
            <div className="top-section">
                <div className="left-panel">
                    <InfoBlock />
                    <EventStream />
                </div>
                <HeapGrid memory={memory} />
            </div>
            <ControlPanel />
        </div>
    );
}

export default Visualization;
