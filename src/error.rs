pub enum VMError {
    AllocationError,
    InvalidOffset,
    SegmentationFault,
    NullPointerException,
    GCError,
    IllegalMemoryAccess,
}
