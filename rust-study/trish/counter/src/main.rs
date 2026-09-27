use std::process::ExitCode;

use counter::{Action, State};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let result = event_loop();

    // Always restore the terminal, even after an error.
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    result
}

fn event_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut state = State::default();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ])
                .split(f.area());

            let counter_text = format!("Counter: {}", state.counter);
            let info_text = format!(
                "Step: {} | j/k: +/- | 1-9: set step | r: reset | q: quit",
                state.step
            );

            let counter_widget = Paragraph::new(counter_text)
                .block(Block::default().title(" Counter ").borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan));
            let info_widget = Paragraph::new(info_text)
                .block(Block::default().title(" Controls ").borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));

            f.render_widget(counter_widget, chunks[0]);
            f.render_widget(info_widget, chunks[1]);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))?
            && let crossterm::event::Event::Key(key) = crossterm::event::read()?
            && key.kind == KeyEventKind::Press
            && let KeyCode::Char(c) = key.code
            && state.on_char(c) == Action::Quit
        {
            return Ok(());
        }
    }
}
