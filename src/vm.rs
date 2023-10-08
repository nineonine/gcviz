use crate::gc::GarbageCollector;
use crate::{allocator::Allocator, heap::Heap, mutator::Mutator};
use crate::{
    error::VMError,
    program::{
        InstrResult,
        Instruction::{self, Allocate, Read, Write, GC},
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

    pub fn tick(&mut self, instr: &Instruction) -> Result<InstrResult, VMError> {
        match instr {
            Allocate { object, is_root } => self
                .allocator
                .allocate(&mut self.heap, object.clone(), *is_root)
                .map(|addr| InstrResult::Allocate {
                    object: object.clone(),
                    addr,
                }),
            Read { addr } => self
                .mutator
                .read(&self.heap, *addr)
                .map(|value| InstrResult::Read { addr: *addr, value }),
            Write { addr, value } => {
                self.mutator
                    .write(&mut self.heap, *addr, *value)
                    .map(|()| InstrResult::Write {
                        addr: *addr,
                        value: *value,
                    })
            }
            GC => {
                let result = self.collector.collect(&mut self.heap);
                self.heap.merge_free_ranges();
                result.map(|stats| InstrResult::GC { stats })
            }
        }
    }

    pub fn reset_heap(&mut self, size: usize) {
        self.heap = Heap::new(size);
    }
}
