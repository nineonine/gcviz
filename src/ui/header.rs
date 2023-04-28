use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<'_, B>, area: Rect) {
    let hdr = Paragraph::new("Press `Esc`, `Ctrl-C` or `q` to quit.".to_string())
        .block(
            Block::default()
                .title("Garbage Collection Visualization Environment")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        )
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);
    f.render_widget(hdr, area);
}
