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

    let hdr = Paragraph::new("Press `Esc`, `Ctrl-C` or `q` to quit.".to_string())
        .block(
            Block::default()
                .title("Garbage Collection Visualization Environment")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        )
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);
    f.render_widget(hdr, chunks[0]);

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let left_panel = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(inner_chunks[0]);

    let control_panel = Block::default().title("Controls").borders(Borders::ALL);
    f.render_widget(control_panel, left_panel[0]);

    let logs: Vec<ListItem> = app
        .logs
        .iter()
        .rev() // Reverse the iterator to start from the most recent log
        .enumerate() // Enumerate to get the index
        .filter(|&(i, _)| {
            // Calculate how many lines are available for logs
            let available_lines = left_panel[1].height as usize - 2; // Subtract 2 for the borders

            // Only include the logs that fit into the available space
            i < available_lines
        })
        .map(|(_, log)| {
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
    f.render_widget(logs_list, left_panel[1]);

    let memory_grid = HeapGrid::new(app.memviz.clone());
    f.render_widget(memory_grid, inner_chunks[1]);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    let run_pause_text = if app.program_paused { "Run" } else { "Pause" };
    let run_pause_button = Paragraph::new(run_pause_text.to_string())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);

    let next_button = Paragraph::new("Next")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);

    let restart_button = Paragraph::new("Restart")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);

    let quit_button = Paragraph::new("Quit")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);

    f.render_widget(run_pause_button, footer_chunks[0]);
    f.render_widget(next_button, footer_chunks[1]);
    f.render_widget(restart_button, footer_chunks[2]);
    f.render_widget(quit_button, footer_chunks[3]);
}
