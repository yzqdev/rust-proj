//! counter - ratatui counter TUI demo.
//!
//! The key handling / state logic lives here so it can be unit tested
//! without a real terminal.

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
    pub step: i32,
}

impl Default for State {
    fn default() -> Self {
        State {
            counter: 0,
            step: 1,
        }
    }
}

impl State {
    /// Apply a single pressed character key.
    ///
    /// `j` / `k` add / subtract the step, `r` resets, `q` quits,
    /// digits 1-9 set the step.
    pub fn on_char(&mut self, c: char) -> Action {
        match c {
            'j' => self.counter += self.step,
            'k' => self.counter -= self.step,
            'r' => self.counter = 0,
            'q' => return Action::Quit,
            c if c.is_ascii_digit() && c != '0' => {
                self.step = c.to_digit(10).unwrap_or(1) as i32;
            }
            _ => {}
        }
        Action::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jk_change_counter_by_step() {
        let mut state = State::default();
        state.on_char('j');
        assert_eq!(state.counter, 1);
        state.on_char('k');
        assert_eq!(state.counter, 0);
        state.on_char('5');
        state.on_char('j');
        assert_eq!(state.counter, 5);
    }

    #[test]
    fn digit_sets_step() {
        let mut state = State::default();
        assert_eq!(state.on_char('7'), Action::Continue);
        assert_eq!(state.step, 7);
        assert_eq!(state.on_char('0'), Action::Continue);
        assert_eq!(state.step, 7, "0 must not change the step");
    }

    #[test]
    fn r_resets_counter() {
        let mut state = State {
            counter: 42,
            step: 3,
        };
        state.on_char('r');
        assert_eq!(state.counter, 0);
        assert_eq!(state.step, 3);
    }

    #[test]
    fn q_quits() {
        let mut state = State::default();
        assert_eq!(state.on_char('q'), Action::Quit);
    }
}
