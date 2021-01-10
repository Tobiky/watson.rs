use std::f64::{INFINITY, NAN};

use watson_rs_core::types::Type;

use super::POP_EMPTY_STACK_ERROR;

pub fn finf(stack: &mut Vec<Type>) -> Result<(), &str> {
    Ok(stack.push(Type::Float(INFINITY)))
}

pub fn fnan(stack: &mut Vec<Type>) -> Result<(), &str> {
    Ok(stack.push(Type::Float(NAN)))
}

pub fn fneg(stack: &mut Vec<Type>) -> Result<(), &str> {
    let token = if let Some(value) = stack.pop() {
        value 
    } else {
        return Err(POP_EMPTY_STACK_ERROR);
    };

    if let Type::Float(x) = token {
        return Ok(stack.push(Type::Float(-x)));
    }

    Err("tried popping non-Float object from stack")
}
