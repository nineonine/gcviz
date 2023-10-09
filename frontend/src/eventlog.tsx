import { logSrcToColor } from "./logUtils";
import { GCEvent, InstrResult, LogEntry } from "./types";

export type EventOps = {
    cellsToHighlight: () => number[];
    render: () => JSX.Element;
};

export const logEntryOps = (log: LogEntry): EventOps => ({
    cellsToHighlight: () => {
        const ir: InstrResult | undefined = log.instrResult;
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
    },
    render: () => (
        <>
            [<span
                className="log-source"
                style={{ color: logSrcToColor(log.source) }}>
                <b>{log.source}</b>
            </span>
            ]: {log.msg} {log.frame_id !== null && <span>(Frame ID: <b>{log.frame_id}</b>)</span>}
        </>
    )
});

export const gcEventOps = (event: GCEvent): EventOps => ({
    cellsToHighlight: () => {
        switch (event.type) {
            case "MarkObject":
            case "FreeObject":
                return [event.addr];
            default:
                return [];
        }
    },
    render: () => {
        let message: string;
        switch (event.type) {
            case "GCPhase":
                message = event.msg;
                break;
            case "MarkObject":
                message = `Marked Object at address ${event.addr}`;
                break;
            case "FreeObject":
                message = `Freed Object at address ${event.addr}`;
                break;
        }
        return (
            <> [
                <span className="log-source">
                    <b>{event.type}</b>
                </span>
                ]: {message}
            </>
        );
    }
});
