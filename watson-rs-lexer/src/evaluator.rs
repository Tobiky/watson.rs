use std::collections::HashMap;

use watson_rs_core::{LexemeType, instructions::Instruction, lexeme_sequences, mode::Mode};

pub struct Evaluator {
    bindings: HashMap<LexemeType, Instruction>,
}

impl Evaluator {
    pub fn new(mode: Mode) -> Self {
        let lexemes = lexeme_sequences(mode);
        let instructions = Instruction::create_vector();
        let bindings = lexemes
            .iter()
            .cloned()
            .zip(instructions.iter())
            .map(|(lexe, &inst)| (lexe, inst))
            .collect();

        Evaluator {
            bindings,
        }
    }

    pub fn evaluate(&self, lexeme: LexemeType) -> Instruction {
        self.bindings[&lexeme]
    }
}
