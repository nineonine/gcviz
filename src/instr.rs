use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{
    gc::stats::GCStats,
    object::{Object, Value},
};

pub type Program = VecDeque<Instruction>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Instruction {
    Allocate { object: Object },
    Read { addr: usize },
    Write { addr: usize, value: Value },
    GC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum InstrResult {
    Allocate { addr: usize, object: Object },
    Read { addr: usize, value: Value },
    Write { addr: usize, value: Value },
    GC { stats: GCStats },
}
