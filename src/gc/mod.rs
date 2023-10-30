pub mod compact;
pub mod mark_compact;
pub mod mark_sweep;
pub mod object_marker;
pub mod stats;

use serde::{Deserialize, Deserializer, Serialize};

use self::{
    mark_compact::{CompactAlgorithm, MarkCompact},
    mark_sweep::MarkSweep,
    stats::GCStats,
};
use crate::{error::VMError, heap::Heap};

#[derive(Debug, Clone)]
pub enum GCType {
    MarkSweep,
    MarkCompact(CompactAlgorithm),
}

impl Serialize for GCType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            GCType::MarkSweep => serializer.serialize_str("MarkSweep"),
            GCType::MarkCompact(algo) => {
                let value = format!("MarkCompact_{}", algo.to_string());
                serializer.serialize_str(&value)
            }
        }
    }
}

impl ToString for CompactAlgorithm {
    fn to_string(&self) -> String {
        match self {
            CompactAlgorithm::TwoFinger => "TwoFinger".to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for GCType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "MarkSweep" => Ok(GCType::MarkSweep),
            "MarkCompact_TwoFinger" => Ok(GCType::MarkCompact(CompactAlgorithm::TwoFinger)),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["MarkSweep", "MarkCompactTwoFinger"],
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GCEvent {
    GCPhase { msg: String },
    MarkObject { addr: usize, size: usize },
    FreeObject { addr: usize, size: usize },
    MoveObject { from: usize, to: usize, size: usize },
    UpdateFwdPtr { old: usize, new: usize },
}

impl GCEvent {
    fn phase(msg: String) -> Self {
        GCEvent::GCPhase { msg }
    }
}

pub fn init_collector(gc_ty: &GCType) -> Box<dyn GarbageCollector> {
    match gc_ty {
        GCType::MarkSweep => Box::new(MarkSweep::new()),
        GCType::MarkCompact(algo) => Box::new(MarkCompact::new(algo.clone())),
    }
}

pub trait GarbageCollector: Send {
    fn collect(&self, heap: &mut Heap) -> Result<(GCStats, Vec<GCEvent>), VMError>;
    fn ty(&self) -> GCType;
    fn new_instance(&self) -> Box<dyn GarbageCollector>;
}
