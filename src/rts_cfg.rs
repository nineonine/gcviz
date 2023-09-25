use serde::{Deserialize, Serialize};

use crate::gc::GCType;

static ALIGNMENT: usize = 4;
static HEAP_SIZE: usize = 1024;

/// Program simulation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramRuntimeConfig {
    pub heap_size: usize,
    pub alignment: usize,
    pub gc_ty: GCType,
}

impl Default for ProgramRuntimeConfig {
    fn default() -> Self {
        ProgramRuntimeConfig {
            heap_size: HEAP_SIZE,
            alignment: ALIGNMENT,
            gc_ty: GCType::MarkSweep,
        }
    }
}

impl ProgramRuntimeConfig {
    pub fn new(heap_size: usize, alignment: usize, gc_ty: GCType) -> Self {
        ProgramRuntimeConfig {
            heap_size,
            alignment,
            gc_ty,
        }
    }
}
