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
    | { _type: 'GC', gc_eventlog: GCEvent[] }

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
    instrResult?: InstrResult; // could be absent for client or system log messages
}

export enum LogSource {
    GC = "GC",
    MUT = "MUT",
    ALLOC = "ALLOC",
    VM = "VM",
    ERROR = "ERROR",
    CLIENT = "CLIENT"
}

export interface InfoBlockData {
    gc_type: string;
    alignment: number;
    heap_size: number;
    allocd_objects: number;
    free_memory: number;
}

export const INFOBLOCK_DEFAULT: InfoBlockData = {
    gc_type: '',
    alignment: -1,
    heap_size: -1,
    allocd_objects: -1,
    free_memory: -1,
}

export type GCEvent =
    | { type: "GCPhase", msg: string }
    | { type: "MarkObject", addr: number, size: number }
    | { type: "FreeObject", addr: number, size: number }
    | { type: "UpdateFwdPtr", old: number, new: number };
