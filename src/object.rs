use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub type ObjAddr = usize;
pub type Value = usize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    #[allow(dead_code)]
    #[serde(rename = "header")]
    header: ObjHeader,
    pub fields: Vec<Field>,
}

impl Object {
    pub fn size(&self) -> usize {
        self.fields.len()
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
            header: ObjHeader {},
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ObjHeader {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Field {
    Ref { addr: Address },
    Scalar { value: Value },
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
            Address::Ptr(addr) => {
                let value = serde_json::json!({ "ptr": *addr });
                value.serialize(serializer)
            }
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
            Ptr { ptr: ObjAddr },
            Null(String),
        }

        match AddressHelper::deserialize(deserializer)? {
            AddressHelper::Ptr { ptr } => Ok(Address::Ptr(ptr)),
            AddressHelper::Null(_) => Ok(Address::Null),
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
