use std::cmp;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

#[derive(Clone, Copy, Debug)]
pub struct MemoryCell {
    pub status: CellStatus,
}

#[derive(Clone, Copy, Debug)]
pub enum CellStatus {
    Free,
    Allocated,
    Marked,
}

pub struct MemoryGrid<'a> {
    block: Block<'a>,
    memory: Vec<MemoryCell>,
    num_cols: usize,
    num_rows: usize,
}

impl<'a> MemoryGrid<'a> {
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

impl<'a> Widget for MemoryGrid<'a> {
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
                    CellStatus::Free => Style::default().bg(Color::Green),
                    CellStatus::Allocated => Style::default().bg(Color::Blue),
                    CellStatus::Marked => Style::default().bg(Color::Red),
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
