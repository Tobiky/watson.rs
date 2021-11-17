use watson_rs_core::{error::Error, instruction::Instruction, state::State};

use crate::lexer::Lexer;


pub struct Scanner {
    instructions: Vec<Instruction>,
    state: State,
    lexer: Lexer,
}

impl Scanner {
    pub fn new() -> Self {
        let state = State::new();
        let lexer = Lexer::new(state.mode());

        Scanner {
            instructions: Vec::new(),
            state,
            lexer,
        }
    }

    pub fn pop(&mut self) -> Option<Instruction> {
        self.instructions.pop()
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    pub fn instructions(&self) -> Vec<Instruction> {
        self.instructions.clone()
    }

    pub fn next_mode(&mut self) {
        self.state.next_mode();
        self.lexer = Lexer::new(self.state.mode());
    }

    fn generate_error_with_message(&self, message: String) -> Result<Vec<Instruction>, Error> {
        Err(Error::with_info(self.state, message))
    }

    fn feed(&mut self, symbol: char) -> Result<(), Error>
    {
        if symbol.is_whitespace() || symbol.is_ascii_whitespace() {
            return ();
        }

        if self.lexer.is_lexeme(symbol) {
            let token = self.lexer.feed(symbol)?;

            self.push(token);

            if token == Instruction::Snew {
                self.next_mode();
            }
        } else {
            return self.generate_error_with_message(format!(
                "`{}`sequence is not valid for lexeme mode", character
            ));
        }
    }

    // TODO: ASCII version
    // TODO: Not-mutable version
    pub fn tokenize_str(&mut self, input: &str) -> Result<Vec<Instruction>, Error> {
        for character in input.chars() {
            self.feed(character)?;
            self.state.increment_column();
        }
        Ok(self.instructions.clone())
    }

    // TODO: Make this function return a generator
    // TODO: Abstract input to stream of LexemeType
    pub fn tokenize<T: std::io::BufRead>(&mut self, input: T) -> Result<Vec<Instruction>, Error> {
        for line_result in input.lines() {
            if let Ok(line) = line_result {
                let _ = self.tokenize_str(line.as_str())?;
            } else {
                return self.generate_error_with_message(line_result.unwrap_err().to_string());
            }

            self.state.increment_line();
            self.state.reset_column();
        }

        let instructions = self.instructions();
        Ok(instructions)
    }
}
