use crate::object::{Object, Value, ObjAddr};

pub type Program = Vec<ExecFrame>;

pub enum ExecFrame {
    Allocate(Object),
    Read(ObjAddr),
    Write(ObjAddr, Value),
    GC,
}
