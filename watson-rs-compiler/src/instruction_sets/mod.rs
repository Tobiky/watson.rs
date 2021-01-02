use watson_rs_core::{instructions::Instruction, types::Type};

pub(crate) mod float;
pub(crate) mod string;
pub(crate) mod integer;
pub(crate) mod object;
pub(crate) mod array;
pub(crate) mod bool;
pub(crate) mod global;
pub(crate) mod nil;

use self::{float::*, string::*, integer::*, object::*, array::*, bool::*, global::*, nil::*};

// TODO: Find some way to automate the match arms
#[allow(unreachable_patterns)]
pub fn get_instruction(instruction: Instruction) -> impl Fn(&mut Vec<Type>) -> () {
    match instruction {
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
    }
}