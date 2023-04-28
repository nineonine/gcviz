use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::App;

pub fn render<B: Backend>(app: &App, f: &mut Frame<'_, B>, area: Rect) {
    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    let left_block = Block::default().borders(Borders::ALL);
    f.render_widget(left_block, info_chunks[0]);

    let right_block = Block::default().borders(Borders::ALL);
    f.render_widget(right_block, info_chunks[1]);

    let padding = 2;
    let left_margin = 3;
    let left_table_area = Rect::new(
        info_chunks[0].x + padding + left_margin,
        info_chunks[0].y + padding,
        info_chunks[0].width - 2 * padding - left_margin,
        info_chunks[0].height - 2 * padding,
    );
    let right_table_area = Rect::new(
        info_chunks[1].x + padding + left_margin,
        info_chunks[1].y + padding,
        info_chunks[1].width - 2 * padding - left_margin,
        info_chunks[1].height - 2 * padding,
    );

    // Left block
    let left_table = Table::new(vec![
        Row::new(vec![
            Cell::from("GC Type"),
            Cell::from(format!("{:?}", app.vm.collector.ty())),
        ]),
        Row::new(vec![
            Cell::from("Alignment"),
            Cell::from(format!("{}", app.vm.allocator.alignment)),
        ]),
        Row::new(vec![
            Cell::from("Heap Size"),
            Cell::from(format!("{}", app.vm.heap.memory.len())),
        ]),
        Row::new(vec![
            Cell::from("Allocated objects"),
            Cell::from(format!("{}", app.vm.heap.objects.len())),
        ]),
        Row::new(vec![
            Cell::from("Free Memory"),
            Cell::from(format!("{}", app.vm.heap.free_memory())),
        ]),
    ])
    .block(Block::default().borders(Borders::NONE))
    .widths(&[Constraint::Length(25), Constraint::Length(25)])
    .column_spacing(10);

    f.render_widget(left_table, left_table_area);

    // Right block
    let right_table = Table::new(vec![
        Row::new(vec![
            Cell::from("Alloc Probability"),
            Cell::from(format!("{:.2}", app.sim_params.probs.prob_alloc)),
        ]),
        Row::new(vec![
            Cell::from("Read Probability"),
            Cell::from(format!("{:.2}", app.sim_params.probs.prob_read)),
        ]),
        Row::new(vec![
            Cell::from("Write Probability"),
            Cell::from(format!("{:.2}", app.sim_params.probs.prob_write)),
        ]),
        Row::new(vec![
            Cell::from("GC Probability"),
            Cell::from(format!("{:.2}", app.sim_params.probs.prob_gc)),
        ]),
    ])
    .block(Block::default().borders(Borders::NONE))
    .widths(&[Constraint::Length(25), Constraint::Length(25)])
    .column_spacing(10);

    f.render_widget(right_table, right_table_area);
}
