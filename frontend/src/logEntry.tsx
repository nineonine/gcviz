export interface LogEntry {
    frame_id: number | null;
    msg: string;
    source: string;
}

export function pprLogEntry(logEntry: LogEntry) {

}

export const SUGGEST_INIT_LOG_ENTRY: LogEntry = {
    frame_id: null,
    msg: "Hit '(Space)' to start program",
    source: 'VM'
}

export const mkLogEntry = (msg: string): LogEntry => {
    return { msg, source: 'Client', frame_id: null };
}
