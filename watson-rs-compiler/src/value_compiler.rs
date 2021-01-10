use state::State;
use watson_rs_core::{error::Error, instruction::Instruction, state, types::Type};

use crate::{Compiler, instruction_sets::get_instruction};

pub struct ValueCompiler {
    instructions: Vec<Instruction>,
}

impl ValueCompiler {
    pub fn new(instructions: Vec<Instruction>) -> ValueCompiler {
        ValueCompiler { instructions }
    }
}

impl Compiler for ValueCompiler {
    type Out = Vec<Type>;

    fn compile(self) -> Result<Self::Out, Error> {
        let instructions = self.instructions;
        let mut stack = Self::Out::new();
        let mut state = State::new(); 

        for instruction in instructions
        {
            if let Err(error_message) = get_instruction(instruction)(&mut stack) {
                return Err(Error::with_info(state, error_message.to_string()));
            }
            state.increment_column();
        }

        Ok(stack)
    }
}
