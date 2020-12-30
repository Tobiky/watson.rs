use watson_rs_core::{instructions::Instruction, types::TokenType};
use operations::*;

use crate::operations;

// Find some way to automate the match arms
pub fn instructions_to_objects(instructions: Vec<Instruction>) -> Vec<TokenType> {
    let mut object_stack: Vec<TokenType> = Vec::new();

    instructions.iter()
                .rev()
                .for_each(|instruction| {
                    (match instruction {
                        Instruction::Inew => { inew }
                        Instruction::Iinc => { iinc }
                        Instruction::Ishl => { ishl }
                        Instruction::Iadd => { iadd }
                        Instruction::Ineg => { ineg }
                        Instruction::Isht => { isht }
                        Instruction::Itof => { itof }
                        Instruction::Itou => { itou }
                        Instruction::Finf => { finf }
                        Instruction::Fnan => { fnan }
                        Instruction::Fneg => { fneg }
                        Instruction::Snew => { snew }
                        Instruction::Sadd => { sadd }
                        Instruction::Onew => { onew }
                        Instruction::Oadd => { oadd }
                        Instruction::Anew => { anew }
                        Instruction::Aadd => { aadd }
                        Instruction::Bnew => { bnew }
                        Instruction::Bneg => { bneg }
                        Instruction::Nnew => { nnew }
                        Instruction::Gdup => { gdup }
                        Instruction::Gpop => { gpop }
                        Instruction::Gswp => { gswp }
                    })(&mut object_stack);
                });

    object_stack
}