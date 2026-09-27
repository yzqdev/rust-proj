//! seele - Interactive ratatui TUI demo.
//!
//! State / key handling lives here so it can be unit tested without a
//! real terminal.

/// What to do after a key press.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Keep running.
    Continue,
    /// Quit the application.
    Quit,
}

/// Application state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    pub counter: i32,
    pub selected: usize,
    pub item_count: usize,
}

impl State {
    /// Build a state for a list with `item_count` entries.
    pub fn new(item_count: usize) -> Self {
        State {
            counter: 0,
            selected: 0,
            item_count,
        }
    }

    /// Apply a single pressed character key.
    pub fn on_char(&mut self, c: char) -> Action {
        match c {
            'q' => return Action::Quit,
            'j' => self.counter += 1,
            'k' => self.counter -= 1,
            'r' => self.counter = 0,
            '+' | '=' => self.counter += 10,
            '-' | '_' => self.counter -= 10,
            _ => {}
        }
        Action::Continue
    }

    /// Move the list selection up, clamped at the first item.
    pub fn select_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Move the list selection down, clamped at the last item.
    pub fn select_down(&mut self) {
        if self.selected + 1 < self.item_count {
            self.selected += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_keys() {
        let mut state = State::new(3);
        state.on_char('j');
        assert_eq!(state.counter, 1);
        state.on_char('k');
        assert_eq!(state.counter, 0);
        state.on_char('+');
        assert_eq!(state.counter, 10);
        state.on_char('-');
        assert_eq!(state.counter, 0);
        state.on_char('j');
        state.on_char('r');
        assert_eq!(state.counter, 0);
    }

    #[test]
    fn q_quits() {
        let mut state = State::new(3);
        assert_eq!(state.on_char('q'), Action::Quit);
    }

    #[test]
    fn selection_is_clamped() {
        let mut state = State::new(3);
        state.select_up();
        assert_eq!(state.selected, 0, "cannot move above the first item");
        state.select_down();
        state.select_down();
        state.select_down();
        assert_eq!(state.selected, 2, "cannot move below the last item");
    }
}
