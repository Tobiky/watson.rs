use watson_rs_core::{instructions::Instruction, types::Type};

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

    fn compile(self) -> Self::Out {
        let instructions = self.instructions;

        instructions
            .iter()
            .fold(Self::Out::new(), |mut stack, &instruction| {
                get_instruction(instruction)(&mut stack);
                stack
            })
    }
}
