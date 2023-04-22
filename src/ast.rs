use crate::object::{Address, Object, Offset};

pub type Program = Vec<ExecFrame>;

pub enum ExecFrame {
    Allocate(Object),
    Read(Address, Offset),
    Write(Address, Offset, usize),
    GC,
}
