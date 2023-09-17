export enum CellStatus {
    Free = "Free",
    ToBeFree = "ToBeFree",
    Allocated = "Allocated",
    Marked = "Marked",
    Used = "Used"
}

export function pprCellStatus(status: CellStatus): string {
    switch(status) {
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
