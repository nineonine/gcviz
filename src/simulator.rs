use std::collections::{HashSet, VecDeque};

use rand::{distributions::WeightedIndex, prelude::Distribution, seq::SliceRandom, Rng};

use crate::{
    error::VMError,
    gc::init_collector,
    object::{Field, ObjAddr, Object},
    program::{Instruction, Program},
    rts_cfg::ProgramRuntimeConfig,
    vm::VirtualMachine,
};

static NUM_INSTRS: usize = 100;

#[derive(Debug, Clone)]
pub struct ProgGenConfig {
    pub num_instrs: usize,
    pub prob_alloc: f32,
    pub prob_read: f32,
    pub prob_write: f32,
    pub prob_gc: f32,
    pub prob_write_scalar: f32,
    pub prob_write_pointer: f32,
}

impl Default for ProgGenConfig {
    fn default() -> Self {
        ProgGenConfig {
            num_instrs: NUM_INSTRS,
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
    pub rts_cfg: ProgramRuntimeConfig,
    prog_gen_cfg: ProgGenConfig,
}

impl Simulator {
    pub fn new(rts_cfg: ProgramRuntimeConfig) -> Simulator {
        Simulator {
            vm: VirtualMachine::new(
                rts_cfg.alignment,
                rts_cfg.heap_size,
                init_collector(&rts_cfg.gc_ty),
            ),
            rts_cfg,
            prog_gen_cfg: ProgGenConfig::default(),
        }
    }

    pub fn gen_program(&mut self) -> Program {
        let mut program = VecDeque::new();
        let mut rng = rand::thread_rng();

        let weights = [
            self.prog_gen_cfg.prob_alloc,
            self.prog_gen_cfg.prob_read,
            self.prog_gen_cfg.prob_write,
            self.prog_gen_cfg.prob_gc,
        ];
        let dist = WeightedIndex::new(weights).unwrap();
        for _ in 0..self.prog_gen_cfg.num_instrs {
            let frame = match dist.sample(&mut rng) {
                0 => self.gen_allocate(),
                1 => self.gen_read(),
                2 => self.gen_write(),
                _ => Instruction::GC,
            };
            program.push_back(frame);
        }

        program
    }

    fn gen_allocate(&mut self) -> Instruction {
        // Generate a random Object
        let object = Object::random();
        // TODO: hook up is_root generation
        let is_root = true;
        match self
            .vm
            .allocator
            .allocate(&mut self.vm.heap, object.clone(), is_root)
        {
            Ok(_) => Instruction::Allocate {
                object,
                is_root: true,
            },
            Err(_) => panic!("gen_allocate"),
        }
    }

    fn gen_read(&mut self) -> Instruction {
        // Generate a random valid address from the heap
        let mut rng = rand::thread_rng();
        if let Some(obj_addr) = self.random_object_address() {
            let object = &self.vm.heap.objects[&obj_addr];
            let valid_indexes = object
                .fields
                .iter()
                .enumerate()
                .filter(|(_, field)| matches!(field, Field::Scalar { value: _ }))
                .map(|(idx, _)| idx)
                .collect::<Vec<_>>();

            if valid_indexes.is_empty() {
                // If there are no valid fields to read, just allocate
                self.gen_allocate()
            } else {
                let field_offset = valid_indexes.choose(&mut rng).unwrap();
                Instruction::Read {
                    addr: obj_addr + field_offset,
                }
            }
        } else {
            // If there are no objects in the heap, just allocate
            self.gen_allocate()
        }
    }

    fn gen_write(&mut self) -> Instruction {
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

            let p_scalar = self.prog_gen_cfg.prob_write_scalar;
            let p_pointer = self.prog_gen_cfg.prob_write_pointer;
            let p_total = p_scalar + p_pointer;

            let value = if rng.gen_range(0.0..p_total) < p_scalar {
                // Write a scalar value with probability `prob_write_scalar`
                rng.gen_range(0..9)
            } else {
                // Write a pointer to another object with probability `prob_write_pointer`
                let ref_chain = self.reference_chain(obj_addr);
                let possible_addresses: Vec<ObjAddr> = self
                    .vm
                    .heap
                    .objects
                    .keys()
                    .cloned()
                    .filter(|a| {
                        !ref_chain.contains(a)
                            && self.vm.heap.objects[a]
                                .fields
                                .iter()
                                .any(|field| matches!(field, Field::Scalar { value: _ }))
                    })
                    .collect();

                if let Some(new_obj_addr) = possible_addresses.choose(&mut rng).cloned() {
                    new_obj_addr
                } else {
                    // No valid object address found, allocate a new object instead
                    return self.gen_allocate();
                }
            };

            match self.vm.mutator.write(&mut self.vm.heap, address, value) {
                Ok(_) => Instruction::Write {
                    addr: address,
                    value,
                },
                Err(e) => match e {
                    VMError::AllocationError => Instruction::GC,
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

    fn reference_chain(&self, addr: ObjAddr) -> HashSet<ObjAddr> {
        let mut chain = HashSet::new();
        let mut current_addr = Some(addr);

        while let Some(address) = current_addr {
            if chain.insert(address) {
                match self.vm.mutator.read(&self.vm.heap, address) {
                    Ok(value) => {
                        if self.vm.heap.objects.contains_key(&value) {
                            current_addr = Some(value);
                        } else {
                            current_addr = None;
                        }
                    }
                    Err(_) => current_addr = None,
                }
            } else {
                // Circular reference detected, stop
                break;
            }
        }

        chain
    }
}
