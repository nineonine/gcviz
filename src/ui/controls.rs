use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

use crate::app::App;

pub fn render<B: Backend>(_app: &App, f: &mut Frame<'_, B>, area: Rect) {
    let control_panel = Block::default().title("Controls").borders(Borders::ALL);
    f.render_widget(control_panel, area);
}
