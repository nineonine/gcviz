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

#[derive(Clone, Debug)]
struct ObjHeader {}

#[derive(Clone, Debug)]
pub enum Field {
    Ref(Address),
    Scalar(Value),
}

#[derive(Clone, Debug)]
pub enum Address {
    Ptr(ObjAddr),
    Null,
}
