use std::collections::VecDeque;

use rand::{distributions::WeightedIndex, prelude::Distribution, seq::SliceRandom, Rng};

use crate::{
    error::VMError,
    frame::{ExecFrame, Program},
    gc::collector::{init_collector, GCType},
    object::{ObjAddr, Object},
    vm::VirtualMachine,
};

pub struct Parameters {
    pub heap_size: usize,
    pub alignment: usize,
    pub num_frames: usize,
    pub probs: FramePropabilities,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            heap_size: 1024,
            alignment: 0,
            num_frames: 100,
            probs: FramePropabilities::default(),
        }
    }
}

pub struct FramePropabilities {
    prob_alloc: f32,
    prob_read: f32,
    prob_write: f32,
    prob_gc: f32,
    prob_write_scalar: f32,
    prob_write_pointer: f32,
}

impl Default for FramePropabilities {
    fn default() -> Self {
        FramePropabilities {
            prob_alloc: 0.55,
            prob_read: 0.2,
            prob_write: 0.2,
            prob_gc: 0.05,
            prob_write_scalar: 0.5,
            prob_write_pointer: 0.5,
        }
    }
}

pub struct Simulator {
    vm: VirtualMachine,
    pub params: Parameters,
}

impl Simulator {
    pub fn new(params: Parameters, gc_ty: &GCType) -> Simulator {
        Simulator {
            vm: VirtualMachine::new(params.alignment, params.heap_size, init_collector(gc_ty)),
            params,
        }
    }

    pub fn gen_program(&mut self) -> Program {
        let mut program = VecDeque::new();
        let mut rng = rand::thread_rng();

        let weights = [
            self.params.probs.prob_alloc,
            self.params.probs.prob_read,
            self.params.probs.prob_write,
            self.params.probs.prob_gc,
        ];
        let dist = WeightedIndex::new(&weights).unwrap();
        for _ in 0..self.params.num_frames {
            let frame = match dist.sample(&mut rng) {
                0 => self.gen_allocate(),
                1 => self.gen_read(),
                2 => self.gen_write(),
                _ => ExecFrame::GC,
            };
            program.push_back(frame);
        }

        program
    }

    fn gen_allocate(&mut self) -> ExecFrame {
        // Generate a random Object
        let object = Object::random();
        match self
            .vm
            .allocator
            .allocate(&mut self.vm.heap, object.clone())
        {
            Ok(_) => ExecFrame::Allocate(object),
            Err(_) => panic!("gen_allocate"),
        }
    }

    fn gen_read(&mut self) -> ExecFrame {
        // Generate a random valid address from the heap
        let mut rng = rand::thread_rng();
        if let Some(obj_addr) = self.random_object_address() {
            let object = &self.vm.heap.objects[&obj_addr];
            let field_offset = rng.gen_range(0..object.fields.len());
            ExecFrame::Read(obj_addr + field_offset)
        } else {
            // If there are no objects in the heap, just allocate
            self.gen_allocate()
        }
    }

    fn gen_write(&mut self) -> ExecFrame {
        let mut rng = rand::thread_rng();

        if let Some(obj_addr) = self
            .vm
            .heap
            .objects
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .choose(&mut rng)
            .cloned()
        {
            let object = &self.vm.heap.objects[&obj_addr];
            let field_offset = rng.gen_range(0..object.fields.len());
            let address = obj_addr + field_offset;

            let p_scalar = self.params.probs.prob_write_scalar;
            let p_pointer = self.params.probs.prob_write_pointer;
            let p_total = p_scalar + p_pointer;

            let value = if rng.gen_range(0.0..p_total) < p_scalar {
                // Write a scalar value with probability `prob_write_scalar`
                rng.gen_range(0..9)
            } else {
                // Write a pointer to another object with probability `prob_write_pointer`
                self.random_object_address().unwrap()
            };

            match self.vm.mutator.write(&mut self.vm.heap, address, value) {
                Ok(_) => ExecFrame::Write(address, value),
                Err(e) => match e {
                    VMError::AllocationError => ExecFrame::GC,
                    _ => panic!("gen_write"),
                },
            }
        } else {
            // If there are no objects in the heap, just allocate
            self.gen_allocate()
        }
    }

    fn random_object_address(&self) -> Option<ObjAddr> {
        let mut rng = rand::thread_rng();
        self.vm
            .heap
            .objects
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .choose(&mut rng)
            .cloned()
    }
}
