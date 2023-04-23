use std::collections::HashSet;

pub struct VirtualMachine {
    allocator: Allocator,
    mutator: Mutator,
    collector: Collector,

    heap: Heap,
    roots: HashSet<ObjAddr>,

    heap_size: usize,
    alignment: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            allocator: Allocator::new(),
            mutator: Mutator::new(),
            collector: Collector::new(),
        }
    }

    pub fn run(&self, mut program: Program) {
        while Some(frame) = program.pop() {
            match frame {
                Allocate(obj) => {
                    match allocator.allocate(heap, obj) {
                        Ok(addr) => {},
                        Err(err) => {},
                    }
                },
                Read(addr) => {
                    match mutator.read(heap, addr) {
                        Ok(()) => {},
                        Err(err) => {},
                    }
                },
                Write(addr, value) => {
                    match mutator.write(heap, addr, value) {
                        Ok(addr) => {},
                        Err(err) => {},
                    }
                },
                GC => {
                    match collector.collect() {
                        None => {},
                        Some(_) => {}
                    }
                }
            }
        }
    }
}
