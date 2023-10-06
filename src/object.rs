use rand::Rng;
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fmt};

pub type ObjAddr = usize;
pub type Value = usize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    #[allow(dead_code)]
    #[serde(rename = "header")]
    pub header: ObjHeader,
    pub fields: Vec<Field>,
}

impl Object {
    pub fn size(&self) -> usize {
        self.fields.len()
    }

    pub fn new(fields: Vec<Field>) -> Self {
        Self {
            header: ObjHeader { marked: false },
            fields,
        }
    }

    pub fn random() -> Object {
        let mut rng = rand::thread_rng();

        // Generate a random number of fields
        let num_fields = rng.gen_range(1..=10);

        // Generate random fields
        let fields: Vec<Field> = (0..num_fields)
            .map(|_| {
                if rng.gen_bool(0.5) {
                    Field::Ref {
                        addr: Address::Null,
                    }
                } else {
                    Field::Scalar {
                        value: rng.gen_range(0..=9),
                    }
                }
            })
            .collect();

        // Create an object with generated fields
        Object {
            header: ObjHeader { marked: false },
            fields,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object [size: {}] [", self.size())?;

        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{field}")?;
        }

        write!(f, "]")
    }
}

#[derive(Clone, Debug)]
pub struct ObjHeader {
    pub marked: bool,
}

impl Serialize for ObjHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_map(None)?.end()
    }
}

impl<'de> Deserialize<'de> for ObjHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut marked = false;

        let v: Option<HashMap<String, bool>> = Option::deserialize(deserializer)?;

        if let Some(map) = v {
            if let Some(m) = map.get("marked") {
                marked = *m;
            }
        }

        Ok(ObjHeader { marked })
    }
}

#[derive(Clone, Debug)]
pub enum Field {
    Ref { addr: Address },
    Scalar { value: Value },
}

impl Field {
    pub fn new_scalar(value: usize) -> Self {
        Field::Scalar { value }
    }
}

impl Serialize for Field {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Field::Scalar { value } => {
                let mut map = serde::ser::Serializer::serialize_map(serializer, Some(1))?;
                map.serialize_entry("value", value)?;
                map.end()
            }
            Field::Ref { addr } => {
                let mut map = serde::ser::Serializer::serialize_map(serializer, Some(1))?;
                map.serialize_entry("addr", addr)?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum FieldHelper {
            Ref { addr: Address },
            Scalar { value: Value },
        }

        match FieldHelper::deserialize(deserializer)? {
            FieldHelper::Scalar { value } => Ok(Field::Scalar { value }),
            FieldHelper::Ref { addr } => Ok(Field::Ref { addr }),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Ref { addr } => write!(f, "({addr})"),
            Field::Scalar { value } => write!(f, "{value}"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Address {
    Ptr(ObjAddr),
    Null,
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Address::Ptr(addr) => serializer.serialize_u64(*addr as u64),
            Address::Null => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum AddressHelper {
            Ptr(ObjAddr),
            NamedPtr { ptr: ObjAddr },
            Null,
        }

        match AddressHelper::deserialize(deserializer)? {
            AddressHelper::Ptr(ptr) => Ok(Address::Ptr(ptr)),
            AddressHelper::NamedPtr { ptr } => Ok(Address::Ptr(ptr)),
            AddressHelper::Null => Ok(Address::Null),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Address::Ptr(addr) => write!(f, "{addr}"),
            Address::Null => write!(f, "Null"),
        }
    }
}
