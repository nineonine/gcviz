export enum CellStatus {
    Free = "Free",
    ToBeFree = "ToBeFree",
    Allocated = "Allocated",
    Marked = "Marked",
    Used = "Used"
}

export type InstrResult
    = { _type: 'Allocate'; addr: number, object: Object }
    | { _type: 'Read'; addr: number }
    | { _type: 'Write'; addr: number, value: Value }
    | { _type: 'GC' }

interface Object {
    header: {};
    fields: Field[];
}

type Field = { ref: number } | { scalar: number };

type Value = number;


export interface MemoryCell {
    status: CellStatus;
}

export type WSMsgRequest
    = { type: 'Tick', pause_on_return: boolean }
    | { type: 'Reset' }
    | { type: 'LoadProgram', program_name: string }

export const TICK_MSG: WSMsgRequest = { type: 'Tick', pause_on_return: false }
export const STEP_MSG: WSMsgRequest = { type: 'Tick', pause_on_return: true }
export const RESET_MSG: WSMsgRequest = { type: 'Reset' }


export interface LogEntry {
    frame_id: number | null;
    msg: string;
    source: LogSource;
    InstrResult?: InstrResult;
}

export enum LogSource {
    GC = "GC",
    MUT = "MUT",
    ALLOC = "ALLOC",
    VM = "VM",
    ERROR = "ERROR",
    CLIENT = "CLIENT"
}

export type EventLogDetails = [LogEntry, InstrResult | undefined];
