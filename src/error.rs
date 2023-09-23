use std::fmt;

#[derive(Debug)]
pub enum VMError {
    AllocationError,
    SegmentationFault,
    NullPointerException(String),
    GCError,
    UnknownError,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VMError::AllocationError => write!(f, "Allocation error"),
            VMError::SegmentationFault => write!(f, "Segmentation fault"),
            VMError::NullPointerException(detail) => {
                write!(f, "Null pointer exception: {detail}")
            }
            VMError::GCError => write!(f, "Garbage collector error"),
            VMError::UnknownError => write!(f, "UnknownError error"),
        }
    }
}
