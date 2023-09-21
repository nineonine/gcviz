export interface LogEntry {
    frame_id: number | null;
    msg: string;
    source: LogSource;
}


export enum LogSource {
    GC = "GC",
    MUT = "MUT",
    ALLOC = "ALLOC",
    VM = "VM",
    ERROR = "ERROR",
    CLIENT = "CLIENT"
}

export const SUGGEST_INIT_LOG_ENTRY: LogEntry = {
    frame_id: null,
    msg: "Hit '(Space)' to start program",
    source: LogSource.CLIENT
}

export const mkLogEntry = (msg: string): LogEntry => {
    return { msg, source: LogSource.CLIENT, frame_id: null };
}

export const logSrcToColor = (source: LogSource): string => {
    switch (source) {
        case LogSource.GC:
            return "#FF8C00";
        case LogSource.MUT:
            return "#228B22";
        case LogSource.ALLOC:
            return "#7CCD7C";
        case LogSource.VM:
            return "#DAA520";
        case LogSource.ERROR:
            return "#FF00FF";
        //pure client realm concepts
        case LogSource.CLIENT:
            return "#D87093";
        default:
            throw new Error("Unknown LogSource" + source);
    }
}
