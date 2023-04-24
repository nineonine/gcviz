#[derive(Debug)]
pub enum VMError {
    AllocationError,
    SegmentationFault,
    NullPointerException,
    GCError,
}
