use crate::object::{Address, Object, Value};

pub type Program = Vec<ExecFrame>;

pub enum ExecFrame {
    Allocate(Object),
    Read(Address),
    Write(Address, Value),
    GC,
}
