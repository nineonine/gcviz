use crate::gc::GarbageCollector;
use crate::{allocator::Allocator, heap::Heap, mutator::Mutator};
use crate::{
    error::VMError,
    instr::{
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
            Allocate { object } => {
                self.allocator
                    .allocate(&mut self.heap, object.clone())
                    .map(|addr| InstrResult::Allocate {
                        object: object.clone(),
                        addr,
                    })
            }
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
            GC => self
                .collector
                .collect()
                .map(|stats| InstrResult::GC { stats }),
        }
    }
}
