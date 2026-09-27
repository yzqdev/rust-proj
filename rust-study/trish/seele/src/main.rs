use std::io::stdout;
use std::process::ExitCode;

use crossterm::{
    ExecutableCommand,
    event::{self, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};
use seele::{Action, State};

const ITEMS: [&str; 7] = ["Rust", "Go", "Python", "TypeScript", "Java", "C++", "Zig"];

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let result = event_loop();

    // Always restore the terminal, even after an error.
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    result
}

fn event_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut state = State::new(ITEMS.len());

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
                .split(frame.area());

            // Title
            frame.render_widget(
                Paragraph::new("Seele Interactive TUI")
                    .white()
                    .on_blue()
                    .bold(),
                chunks[0],
            );

            // Counter with progress bar
            let progress = ((state.counter as f64).clamp(0.0, 100.0)) / 100.0;
            let gauge = Gauge::default()
                .block(Block::default().title(" Counter ").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Cyan))
                .percent((progress * 100.0) as u16)
                .label(format!("Value: {} (j/k or +/- to change)", state.counter));
            frame.render_widget(gauge, chunks[1]);

            // Language list
            let list_items: Vec<ListItem> = ITEMS
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == state.selected {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(format!(
                        " {} {}",
                        if i == state.selected { ">" } else { " " },
                        item
                    ))
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
                Paragraph::new(" q: quit | j/k: counter +/- | ↑/↓: select | r: reset ").gray(),
                chunks[3],
            );
        })?;

        if event::poll(std::time::Duration::from_millis(50))?
            && let event::Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Char(c) => {
                    if state.on_char(c) == Action::Quit {
                        return Ok(());
                    }
                }
                KeyCode::Up => state.select_up(),
                KeyCode::Down => state.select_down(),
                _ => {}
            }
        }
    }
}
