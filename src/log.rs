pub static LOG_CAPACITY: usize = 1 << 4;

#[derive(Debug, Clone)]
pub struct Log {
    pub msg: String,
    pub source: LogSource,
    pub frame_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum LogSource {
    GC,
    MUT,
    ALLOC,
    VM,
    ERROR,
}

impl Log {
    pub fn new(msg: String, source: LogSource, frame_id: Option<usize>) -> Self {
        Log {
            msg,
            source,
            frame_id,
        }
    }
}
