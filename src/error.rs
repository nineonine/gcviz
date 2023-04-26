#[derive(Debug)]
pub enum VMError {
    AllocationError,
    SegmentationFault,
    NullPointerException(String),
    GCError,
}
