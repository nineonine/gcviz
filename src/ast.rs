use crate::object::{ObjAddr, Object, Value};

pub type Program = Vec<ExecFrame>;

pub enum ExecFrame {
    Allocate(Object),
    Read(ObjAddr),
    Write(ObjAddr, Value),
    GC,
}
