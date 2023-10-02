use crate::{gc::GCType, heap::MemoryCell, log::Log, program::InstrResult};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};

#[derive(Deserialize, Debug)]
pub struct WSMessageRequest {
    #[serde(rename = "type")]
    pub msg_type: WSMessageRequestType,
    pub pause_on_return: Option<bool>,
    pub program_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum WSMessageRequestType {
    Tick,
    Reset,
    LoadProgram,
}

#[derive(Debug, Deserialize)]
pub enum WSMessageResponse {
    Tick {
        memory: Vec<MemoryCell>,
        log_entry: Option<Log>,
        pause_on_return: Option<bool>,
        instr_result: Option<InstrResult>,
        info_block: InfoBlockData,
    },
    Halt,
}

impl Serialize for WSMessageResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Start a struct serialization with 3 fields.
        let mut state = serializer.serialize_struct("WSMessageResponse", 3)?;

        match self {
            WSMessageResponse::Tick {
                memory,
                log_entry,
                pause_on_return,
                instr_result,
                info_block,
            } => {
                state.serialize_field("msgType", "TICK")?;
                state.serialize_field("memory", memory)?;
                state.serialize_field("log_entry", log_entry)?;
                state.serialize_field("pause_on_return", pause_on_return)?;
                state.serialize_field("instr_result", instr_result)?;
                state.serialize_field("info_block", info_block)?;
            }
            WSMessageResponse::Halt => {
                state.serialize_field("msgType", "HALT")?;
            }
        }

        // Finish the struct.
        state.end()
    }
}

impl WSMessageResponse {
    pub fn new_tick(
        memory: Vec<MemoryCell>,
        log_entry: Option<Log>,
        pause_on_return: Option<bool>,
        instr_result: Option<InstrResult>,
        info_block: InfoBlockData,
    ) -> Self {
        WSMessageResponse::Tick {
            memory,
            log_entry,
            pause_on_return,
            instr_result,
            info_block,
        }
    }

    pub fn halt() -> Self {
        WSMessageResponse::Halt
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoBlockData {
    pub gc_type: GCType,
    pub alignment: usize,
    pub heap_size: usize,
    pub allocd_objects: usize,
    pub free_memory: usize,
}
