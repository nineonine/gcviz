use crate::{heap::MemoryCell, log::Log};
use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize,
};

#[derive(Debug, Deserialize)]
pub enum WSMessageResponse {
    Tick {
        memory: Vec<MemoryCell>,
        log_entry: Option<Log>,
    },
}

impl Serialize for WSMessageResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Start a struct serialization with 3 fields.
        let mut state = serializer.serialize_struct("WSMessageResponse", 3)?;

        match self {
            WSMessageResponse::Tick { memory, log_entry } => {
                state.serialize_field("msgType", "TICK")?;
                state.serialize_field("memory", memory)?;
                state.serialize_field("log_entry", log_entry)?;
            }
        }

        // Finish the struct.
        state.end()
    }
}

impl WSMessageResponse {
    pub fn new_tick(memory: Vec<MemoryCell>, log_entry: Option<Log>) -> Self {
        WSMessageResponse::Tick { memory, log_entry }
    }
}
