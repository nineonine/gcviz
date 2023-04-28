use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
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

    f.render_widget(run_pause_button(app), footer_chunks[0]);
    f.render_widget(next_button(), footer_chunks[1]);
    f.render_widget(restart_button(), footer_chunks[2]);
    f.render_widget(quit_button(), footer_chunks[3]);
}

fn create_button<'a>(label: &'a str, key: &'a str) -> Paragraph<'a> {
    let button_text = vec![
        Span::raw(format!("{label} ")),
        Span::styled(
            format!("({key})"),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    Paragraph::new(Spans::from(button_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center)
}

fn run_pause_button(app: &App) -> Paragraph {
    let label = if app.program_paused { "Run" } else { "Pause" };
    create_button(label, "Space")
}

fn next_button<'a>() -> Paragraph<'a> {
    create_button("Step", "S")
}

fn restart_button<'a>() -> Paragraph<'a> {
    create_button("Restart", "R")
}

fn quit_button<'a>() -> Paragraph<'a> {
    create_button("Quit", "Q")
}
