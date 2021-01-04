use std::collections::HashMap;

use watson_rs_core::{
    instruction::Instruction,
    lexeme::{bindings, LexemeType},
    mode::Mode,
};

pub struct Evaluator {
    bindings: HashMap<LexemeType, Instruction>,
}

impl Evaluator {
    pub fn new(mode: Mode) -> Self {
        Evaluator {
            bindings: bindings(mode).iter().cloned().collect(),
        }
    }

    pub fn evaluate(&self, lexeme: LexemeType) -> Instruction {
        self.bindings[&lexeme]
    }
}
