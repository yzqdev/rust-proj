use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Direction, Constraint},
    style::{Style, Color},
};
use crossterm::event::{KeyCode, KeyEventKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut counter = 0;
    let mut step = 1;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)])
                .split(f.size());

            let counter_text = format!("Counter: {}", counter);
            let info_text = format!("Step: {} | j/k: +/- | 1-9: set step | r: reset | q: quit", step);

            let counter_widget = Paragraph::new(counter_text)
                .block(Block::default().title(" Counter ").borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan));
            let info_widget = Paragraph::new(info_text)
                .block(Block::default().title(" Controls ").borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));

            f.render_widget(counter_widget, chunks[0]);
            f.render_widget(info_widget, chunks[1]);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('j') => counter += step,
                        KeyCode::Char('k') => counter -= step,
                        KeyCode::Char('r') => counter = 0,
                        KeyCode::Char('q') => break,
                        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                            step = c.to_digit(10).unwrap() as i32;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
