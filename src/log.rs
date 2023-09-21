use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub static LOG_CAPACITY: usize = 1 << 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub msg: String,
    pub source: LogSource,
    pub frame_id: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogSource {
    GC,
    MUT,
    ALLOC,
    VM,
    ERROR,
}

impl fmt::Display for LogSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogSource::GC => write!(f, "GC"),
            LogSource::MUT => write!(f, "MUT"),
            LogSource::ALLOC => write!(f, "ALLOC"),
            LogSource::VM => write!(f, "VM"),
            LogSource::ERROR => write!(f, "ERROR"),
        }
    }
}

impl Log {
    pub fn new(msg: String, source: LogSource, frame_id: Option<usize>) -> Self {
        Log {
            msg,
            source,
            frame_id,
        }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "frameId": self.frame_id,
            "source": format!("{:?}", self.source),
            "msg": &self.msg
        })
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Frame ID: {:?} [{}]: {:?}",
            self.frame_id, self.source, self.msg
        )
    }
}
