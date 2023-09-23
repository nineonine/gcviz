import React, { useEffect, useState } from 'react';
import './Visualization.css';

import InfoBlock from './InfoBlock';
import EventStream from './EventStream';
import HeapGrid from './HeapGrid';
import ControlPanel from './ControlPanel';
import { CellStatus, MemoryCell, EventLogDetails, RESET_MSG, STEP_MSG, TICK_MSG } from './types';
import { SUGGEST_INIT_LOG_ENTRY, mkLogEntry } from './logUtils';
import Slider from './Slider';
import useHighlightState from './useHightlightState';

const INTERVAL_RATE = 100; // 0.2 second

const Visualization: React.FC = () => {
    const [intervalRate, setIntervalRate] = React.useState<number>(INTERVAL_RATE);
    const [ws, setWs] = useState<WebSocket | null>(null);
    const [memory, setMemory] = useState<Array<MemoryCell>>(new Array(0).fill({ status: CellStatus.Free }));
    const [isRunning, setIsRunning] = useState(false);
    const [eventLogs, setEventLogs] = useState<EventLogDetails[]>([[SUGGEST_INIT_LOG_ENTRY, undefined]]);
    const [isHalt, setIsHalt] = useState<boolean>(false);
    const {
        highlightedCells,
        highlightCells,
        clearHighlightedCells,
    } = useHighlightState();


    const toggleExecution = () => {
        if (isHalt) return;
        setIsRunning(!isRunning);
    };

    const resetViz = () => {
        setIsRunning(false);
        setIsHalt(false);
        setMemory(new Array(0).fill({ status: CellStatus.Free }));
        setEventLogs([[SUGGEST_INIT_LOG_ENTRY, undefined]]);
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
                        // console.log(data.log_entry, data.instr_result);
                        let eventLogEntry: EventLogDetails = [data.log_entry, data.instr_result];
                        setEventLogs(prevLogs => [...prevLogs, eventLogEntry]);
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
                    setEventLogs(prevLogs => [...prevLogs, [mkLogEntry("Program halted. Hit 'R' to restart"), undefined]]);
                    break;
                }
            }
        };

        return () => {
            wsConnection.close();
        };
    }, []);

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
                    <Slider minValue={100} maxValue={2000} intervalRate={intervalRate} updateIntervalRate={setIntervalRate} />
                    <EventStream logs={eventLogs} highlightCells={highlightCells} clearHighlightedCells={clearHighlightedCells} />
                </div>
                <HeapGrid memory={memory} highlightedCells={highlightedCells} />
            </div>
            <ControlPanel isRunning={isRunning}
                toggleExecution={toggleExecution}
                onRestart={handleRestart}
                onStep={stepTick} />
        </div>
    );
}

export default Visualization;
