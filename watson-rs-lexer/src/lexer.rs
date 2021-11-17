use std::collections::HashSet;

use watson_rs_core::{instruction::Instruction, lexeme::{lexeme_sequences, LexemeType}, mode::Mode};

pub struct Lexer {
    state: Mode,
    lexemes: HashSet<LexemeType>,
}

impl Lexer {
    pub fn new(mode: Mode) -> Self {
        let lexemes = lexeme_sequences(mode).iter().cloned().collect();

        Lexer { lexemes }
    }

    pub fn is_lexeme(&self, lexeme: LexemeType) -> bool {
        self.lexemes.contains(&lexeme)
    }

    pub fn feed(&self, character: char) -> Result<Instruction, Error> {
        for (bind_character, bind_instruction) in bindings(self.mode) {
            if bind_character == character {
                return bind_instruction;
            }
        }
        // TODO: Improve error message
        return Error::new("No lexeme using that character in the current mode.");
    }
}
