use watson_rs_core::types::Type;

use super::POP_EMPTY_STACK_ERROR;

pub fn gdup(stack: &mut Vec<Type>) -> Result<(), &str>{
    let token_x = if let Some(value) = stack.pop() {
        value
    } else {
        return Err(POP_EMPTY_STACK_ERROR);
    };

    stack.push(token_x.clone());
    stack.push(token_x);

    Ok(())
}

pub fn gpop(stack: &mut Vec<Type>) -> Result<(), &str> {
    stack.pop();
    
    Ok(())
}

pub fn gswp(stack: &mut Vec<Type>) -> Result<(), &str>{
    let option_token_y = stack.pop();
    let option_token_x = stack.pop();

    if option_token_x.is_none() || option_token_x.is_none() {
        return Err(POP_EMPTY_STACK_ERROR);
    }

    let token_y = option_token_y.unwrap();
    let token_x = option_token_x.unwrap();

    stack.push(token_y);
    stack.push(token_x);

    Ok(())
}
