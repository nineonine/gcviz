use crate::error::VMError;

pub trait GarbageCollector {
    fn collect(&self) -> Result<(), VMError>;
}
