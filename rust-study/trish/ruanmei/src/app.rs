use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// Application title
    pub title: String,
    /// Items list
    pub items: Vec<String>,
    /// Selected item index
    pub selected: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            title: String::from("Ruanmei TUI App"),
            items: vec![
                String::from("Item 1 - Hello"),
                String::from("Item 2 - World"),
                String::from("Item 3 - Rust"),
                String::from("Item 4 - TUI"),
                String::from("Item 5 - Ratatui"),
            ],
            selected: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            ..Self::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    /// Select next item
    pub fn next_item(&mut self) {
        self.selected = self
            .selected
            .saturating_add(1)
            .min(self.items.len().saturating_sub(1));
    }

    /// Select previous item
    pub fn previous_item(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Reset counter
    pub fn reset_counter(&mut self) {
        self.counter = 0;
    }
}
