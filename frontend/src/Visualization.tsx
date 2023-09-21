import React, { useEffect, useState } from 'react';
import './Visualization.css';

import InfoBlock from './InfoBlock';
import EventStream from './EventStream';
import HeapGrid from './HeapGrid';
import ControlPanel from './ControlPanel';
import { CellStatus, MemoryCell, RESET_MSG, STEP_MSG, TICK_MSG } from './types';
import { LogEntry, SUGGEST_INIT_LOG_ENTRY, mkLogEntry } from './logEntry';

const INTERVAL_RATE = 200; // 0.2 second

const Visualization: React.FC = () => {
    const intervalRate = INTERVAL_RATE;
    const [ws, setWs] = useState<WebSocket | null>(null);
    const [memory, setMemory] = useState<Array<MemoryCell>>(new Array(0).fill({ status: CellStatus.Free }));
    const [isRunning, setIsRunning] = useState(false);
    const [eventLogs, setEventLogs] = useState<LogEntry[]>([SUGGEST_INIT_LOG_ENTRY]);
    const [isHalt, setIsHalt] = useState<boolean>(false);

    const toggleExecution = () => {
        if (isHalt) return;
        setIsRunning(!isRunning);
    };

    const resetViz = (): void => {
        setIsRunning(false);
        setIsHalt(false);
        setMemory(new Array(0).fill({ status: CellStatus.Free }));
        setEventLogs([SUGGEST_INIT_LOG_ENTRY]);
    }

    const handleRestart = () => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify(RESET_MSG));
        }
        resetViz();
    };

    const stepTick = () => {
        if (!isRunning && ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify(STEP_MSG));
        }
    }

    useEffect(() => {
        // Initialize WebSocket connection only once when component mounts
        const wsConnection = new WebSocket("ws://127.0.0.1:9002");
        setWs(wsConnection);

        wsConnection.onerror = (error) => {
            console.error("WebSocket Error", error);
        };

        wsConnection.onmessage = (event) => {
            const data = JSON.parse(event.data);
            switch (data.msgType) {
                case 'TICK': {
                    if (data.log_entry) {
                        setEventLogs(prevLogs => [...prevLogs, data.log_entry]);
                    }

                    if (data.memory) {
                        setMemory(data.memory);
                    }

                    if (data.pause_on_return) {
                        setIsRunning(false);
                    }

                    break;
                }
                case 'HALT': {
                    setIsHalt(true);
                    setIsRunning(false);
                    setEventLogs(prevLogs => [...prevLogs, mkLogEntry("Program halted. Hit 'R' to restart")]);
                    break;
                }
            }
        };

        return () => {
            wsConnection.close();
        };
    }, [intervalRate]);

    useEffect(() => {
        let intervalId: any = null;

        if (isRunning && ws?.readyState === WebSocket.OPEN) {
            intervalId = setInterval(() => {
                ws.send(JSON.stringify(TICK_MSG));
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
                    <EventStream logs={eventLogs} />
                </div>
                <HeapGrid memory={memory} />
            </div>
            <ControlPanel isRunning={isRunning}
                toggleExecution={toggleExecution}
                onRestart={handleRestart}
                onStep={stepTick} />
        </div>
    );
}

export default Visualization;
