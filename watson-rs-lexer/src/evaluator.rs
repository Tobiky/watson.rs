use std::collections::HashMap;

use watson_rs_core::{instructions::Instruction, bindings, mode::Mode, LexemeType};

pub struct Evaluator {
    bindings: HashMap<LexemeType, Instruction>,
}

impl Evaluator {
    pub fn new(mode: Mode) -> Self {
        Evaluator { bindings: bindings(mode).iter().cloned().collect() }
    }

    pub fn evaluate(&self, lexeme: LexemeType) -> Instruction {
        self.bindings[&lexeme]
    }
}
