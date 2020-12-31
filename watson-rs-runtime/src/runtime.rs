use watson_rs_core::{instructions::Instruction, types::Type};

use crate::operations::*;

// TODO: Make list of instructions into iterator to generalize
#[derive(Debug)]
pub struct Runtime {
    instructions: Vec<Instruction>,
    stack: Vec<Type>
}

impl Runtime {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Runtime {
            instructions,
            stack: Vec::new()
        }
    }

    // TODO: Find some way to automate the match arms
    #[allow(unreachable_patterns)]
    pub fn execute(stack: &mut Vec<Type>, instruction: Instruction ) {
        (match instruction {
            Instruction::Inew => inew,
            Instruction::Iinc => iinc,
            Instruction::Ishl => ishl,
            Instruction::Iadd => iadd,
            Instruction::Ineg => ineg,
            Instruction::Isht => isht,
            Instruction::Itof => itof,
            Instruction::Itou => itou,
            Instruction::Finf => finf,
            Instruction::Fnan => fnan,
            Instruction::Fneg => fneg,
            Instruction::Snew => snew,
            Instruction::Sadd => sadd,
            Instruction::Onew => onew,
            Instruction::Oadd => oadd,
            Instruction::Anew => anew,
            Instruction::Aadd => aadd,
            Instruction::Bnew => bnew,
            Instruction::Bneg => bneg,
            Instruction::Nnew => nnew,
            Instruction::Gdup => gdup,
            Instruction::Gpop => gpop,
            Instruction::Gswp => gswp,
            _ => {
                panic!("attempt at using unsupported instruction")
            }
        })(stack);
    }

    pub fn materialize(self) -> Vec<Type> {
        let instructions = self.instructions;
        let mut stack = self.stack;

        instructions.iter().for_each(|&instruction| Runtime::execute(&mut stack, instruction));

        stack
    }
}
