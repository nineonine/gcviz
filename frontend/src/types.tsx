export enum CellStatus {
    Free = "Free",
    ToBeFree = "ToBeFree",
    Allocated = "Allocated",
    Marked = "Marked",
    Used = "Used"
}

export function pprCellStatus(status: CellStatus): string {
    switch (status) {
        case CellStatus.Free:
            return "Free";
        case CellStatus.ToBeFree:
            return "To Be Freed";
        case CellStatus.Allocated:
            return "Allocated";
        case CellStatus.Marked:
            return "Marked";
        case CellStatus.Used:
            return "Used";
        default:
            throw new Error('pprCellStatus');
    }
}

export type Session = {
    program: Program
}

type Program = Instruction[];

type Instruction
    = { _type: 'Allocate', object: Object }
    | { _type: 'Read', object: ObjectAddr }
    | { _type: 'Write', addr: ObjectAddr, payload: Value }
    | { _type: 'GC' }

type ObjectAddr = number;
type Value = number;


export interface MemoryCell {
    status: CellStatus;
}

export type WSMsgRequest
    = { type: 'TICK', pause_on_return: boolean }
    | { type: 'RESET' }

export const TICK_MSG: WSMsgRequest = { type: 'TICK', pause_on_return: false }
export const STEP_MSG: WSMsgRequest = { type: 'TICK', pause_on_return: true }
export const RESET_MSG: WSMsgRequest = { type: 'RESET' }
