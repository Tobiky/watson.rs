use crate::mode::Mode;

#[derive(Debug, Clone, Copy)]
pub struct State {
    mode: Mode,
    line: usize,
    column: usize,
}

impl State {
    pub fn new() -> Self {
        Self::with_mode(Mode::A)
    }

    pub fn with_mode(mode: Mode) -> Self {
        State {
            mode,
            line: 0,
            column: 0,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn next_mode(&mut self) {
        self.mode = self.mode.next_mode();
    }

    pub fn increment_line(&mut self) {
        self.line += 1;
    }

    pub fn increment_column(&mut self) {
        self.column += 1;
    }

    pub fn reset_line(&mut self) {
        self.line = 0;
    }

    pub fn reset_column(&mut self) {
        self.column = 0;
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::mode::Mode;

    use super::State;

    #[test]
    fn create_state() {
        let state = State::new();
        assert_eq!(state.mode, Mode::A);
        assert_eq!(state.column, 0);
        assert_eq!(state.line, 0);

        let state = State::with_mode(Mode::S);
        assert_eq!(state.mode, Mode::S);
        assert_eq!(state.column, 0);
        assert_eq!(state.line, 0);
    }

    #[test]
    fn switch_modes() {
        let mut state = State::new();
        assert_eq!(state.mode, Mode::A);
        state.next_mode();
        assert_eq!(state.mode, Mode::S);
        state.next_mode();
        assert_eq!(state.mode, Mode::A);
    }

    #[test]
    fn increment() {
        let mut state = State::new();
        assert_eq!(state.column, 0);
        assert_eq!(state.line,   0);

        state.increment_column();
        assert_eq!(state.column, 1);
        assert_eq!(state.line,   0);
        
        state.increment_line();
        assert_eq!(state.column, 1);
        assert_eq!(state.line,   1);
    }

    #[test]
    fn reset() {
        let mut state = State::new();
        state.increment_column();
        state.increment_line();

        state.reset_column();
        assert_eq!(state.column, 0);

        state.reset_line();
        assert_eq!(state.line, 0);
    }
}