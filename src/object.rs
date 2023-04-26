use std::fmt;

use rand::Rng;

pub type ObjAddr = usize;
pub type Value = usize;

#[derive(Clone, Debug)]
pub struct Object {
    #[allow(dead_code)]
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
                    Field::Ref(Address::Null)
                } else {
                    Field::Scalar(rng.gen_range(0..=9))
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

#[derive(Clone, Debug)]
struct ObjHeader {}

#[derive(Clone, Debug)]
pub enum Field {
    Ref(Address),
    Scalar(Value),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Ref(addr) => write!(f, "({addr})"),
            Field::Scalar(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Address {
    Ptr(ObjAddr),
    Null,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Address::Ptr(addr) => write!(f, "{addr}"),
            Address::Null => write!(f, "Null"),
        }
    }
}
