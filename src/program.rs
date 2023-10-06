use std::{collections::VecDeque, fmt};

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::{
    gc::stats::GCStats,
    object::{Object, Value},
};

pub type Program = VecDeque<Instruction>;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "_type")]
pub enum Instruction {
    Allocate { object: Object, is_root: bool },
    Read { addr: usize },
    Write { addr: usize, value: Value },
    GC,
}

impl<'de> Deserialize<'de> for Instruction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InstructionVisitor;

        impl<'de> Visitor<'de> for InstructionVisitor {
            type Value = Instruction;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Instruction")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Instruction, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut _type: Option<String> = None;
                let mut object: Option<Object> = None;
                let mut addr: Option<usize> = None;
                let mut value: Option<Value> = None;
                let mut is_root: Option<bool> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "_type" => _type = map.next_value()?,
                        "object" => object = map.next_value()?,
                        "addr" => addr = map.next_value()?,
                        "value" => value = map.next_value()?,
                        "is_root" => is_root = map.next_value()?,
                        _ => {}
                    }
                }

                match _type.as_deref() {
                    Some("Allocate") => Ok(Instruction::Allocate {
                        object: object.ok_or_else(|| de::Error::missing_field("object"))?,
                        is_root: is_root.unwrap_or(true),
                    }),
                    Some("Read") => Ok(Instruction::Read {
                        addr: addr.ok_or_else(|| de::Error::missing_field("addr"))?,
                    }),
                    Some("Write") => Ok(Instruction::Write {
                        addr: addr.ok_or_else(|| de::Error::missing_field("addr"))?,
                        value: value.ok_or_else(|| de::Error::missing_field("value"))?,
                    }),
                    Some("GC") => Ok(Instruction::GC),
                    _ => Err(de::Error::custom("Invalid instruction type")),
                }
            }
        }

        const FIELDS: &[&str] = &["_type", "object", "addr", "value", "is_root"];
        deserializer.deserialize_struct("Instruction", FIELDS, InstructionVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum InstrResult {
    Allocate { addr: usize, object: Object },
    Read { addr: usize, value: Value },
    Write { addr: usize, value: Value },
    GC { stats: GCStats },
}
