use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Title bar
    frame.render_widget(
        Paragraph::new(format!(" {} - Press 'q' to quit ", app.title))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center),
        chunks[0],
    );

    // Counter section
    let counter_text = format!("Counter: {}  |  ← → to change  |  r to reset", app.counter);
    let _progress = f64::from(app.counter) / 10.0;
    frame.render_widget(
        Paragraph::new(counter_text)
            .block(
                Block::default()
                    .title(" Counter ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow)),
        chunks[1],
    );

    // Item list
    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!(" {} ", item)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Items (↑ ↓ to navigate) ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(list, chunks[2]);

    // Help bar
    frame.render_widget(
        Paragraph::new(" q: quit | ← →: counter | ↑ ↓: select | r: reset | Esc: quit ")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Gray)),
        chunks[3],
    );
}
