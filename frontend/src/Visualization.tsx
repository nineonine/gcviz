import React, { useCallback, useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';

import './Visualization.css';
import InfoBlock from './InfoBlock';
import EventStream from './EventStream';
import HeapGrid from './HeapGrid';
import ControlPanel from './ControlPanel';
import { CellStatus, MemoryCell, RESET_MSG, STEP_MSG, TICK_MSG, InfoBlockData, INFOBLOCK_DEFAULT, GCEvent, LogEntry, WSMsgRequest } from './types';
import Slider from './Slider';
import Toast from './Toast';

import { SUGGEST_INIT_LOG_ENTRY, mkLogEntry } from './logUtils';
import { EventOps, gcEventOps, logEntryOps } from './eventlog';
import useHeapAnimation, { TimedAnimation, createTimedAnimation } from './useHeapAnimation';

function isLogEntry(event: LogEntry | GCEvent): event is LogEntry {
    return (event as LogEntry).source !== undefined;
}

const INTERVAL_RATE = 100; // 0.1 seconds
const BACKEND = 'ws://127.0.0.1:9002';

const Visualization: React.FC = () => {
    const [intervalRate, setIntervalRate] = React.useState<number>(INTERVAL_RATE);
    const [ws, setWs] = useState<WebSocket | null>(null);
    const [memory, setMemory] = useState<Array<MemoryCell>>(new Array(0).fill({ status: CellStatus.Free }));
    const [isRunning, setIsRunning] = useState(false);
    const [eventLogs, setEventLogs] = useState<LogEntry[]>([SUGGEST_INIT_LOG_ENTRY]);
    const [gcEventLogs, setGCEventLogs] = useState<GCEvent[]>([]);
    const [pendingGCEvents, setPendingGCEvents] = useState<GCEvent[]>([]);
    const [isHalt, setIsHalt] = useState<boolean>(false);
    const {
        highlightedCells,
        highlightCells,
        clearHighlightedCells,
        animatedCells,
        enqueueAnimation,
        clearAnimations
    } = useHeapAnimation();
    const { program_name } = useParams<{ program_name?: string }>();
    const [toastMessage, setToastMessage] = useState<string>('');
    const [infoBlock, setInfoBlock] = useState<InfoBlockData>(INFOBLOCK_DEFAULT);

    const toggleExecution = () => {
        if (isHalt) return;
        setIsRunning(!isRunning);
    };

    const resetViz = () => {
        setIsRunning(false);
        setIsHalt(false);
        setInfoBlock(resetInfoBlock(infoBlock, memory.length))
        setMemory(new Array(0).fill({ status: CellStatus.Free }));
        setEventLogs([SUGGEST_INIT_LOG_ENTRY]);
        setPendingGCEvents([]);
        setGCEventLogs([]);
        clearHighlightedCells();
        clearAnimations();
    }

    const handleRestart = () => {
        if (ws?.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify(RESET_MSG));
        }
        resetViz();
    };

    const closeToast = () => {
        setToastMessage('');
    };

    useEffect(() => {
        const wsConnection = new WebSocket(BACKEND);
        setWs(wsConnection);
        console.log('Established ws conn');

        wsConnection.onopen = () => {
            wsConnection.send(JSON.stringify({
                type: 'LoadProgram',
                program_name
            }));
        };

        wsConnection.onerror = (error) => {
            console.error("WebSocket Error", error);
            let msg = "WebSocket connection error. Please refresh the page.";
            setToastMessage(msg);
        };

        wsConnection.onmessage = (event) => {
            const data = JSON.parse(event.data);
            switch (data.msgType) {
                case 'TICK': {
                    console.info(data);
                    if (data.log_entry) {
                        let eventLogEntry: LogEntry = { ...data.log_entry, instrResult: data.instr_result };
                        setEventLogs(prevLogs => [...prevLogs, eventLogEntry]);
                    }

                    if (data.pause_on_return) {
                        setIsRunning(false);
                    }

                    if (data.info_block
                        // for gc event we update the state in the client so no need to set it
                        //(although we could potentially add some ASSERT here)
                        && data.instr_result._type !== "GC") {
                        setInfoBlock(data.info_block)
                    }

                    if (data.instr_result._type === "GC") {
                        setPendingGCEvents(data.instr_result.gc_eventlog);
                    } else {
                        if (data.memory) {
                            setMemory(data.memory);
                        }
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
            if (wsConnection.readyState === WebSocket.OPEN) {
                console.log("cleanup: closing ws connection");
                wsConnection.close();
            }
        };
    }, [program_name]);

    const nextStep = useCallback((msg: WSMsgRequest) => {
        // Process a single GC event if any
        if (pendingGCEvents.length > 0) {
            const alterHeapState = (event: GCEvent): void => {
                if (event.type === 'FreeObject') {
                    const [cellIndex] = cellIndexesFromEvent(event)!;
                    setMemory(memory.map((c, i) => {
                        if (i === cellIndex) {
                            c.status = CellStatus.Free;
                        }
                        return c
                    }));
                } else if (event.type === 'MarkObject') {
                    const [cellIndex] = cellIndexesFromEvent(event)!;
                    setMemory(memory.map((c, i) => {
                        if (i === cellIndex) {
                            c.status = CellStatus.Marked;
                        }
                        return c
                    }));
                } else if (event.type === 'MoveObject') {
                    const { from, to, size } = event;
                    setMemory(memory.map((c, i) => {
                        if (i >= from && i < from + size) {
                            c.status = CellStatus.Free;
                        } else if (i >= to && i < to + size) {
                            c.status = CellStatus.Allocated;
                        }
                        return c;
                    }));
                }
            }
            const currentGCEvent: GCEvent = pendingGCEvents[0];
            setGCEventLogs(prevLogs => [...prevLogs, currentGCEvent]);
            if (eventHasAnimation(currentGCEvent)) {
                const cellIndexes: number[] = cellIndexesFromEvent(currentGCEvent)!;
                enqueueAnimation(cellIndexes, animationFromGCEvent(currentGCEvent));
            }

            // Update heap cell state if needed.
            // For example, when marking or freeing.
            // Note that at the end of GC cycle, the final state of memory should equal
            // to memory state that comes in next program step / instruction. (TODO: add an assert)
            alterHeapState(currentGCEvent);

            // Update InfoBlock as well
            if (hasEffectOnInfoBlock(currentGCEvent)) {
                setInfoBlock(newInfoBlockFromGCEvent(infoBlock, currentGCEvent));
            }

            setPendingGCEvents(prevGCEvents => prevGCEvents.slice(1));
        } else if (ws?.readyState === WebSocket.OPEN) {
            setGCEventLogs([]);
            ws.send(JSON.stringify(msg));
        }
    }, [setGCEventLogs, enqueueAnimation, pendingGCEvents, ws, memory, infoBlock]);

    useEffect(() => {
        let intervalId: any = null;
        if (isRunning) {
            intervalId = setInterval(() => {
                nextStep(TICK_MSG)
            }, intervalRate);
        }

        return () => {
            intervalId && clearInterval(intervalId);
        };
    }, [isRunning, ws, intervalRate, gcEventLogs, pendingGCEvents, nextStep]);

    const stepTick = () => {
        if (!isRunning) {
            nextStep(STEP_MSG)
        }
    }

    const getLogEntryOps = (event: LogEntry | GCEvent): EventOps => {
        if (isLogEntry(event)) return logEntryOps(event);
        throw new Error("EventStream:ops unexpected for LogEntry");
    };

    const getGCEventOps = (event: LogEntry | GCEvent): EventOps => {
        if (!isLogEntry(event)) return gcEventOps(event);
        throw new Error("EventStream:ops unexpected for GCEvent");
    };

    return (
        <div className="visualization">
            <Toast show={toastMessage !== ''} message={toastMessage} onClose={closeToast} />
            <div className="top-section">
                <div className="left-panel">
                    <InfoBlock gc_type={infoBlock.gc_type}
                        alignment={infoBlock.alignment}
                        heap_size={infoBlock.heap_size}
                        allocd_objects={infoBlock.allocd_objects}
                        free_memory={infoBlock.free_memory}
                    />
                    <Slider minValue={100} maxValue={2000} intervalRate={intervalRate} updateIntervalRate={setIntervalRate} />
                    <EventStream
                        className="log-entry"
                        logs={eventLogs}
                        ops={getLogEntryOps}
                        highlightCells={highlightCells}
                        clearHighlightedCells={clearHighlightedCells}
                    />
                    {gcEventLogs.length !== 0 && <EventStream
                        className="gc-event"
                        logs={gcEventLogs}
                        ops={getGCEventOps}
                        highlightCells={highlightCells}
                        clearHighlightedCells={clearHighlightedCells}
                    />}

                    <div className='extra-details'></div>
                </div>
                <HeapGrid memory={memory} highlightedCells={highlightedCells} animatedCells={animatedCells} />
            </div>
            <ControlPanel isRunning={isRunning}
                toggleExecution={toggleExecution}
                onRestart={handleRestart}
                onStep={stepTick} />
        </div>
    );
}

export default Visualization;

const resetInfoBlock = (infoBlock: InfoBlockData, heapSize: number): InfoBlockData => {
    return {
        gc_type: infoBlock.gc_type,
        alignment: infoBlock.alignment,
        heap_size: infoBlock.heap_size,
        allocd_objects: 0,
        free_memory: heapSize,
    }
}

const eventHasAnimation = (gcevent: GCEvent): boolean => {
    return ['MarkObject', 'FreeObject'].includes(gcevent.type)
}

const hasEffectOnInfoBlock = (gcevent: GCEvent): boolean => {
    return ['FreeObject'].includes(gcevent.type)
}

const animationFromGCEvent = (event: GCEvent): TimedAnimation => {
    let animation: TimedAnimation;

    switch (event.type) {
        case "MarkObject":
            animation = createTimedAnimation(500, 'flashing', 0.5);
            break;
        case "FreeObject":
            animation = createTimedAnimation(500, 'flickering', 0.5);
            break;
        default:
            throw new Error(`Unsupported GCEvent: ${event}`);
    }

    return animation;
}

const cellIndexesFromEvent = (event: GCEvent): number[] => {
    switch (event.type) {
        case 'MarkObject':
        case 'FreeObject':
            return [event.addr];
        case 'MoveObject':
            return [event.from, event.to];
        case 'UpdateFwdPtr':
            return [event.old, event.new];
        case 'GCPhase':
        default:
            return [];
    }
};

const newInfoBlockFromGCEvent = (infoBlock: InfoBlockData, gcevent: GCEvent): InfoBlockData => {
    console.info('before', infoBlock)
    if (gcevent.type === 'FreeObject') {
        infoBlock.free_memory += gcevent.size;
        infoBlock.allocd_objects--;
        console.info('After', infoBlock)
        return infoBlock;
    }
    throw new Error('newInfoBlockFromGCEvent unexpected gcevent');
}
