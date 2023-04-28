use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render<B: Backend>(app: &App, f: &mut Frame<'_, B>, area: Rect) {
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
        .split(area);

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
