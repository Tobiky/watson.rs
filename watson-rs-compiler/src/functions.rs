use std::collections::HashMap;

#[cfg(not(feature = "ascii"))]
use ascii::AsciiChar;

use watson_rs_core::{
    instruction::Instruction,
    types::{Type, WString},
};

// TODO: Optimize using Isht and Gdup
pub fn shift_int(instructions: &mut Vec<Instruction>, amount: u64) {
    if amount == 0 {
        return;
    }

    for _ in 0..amount {
        instructions.push(Instruction::Ishl);
    }
}

// TODO: Optimize using Isht and Gdup
pub fn create_uint(instructions: &mut Vec<Instruction>, uint: &u64) {
    let value = *uint;

    if value == 0 {
        return instructions.push(Instruction::Inew);
    }

    let mut ints_to_combine = 0;

    for idx in 63..=0 {
        if ((uint >> idx) & !0x1) == 1 {
            instructions.push(Instruction::Inew);
            instructions.push(Instruction::Iinc);
            shift_int(instructions, idx);
            ints_to_combine += 1;
        }
    }

    // -1 because we are combining in pairs
    for _ in 0..(ints_to_combine - 1) {
        instructions.push(Instruction::Iadd);
    }

    instructions.push(Instruction::Itou);
}

// TODO: Clean up
pub fn create_int(instructions: &mut Vec<Instruction>, int: &i64) {
    if *int < 0 {
        let value = -*int as u64;
        create_uint(instructions, &value);
        // Remove the Itou instruction at the end
        instructions.pop();
        instructions.push(Instruction::Ineg);
    } else {
        let value = *int as u64;
        create_uint(instructions, &value);
        instructions.pop();
    }
}

pub fn create_bool(instructions: &mut Vec<Instruction>, boolean: &bool) {
    instructions.push(Instruction::Bnew);

    if !boolean {
        instructions.push(Instruction::Bneg);
    }
}

#[cfg(not(feature = "ascii"))]
fn ascii_of(character: char) -> u8 {
    AsciiChar::new(character).as_byte()
}

#[cfg(not(feature = "ascii"))]
pub fn create_ascii(instructions: &mut Vec<Instruction>, character: char) {
    let int = ascii_of(character) as i64;
    create_int(instructions, &int);
}

#[cfg(not(feature = "ascii"))]
pub fn create_string(instructions: &mut Vec<Instruction>, string: &str) {
    string
        .chars()
        .for_each(|character| create_ascii(instructions, character));
}

#[cfg(feature = "ascii")]
pub fn create_string(instructions: &mut Vec<Instruction>, string: &Vec<u8>) {
    instructions.push(Instruction::Snew);

    for &character in string {
        let int = character as i64;
        create_int(instructions, &int);
        instructions.push(Instruction::Sadd);
    }
}

pub fn create_float(instructions: &mut Vec<Instruction>, float: &f64) {
    let bits = float.to_bits();
    let int = bits as i64;

    create_int(instructions, &int);

    instructions.push(Instruction::Itof)
}

pub fn create_object(instructions: &mut Vec<Instruction>, object: &HashMap<WString, Type>) {
    instructions.push(Instruction::Onew);

    for (string, type_object) in object {
        create_string(instructions, string);
        create_type(instructions, type_object);
        instructions.push(Instruction::Oadd);
    }
}

fn create_array(instructions: &mut Vec<Instruction>, array: &Vec<Type>) {
    instructions.push(Instruction::Anew);

    for type_object in array {
        create_type(instructions, type_object);
        instructions.push(Instruction::Aadd);
    }
}

pub fn create_type(instructions: &mut Vec<Instruction>, object: &Type) {
    match object {
        Type::Int(value) => create_int(instructions, value),
        Type::Uint(value) => create_uint(instructions, value),
        Type::Float(float) => create_float(instructions, float),
        Type::String(string) => create_string(instructions, string),
        Type::Object(object) => create_object(instructions, object),
        Type::Array(array) => create_array(instructions, array),
        Type::Bool(boolean) => create_bool(instructions, boolean),
        Type::Nil => instructions.push(Instruction::Nnew),
    }
}
