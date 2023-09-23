use crate::{heap::MemoryCell, instr::InstrResult, log::Log};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};

#[derive(Deserialize, Debug)]
pub struct WSMessageRequest {
    #[serde(rename = "type")]
    pub msg_type: WSMessageRequestType,
    pub pause_on_return: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum WSMessageRequestType {
    TICK,
    RESET,
}

#[derive(Debug, Deserialize)]
pub enum WSMessageResponse {
    Tick {
        memory: Vec<MemoryCell>,
        log_entry: Option<Log>,
        pause_on_return: Option<bool>,
        instr_result: Option<InstrResult>,
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
            } => {
                state.serialize_field("msgType", "TICK")?;
                state.serialize_field("memory", memory)?;
                state.serialize_field("log_entry", log_entry)?;
                state.serialize_field("pause_on_return", pause_on_return)?;
                state.serialize_field("instr_result", instr_result)?;
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
    ) -> Self {
        WSMessageResponse::Tick {
            memory,
            log_entry,
            pause_on_return,
            instr_result,
        }
    }

    pub fn halt() -> Self {
        WSMessageResponse::Halt
    }
}
