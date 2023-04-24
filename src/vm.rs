use crate::gc::collector::GarbageCollector;
use crate::{allocator::Allocator, heap::Heap, mutator::Mutator};
use crate::{
    ast::ExecFrame::{self, Allocate, Read, Write, GC},
    error::VMError,
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

    pub fn tick(&mut self, frame: ExecFrame) -> Result<(), VMError> {
        match frame {
            Allocate(obj) => if let Ok(addr) = self.allocator.allocate(&mut self.heap, obj) {},
            Read(addr) => match self.mutator.read(&self.heap, addr) {
                Ok(_value) => {}
                Err(err) => {}
            },
            Write(addr, value) => match self.mutator.write(&mut self.heap, addr, value) {
                Ok(addr) => {}
                Err(err) => {}
            },
            GC => match self.collector.collect() {
                Ok(addr) => {}
                Err(err) => {}
            },
        }
        Ok(())
    }
}
