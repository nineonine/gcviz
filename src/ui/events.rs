// Import necessary modules and types

use ratatui::{
    backend::Backend,
    layout::{Corner, Rect},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

pub fn render<B: Backend>(app: &App, f: &mut Frame<'_, B>, area: Rect) {
    let logs: Vec<ListItem> = app
        .logs
        .iter()
        .rev() // Reverse the iterator to start from the most recent log
        .enumerate() // Enumerate to get the index
        .filter(|&(i, _)| {
            // Calculate how many lines are available for logs
            let available_lines = area.height as usize - 2; // Subtract 2 for the borders

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
                Spans::from("-".repeat(area.width as usize)),
                Spans::from(contents),
            ])
        })
        .collect();

    let logs_list = List::new(logs)
        .block(Block::default().borders(Borders::ALL).title("Events"))
        .start_corner(Corner::BottomLeft);
    f.render_widget(logs_list, area);
}
