use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{app::App, mem::MemoryGrid};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, f: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/tui-rs-revival/ratatui/tree/master/examples
    f.render_widget(
        Paragraph::new("Press `Esc`, `Ctrl-C` or `q` to quit.".to_string())
            .block(
                Block::default()
                    .title("Garbage Collection Visualization Environment")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        f.size(),
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let block = Block::default().title("Header").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    // inner
    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(1)
        .horizontal_margin(2)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let memory_grid = MemoryGrid::new(app.mem.clone());
    f.render_widget(memory_grid, inner_chunks[2]);

    let block = Block::default().title("Footer").borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
}
