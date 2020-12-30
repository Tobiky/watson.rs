use watson_rs_core::{instructions::Instruction, state::State};

use crate::{error::Error, evaluator::Evaluator, scanner::Scanner};


pub struct Lexer {
    instructions: Vec<Instruction>,
    state: State,
    evaluator: Evaluator,
    scanner: Scanner
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
            scanner
        }
    }

    pub fn pop(&mut self) -> Option<Instruction> {
        self.instructions.pop()
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    pub fn instructions(self) -> Vec<Instruction> {
        self.instructions
    }

    pub fn switch_mode(&mut self) {
        self.state.next_mode();
        self.evaluator = Evaluator::new(self.state.mode());
        self.scanner = Scanner::new(self.state.mode());
    }

    fn generate_error_with_message(self, message: String) -> Result<Vec<Instruction>, Error> {
        Err(Error::with_info(self.state, message))
    }

    // TODO: Make a generator based on
    // TODO: Abstract to stream of LexemeType
    pub fn tokenize<T: std::io::BufRead>(mut self, input: T) -> Result<Vec<Instruction>, Error> {
        for line in input.lines() {
            if let Ok(line_str) = line {
                for lexeme in line_str.chars() {
                    if self.scanner.is_lexeme(lexeme) {
                        let token = self.evaluator.evaluate(lexeme);
                        self.push(token);
                        
                        if token == Instruction::Snew {
                            
                            self.switch_mode();
                        }
                    } else {
                        return self.generate_error_with_message(String::from("sequence is not valid for lexeme mode"));
                    }
                    
                    self.state.increment_column();
                }
            } else {
                return self.generate_error_with_message(line.unwrap_err().to_string());
            }

            self.state.increment_line();
            self.state.reset_column();
        }

        Ok(self.instructions())
    }
}
