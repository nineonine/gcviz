use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};
use std::{
    cmp,
    collections::{BTreeMap, HashSet},
};

use crate::{
    error::VMError,
    object::{ObjAddr, Object},
};

pub struct Heap {
    pub roots: HashSet<ObjAddr>,
    pub objects: BTreeMap<ObjAddr, Object>,
    pub free_list: Vec<(usize, usize)>,
    pub memory: Vec<MemoryCell>,
}

impl Heap {
    pub fn new(size: usize) -> Self {
        Heap {
            roots: HashSet::new(),
            objects: BTreeMap::new(),
            memory: vec![MemoryCell::free(); size],
            free_list: vec![(0, size)],
        }
    }

    pub fn lookup_object(&self, address: ObjAddr) -> Result<ObjAddr, VMError> {
        let mut iter = self.objects.range(..=address).rev(); // Get an iterator in reverse order up to the given address
        if let Some((obj_addr, object)) = iter.next() {
            let object_size = object.size();
            if *obj_addr <= address && address < *obj_addr + object_size {
                return Ok(*obj_addr);
            }
        }
        Err(VMError::SegmentationFault)
    }

    /// Builds memory grid for visualizations
    pub fn visualize(&self, memory: Option<&Vec<MemoryCell>>) -> Vec<MemoryCell> {
        let mut visualized_memory = match memory {
            Some(mem) => mem.clone(),
            None => vec![MemoryCell::free(); self.memory.len()],
        };

        for (&address, object) in &self.objects {
            let object_size = object.size();
            visualized_memory[address..(object_size + address)]
                .copy_from_slice(&self.memory[address..(object_size + address)]);
        }
        visualized_memory
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MemoryCell {
    pub status: CellStatus,
}

impl MemoryCell {
    pub fn new(status: CellStatus) -> Self {
        MemoryCell { status }
    }

    pub fn free() -> Self {
        MemoryCell {
            status: CellStatus::Freed,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CellStatus {
    Freed,
    ToBeFreed,
    Allocated,
    Marked,
    Used,
}

pub struct HeapGrid<'a> {
    block: Block<'a>,
    memory: Vec<MemoryCell>,
    num_cols: usize,
    num_rows: usize,
}

impl<'a> HeapGrid<'a> {
    pub fn new(memory: Vec<MemoryCell>) -> Self {
        let memory_len = memory.len();
        let num_cols = (memory_len as f64).sqrt().ceil() as usize;
        let num_rows = cmp::max((memory_len as f64 / num_cols as f64).ceil() as usize, 1);
        Self {
            block: Block::default().title("Heap").borders(Borders::ALL),
            memory,
            num_cols,
            num_rows,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }
}

impl<'a> Widget for HeapGrid<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Self {
            block,
            memory,
            num_cols,
            num_rows,
        } = self;

        let inner_area = block.inner(area);
        block.render(area, buf);

        let cell_width = inner_area.width / num_cols as u16;
        let cell_height = inner_area.height / num_rows as u16;

        let remaining_width = inner_area.width % num_cols as u16;
        let remaining_height = inner_area.height % num_rows as u16;

        for (i, cell) in memory.iter().enumerate() {
            let row = i / num_cols;
            let col = i % num_cols;

            if row < num_rows && col < num_cols {
                let extra_width = if col < remaining_width as usize { 1 } else { 0 };
                let extra_height = if row < remaining_height as usize {
                    1
                } else {
                    0
                };

                let x = inner_area.x + col as u16 * cell_width + (col as u16).min(remaining_width);
                let y =
                    inner_area.y + row as u16 * cell_height + (row as u16).min(remaining_height);

                let cell_rect = Rect {
                    x,
                    y,
                    width: cell_width + extra_width,
                    height: cell_height + extra_height,
                };

                let cell_style = match cell.status {
                    CellStatus::Freed => Style::default().bg(Color::Black),
                    CellStatus::ToBeFreed => Style::default().bg(Color::Magenta),
                    CellStatus::Allocated => Style::default().bg(Color::Green),
                    CellStatus::Marked => Style::default().bg(Color::Yellow),
                    CellStatus::Used => Style::default().bg(Color::LightGreen),
                };

                for y in cell_rect.top()..cell_rect.bottom() {
                    for x in cell_rect.left()..cell_rect.right() {
                        buf.get_mut(x, y).set_style(cell_style);
                    }
                }
            }
        }
    }
}

pub fn reset_highlights(memory: &mut Vec<MemoryCell>) {
    for cell in memory.iter_mut() {
        match cell.status {
            CellStatus::ToBeFreed => {
                cell.status = CellStatus::Freed;
            }
            CellStatus::Used => {
                cell.status = CellStatus::Allocated;
            }
            _ => {}
        }
    }
}

pub fn visualize_mutator(memory: &mut Vec<MemoryCell>, addr: usize) {
    memory[addr] = MemoryCell::new(CellStatus::Used);
}
