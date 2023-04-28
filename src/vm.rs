use crate::gc::GarbageCollector;
use crate::{allocator::Allocator, heap::Heap, mutator::Mutator};
use crate::{
    error::VMError,
    frame::{
        ExecFrame::{self, Allocate, Read, Write, GC},
        FrameResult,
    },
};

pub struct VirtualMachine {
    pub allocator: Allocator,
    pub mutator: Mutator,
    pub collector: Box<dyn GarbageCollector>,
    pub heap: Heap,
}

impl VirtualMachine {
    pub fn new(alignment: usize, heap_size: usize, gc: Box<dyn GarbageCollector>) -> Self {
        VirtualMachine {
            allocator: Allocator::new(alignment),
            mutator: Mutator::new(),
            collector: gc,
            heap: Heap::new(heap_size),
        }
    }

    pub fn tick(&mut self, frame: &ExecFrame) -> Result<FrameResult, VMError> {
        match frame {
            Allocate(obj) => self
                .allocator
                .allocate(&mut self.heap, obj.clone())
                .map(|addr| FrameResult::AllocResult(obj.clone(), addr)),
            Read(addr) => self
                .mutator
                .read(&self.heap, *addr)
                .map(|res| FrameResult::ReadResult(*addr, res)),
            Write(addr, value) => self
                .mutator
                .write(&mut self.heap, *addr, *value)
                .map(|()| FrameResult::WriteResult(*addr, *value)),
            GC => self.collector.collect().map(FrameResult::GCResult),
        }
    }
}
