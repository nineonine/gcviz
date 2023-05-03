use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use self::heap::HeapGrid;
use crate::app::App;

mod events;
mod footer;
mod header;
pub mod heap;
mod info;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, f: &mut Frame<'_, B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(85),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    header::render(f, chunks[0]);

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let left_panel = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(inner_chunks[0]);

    info::render(app, f, left_panel[0]);
    events::render(app, f, left_panel[1]);

    let memory_grid = HeapGrid::new(app.vm.heap.memory.clone());
    f.render_widget(memory_grid, inner_chunks[1]);

    footer::render(app, f, chunks[2]);
}
