use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Corner, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{app::App, heap::HeapGrid};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, f: &mut Frame<'_, B>) {
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
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let logs: Vec<ListItem> = app
        .logs
        .iter()
        .map(|log| {
            let s = log.style();
            let frame_id = match log.frame_id {
                None => String::new(),
                Some(i) => format!("{i}. "),
            };
            let contents = vec![Span::styled(
                format!("{frame_id}[{:?}] {}", log.source, log.msg),
                s,
            )];
            ListItem::new(vec![
                Spans::from("-".repeat(chunks[1].width as usize)),
                Spans::from(contents),
            ])
        })
        .collect();

    let logs_list = List::new(logs)
        .block(Block::default().borders(Borders::ALL).title("Events"))
        .start_corner(Corner::BottomLeft);
    f.render_widget(logs_list, inner_chunks[0]);

    let memory_grid = HeapGrid::new(app.memviz.clone());
    f.render_widget(memory_grid, inner_chunks[1]);

    let block = Block::default().title("Footer").borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
}
