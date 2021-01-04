use watson_rs_core::{instruction::Instruction, state::State};

use crate::{error::Error, evaluator::Evaluator, scanner::Scanner};

pub struct Lexer {
    instructions: Vec<Instruction>,
    state: State,
    evaluator: Evaluator,
    scanner: Scanner,
}

impl Lexer {
    pub fn new() -> Self {
        let state = State::new();
        let evaluator = Evaluator::new(state.mode());
        let scanner = Scanner::new(state.mode());

        Lexer {
            instructions: Vec::new(),
            state,
            evaluator,
            scanner,
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
        self.evaluator = Evaluator::new(self.state.mode());
        self.scanner = Scanner::new(self.state.mode());
    }

    fn generate_error_with_message(&self, message: String) -> Result<Vec<Instruction>, Error> {
        Err(Error::with_info(self.state, message))
    }

    // TODO: ASCII version
    // TODO: Not-mutable version
    pub fn tokenize_str(&mut self, input: &str) -> Result<Vec<Instruction>, Error> {
        for lexeme in input.chars() {
            if self.scanner.is_lexeme(lexeme) {
                let token = self.evaluator.evaluate(lexeme);
                self.push(token);

                if token == Instruction::Snew {
                    self.next_mode();
                }
            } else {
                return self.generate_error_with_message(String::from(
                    "sequence is not valid for lexeme mode",
                ));
            }

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
