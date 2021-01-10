use std::collections::HashMap;

use watson_rs_core::{error::Error, instruction::Instruction, lexeme::all_bindings, mode::Mode, state::State, types::Type};

use crate::{functions::create_type, Compiler};

#[derive(Debug, Clone)]
pub struct WatsonCompiler {
    stack: Vec<Type>,
}

impl WatsonCompiler {
    pub fn new(stack: Vec<Type>) -> WatsonCompiler {
        WatsonCompiler { stack }
    }
}

impl Compiler for WatsonCompiler {
    type Out = Vec<Instruction>;

    fn compile(self) -> Result<Self::Out, Error> {
        Ok(self.stack
            .iter()
            .fold(Vec::new(), |mut instructions, type_object| {
                create_type(&mut instructions, type_object);
                instructions
            }))
    }
}

// TODO: Use generator instead
impl WatsonCompiler {
    pub fn string_compile(self) -> Result<String, Error> {
        let instructions = self.compile()?;
        let mut state = State::new();

        let text_bindings = all_bindings()
            .iter()
            .map(|(mode, bindings)| {
                (
                    *mode,
                    bindings
                        .iter()
                        .map(|&(lexeme, instruction)| (instruction, lexeme))
                        .collect(),
                )
            })
            .collect::<HashMap<Mode, HashMap<Instruction, char>>>();

        Ok(instructions
            .iter()
            .fold(String::new(), |mut string, instruction| {
                string.push(text_bindings[&state.mode()][instruction]);

                if *instruction == Instruction::Snew {
                    state.next_mode();
                }

                string
            }))
    }
}
