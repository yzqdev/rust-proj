use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // State
    let mut counter: i32 = 0;
    let mut selected: usize = 0;
    let items = vec![
        "Rust", "Go", "Python", "TypeScript", "Java", "C++", "Zig",
    ];

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(4),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ])
                .split(frame.size());

            // Title
            frame.render_widget(
                Paragraph::new("Seele Interactive TUI")
                    .white()
                    .on_blue()
                    .bold(),
                chunks[0],
            );

            // Counter with progress bar
            let progress = ((counter as f64).clamp(0.0, 100.0)) / 100.0;
            let gauge = Gauge::default()
                .block(Block::default().title(" Counter ").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Cyan))
                .percent((progress * 100.0) as u16)
                .label(format!("Value: {} (j/k or +/- to change)", counter));
            frame.render_widget(gauge, chunks[1]);

            // Language list
            let list_items: Vec<ListItem> = items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == selected {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(format!(" {} {}", if i == selected { ">" } else { " " }, item))
                        .style(style)
                })
                .collect();

            let list = List::new(list_items).block(
                Block::default()
                    .title(" Languages (↑/↓ to select) ")
                    .borders(Borders::ALL),
            );
            frame.render_widget(list, chunks[2]);

            // Help
            frame.render_widget(
                Paragraph::new(" q: quit | j/k: counter +/- | ↑/↓: select | r: reset ")
                    .gray(),
                chunks[3],
            );
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('j') => counter += 1,
                        KeyCode::Char('k') => counter -= 1,
                        KeyCode::Char('r') => counter = 0,
                        KeyCode::Char('+') | KeyCode::Char('=') => counter += 10,
                        KeyCode::Char('-') | KeyCode::Char('_') => counter -= 10,
                        KeyCode::Up => {
                            selected = selected.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            if selected + 1 < items.len() {
                                selected += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
