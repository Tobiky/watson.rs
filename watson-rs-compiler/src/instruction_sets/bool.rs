use watson_rs_core::types::Type;

use super::POP_EMPTY_STACK_ERROR;

pub fn bnew(stack: &mut Vec<Type>) -> Result<(), &str> {
    Ok(stack.push(Type::Bool(false)))
}

pub fn bneg(stack: &mut Vec<Type>) -> Result<(), &str>{
    let token = if let Some(value) = stack.pop() {
        value 
    } else {
        return Err(POP_EMPTY_STACK_ERROR);
    };

    if let Type::Bool(x) = token {
        return Ok(stack.push(Type::Bool(!x)));
    }

    Err("tried popping non-Bool object from stack")
}
