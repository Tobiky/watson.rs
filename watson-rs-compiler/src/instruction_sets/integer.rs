use watson_rs_core::types::Type;

use super::POP_EMPTY_STACK_ERROR;

pub fn inew(stack: &mut Vec<Type>) -> Result<(), &str> {
    Ok(stack.push(Type::Int(0)))
}

pub fn iinc(stack: &mut Vec<Type>) -> Result<(), &str> {
    let token = if let Some(value) = stack.pop() {
        value
    } else {
        return Err(POP_EMPTY_STACK_ERROR);
    };

    if let Type::Int(x) = token {
        return Ok(stack.push(Type::Int(x + 1)));
    }

    Err("tried popping non-Int object from stack")
}

pub fn ishl(stack: &mut Vec<Type>) -> Result<(), &str> {
    let token = if let Some(value) = stack.pop() {
        value 
    } else {
        return Err(POP_EMPTY_STACK_ERROR);
    };

    if let Type::Int(x) = token {
        return Ok(stack.push(Type::Int(x << 1)));
    }

    Err("tried popping non-Int object from stack")
}

pub fn iadd(stack: &mut Vec<Type>) -> Result<(), &str>{
    let option_token_x = stack.pop();
    let option_token_y = stack.pop();

    if option_token_x.is_none() || option_token_y.is_none() {
        return Err(POP_EMPTY_STACK_ERROR);
    }

    let token_y = option_token_x.unwrap();
    let token_x = option_token_y.unwrap();

    if let Type::Int(x) = token_x {
        if let Type::Int(y) = token_y {
            return Ok(stack.push(Type::Int(x + y)));
        }
    }

    Err("tried popping non-Int object from stack")
}

pub fn ineg(stack: &mut Vec<Type>) -> Result<(), &str>{
    let token = if let Some(value) = stack.pop() {
        value 
    } else {
        return Err(POP_EMPTY_STACK_ERROR);
    };

    if let Type::Int(x) = token {
        return Ok(stack.push(Type::Int(-x)));
    }

    Err("tried popping non-Int object from stack")
}

pub fn isht(stack: &mut Vec<Type>) -> Result<(), &str> {
    let option_token_x = stack.pop();
    let option_token_y = stack.pop();

    if option_token_x.is_none() || option_token_y.is_none() {
        return Err(POP_EMPTY_STACK_ERROR);
    }

    let token_y = option_token_x.unwrap();
    let token_x = option_token_y.unwrap();

    if let Type::Int(x) = token_x {
        if let Type::Int(y) = token_y {
            return Ok(stack.push(Type::Int(x << y)));
        }
    }

    Err("tried popping non-Int object from stack")
}

pub fn itof(stack: &mut Vec<Type>) -> Result<(), &str> {
    let token = if let Some(value) = stack.pop() {
        value 
    } else {
        return Err(POP_EMPTY_STACK_ERROR);
    };

    if let Type::Int(x) = token {
        return Ok(stack.push(Type::Float(x as f64)));
    }

    Err("tried popping non-Int object from stack")
}

pub fn itou(stack: &mut Vec<Type>) -> Result<(), &str> {
    let token = if let Some(value) = stack.pop() {
        value 
    } else {
        return Err(POP_EMPTY_STACK_ERROR);
    };

    if let Type::Int(x) = token {
        return Ok(stack.push(Type::Uint(x as u64)));
    }

    Err("tried popping non-Int object from stack")
}
