use crate::mode::Mode;

pub struct State {
    mode: Mode,
    line: usize,
    column: usize
}

impl State {
    pub fn new() -> Self {
        State {
            mode: Mode::A,
            line: 0,
            column: 0
        }
    }

    pub fn next_mode(&mut self) {
        self.mode = self.mode.next_mode();
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