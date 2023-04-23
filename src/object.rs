pub type ObjAddr = usize;
pub type Offset = usize;
pub type Value = usize;

pub struct Object {
    header: ObjHeader,
    pub fields: Vec<Field>,
}

impl Object {
    pub fn size(&self) -> usize {
        self.fields.len()
    }
}

struct ObjHeader {}

pub enum Field {
    Ref(Address),
    Scalar(Value),
}

pub enum Address {
    Ptr(ObjAddr),
    Null,
}
