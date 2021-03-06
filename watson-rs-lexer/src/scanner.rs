use std::collections::HashSet;

use watson_rs_core::{
    lexeme::{lexeme_sequences, LexemeType},
    mode::Mode,
};
pub struct Scanner {
    lexemes: HashSet<LexemeType>,
}

impl Scanner {
    pub fn new(mode: Mode) -> Self {
        let lexemes = lexeme_sequences(mode).iter().cloned().collect();

        Scanner { lexemes }
    }

    pub fn is_lexeme(&self, lexeme: LexemeType) -> bool {
        self.lexemes.contains(&lexeme)
    }
}
