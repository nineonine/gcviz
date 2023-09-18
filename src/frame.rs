use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{
    gc::stats::GCStats,
    object::{ObjAddr, Object, Value},
};

pub type Program = VecDeque<Instruction>;

#[derive(Debug, Serialize, Deserialize)]
pub enum Instruction {
    Allocate(Object),
    Read(ObjAddr),
    Write(ObjAddr, Value),
    GC,
}

pub enum FrameResult {
    AllocResult(Object, usize),
    ReadResult(ObjAddr, Value),
    WriteResult(ObjAddr, Value),
    GCResult(GCStats),
}
