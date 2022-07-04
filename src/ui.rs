use crate::app;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &app::App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    //    if app.files.len() > app.window_size.1.into() {
    //        files = app.files[(app.files.len() - usize::from(app.window_size.1))..]
    //            .iter()
    //            .enumerate()
    //            .filter(|(i, _)| i > &40)
    //            .map(|(i, m)| {
    //                let content = vec![Spans::from(Span::raw(format!(
    //                    "{}: {} + {} + {}",
    //                    i,
    //                    m,
    //                    app.files.len(),
    //                    app.window_size.1
    //                )))];
    //                ListItem::new(content)
    //            })
    //            .collect();
    //    } else {
    //        files = app
    //            .files
    //            .iter()
    //            .enumerate()
    //            .filter(|(i, _)| i > &1)
    //            .map(|(i, m)| {
    //                let content = vec![Spans::from(Span::raw(format!(
    //                    "{}: {} + {} + {}",
    //                    i,
    //                    m,
    //                    app.files.len(),
    //                    app.window_size.1,
    //                )))];
    //                ListItem::new(content)
    //            })
    //            .collect();
    //    }
    let files: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            if app.files.len() > usize::from(app.window_size.1 - 5) {
                i >= &(app.files.len() - usize::from(app.window_size.1 - 5))
            } else {
                true
            }
        })
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!(
                "{}: {} + {} + {}",
                i,
                m,
                app.files.len(),
                app.window_size.1,
            )))];
            ListItem::new(content)
        })
        .collect();
    let files = List::new(files).block(Block::default().borders(Borders::ALL).title("Files"));

    f.render_widget(files, chunks[0]);
}
