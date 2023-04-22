pub type ObjAddr = usize;
pub struct Object {
    header: ObjHeader,
    size: usize,
    fields: Vec<Field>,
}

struct ObjHeader {}

enum Field {
    Ref(Address),
    Scalar(usize),
}

pub enum Address {
    Ptr(usize),
    Null,
}

pub type Offset = usize;
pub type Value = usize;
